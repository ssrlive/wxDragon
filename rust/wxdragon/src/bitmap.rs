//!
//! Safe wrapper for wxBitmap.

use std::os::raw::{c_int, c_uchar};
use wxdragon_sys as ffi;

/// Represents a platform-dependent bitmap image.
#[derive(Debug)] // Keep Debug if useful, or remove if pointer isn't meaningful for debug
pub struct Bitmap {
    ptr: *mut ffi::wxd_Bitmap_t,
    is_owned: bool, // Tracks whether Rust owns this bitmap and should destroy it
}

impl Bitmap {
    /// Creates a new empty bitmap with the specified width and height.
    pub fn new(width: i32, height: i32) -> Option<Self> {
        if width <= 0 || height <= 0 {
            return None;
        }

        // Create RGBA data (4 bytes per pixel)
        let pixel_count = (width * height * 4) as usize;
        let data = vec![0; pixel_count]; // All zeros for a fully transparent bitmap

        Self::from_rgba(&data, width as u32, height as u32)
    }

    /// Creates a new bitmap from raw RGBA pixel data.
    ///
    /// # Arguments
    /// * `data` - A slice containing the raw RGBA pixel data (4 bytes per pixel).
    /// * `width` - The width of the image in pixels.
    /// * `height` - The height of the image in pixels.
    ///
    /// Returns `None` if the bitmap creation fails (e.g., invalid dimensions, memory allocation error).
    pub fn from_rgba(data: &[u8], width: u32, height: u32) -> Option<Self> {
        let expected_len = (width * height * 4) as usize;
        if data.len() != expected_len || width == 0 || height == 0 {
            eprintln!(
                "Bitmap::from_rgba: Invalid data length or dimensions. Expected {}, got {}, w={}, h={}", 
                expected_len, data.len(), width, height
            );
            return None;
        }

        let ptr = unsafe {
            ffi::wxd_Bitmap_CreateFromRGBA(
                data.as_ptr() as *const c_uchar,
                width as c_int,
                height as c_int,
            )
        };

        if ptr.is_null() {
            None
        } else {
            Some(Bitmap {
                ptr,
                is_owned: true,
            }) // We own bitmaps created this way
        }
    }

    /// Creates a bitmap wrapper around an existing bitmap pointer, transferring ownership to Rust.
    /// The bitmap will be destroyed when the wrapper is dropped.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid wxBitmap pointer, and no other code should destroy it.
    pub(crate) fn from_ptr_owned(ptr: *mut ffi::wxd_Bitmap_t) -> Self {
        Bitmap {
            ptr,
            is_owned: true,
        }
    }

    /// Creates a bitmap wrapper around an existing bitmap pointer without taking ownership.
    /// Rust will NOT destroy the bitmap when the wrapper is dropped.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid `wxd_Bitmap_t` pointer that is managed (lifetime-wise)
    /// by other code (e.g., wxWidgets internals). The pointer must remain valid for the
    /// lifetime of this `Bitmap` object.
    pub(crate) unsafe fn from_ptr_unowned(ptr: *mut ffi::wxd_Bitmap_t) -> Self {
        Bitmap {
            ptr,
            is_owned: false,
        }
    }

    /// Returns the raw underlying bitmap pointer.
    /// Use with caution, primarily for internal FFI calls.
    pub fn as_ptr(&self) -> *mut ffi::wxd_Bitmap_t {
        self.ptr
    }

    /// Returns a pointer suitable for borrowing by C++ (e.g., for DataViewCtrl).
    /// If the bitmap is not ok or its internal pointer is null, this returns null.
    pub fn as_borrowable_ptr(&self) -> *mut ffi::wxd_Bitmap_t {
        if self.is_ok() && !self.ptr.is_null() {
            self.ptr
        } else {
            std::ptr::null_mut()
        }
    }

    /// Returns the width of the bitmap in pixels.
    pub fn get_width(&self) -> i32 {
        if self.ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Bitmap_GetWidth(self.ptr) as i32 }
    }

    /// Returns the height of the bitmap in pixels.
    pub fn get_height(&self) -> i32 {
        if self.ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Bitmap_GetHeight(self.ptr) as i32 }
    }

    /// Checks if the bitmap is valid.
    pub fn is_ok(&self) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Bitmap_IsOk(self.ptr) }
    }
}

impl Clone for Bitmap {
    fn clone(&self) -> Self {
        unsafe {
            let cloned_ptr = ffi::wxd_Bitmap_Clone(self.ptr);
            if cloned_ptr.is_null() {
                panic!(
                    "Failed to clone wxBitmap: wxd_Bitmap_Clone returned null. Original: {:?}",
                    self.ptr
                );
            }
            // A cloned bitmap is always owned by Rust
            Bitmap {
                ptr: cloned_ptr,
                is_owned: true,
            }
        }
    }
}

impl Drop for Bitmap {
    /// Destroys the associated C++ wxBitmap object if Rust owns the bitmap.
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.is_owned {
            unsafe {
                ffi::wxd_Bitmap_Destroy(self.ptr);
            }
        }
    }
}
