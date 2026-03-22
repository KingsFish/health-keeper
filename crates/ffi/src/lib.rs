//! FFI bindings for HealthKeeper
//!
//! This crate provides foreign function interface bindings for integrating
//! HealthKeeper with mobile and desktop applications (Android, macOS, etc.)

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Initialize the HealthKeeper library
/// Must be called before any other functions
#[no_mangle]
pub extern "C" fn hk_init(data_dir: *const c_char) -> i32 {
    // TODO: Implement initialization
    0
}

/// Free a string returned by the library
#[no_mangle]
pub extern "C" fn hk_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

// Placeholder for future FFI implementations
// Will include:
// - Person CRUD operations
// - Visit CRUD operations
// - File import
// - OCR and LLM operations
// - Search functionality