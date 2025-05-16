//!
//! Safe wrapper for wxStaticBitmap

use crate::bitmap::Bitmap;
use crate::event::WxEvtHandler;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// Since there are no specific styles for StaticBitmap, we'll use a thin wrapper around i64
widget_style_enum!(
    name: StaticBitmapStyle,
    doc: "Style flags for the StaticBitmap widget.",
    variants: {
        Default: 0, "Default style with no special behavior."
    },
    default_variant: Default
);

/// Represents a wxStaticBitmap widget, used to display a bitmap.
pub struct StaticBitmap {
    window: Window,
}

impl StaticBitmap {
    /// Creates a new StaticBitmap builder.
    pub fn builder<W: WxWidget>(parent: &W) -> StaticBitmapBuilder<'_> {
        StaticBitmapBuilder::new(parent)
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

    /// Gets the current bitmap from the control.
    /// Returns a new bitmap instance that the caller owns.
    pub fn get_bitmap(&self) -> Option<Bitmap> {
        unsafe {
            let ptr = ffi::wxd_StaticBitmap_GetBitmap(
                self.window.handle_ptr() as *mut ffi::wxd_StaticBitmap_t
            );
            
            if ptr.is_null() {
                None
            } else {
                // We get ownership of the bitmap from C++
                Some(Bitmap::from_ptr_owned(ptr))
            }
        }
    }
}

widget_builder!(
    name: StaticBitmap,
    parent_type: &'a dyn WxWidget,
    style_type: StaticBitmapStyle,
    fields: {
        bitmap: Option<Bitmap> = None,
        name: String = "StaticBitmap".to_string()
    },
    build_impl: |slf| {
        let bmp_to_use = slf
            .bitmap
            .as_ref()
            .expect("Bitmap must be set for StaticBitmap using with_bitmap.");

        let name_cstr = CString::new(&slf.name[..]).unwrap_or_default();

        unsafe {
            let ptr = ffi::wxd_StaticBitmap_CreateWithBitmap(
                slf.parent.handle_ptr(),
                slf.id as c_int,
                bmp_to_use.as_ptr(),
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
                name_cstr.as_ptr(),
            );

            if ptr.is_null() {
                panic!("Failed to create StaticBitmap widget");
            }
            StaticBitmap::from_ptr(ptr)
        }
    }
);

// Use the macro to implement all the standard traits
implement_widget_traits_with_target!(StaticBitmap, window, Window);
