//! Source directory: `sysdeps/nptl/`
//!
//! Native POSIX threading library.
//!
//! <https://github.com/bminor/glibc/tree/master/sysdeps/nptl>

pub(crate) mod bits {
    pub(crate) mod pthreadtypes;
}

pub(crate) mod pthread;
