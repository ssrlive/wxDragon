use crate::prelude::*;
use crate::widgets::AuiMdiParentFrame;
use std::ffi::CString;
use wxdragon_sys as ffi;

// Constants for wxAuiMDIChildFrame - typically uses wxDEFAULT_FRAME_STYLE or specific wxFrame styles.
// pub const AUI_MDI_CHILD_FRAME_DEFAULT_STYLE: i64 = ffi::WXD_DEFAULT_FRAME_STYLE;

#[derive(Clone)]
pub struct AuiMdiChildFrame {
    ptr: *mut ffi::wxd_AuiMDIChildFrame_t,
}

impl AuiMdiChildFrame {
    fn from_ptr(ptr: *mut ffi::wxd_AuiMDIChildFrame_t) -> Self {
        AuiMdiChildFrame { ptr }
    }

    pub fn builder(parent: &AuiMdiParentFrame) -> AuiMdiChildFrameBuilder {
        AuiMdiChildFrameBuilder::new(parent)
    }
}

impl WxWidget for AuiMdiChildFrame {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.ptr as *mut ffi::wxd_Window_t // Cast to base window type for generic operations
    }
}

impl WxEvtHandler for AuiMdiChildFrame {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.ptr as *mut ffi::wxd_EvtHandler_t
    }
}

pub struct AuiMdiChildFrameBuilder {
    parent: *mut ffi::wxd_AuiMDIParentFrame_t,
    id: i32,
    title: String,
    pos: Point,
    size: Size,
    style: i64, // wxWidgets style flags
    name: String,
}

impl AuiMdiChildFrameBuilder {
    pub fn new(parent: &AuiMdiParentFrame) -> Self {
        AuiMdiChildFrameBuilder {
            parent: parent.handle_ptr() as *mut ffi::wxd_AuiMDIParentFrame_t, // Store parent ptr
            id: ID_ANY as i32,
            title: String::new(),
            pos: Point::default(),   // wxDefaultPosition
            size: Size::default(),   // wxDefaultSize
            style: ffi::WXD_DEFAULT_FRAME_STYLE, // Standard frame style
            name: "wxAuiMDIChildFrame".to_string(), // Default name
        }
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

    pub fn build(self) -> AuiMdiChildFrame {
        let title_c = CString::new(self.title).expect("CString::new failed for title");
        let name_c = CString::new(self.name).expect("CString::new failed for name");

        let ptr = unsafe {
            ffi::wxd_AuiMDIChildFrame_Create(
                self.parent,
                self.id,
                title_c.as_ptr(),
                self.pos.into(),
                self.size.into(),
                self.style,
                name_c.as_ptr()
            )
        };

        if ptr.is_null() {
            panic!("Failed to create AuiMdiChildFrame: wxWidgets returned a null pointer.");
        } else {
            AuiMdiChildFrame::from_ptr(ptr)
        }
    }
}

impl Drop for AuiMdiChildFrame {
    fn drop(&mut self) {
        // Child frames are typically managed and destroyed by their parent AUI manager or MDI parent.
        // However, if created independently and not added, or if specific wxWidgets behavior dictates,
        // a call to wxWindow::Destroy() might be needed.
        // For now, assuming parentage handles destruction as is common with MDI children.
        // If issues arise, uncomment and test:
        // unsafe { ffi::wxd_Window_Destroy(self.handle_ptr()) }
    }
} 