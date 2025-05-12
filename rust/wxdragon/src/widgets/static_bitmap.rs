//!
//! Safe wrapper for wxStaticBitmap

use crate::geometry::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE};
use crate::id::ID_ANY;
use crate::bitmap::Bitmap;
use crate::event::WxEvtHandler; // Corrected: Though StaticBitmap rarely has specific events to bind
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// Constants for wxStaticBitmap if any (e.g., specific styles)
// pub const SB_SOME_STYLE: i64 = ffi::WXD_SB_SOME_STYLE; // Example

/// Represents a wxStaticBitmap widget, used to display a bitmap.
pub struct StaticBitmap {
    window: Window,
}

impl StaticBitmap {
    /// Creates a new StaticBitmap builder.
    pub fn builder<W: WxWidget>(parent: &W) -> StaticBitmapBuilder {
        StaticBitmapBuilder {
            parent_ptr: parent.handle_ptr(),
            id: ID_ANY as i32,
            bitmap: None, // Must be set via with_bitmap
            pos: DEFAULT_POSITION,
            size: DEFAULT_SIZE,
            style: 0, // Default style, e.g., wxBORDER_NONE if defined and desired
            name: CString::new("StaticBitmap").unwrap(),
        }
    }

    /// Creates a StaticBitmap from a raw wxStaticBitmap pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_StaticBitmap_t` pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_StaticBitmap_t) -> Self {
        StaticBitmap {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Sets or replaces the bitmap shown in the control.
    pub fn set_bitmap(&self, bitmap: &Bitmap) {
        unsafe {
            ffi::wxd_StaticBitmap_SetBitmap(
                self.window.handle_ptr() as *mut ffi::wxd_StaticBitmap_t,
                bitmap.as_ptr(),
            );
        }
    }

    // // Optional: GetBitmap - would require wxd_StaticBitmap_GetBitmap FFI
    // pub fn get_bitmap(&self) -> Option<Bitmap> {
    //     unsafe {
    //         let bmp_ptr = ffi::wxd_StaticBitmap_GetBitmap(self.window.handle_ptr() as *mut ffi::wxd_StaticBitmap_t);
    //         if bmp_ptr.is_null() { None } else { Some(Bitmap::from_ptr(bmp_ptr)) }
    //     }
    // }
}

/// Builder for `StaticBitmap` widgets.
#[derive(Clone)] // Bitmap field is Option<Bitmap>, which is Clone if Bitmap is Clone
pub struct StaticBitmapBuilder {
    parent_ptr: *mut ffi::wxd_Window_t,
    id: i32,
    bitmap: Option<Bitmap>, // Bitmap is now owned by the builder, then passed to C++
    pos: Point,
    size: Size,
    style: i64,
    name: CString,
}

impl StaticBitmapBuilder {
    pub fn with_id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    /// Sets the bitmap to be displayed. This is mandatory.
    pub fn with_bitmap(mut self, bitmap: &Bitmap) -> Self {
        self.bitmap = Some(bitmap.clone()); // Clone the bitmap for ownership
        self
    }

    pub fn with_pos(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    /// Sets the size. If not set, it might be derived from the bitmap size by wxWidgets.
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = CString::new(name).unwrap_or_default();
        self
    }

    /// Creates the `StaticBitmap`.
    /// Panics if `with_bitmap` was not called.
    pub fn build(self) -> StaticBitmap {
        let bmp_to_use = self
            .bitmap
            .as_ref()
            .expect("Bitmap must be set for StaticBitmap using with_bitmap.");

        unsafe {
            let ptr = ffi::wxd_StaticBitmap_CreateWithBitmap(
                self.parent_ptr,
                self.id as c_int,
                bmp_to_use.as_ptr(),
                self.pos.into(),
                self.size.into(),
                self.style as ffi::wxd_Style_t,
                self.name.as_ptr(),
            );

            if ptr.is_null() {
                panic!("Failed to create StaticBitmap widget");
            }
            StaticBitmap::from_ptr(ptr)
        }
    }
}

// --- Trait Implementations ---

impl WxWidget for StaticBitmap {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

impl Deref for StaticBitmap {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl DerefMut for StaticBitmap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}

// StaticBitmap typically doesn't emit specific events that need custom handling,
// but it can still be a WxEvtHandler for common events like mouse clicks if needed.
impl WxEvtHandler for StaticBitmap {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.window.handle_ptr() as *mut ffi::wxd_EvtHandler_t
    }
}

// No Drop needed typically, as wxStaticBitmap is a wxWindow and managed by its parent.
// The wxBitmap it holds is also copied by wxStaticBitmap, so the original Bitmap passed to
// the builder or SetBitmap can be dropped by Rust without affecting the control.
