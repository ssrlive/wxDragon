use crate::base::{Point, RawWxProps, Size, ID_ANY};
use crate::event::WxEvtHandler; // Though ActivityIndicator doesn't have specific events
use crate::id::Id;
use crate::window::WxWidget;
use wxdragon_sys as ffi;

use std::default::Default;
use std::os::raw::c_long;
use std::ptr;

// --- ActivityIndicator --- //

#[derive(Debug, Clone)]
pub struct ActivityIndicator {
    ptr: *mut ffi::wxd_ActivityIndicator_t,
}

impl ActivityIndicator {
    pub fn builder(parent: &impl WxWidget) -> ActivityIndicatorBuilder {
        let mut builder = ActivityIndicatorBuilder::default();
        builder.parent_ptr = parent.handle_ptr();
        builder
    }

    fn from_ptr(ptr: *mut ffi::wxd_ActivityIndicator_t) -> Self {
        ActivityIndicator { ptr }
    }

    pub fn start(&self) {
        unsafe { ffi::wxd_ActivityIndicator_Start(self.ptr) }
    }

    pub fn stop(&self) {
        unsafe { ffi::wxd_ActivityIndicator_Stop(self.ptr) }
    }

    pub fn is_running(&self) -> bool {
        unsafe { ffi::wxd_ActivityIndicator_IsRunning(self.ptr) }
    }
}

impl WxWidget for ActivityIndicator {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.ptr as *mut ffi::wxd_Window_t
    }
}

impl WxEvtHandler for ActivityIndicator {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.ptr as *mut ffi::wxd_EvtHandler_t
    }
}

impl RawWxProps for ActivityIndicator {
    type RawWxPtr = ffi::wxd_ActivityIndicator_t;
    fn raw_wx_ptr(&self) -> *mut Self::RawWxPtr {
        self.ptr
    }
}

impl Drop for ActivityIndicator {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            self.ptr = ptr::null_mut();
        }
    }
}

// --- ActivityIndicatorBuilder --- //

pub struct ActivityIndicatorBuilder {
    parent_ptr: *mut ffi::wxd_Window_t,
    id: Id,
    pos: Point,
    size: Size,
    style: i64,
}

impl Default for ActivityIndicatorBuilder {
    fn default() -> Self {
        Self {
            parent_ptr: ptr::null_mut(),
            id: ID_ANY,
            pos: Point { x: -1, y: -1 },
            size: Size {
                width: -1,
                height: -1,
            },
            style: 0,
        }
    }
}

impl ActivityIndicatorBuilder {
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    pub fn with_pos(mut self, x: i32, y: i32) -> Self {
        self.pos = Point { x, y };
        self
    }

    pub fn with_pos_point(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    pub fn with_size(mut self, w: i32, h: i32) -> Self {
        self.size = Size {
            width: w,
            height: h,
        };
        self
    }

    pub fn with_size_obj(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    pub fn build(self) -> ActivityIndicator {
        assert!(
            !self.parent_ptr.is_null(),
            "ActivityIndicator requires a parent"
        );
        let raw_ptr = unsafe {
            ffi::wxd_ActivityIndicator_Create(
                self.parent_ptr,
                self.id,
                self.pos.x,
                self.pos.y,
                self.size.width,
                self.size.height,
                self.style as c_long,
            )
        };
        if raw_ptr.is_null() {
            panic!("Failed to create wxActivityIndicator");
        }
        ActivityIndicator::from_ptr(raw_ptr)
    }
}
