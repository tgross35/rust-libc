//! Header: `bits/alltypes.h`
//!
//! The header `alltypes.h` is generated from a sed script that runs on `alltypes.h.in`. All the
//! script does is gate types behind `NEED` checks, e.g.
//!
//! ```c
//! #if defined(__NEED_foo) && !defined(__DEFINED_foo)
//! struct foo { int };
//! #define __DEFINED_foo
//! #endif
//! ```
//!
//! We make these definitions available unconditionally.
//!
//! <https://github.com/kraj/musl/blob/master/include/alltypes.h.in>

use crate::prelude::*;

s! {
    pub struct pthread_mutexattr_t {
        __attr: c_uint,
    }

    pub struct pthread_condattr_t {
        __attr: c_uint,
    }

    pub struct pthread_barrierattr_t {
        __attr: c_uint,
    }

    pub struct pthread_rwlockattr_t {
        __attr: [c_uint; 2],
    }
}
