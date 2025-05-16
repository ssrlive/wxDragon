//! VariantType implementation.

use std::ffi::CString;
use wxdragon_sys as ffi;
use std::os::raw::c_char;
use crate::{DateTime, Bitmap};

/// Represents the type of data stored in a variant.
///
/// This enum defines the standard data types that can be used with
/// DataViewRenderer and wxVariant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariantType {
    /// Boolean value (true/false)
    Bool,
    /// 32-bit integer
    Int32,
    /// 64-bit integer
    Int64,
    /// Floating point number
    Double,
    /// Text string
    String,
    /// Date and time value
    DateTime,
    /// Binary data
    Bitmap,
    /// Progress value (typically 0-100)
    Progress,
}

impl VariantType {
    /// Converts the enum variant to a C string compatible with wxWidgets
    pub fn to_type_string(&self) -> &'static str {
        match self {
            VariantType::Bool => "bool",
            VariantType::Int32 => "long",
            VariantType::Int64 => "longlong",
            VariantType::Double => "double",
            VariantType::String => "string",
            VariantType::DateTime => "datetime",
            VariantType::Bitmap => "bitmap",
            VariantType::Progress => "long",
        }
    }
}

/// A wrapper for wxd_Variant_t that provides a safe Rust interface.
///
/// Variant is used to store and pass data of different types between
/// the application and the DataViewModel.
pub enum Variant {
    /// Boolean value
    Bool(bool),
    /// 32-bit integer value
    Int32(i32),
    /// 64-bit integer value
    Int64(i64),
    /// Floating point value
    Double(f64),
    /// String value
    String(String),
    /// Date and time value
    DateTime(DateTime),
    /// Bitmap image data
    Bitmap(Bitmap),
}

impl Variant {
    /// Creates a new empty variant.
    pub fn new() -> Self {
        Variant::Int32(0)
    }

    /// Gets the raw pointer to the native wxd_Variant_t.
    /// 
    /// IMPORTANT: This creates a Box<wxd_Variant_t> and leaks it intentionally
    /// since the C++ side is expected to take ownership of this memory and free it
    /// when done. We're only leaking a small amount of memory here, and it's a necessary
    /// part of FFI operations where ownership crosses language boundaries.
    pub fn as_raw(&self) -> *const ffi::wxd_Variant_t {
        // Create a heap-allocated wxd_Variant_t to ensure it doesn't get dropped
        let mut variant = Box::new(ffi::wxd_Variant_t {
            type_: ffi::WXD_VARIANT_TYPE_INVALID as i32,
            data: unsafe { std::mem::zeroed() },
        });
        
        // Set the value based on the variant type
        match self {
            Variant::Bool(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_BOOL as i32;
                variant.data.bool_val = *value;
            },
            Variant::Int32(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_INT32 as i32;
                variant.data.int32_val = *value;
            },
            Variant::Int64(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_INT64 as i32;
                variant.data.int64_val = *value;
            },
            Variant::Double(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_DOUBLE as i32;
                variant.data.double_val = *value;
            },
            Variant::String(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_STRING as i32;
                let cstr = CString::new(value.as_str()).unwrap();
                variant.data.string_val = unsafe { 
                    // Use standard C function strdup instead of libc::strdup
                    let s = cstr.as_ptr();
                    let len = libc::strlen(s) + 1;
                    let new_s = libc::malloc(len) as *mut c_char;
                    if !new_s.is_null() {
                        libc::strcpy(new_s, s);
                    }
                    new_s
                };
            },
            Variant::DateTime(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_DATETIME as i32;
                // Convert DateTime to raw datetime struct
                variant.data.datetime_val = unsafe {
                    *(value.as_ptr())
                };
            },
            Variant::Bitmap(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_BITMAP as i32;
                variant.data.bitmap_val = value.as_ptr();
            },
        }
        
        // Leak the Box to ensure the memory lives long enough
        Box::into_raw(variant)
    }
    
    /// Gets a mutable raw pointer to the native wxd_Variant_t.
    ///
    /// This is primarily used by event.rs for event data.
    pub fn as_raw_mut(&self) -> *mut ffi::wxd_Variant_t {
        self.as_raw() as *mut _
    }
    
    /// Gets the type of the variant
    pub fn get_type(&self) -> VariantType {
        match self {
            Variant::Bool(_) => VariantType::Bool,
            Variant::Int32(_) => VariantType::Int32,
            Variant::Int64(_) => VariantType::Int64,
            Variant::Double(_) => VariantType::Double,
            Variant::String(_) => VariantType::String,
            Variant::DateTime(_) => VariantType::DateTime,
            Variant::Bitmap(_) => VariantType::Bitmap,
        }
    }
}

impl Clone for Variant {
    fn clone(&self) -> Self {
        match self {
            Variant::Bool(value) => Variant::Bool(*value),
            Variant::Int32(value) => Variant::Int32(*value),
            Variant::Int64(value) => Variant::Int64(*value),
            Variant::Double(value) => Variant::Double(*value),
            Variant::String(value) => Variant::String(value.clone()),
            Variant::DateTime(value) => Variant::DateTime(*value),
            Variant::Bitmap(value) => Variant::Bitmap(value.clone()),
        }
    }
}

impl Drop for Variant {
    fn drop(&mut self) {}
}

impl Default for Variant {
    fn default() -> Self {
        Self::new()
    }
}

impl From<bool> for Variant {
    fn from(value: bool) -> Self {
        Variant::Bool(value)
    }
}

impl From<i32> for Variant {
    fn from(value: i32) -> Self {
        Variant::Int32(value)
    }
}

impl From<i64> for Variant {
    fn from(value: i64) -> Self {
        Variant::Int64(value)
    }
}

impl From<f64> for Variant {
    fn from(value: f64) -> Self {
        Variant::Double(value)
    }
}

impl From<&str> for Variant {
    fn from(value: &str) -> Self {
        Variant::String(value.to_string())
    }
}

impl From<String> for Variant {
    fn from(value: String) -> Self {
        Variant::String(value)
    }
}

impl From<DateTime> for Variant {
    fn from(value: DateTime) -> Self {
        Variant::DateTime(value)
    }
}

impl From<Bitmap> for Variant {
    fn from(value: Bitmap) -> Self {
        Variant::Bitmap(value)
    }
} 