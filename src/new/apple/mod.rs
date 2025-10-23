//! Entrypoint for Apple headers, usually found as part of the xcode SDK.

pub(crate) mod xnu;
pub(crate) use xnu::*;

pub(crate) mod alibc {
    pub(crate) mod signal;
}

pub(crate) use alibc::*;
