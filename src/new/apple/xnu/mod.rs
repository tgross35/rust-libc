//! Source from XNU <https://github.com/apple-oss-distributions/xnu/tree/main>
//!
//! We omit nesting for the `bsd` module since most items of interest are in there.

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub(crate) mod arm {
    pub(crate) mod _mcontext;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) mod i386 {
    pub(crate) mod _mcontext;
}

pub(crate) mod mach;
// pub use mach::*;

pub(crate) mod machine {
    pub(crate) mod _mcontext;
}

pub(crate) mod sys;
