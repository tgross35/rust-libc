use askama::Template;
use quote::ToTokens;
use syn::TypePtr;

use crate::ffi_items::FfiItems;
use crate::generator::GenerationError;
use crate::translator::Translator;
use crate::{BoxStr, MapInput, Result, TestGenerator};

/// Represents the Rust side of the generated testing suite.
#[derive(Template, Clone)]
#[template(path = "test.rs")]
pub(crate) struct RustTestTemplate {
    pub template: TestTemplate,
}

impl RustTestTemplate {
    pub fn new(ffi_items: &FfiItems, tg: &TestGenerator) -> Result<Self, GenerationError> {
        Ok(Self {
            template: TestTemplate::new(ffi_items, tg)?,
        })
    }
}

/// Represents the C side of the generated testing suite.
#[derive(Template, Clone)]
#[template(path = "test.c")]
pub(crate) struct CTestTemplate {
    pub template: TestTemplate,
    pub headers: Vec<String>,
}

impl CTestTemplate {
    pub fn new(ffi_items: &FfiItems, tg: &TestGenerator) -> Result<Self, GenerationError> {
        Ok(Self {
            template: TestTemplate::new(ffi_items, tg)?,
            headers: tg.headers.clone(),
        })
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct TestTemplate {
    /// Generate tests for size and alignment
    pub size_align_tests: Vec<TestSizeAlign>,
    pub c_str_tests: Vec<TestCstr>,
    pub const_tests: Vec<TestConst>,
    pub test_idents: Vec<BoxStr>,
}

impl TestTemplate {
    pub fn new(ffi_items: &FfiItems, tg: &TestGenerator) -> Result<Self, GenerationError> {
        let mut this = Self::default();
        let th = TranslateHelper {
            ffi_items,
            generator: tg,
            translator: Translator {},
        };

        for c in &ffi_items.constants {
            if let syn::Type::Ptr(TypePtr { elem, .. }) = &c.ty {
                // && elem is `c_char`
                let item = TestCstr {
                    test_ident: cstr_test_ident(&c.ident),
                    ident: c.ident.clone(),
                    c_ident: th.c_ident(c)?.into(),
                    c_ty: th.c_type(c)?.into(),
                };
                this.c_str_tests.push(item)
            } else {
                let item = TestConst {
                    test_ident: cstr_test_ident(&c.ident),
                    ident: c.ident.clone(),
                    c_ident: th.c_ident(c)?.into(),
                    c_ty: th.c_type(c)?.into(),
                };
                this.const_tests.push(item)
            }
        }

        for alias in &ffi_items.aliases {
            let item = TestSizeAlign {
                test_ident: size_align_test_ident(&alias.ident),
                ident: alias.ident.clone(),
                rust_ty: alias.ident.clone(),
                c_ty: tg.map(alias).unwrap().into(),
            };
            this.size_align_tests.push(item);
        }

        for u in &ffi_items.unions {
            let item = TestSizeAlign {
                test_ident: size_align_test_ident(&u.ident),
                ident: u.ident.clone(),
                rust_ty: u.ident.clone(),
                c_ty: todo!(),
            };
            this.size_align_tests.push(item);
        }

        for st in &ffi_items.structs {
            let item = TestSizeAlign {
                test_ident: size_align_test_ident(&st.ident),
                ident: st.ident.clone(),
                rust_ty: st.ident.clone(),
                c_ty: tg.map(st)?.into(),
            };
            this.size_align_tests.push(item);
        }

        for test in &this.size_align_tests {
            this.test_idents.push(test.test_ident.clone());
        }
        for test in &this.c_str_tests {
            this.test_idents.push(test.test_ident.clone());
        }
        for test in &this.const_tests {
            this.test_idents.push(test.test_ident.clone());
        }

        Ok(this)
    }
}

/// Information needed to provide a size and alignment test.
#[derive(Clone, Debug)]
pub(crate) struct TestSizeAlign {
    pub test_ident: BoxStr,
    /// A unique and valid identifier
    pub ident: BoxStr,
    /// Rust type, possibly with a qualification e.g. "some_mod::foo". For now, this is usually
    /// the same as `ident`.
    pub rust_ty: BoxStr,
    /// C type, possibly with qualification e.g. "struct foo".
    pub c_ty: BoxStr,
}

#[derive(Clone, Debug)]
pub(crate) struct TestCstr {
    pub test_ident: BoxStr,
    pub ident: BoxStr,
    pub c_ident: BoxStr,
    pub c_ty: BoxStr,
}

#[derive(Clone, Debug)]
pub(crate) struct TestConst {
    pub test_ident: BoxStr,
    pub ident: BoxStr,
    pub c_ident: BoxStr,
    pub c_ty: BoxStr,
}

fn cstr_test_ident(ident: &str) -> BoxStr {
    format!("test_const_cstr_{ident}").into()
}

fn const_test_ident(ident: &str) -> BoxStr {
    format!("test_const_{ident}").into()
}

fn size_align_test_ident(ident: &str) -> BoxStr {
    format!("test_size_align_{ident}").into()
}

/// Wrap methods that depend on both ffi items and the generator
struct TranslateHelper<'a> {
    ffi_items: &'a FfiItems,
    generator: &'a TestGenerator,
    translator: Translator,
}

impl<'a> TranslateHelper<'a> {
    /// Returns the equivalent C/Cpp identifier of the Rust item.
    pub fn c_ident(&self, item: impl Into<MapInput<'a>>) -> Result<String, GenerationError> {
        self.generator.map(item)
    }

    /// Returns the equivalent C/Cpp type of the Rust item.
    pub fn c_type(&self, item: impl Into<MapInput<'a>>) -> Result<String, GenerationError> {
        let item: MapInput = item.into();

        let (ident, ty) = match item {
            MapInput::Const(c) => (c.ident(), self.translator.translate_type(&c.ty)),
            MapInput::Alias(a) => (a.ident(), self.translator.translate_type(&a.ty)),
            MapInput::Field(_, f) => (f.ident(), self.translator.translate_type(&f.ty)),
            MapInput::Static(s) => (s.ident(), self.translator.translate_type(&s.ty)),
            MapInput::Fn(_) => unimplemented!(),
            MapInput::Struct(_) => unimplemented!(),
            MapInput::StructType(_) => panic!("MapInput::StructType is not allowed!"),
            MapInput::UnionType(_) => panic!("MapInput::UnionType is not allowed!"),
            MapInput::Type(_) => panic!("MapInput::Type is not allowed!"),
        };

        let ty = ty.map_err(|e| GenerationError::TemplateRender("C".to_string(), e.to_string()))?;

        let item = if self.ffi_items.contains_struct(ident) {
            MapInput::StructType(&ty)
        } else if self.ffi_items.contains_union(ident) {
            MapInput::UnionType(&ty)
        } else {
            MapInput::Type(&ty)
        };
        self.generator.map(item)
    }
}
