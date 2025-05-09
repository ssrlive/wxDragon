//!
//! Safe wrapper for wxBitmap.

use std::os::raw::{c_int, c_uchar};
use wxdragon_sys as ffi;

/// Represents a platform-dependent bitmap image.
#[derive(Debug)] // Keep Debug if useful, or remove if pointer isn't meaningful for debug
pub struct Bitmap(pub(crate) *mut ffi::wxd_Bitmap_t);

impl Bitmap {
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
            Some(Bitmap(ptr))
        }
    }

    // TODO: Add `from_dynamic_image` helper using the `image` crate?

    /// Returns the raw underlying bitmap pointer.
    /// Use with caution, primarily for internal FFI calls.
    pub(crate) fn as_ptr(&self) -> *mut ffi::wxd_Bitmap_t {
        self.0
    }

    /// Returns the width of the bitmap in pixels.
    pub fn get_width(&self) -> i32 {
        if self.0.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Bitmap_GetWidth(self.0) as i32 }
    }

    /// Returns the height of the bitmap in pixels.
    pub fn get_height(&self) -> i32 {
        if self.0.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Bitmap_GetHeight(self.0) as i32 }
    }

    /// Checks if the bitmap is valid.
    pub fn is_ok(&self) -> bool {
        if self.0.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Bitmap_IsOk(self.0) }
    }
}

impl Clone for Bitmap {
    fn clone(&self) -> Self {
        unsafe {
            let cloned_ptr = ffi::wxd_Bitmap_Clone(self.0);
            if cloned_ptr.is_null() {
                panic!(
                    "Failed to clone wxBitmap: wxd_Bitmap_Clone returned null. Original: {:?}",
                    self.0
                );
            }
            Bitmap(cloned_ptr)
        }
    }
}

impl Drop for Bitmap {
    /// Destroys the associated C++ wxBitmap object.
    /// Note: This should only be called if Rust has unique ownership.
    /// If the bitmap is passed to a control (e.g., BitmapButton),
    /// wxWidgets might take ownership, and dropping here could cause a double free.
    /// Careful lifetime management is needed.
    fn drop(&mut self) {
        // TODO: Implement proper ownership tracking. For now, assume Rust owns
        //       bitmaps created via `from_rgba` unless explicitly given away.
        if !self.0.is_null() {
            unsafe {
                ffi::wxd_Bitmap_Destroy(self.0);
            }
        }
    }
}
