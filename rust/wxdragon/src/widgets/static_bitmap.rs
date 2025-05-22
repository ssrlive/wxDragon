//!
//! Safe wrapper for wxStaticBitmap

use crate::bitmap::Bitmap;
use crate::bitmap_bundle::BitmapBundle;
use crate::event::WindowEvents;
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
#[derive(Clone)]
pub struct StaticBitmap {
    window: Window,
}

impl StaticBitmap {
    /// Creates a new StaticBitmap builder.
    pub fn builder<W: WxWidget>(parent: &W) -> StaticBitmapBuilder<'_> {
        StaticBitmapBuilder::new(parent)
    }

    /// Creates a new StaticBitmap with a bitmap.
    pub fn new_with_bitmap(parent: &dyn WxWidget, id: Id, bitmap: &Bitmap) -> Self {
        let name_cstr = CString::new("StaticBitmap").unwrap_or_default();
        
        unsafe {
            let ptr = ffi::wxd_StaticBitmap_CreateWithBitmap(
                parent.handle_ptr(),
                id as c_int,
                bitmap.as_ptr(),
                ffi::wxd_Point { x: -1, y: -1 }, // DEFAULT_POSITION
                ffi::wxd_Size { width: -1, height: -1 }, // DEFAULT_SIZE
                0, // Default style
                name_cstr.as_ptr(),
            );

            if ptr.is_null() {
                panic!("Failed to create StaticBitmap widget");
            }
            Self::from_ptr(ptr)
        }
    }

    /// Creates a new StaticBitmap with a bitmap bundle.
    pub fn new_with_bitmap_bundle(parent: &dyn WxWidget, id: Id, bundle: &BitmapBundle) -> Self {
        unsafe {
            let ptr = ffi::wxd_StaticBitmap_CreateWithBitmapBundle(
                parent.handle_ptr(),
                id as c_int,
                bundle.as_ptr(),
            );

            if ptr.is_null() {
                panic!("Failed to create StaticBitmap widget with BitmapBundle");
            }
            Self::from_ptr(ptr)
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

    /// Sets or replaces the bitmap bundle shown in the control.
    /// 
    /// Using a bitmap bundle allows for better DPI scaling on high-resolution displays.
    pub fn set_bitmap_bundle(&self, bundle: &BitmapBundle) {
        unsafe {
            ffi::wxd_StaticBitmap_SetBitmapBundle(
                self.window.handle_ptr() as *mut ffi::wxd_StaticBitmap_t,
                bundle.as_ptr(),
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
        bitmap_bundle: Option<BitmapBundle> = None,
        name: String = "StaticBitmap".to_string()
    },
    build_impl: |slf| {
        let name_cstr = CString::new(&slf.name[..]).unwrap_or_default();

        // Prioritize BitmapBundle if both are set
        if let Some(bundle) = &slf.bitmap_bundle {
            unsafe {
                let ptr = ffi::wxd_StaticBitmap_CreateWithBitmapBundle(
                    slf.parent.handle_ptr(),
                    slf.id as c_int,
                    bundle.as_ptr(),
                );

                if ptr.is_null() {
                    panic!("Failed to create StaticBitmap widget with BitmapBundle");
                }
                return StaticBitmap::from_ptr(ptr);
            }
        } else if let Some(bmp) = &slf.bitmap {
            unsafe {
                let ptr = ffi::wxd_StaticBitmap_CreateWithBitmap(
                    slf.parent.handle_ptr(),
                    slf.id as c_int,
                    bmp.as_ptr(),
                    slf.pos.into(),
                    slf.size.into(),
                    slf.style.bits() as ffi::wxd_Style_t,
                    name_cstr.as_ptr(),
                );

                if ptr.is_null() {
                    panic!("Failed to create StaticBitmap widget");
                }
                return StaticBitmap::from_ptr(ptr);
            }
        }
        
        panic!("Either bitmap or bitmap_bundle must be set for StaticBitmap");
    }
);

// Use the macro to implement all the standard traits
implement_widget_traits_with_target!(StaticBitmap, window, Window);

impl WindowEvents for StaticBitmap {}
