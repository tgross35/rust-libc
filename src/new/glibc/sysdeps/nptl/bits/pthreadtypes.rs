//! Source header: `sysdeps/nptl/bits/pthreadtypes.h`
//!
//! <https://github.com/bminor/glibc/blob/master/sysdeps/nptl/bits/pthreadtypes.h>

use crate::prelude::*;

pub type pthread_t = c_ulong;

s! {
    pub union pthread_mutexattr_t {
        __size: [c_char; __SIZEOF_PTHREAD_MUTEXATTR_T],
        __align: c_int,
    }

    pub union pthread_condattr_t {
        __size: [c_char; __SIZEOF_PTHREAD_CONDATTR_T],
        __align: c_int,
    }
}

pub type pthread_key_t = c_uint;

// This uses a `#define __ONCE_ALIGNMENT __attribute__ ((__aligned__ (4)))` on m68k, likely becuase
// the GCC ABI gives `int` an alignment of 2. We _probably_ don't need to worry about this.
pub type pthread_once_t = c_int;

s! {
    pub union pthread_attr_t {
      __size: [c_char; __SIZEOF_PTHREAD_ATTR_T];
      __align: c_long;
    }

    // Note that the locks for the primitives have a `__data` field that is a platform-dependent
    // struct. This is smaller than the `__SIZEOF` constants, so we ignore them for simplicity.

    pub union pthread_mutex_t {
      __size: [c_char; __SIZEOF_PTHREAD_MUTEX_T];
      __align: c_long;
    }

    pub union pthread_cond_t {
      __size: [c_char; __SIZEOF_PTHREAd_COND_t];
      __align: c_longlong;
    }

    pub union pthread_rwlock_t {
      __size: [c_char; __SIZEOF_PTHREAD_RWLOCK_T];
      __align: c_long;
    }

    pub union pthread_rwlockattr_t {
        __size: [c_char; __SIZEOF_PTHREAD_RWLOCKATTR_T],
        __align: c_long,
    }
}

pub type pthread_spinlock_t = c_int;

s! {
    pub union pthread_barrier_t {
      __size: [c_char; __SIZEOF_PTHREAD_BARRIER_T];
      __align: c_long;
    }

    pub union pthread_barrierattr_t {
        __size: [c_char; __SIZEOF_PTHREAD_BARRIERATTR_T],
        __align: c_int,
    }
}
