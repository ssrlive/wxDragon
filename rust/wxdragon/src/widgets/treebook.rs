use crate::geometry::{Point, Size};
use crate::event::WxEvtHandler;
use crate::id::{Id, ID_ANY};
use crate::window::WxWidget;
use std::ffi::CString;
use wxdragon_sys as ffi; // Ensure ffi is in scope for constants

// Styles (wxBookCtrlBase styles)
pub const BK_DEFAULT: i64 = ffi::WXD_BK_DEFAULT as i64;
pub const BK_TOP: i64 = ffi::WXD_BK_TOP as i64;
pub const BK_BOTTOM: i64 = ffi::WXD_BK_BOTTOM as i64;
pub const BK_LEFT: i64 = ffi::WXD_BK_LEFT as i64;
pub const BK_RIGHT: i64 = ffi::WXD_BK_RIGHT as i64;

// Opaque wxTreebook pointer from wxdragon-sys
pub type RawTreebook = wxdragon_sys::wxd_Treebook_t;

/// Represents a wxTreebook control.
#[derive(Clone)]
pub struct Treebook {
    ptr: *mut RawTreebook,
}

impl Treebook {
    /// Creates a new Treebook.
    /// This is typically called by TreebookBuilder.
    fn new_impl(
        parent_ptr: *mut wxdragon_sys::wxd_Window_t,
        id: Id,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        assert!(!parent_ptr.is_null(), "Treebook parent cannot be null");
        let treebook_ptr = unsafe {
            wxdragon_sys::wxd_Treebook_new(
                parent_ptr,
                id,
                pos.x,
                pos.y,
                size.width,
                size.height,
                style as ffi::wxd_Style_t,
            )
        };
        Treebook { ptr: treebook_ptr }
    }

    /// Creates a new Treebook builder.
    pub fn builder<'a>(parent: &'a dyn WxWidget) -> TreebookBuilder<'a> {
        TreebookBuilder::new(parent)
    }

    /// Adds a new page to the treebook control.
    pub fn add_page<W: WxWidget>(&self, page: &W, text: &str, select: bool, image_id: i32) -> i32 {
        let page_ptr = page.handle_ptr() as *mut wxdragon_sys::wxd_Window_t;
        let text_c = CString::new(text).unwrap_or_default();
        unsafe {
            wxdragon_sys::wxd_Treebook_AddPage(
                self.ptr,
                page_ptr,
                text_c.as_ptr(),
                select as i32,
                image_id,
            )
        }
    }

    /// Adds a new sub-page to the last top-level page added to the treebook control.
    pub fn add_sub_page<W: WxWidget>(
        &self,
        page: &W,
        text: &str,
        select: bool,
        image_id: i32,
    ) -> i32 {
        let page_ptr = page.handle_ptr() as *mut wxdragon_sys::wxd_Window_t;
        let text_c = CString::new(text).unwrap_or_default();
        unsafe {
            wxdragon_sys::wxd_Treebook_AddSubPage(
                self.ptr,
                page_ptr,
                text_c.as_ptr(),
                select as i32,
                image_id,
            )
        }
    }

    pub fn get_page_count(&self) -> i32 {
        unsafe { wxdragon_sys::wxd_Treebook_GetPageCount(self.ptr) }
    }

    pub fn get_selection(&self) -> i32 {
        unsafe { wxdragon_sys::wxd_Treebook_GetSelection(self.ptr) }
    }

    pub fn set_selection(&self, n: usize) -> i32 {
        unsafe { wxdragon_sys::wxd_Treebook_SetSelection(self.ptr, n) }
    }

    pub fn set_page_text(&self, n: usize, text: &str) {
        let text_c = CString::new(text).unwrap_or_default();
        unsafe {
            wxdragon_sys::wxd_Treebook_SetPageText(self.ptr, n, text_c.as_ptr());
        }
    }

    pub fn get_page_text(&self, n: usize) -> String {
        unsafe {
            // First call to get the size needed
            let needed_len_with_null =
                wxdragon_sys::wxd_Treebook_GetPageText(self.ptr, n, std::ptr::null_mut(), 0);
            if needed_len_with_null <= 1 {
                // 0 or 1 means error or empty string
                return String::new();
            }

            let buffer_size = needed_len_with_null as usize;
            let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size);

            // Second call to actually get the string
            let copied_len_with_null = wxdragon_sys::wxd_Treebook_GetPageText(
                self.ptr,
                n,
                buffer.as_mut_ptr() as *mut i8,
                buffer_size as i32,
            );

            if copied_len_with_null <= 0 {
                // Check for error on second call
                return String::new();
            }

            // Assuming the C++ side returned needed size including null,
            // and successfully copied that amount (or truncated).
            // The actual number of bytes excluding null is needed_len_with_null - 1.
            let actual_len = (copied_len_with_null - 1) as usize;
            buffer.set_len(actual_len.min(buffer_size - 1)); // Set length to actual content size (excluding null)

            String::from_utf8_lossy(&buffer).into_owned()
        }
    }
    // TODO: Add other wxBookCtrlBase methods like GetPage, InsertPage, DeletePage etc.
    // For GetPage, it should return an Option<Window> or similar, requiring careful handling
    // of the returned wxd_Window_t* and its actual type.
}

impl WxWidget for Treebook {
    fn handle_ptr(&self) -> *mut wxdragon_sys::wxd_Window_t {
        self.ptr as *mut wxdragon_sys::wxd_Window_t
    }
}

impl WxEvtHandler for Treebook {
    unsafe fn get_event_handler_ptr(&self) -> *mut wxdragon_sys::wxd_EvtHandler_t {
        self.ptr as *mut wxdragon_sys::wxd_EvtHandler_t
    }
}

/// Builder for creating Treebook instances.
pub struct TreebookBuilder<'a> {
    parent: &'a dyn WxWidget,
    id: Id,
    pos: Option<Point>,
    size: Option<Size>,
    style: i64,
}

impl<'a> TreebookBuilder<'a> {
    /// Creates a new Treebook builder with default values for fields other than parent.
    pub fn new(parent: &'a dyn WxWidget) -> Self {
        Self {
            parent,
            id: ID_ANY as Id,
            pos: None,
            size: None,
            style: BK_DEFAULT,
        }
    }

    /// Sets the window identifier.
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    /// Sets the position of the treebook.
    pub fn with_pos(mut self, x: i32, y: i32) -> Self {
        self.pos = Some(Point { x, y });
        self
    }

    /// Sets the size of the treebook.
    pub fn with_size(mut self, width: i32, height: i32) -> Self {
        self.size = Some(Size { width, height });
        self
    }

    /// Sets the window style.
    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    /// Builds the Treebook instance.
    /// Panics if the parent is not set.
    pub fn build(self) -> Treebook {
        let parent_ptr = self.parent.handle_ptr() as *mut wxdragon_sys::wxd_Window_t;
        let pos = self.pos.unwrap_or_else(|| Point { x: -1, y: -1 });
        let size = self.size.unwrap_or_else(|| Size {
            width: -1,
            height: -1,
        });

        Treebook::new_impl(parent_ptr, self.id, pos, size, self.style)
    }
}

// TODO: Implement EvtHandlerMethods for Treebook if it can source events directly.
// Common events: wxEVT_TREEBOOK_PAGE_CHANGED, wxEVT_TREEBOOK_PAGE_CHANGING, etc.
// These are already in WXDEventTypeCEnum and mapped in event.cpp.

// Wrapper for easy creation in a builder pattern style, if desired.
// pub fn treebook(parent: &impl WindowMethods, id: Id) -> TreebookBuilder { ... }
