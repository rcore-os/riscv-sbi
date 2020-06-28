//! Low level access to RISC-V Supervisor Binary Interface (SBI) implementations
//!
//! Ref: https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc
//!
//!
//! # Features
//!
//! This crate provides access to standard SBI functions, both base and legacy ones.

#![no_std]
#![deny(warnings, missing_docs)]
#![feature(llvm_asm)]

pub mod base;
pub mod legacy;

pub mod log;

#[macro_use]
#[doc(hidden)]
pub mod io;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct SbiReturn {
    error: SbiError,
    value: usize,
}

impl SbiReturn {
    fn unwrap(self) -> usize {
        assert_eq!(self.error, SbiError::Success);
        self.value
    }
}

/// The error type which is returned from SBI.
#[repr(isize)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum SbiError {
    Success = 0,
    Failed = -1,
    NotSupported = -2,
    InvalidParam = -3,
    Denied = -4,
    InvalidAddress = -5,
    AlreadyAvailable = -6,
}

/// The type returned by SBI functions.
pub type SbiResult<T = ()> = Result<T, SbiError>;

impl From<SbiReturn> for SbiResult<usize> {
    fn from(ret: SbiReturn) -> Self {
        match ret.error {
            SbiError::Success => Ok(ret.value),
            err => Err(err),
        }
    }
}

#[inline(always)]
fn sbi_call(ext_id: usize, func_id: usize, arg0: usize, arg1: usize, arg2: usize) -> SbiReturn {
    let error;
    let value;
    unsafe {
        llvm_asm!(
            "ecall"
            : "={x10}" (error), "={x11}"(value)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x16}"(func_id), "{x17}" (ext_id)
            : "memory"
            : "volatile"
        );
    }
    SbiReturn { error, value }
}

const EXTENSION_TIME: usize = 0x54494D45;
// const EXTENSION_IPI: usize = 0x735049;

/// Timer Extension, Extension ID: 0x54494D45 (TIME)
///
/// Caller should validate if this extension exists using [`probe_extension`].
///
/// [`probe_extension`]: ../base/fn.probe_extension.html
pub mod time {
    use super::*;
    const FUNCTION_SET_TIMER: usize = 0;

    // todo: verify this function
    /// Programs the clock for next event after `stime_value` time.
    ///
    /// This function also clears the pending timer interrupt bit.
    ///
    /// If the supervisor wishes to clear the timer interrupt without scheduling the next timer event,
    /// it can either request a timer interrupt infinitely far into the future (i.e., (uint64_t)-1),
    /// or it can instead mask the timer interrupt by clearing `sie.STIE`.
    pub fn set_timer(stime_value: u64) {
        #[cfg(target_pointer_width = "32")]
        sbi_call(
            EXTENSION_TIME,
            FUNCTION_SET_TIMER,
            stime_value as usize,
            (stime_value >> 32) as usize,
            0,
        );
        #[cfg(target_pointer_width = "64")]
        sbi_call(
            EXTENSION_TIME,
            FUNCTION_SET_TIMER,
            stime_value as usize,
            0,
            0,
        );
    }
}

/// IPI Extension, Extension ID: 0x735049 (sPI: s-mode IPI)
///
/// Caller should validate if this extension exists using [`probe_extension`].
///
/// [`probe_extension`]: ../base/fn.probe_extension.html
pub mod ipi {
    // todo

    // use super::*;
    // const FUNCTION_SEND_IPI: usize = 0;

    // /// Send an inter-processor interrupt to all the harts defined in `hart_mask`.
    // ///
    // /// `hart_mask` is a virtual address that points to a bit-vector of harts. The bit vector is
    // /// represented as a sequence of unsigned longs whose length equals the number of harts in the
    // /// system divided by the number of bits in an unsigned long, rounded up to the next integer.
    // pub fn send_ipi(hart_mask: usize, hart_mask_base: usize) {
    //     sbi_call(SBI_SEND_IPI, &hart_mask as *const _ as usize, 0, 0);
    // }
}

/// RFENCE Extension, Extension ID: 0x52464E43 (RFNC)
pub mod rfnc {
    // todo
}

/// Hart State Management Extension, Extension ID: 0x48534D (HSM)
pub mod hsm {
    // todo
}
