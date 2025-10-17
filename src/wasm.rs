// FFI (Foreign Function Interface) bindings for Flutter integration
// This provides C-compatible functions that can be called from Flutter/Dart

use libc::{c_char, c_int, size_t};
use std::ffi::{CStr, CString};
use std::ptr;

// Re-export the main functionality
use crate::{parse_cdl, validate_circuit, export_spice, parse_and_render};
use crate::renderer::{SvgTheme, SvgStyle};

/// Parse CDL text and return an SVG string (C-compatible)
/// 
/// # Safety
/// This function is unsafe because it handles raw C strings.
/// The caller must ensure that:
/// - cdl_input points to a valid null-terminated string
/// - The returned string is freed with free_string
#[no_mangle]
pub unsafe extern "C" fn parse_cdl_to_svg(
    cdl_input: *const c_char,
    theme: *const c_char,
    style: *const c_char,
    result_length: *mut size_t,
) -> *mut c_char {
    // Check for null pointers
    if cdl_input.is_null() || theme.is_null() || style.is_null() {
        return ptr::null_mut();
    }

    // Convert C strings to Rust strings
    let cdl_str = match CStr::from_ptr(cdl_input).to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let theme_str = match CStr::from_ptr(theme).to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let style_str = match CStr::from_ptr(style).to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    // Parse theme and style
    let theme = match theme_str {
        "dark" => SvgTheme::Dark,
        _ => SvgTheme::Light,
    };

    let style = match style_str {
        "iec" => SvgStyle::Iec,
        "din" => SvgStyle::Din,
        _ => SvgStyle::Ieee,
    };

    // Call the main function
    match parse_and_render(cdl_str, theme, style) {
        Ok(svg) => {
            // Convert result to C string
            match CString::new(svg) {
                Ok(c_string) => {
                    if !result_length.is_null() {
                        *result_length = c_string.as_bytes().len();
                    }
                    c_string.into_raw() // Transfer ownership to C code
                }
                Err(_) => ptr::null_mut(),
            }
        }
        Err(_) => ptr::null_mut(),
    }
}

/// Validate CDL text (C-compatible)
/// 
/// # Safety
/// This function is unsafe because it handles raw C strings.
/// The caller must ensure that:
/// - cdl_input points to a valid null-terminated string
#[no_mangle]
pub unsafe extern "C" fn validate_cdl(
    cdl_input: *const c_char,
) -> c_int {
    // Check for null pointer
    if cdl_input.is_null() {
        return -1; // Error code for invalid input
    }

    // Convert C string to Rust string
    let cdl_str = match CStr::from_ptr(cdl_input).to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    // Call the validation function
    match validate_circuit(cdl_str) {
        Ok(()) => 1,  // Valid
        Err(_) => 0,  // Invalid
    }
}

/// Export SPICE netlist (C-compatible)
/// 
/// # Safety
/// This function is unsafe because it handles raw C strings.
/// The caller must ensure that:
/// - cdl_input points to a valid null-terminated string
/// - The returned string is freed with free_string
#[no_mangle]
pub unsafe extern "C" fn export_spice_c(
    cdl_input: *const c_char,
    result_length: *mut size_t,
) -> *mut c_char {
    // Check for null pointer
    if cdl_input.is_null() {
        return ptr::null_mut();
    }

    // Convert C string to Rust string
    let cdl_str = match CStr::from_ptr(cdl_input).to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    // Call the SPICE export function
    match export_spice(cdl_str) {
        Ok(spice) => {
            // Convert result to C string
            match CString::new(spice) {
                Ok(c_string) => {
                    if !result_length.is_null() {
                        *result_length = c_string.as_bytes().len();
                    }
                    c_string.into_raw() // Transfer ownership to C code
                }
                Err(_) => ptr::null_mut(),
            }
        }
        Err(_) => ptr::null_mut(),
    }
}

/// Free a string allocated by the library
/// 
/// # Safety
/// This function is unsafe because it handles raw C strings.
/// The caller must ensure that:
/// - str_to_free was allocated by a function in this library
#[no_mangle]
pub unsafe extern "C" fn free_string(str_to_free: *mut c_char) {
    if !str_to_free.is_null() {
        let _ = CString::from_raw(str_to_free);
        // The CString's Drop implementation will free the memory
    }
}

/// Get the last error message (not implemented in this version)
/// Returns null for now
#[no_mangle]
pub unsafe extern "C" fn get_last_error() -> *const c_char {
    ptr::null()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_parse_cdl_to_svg_ffi() {
        let cdl = CString::new("R1 resistor 1k (0, 0)\nR2 resistor 2k (100, 0)\nR1.2 -> R2.1").unwrap();
        let theme = CString::new("light").unwrap();
        let style = CString::new("ieee").unwrap();
        
        let mut length = 0;
        let result = unsafe {
            parse_cdl_to_svg(cdl.as_ptr(), theme.as_ptr(), style.as_ptr(), &mut length)
        };
        
        if !result.is_null() {
            unsafe {
                free_string(result);
            }
        }
        // The function should return a valid pointer for valid input
        assert!(!result.is_null());
    }
}