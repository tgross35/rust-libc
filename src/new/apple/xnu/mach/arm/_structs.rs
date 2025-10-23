//! Header: `arm/_structs.h`
//!
//! https://github.com/apple-oss-distributions/xnu/blob/main/osfmk/mach/arm/_structs.h

s! {
    pub struct __darwin_arm_exception_state64 {
        pub __far: u64,
        pub __esr: u32,
        pub __exception: u32,
    }

    pub struct __darwin_arm_thread_state64 {
        pub __x: [u64; 29],
        pub __fp: u64,
        pub __lr: u64,
        pub __sp: u64,
        pub __pc: u64,
        pub __cpsr: u32,
        pub __pad: u32,
    }

    pub struct __darwin_arm_neon_state64 {
        pub __v: [crate::__uint128_t; 32],
        pub __fpsr: u32,
        pub __fpcr: u32,
    }
}
