use crate::prelude::*;
use std::ffi::CString;
use wxdragon_sys as ffi;

// Constants for wxAuiMDIParentFrame (if any specific ones are needed beyond standard frame styles)
// Example: pub const AUI_MDI_PARENT_FRAME_DEFAULT_STYLE: i64 = ffi::WXD_DEFAULT_FRAME_STYLE; // Assuming a generic one for now

#[derive(Clone)]
pub struct AuiMdiParentFrame {
    ptr: *mut ffi::wxd_AuiMDIParentFrame_t,
}

impl AuiMdiParentFrame {
    fn from_ptr(ptr: *mut ffi::wxd_AuiMDIParentFrame_t) -> Self {
        AuiMdiParentFrame { ptr }
    }

    pub fn builder() -> AuiMdiParentFrameBuilder {
        AuiMdiParentFrameBuilder::new()
    }

    // Common methods inherited from WxWindow can be implemented via WxWindowMethods trait
    // or directly if specific overrides/behavior are needed.
}

impl WxWidget for AuiMdiParentFrame {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.ptr as *mut ffi::wxd_Window_t
    }
}

impl WxEvtHandler for AuiMdiParentFrame {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.ptr as *mut ffi::wxd_EvtHandler_t // AuiMDIParentFrame is an EvtHandler
    }
}

// wxAuiMDIParentFrame is a top-level window, so it owns itself until closed by user/system.
// No specific Drop implementation is needed here if parent class (Frame/TopLevelWindow) handles it,
// or if it's managed by wxWidgets' lifecycle.
// If we were to call Destroy() in Drop, it would be for child widgets.
// Top-level windows like Frame are typically destroyed by wxWidgets when closed.

pub struct AuiMdiParentFrameBuilder {
    parent: Option<*mut ffi::wxd_Window_t>,
    id: i32,
    title: String,
    pos: Point,
    size: Size,
    style: i64,
    name: String,
}

impl AuiMdiParentFrameBuilder {
    pub fn new() -> Self {
        AuiMdiParentFrameBuilder {
            parent: None,
            id: ID_ANY as i32,
            title: String::new(),
            pos: Point::default(),
            size: Size::default(),
            style: ffi::WXD_DEFAULT_FRAME_STYLE, // style field is i64, FFI const is i64. Cast occurs at ffi call site.
            name: "wxDragon AUI Frame".to_string(),
        }
    }

    pub fn with_parent(mut self, parent: &impl WxWidget) -> Self {
        self.parent = Some(parent.handle_ptr());
        self
    }

    pub fn with_id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
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

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn build(self) -> AuiMdiParentFrame {
        let title_c = CString::new(self.title).expect("CString::new failed for title");
        let name_c = CString::new(self.name).expect("CString::new failed for name");

        let parent_ptr = self.parent.unwrap_or(std::ptr::null_mut());

        let ptr = unsafe {
            ffi::wxd_AuiMDIParentFrame_Create(
                parent_ptr,
                self.id,
                title_c.as_ptr(),
                self.pos.into(),
                self.size.into(),
                self.style as ffi::wxd_Style_t, // Corrected: Cast self.style to ffi::wxd_Style_t (long)
                name_c.as_ptr(),
            )
        };
        if ptr.is_null() {
            panic!("Failed to create wxAuiMDIParentFrame");
        }
        AuiMdiParentFrame::from_ptr(ptr)
    }
}

// Default builder without a parent, as AuiMdiParentFrame is a top-level window.
impl Default for AuiMdiParentFrameBuilder {
    fn default() -> Self {
        Self::new()
    }
} 