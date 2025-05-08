use wxdragon_sys as wx_bindings; // Alias for the sys crate
                                 // REMOVED: use wx_bindings::wxd_Size; // Unused
                                 // REMOVED: use wx_bindings::wxd_Point; // Unused
                                 // wxID_ANY will be used as wx_bindings::WXD_ID_ANY directly in builder
                                 // wxBitmapType_wxBITMAP_TYPE_ANY is not needed for current C API
use crate::base::{Point, Size}; // Local Point, Size structs
use crate::window::WxWidget; // Correct path for WxWidget trait
                             // Removed Window and WxWindow import as StaticBitmap doesn't seem to need them directly in its impl block
                             // Removed Control import as its existence/location is unclear and might not be needed
use crate::bitmap::Bitmap; // Added import for Bitmap
use std::ffi::CString; // Keep CString import
use std::os::raw::c_int;
// use std::marker::PhantomData; // Removed unused import
use wxdragon_sys as ffi; // Use standard ffi alias

// Opaque pointer type from FFI
pub type RawStaticBitmap = ffi::wxd_StaticBitmap;

#[derive(Clone)]
pub struct StaticBitmap {
    ptr: *mut RawStaticBitmap,
}

impl StaticBitmap {
    pub fn new(ptr: *mut RawStaticBitmap) -> Self {
        StaticBitmap { ptr }
    }

    pub fn builder(parent: &impl WxWidget) -> StaticBitmapBuilder {
        StaticBitmapBuilder::new(parent)
    }
}

impl WxWidget for StaticBitmap {
    fn handle_ptr(&self) -> *mut wx_bindings::wxd_Window_t {
        self.ptr as *mut wx_bindings::wxd_Window_t
    }
}
// REMOVED: impl WxWindow for StaticBitmap {}
// REMOVED: impl Control for StaticBitmap {}

unsafe impl Send for StaticBitmap {}
unsafe impl Sync for StaticBitmap {}

// Builder
pub struct StaticBitmapBuilder {
    parent: *mut ffi::wxd_Window_t,
    id: i32,
    bitmap_path: Option<String>,
    bitmap_object: Option<Bitmap>,
    pos: Point,
    size: Size,
    scale_mode: i32,
    style: i64,
}

impl StaticBitmapBuilder {
    pub fn new(parent: &impl WxWidget) -> Self {
        StaticBitmapBuilder {
            parent: parent.handle_ptr(),
            id: ffi::WXD_ID_ANY as i32,
            bitmap_path: None,
            bitmap_object: None,
            pos: Point::new(-1, -1),
            size: Size::new(-1, -1),
            scale_mode: SCALE_NONE,
            style: 0,
        }
    }

    pub fn with_id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    pub fn with_bitmap_path(mut self, path: &str) -> Self {
        self.bitmap_path = Some(path.to_string());
        self.bitmap_object = None; // Clear bitmap object if path is set
        self
    }

    pub fn with_bitmap(mut self, bitmap: Bitmap) -> Self {
        self.bitmap_object = Some(bitmap);
        self.bitmap_path = None; // Clear path if bitmap object is set
        self
    }

    pub fn with_pos(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    /// Sets the scaling mode for the bitmap.
    pub fn with_scale_mode(mut self, mode: i32) -> Self {
        self.scale_mode = mode;
        self
    }

    pub fn build(self) -> Option<StaticBitmap> {
        let raw_ptr = unsafe {
            if let Some(bitmap) = self.bitmap_object {
                // Use the Bitmap object
                ffi::wxd_StaticBitmap_CreateWithBitmap(
                    self.parent,
                    self.id,
                    bitmap.as_ptr(),
                    self.pos.x,
                    self.pos.y,
                    self.size.width,
                    self.size.height,
                    self.style as ffi::wxd_Style_t,
                    self.scale_mode as c_int,
                )
            } else if let Some(path_str) = self.bitmap_path {
                // Use the bitmap path
                let c_bitmap_path =
                    CString::new(path_str).expect("CString::new failed for bitmap_path");
                ffi::wxd_StaticBitmap_Create(
                    self.parent,
                    self.id,
                    c_bitmap_path.as_ptr(),
                    self.pos.x,
                    self.pos.y,
                    self.size.width,
                    self.size.height,
                    self.style as ffi::wxd_Style_t,
                )
            } else {
                // Neither path nor object provided, create with null bitmap (or handle as error)
                // wxStaticBitmap handles wxNullBitmap if path is empty, let's replicate by calling Create with empty path
                let c_empty_path = CString::new("").unwrap();
                ffi::wxd_StaticBitmap_Create(
                    self.parent,
                    self.id,
                    c_empty_path.as_ptr(),
                    self.pos.x,
                    self.pos.y,
                    self.size.width,
                    self.size.height,
                    self.style as ffi::wxd_Style_t,
                )
            }
        };

        if raw_ptr.is_null() {
            // This can happen if C++ side returns nullptr due to load failure or other issues
            None
        } else {
            Some(StaticBitmap::new(raw_ptr))
        }
    }
}

// Constants for wxStaticBitmap styles and scale modes
// These should align with what's added to const_extractor
pub const BORDER_NONE: i64 = ffi::WXD_BORDER_NONE as i64;
pub const BORDER_SIMPLE: i64 = ffi::WXD_BORDER_SIMPLE as i64;
// Add other border styles (WXD_BORDER_STATIC, etc.) if they were added to const_extractor and are distinct

pub const ALIGN_CENTRE_HORIZONTAL: i64 = ffi::WXD_ALIGN_CENTRE_HORIZONTAL as i64;
pub const ALIGN_CENTER_HORIZONTAL: i64 = ffi::WXD_ALIGN_CENTRE_HORIZONTAL as i64; // Alias for WXD_ALIGN_CENTRE_HORIZONTAL
pub const ALIGN_CENTRE_VERTICAL: i64 = ffi::WXD_ALIGN_CENTRE_VERTICAL as i64;
pub const ALIGN_CENTER_VERTICAL: i64 = ffi::WXD_ALIGN_CENTRE_VERTICAL as i64; // Alias for WXD_ALIGN_CENTRE_VERTICAL
pub const ALIGN_LEFT: i64 = ffi::WXD_ALIGN_LEFT as i64;
pub const ALIGN_RIGHT: i64 = ffi::WXD_ALIGN_RIGHT as i64;

// Scale modes (map directly from enum values)
pub const SCALE_NONE: i32 = ffi::WXD_StaticBitmap_Scale_None as i32;
pub const SCALE_FILL: i32 = ffi::WXD_StaticBitmap_Scale_Fill as i32;
pub const SCALE_ASPECT_FIT: i32 = ffi::WXD_StaticBitmap_Scale_AspectFit as i32;
pub const SCALE_ASPECT_FILL: i32 = ffi::WXD_StaticBitmap_Scale_AspectFill as i32;

// TODO: Add methods to StaticBitmap to SetBitmap, GetBitmap, SetIcon, SetScaleMode, GetScaleMode etc.
// For SetScaleMode, we'd need a C API function: wxd_StaticBitmap_SetScaleMode(wxd_StaticBitmap* self, int mode);
// And the corresponding C++ implementation.
