use crate::prelude::*;
use std::ffi::CString;
use std::os::raw::c_long;
use wxdragon_sys as ffi;

// Placeholder: This constant needs to be properly generated and available via ffi.
// For now, assuming it will be an i64 value from the constants generation.
pub const AUI_NOTEBOOK_DEFAULT_STYLE_I64: i64 =
    0x00000001 | 0x00000002 | 0x00000004 | 0x00000010 | 0x00000040 | 0x00000200; // wxAUI_NB_DEFAULT_STYLE components
                                                                                 // pub const AUI_NOTEBOOK_DEFAULT_STYLE: c_long = ffi::WXD_AUI_NB_DEFAULT_STYLE as c_long; // Ideal, if WXD_AUI_NB_DEFAULT_STYLE is i64

#[derive(Clone)]
pub struct AuiNotebook {
    ptr: *mut ffi::wxd_AuiNotebook_t,
}

impl AuiNotebook {
    fn from_ptr(ptr: *mut ffi::wxd_AuiNotebook_t) -> Self {
        AuiNotebook { ptr }
    }

    pub fn builder(parent: &impl WxWidget) -> AuiNotebookBuilder {
        AuiNotebookBuilder::new(parent)
    }

    pub fn add_page(&self, page: &impl WxWidget, caption: &str, select: bool) -> bool {
        let caption_c = CString::new(caption).expect("CString::new failed for caption");
        unsafe {
            // Pass -1 for bitmap_id as a default, assuming no specific bitmap support yet in this wrapper
            ffi::wxd_AuiNotebook_AddPage(
                self.ptr,
                page.handle_ptr(),
                caption_c.as_ptr(),
                select,
                -1,
            )
        }
    }

    pub fn page_count(&self) -> usize {
        unsafe { ffi::wxd_AuiNotebook_GetPageCount(self.ptr) as usize }
    }

    pub fn set_selection(&self, new_page: usize) -> usize {
        // The FFI function wxd_AuiNotebook_SetSelection expects size_t, which bindgen maps to Rust's usize.
        unsafe { ffi::wxd_AuiNotebook_SetSelection(self.ptr, new_page) as usize }
    }

    // Add other methods like get_page, insert_page, remove_page etc. as needed
}

impl WxWidget for AuiNotebook {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.ptr as *mut ffi::wxd_Window_t
    }
}

impl WxEvtHandler for AuiNotebook {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.ptr as *mut ffi::wxd_EvtHandler_t
    }
}

// AuiNotebook is a WxWindow and WxEvtHandler.
// It is not a WxNonOwnedWindow, WxTopLevelWindow, or WxFrame.

pub struct AuiNotebookBuilder {
    parent: *mut ffi::wxd_Window_t,
    id: i32,
    pos: Point,
    size: Size,
    style: c_long, // Use c_long directly to match FFI
}

impl AuiNotebookBuilder {
    pub fn new(parent: &impl WxWidget) -> Self {
        // Assuming WXD_AUI_NB_DEFAULT_STYLE will be available from ffi as an i64 constant.
        // If it's not yet generated, use a placeholder value for now.
        let default_style_val = if false {
            // This branch would be taken if ffi::WXD_AUI_NB_DEFAULT_STYLE exists
            // ffi::WXD_AUI_NB_DEFAULT_STYLE as c_long
            0 // temp placeholder
        } else {
            AUI_NOTEBOOK_DEFAULT_STYLE_I64 as c_long // Use a manually defined i64 and cast
        };

        AuiNotebookBuilder {
            parent: parent.handle_ptr(),
            id: ID_ANY as i32,
            pos: Point::default(),
            size: Size::default(),
            style: default_style_val,
        }
    }

    pub fn with_id(mut self, id: i32) -> Self {
        self.id = id;
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

    pub fn with_style(mut self, style: c_long) -> Self {
        self.style = style;
        self
    }

    pub fn build(self) -> AuiNotebook {
        let ptr = unsafe {
            ffi::wxd_AuiNotebook_Create(
                self.parent,
                self.id,
                self.pos.into(),
                self.size.into(),
                self.style, // Already c_long
            )
        };
        if ptr.is_null() {
            panic!("Failed to create AuiNotebook");
        }
        AuiNotebook::from_ptr(ptr)
    }
}

// Type alias for size_t for clarity, matching C's size_t which bindgen should handle.
// Rust's `usize` is typically equivalent to C's `size_t`.
// REMOVED custom size_t alias, as bindgen handles it via usize in FFI signatures.
// #[cfg(target_pointer_width = "64")]
// type size_t = u64;
// #[cfg(target_pointer_width = "32")]
// type size_t = u32;

// Define long_t for FFI style argument if necessary
// In C++, long is typically 32-bit on Windows and 64-bit on 64-bit Linux/macOS.
// This needs to match the `long style` in the C FFI declaration.
// For simplicity, if wxWidgets uses 'long' for style consistently, and bindgen maps it to Rust's i64 or i32 appropriately, direct use of self.style (i64) should be fine.
// If there's a mismatch, a specific long_t (e.g. i32 or i64 based on target) might be needed.
// For now, assuming direct i64 from self.style is compatible or bindgen handles it.
// The C API uses `long style`, which bindgen should map. If it maps to i64, no cast is needed.
// If it maps to i32, then `self.style as i32` would be needed.
// Given we use `long` in C, and `i64` in Rust builder for style, this should generally align on 64-bit systems.
// For `build` method: use `self.style` (which is i64), assuming `long` in FFI is compatible with i64.
// If FFI `long` is consistently `i32` across platforms for wx, then builder `style` should be `i32`.
// The header `wxd_types.h` defines `typedef long wxd_Style_t;`
// `build.rs` for bindgen defines `pub type wxd_Style_t = ::std::os::raw::c_long;`
// `std::os::raw::c_long` is i32 on windows, i64 on unix-like.
// So, the FFI function expects `c_long`. Our builder uses `i64` for style.
// This means `self.style as std::os::raw::c_long` is the most robust cast.

// In build(): self.style as std::os::raw::c_long
// In AuiNotebookBuilder: style: std::os::raw::c_long // Better to match the FFI type directly
// And default style: AUI_NOTEBOOK_DEFAULT_STYLE_I64 as std::os::raw::c_long,
// Or ensure AUI_NOTEBOOK_DEFAULT_STYLE_I64 is already a c_long.
// The ffi::WXD_... constants are typically i64. So casting ffi constant to c_long for builder init is correct.
// And using self.style (which would be c_long) directly in ffi call is correct.
