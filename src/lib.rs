#![no_std]

include!("generated.rs");

// Functions that are implemented in assembly that are missed by bindgen
// TODO: the SVCall and PendSV call are probably specific to Arm Cortex
// and should be enabled based on the selected target.
extern "C" {
    pub fn _tx_timer_interrupt() -> ();
    pub fn __tx_SVCallHandler() -> ();
    pub fn __tx_PendSVHandler() -> ();
}

// Constants that are not parsed by bindgen

// API Input parameters and general constants
pub const TX_NO_WAIT : ULONG = 0;
pub const TX_WAIT_FOREVER : ULONG = 0xFFFFFFFF;
pub const TX_AND : UINT = 2;
pub const TX_AND_CLEAR : UINT = 3;
pub const TX_OR : UINT = 0;
pub const TX_OR_CLEAR : UINT = 1;
pub const TX_1_ULONG : UINT = 1;
pub const TX_2_ULONG : UINT = 2;
pub const TX_4_ULONG : UINT = 4;
pub const TX_8_ULONG : UINT = 8;
pub const TX_16_ULONG : UINT = 16;
pub const TX_NO_TIME_SLICE : ULONG = 0;
pub const TX_AUTO_START : UINT = 1;
pub const TX_DONT_START : UINT = 0;
pub const TX_AUTO_ACTIVATE : UINT = 1;
pub const TX_NO_ACTIVATE : UINT = 0;
pub const TX_TRUE : UINT = 1;
pub const TX_FALSE : UINT = 0;
pub const TX_NULL : *mut core::ffi::c_void = core::ptr::null_mut();
pub const TX_INHERIT : UINT = 1;
pub const TX_NO_INHERIT : UINT = 0;
pub const TX_THREAD_ENTRY : UINT = 0;
pub const TX_THREAD_EXIT : UINT = 1;
pub const TX_NO_SUSPENSIONS : UINT = 0;
pub const TX_NO_MESSAGES : UINT = 0;
pub const TX_EMPTY : UINT = 0;
pub const TX_CLEAR_ID : UINT = 0;
