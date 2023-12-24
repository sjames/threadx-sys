#![no_std]
include!("generated.rs");

// Functions that are implemented in assembly that are missed by bindgen
extern "C" {
    pub fn _tx_timer_interrupt() -> ();
    pub fn __tx_SVCallHandler() -> ();
    pub fn __tx_PendSVHandler() -> ();
}
