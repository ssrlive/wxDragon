//!
//! wxStaticBox wrapper
//!

use crate::base::{Point, Size};
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::window::WxWidget;
use std::ffi::CString;
use wxdragon_sys as ffi;

// Represents the wxStaticBox widget.
pub struct StaticBox {
    ptr: *mut ffi::wxd_StaticBox_t,
}

impl WxEvtHandler for StaticBox {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.ptr as *mut ffi::wxd_EvtHandler_t
    }
}

impl WxWidget for StaticBox {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.ptr as *mut ffi::wxd_Window_t
    }
}

impl StaticBox {
    // Creates a new StaticBox from a raw pointer.
    // Does NOT assume ownership.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_StaticBox_t) -> Self {
        StaticBox { ptr }
    }
}

// Builder for StaticBox
#[derive(Default)]
pub struct StaticBoxBuilder {
    parent: Option<*mut ffi::wxd_Window_t>,
    id: Id,
    label: String,
    pos: Point,
    size: Size,
    style: i64,
}

impl StaticBoxBuilder {
    pub(crate) fn new() -> Self {
        Self {
            parent: None,
            id: crate::id::ID_ANY as Id,
            label: String::new(),
            pos: Point { x: -1, y: -1 },
            size: Size {
                width: -1,
                height: -1,
            },
            style: 0,
        }
    }

    pub fn parent<W: WxWidget>(mut self, parent: &W) -> Self {
        self.parent = Some(parent.handle_ptr());
        self
    }

    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label = label.to_string();
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

    pub fn build(self) -> StaticBox {
        let parent_ptr = self.parent.expect("StaticBox requires a parent");
        let c_label = CString::new(self.label).unwrap_or_default();
        let ptr = unsafe {
            ffi::wxd_StaticBox_Create(
                parent_ptr,
                self.id,
                c_label.as_ptr(),
                self.pos.into(),
                self.size.into(),
                self.style as ffi::wxd_Style_t,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create StaticBox");
        }
        unsafe { StaticBox::from_ptr(ptr) }
    }
}

impl StaticBox {
    // Creates a new StaticBox using the builder pattern.
    pub fn builder() -> StaticBoxBuilder {
        StaticBoxBuilder::new()
    }
}
