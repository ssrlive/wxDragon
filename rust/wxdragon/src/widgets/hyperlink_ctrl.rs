use crate::base::{Colour, Point, RawWxProps, Size, DEFAULT_POSITION, DEFAULT_SIZE, ID_ANY};
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::window::WxWidget;
use wxdragon_sys as ffi;

use std::default::Default;
use std::ffi::{CStr, CString};
use std::os::raw::c_long;
use std::ptr;

// --- HyperlinkCtrl --- //

#[derive(Debug, Clone)]
pub struct HyperlinkCtrl {
    ptr: *mut ffi::wxd_HyperlinkCtrl_t,
}

impl HyperlinkCtrl {
    pub fn builder(parent: &impl WxWidget) -> HyperlinkCtrlBuilder {
        let mut builder = HyperlinkCtrlBuilder::default();
        builder.parent_ptr = parent.handle_ptr();
        builder
    }

    fn from_ptr(ptr: *mut ffi::wxd_HyperlinkCtrl_t) -> Self {
        HyperlinkCtrl { ptr }
    }

    pub fn get_url(&self) -> String {
        unsafe {
            let c_str_ptr = ffi::wxd_HyperlinkCtrl_GetURL(self.ptr);
            if c_str_ptr.is_null() {
                String::new()
            } else {
                CStr::from_ptr(c_str_ptr).to_string_lossy().into_owned()
            }
        }
    }

    pub fn set_url(&self, url: &str) {
        let c_url = CString::new(url).expect("CString::new failed for url");
        unsafe { ffi::wxd_HyperlinkCtrl_SetURL(self.ptr, c_url.as_ptr()) }
    }

    pub fn get_visited(&self) -> bool {
        unsafe { ffi::wxd_HyperlinkCtrl_GetVisited(self.ptr) }
    }

    pub fn set_visited(&self, visited: bool) {
        unsafe { ffi::wxd_HyperlinkCtrl_SetVisited(self.ptr, visited) }
    }

    pub fn get_hover_colour(&self) -> Colour {
        let val = unsafe { ffi::wxd_HyperlinkCtrl_GetHoverColour(self.ptr) };
        Colour::from_u32(val as u32)
    }

    pub fn set_hover_colour(&self, colour: Colour) {
        unsafe {
            ffi::wxd_HyperlinkCtrl_SetHoverColour(
                self.ptr,
                colour.as_u32() as std::os::raw::c_ulong,
            )
        }
    }

    pub fn get_normal_colour(&self) -> Colour {
        let val = unsafe { ffi::wxd_HyperlinkCtrl_GetNormalColour(self.ptr) };
        Colour::from_u32(val as u32)
    }

    pub fn set_normal_colour(&self, colour: Colour) {
        unsafe {
            ffi::wxd_HyperlinkCtrl_SetNormalColour(
                self.ptr,
                colour.as_u32() as std::os::raw::c_ulong,
            )
        }
    }

    pub fn get_visited_colour(&self) -> Colour {
        let val = unsafe { ffi::wxd_HyperlinkCtrl_GetVisitedColour(self.ptr) };
        Colour::from_u32(val as u32)
    }

    pub fn set_visited_colour(&self, colour: Colour) {
        unsafe {
            ffi::wxd_HyperlinkCtrl_SetVisitedColour(
                self.ptr,
                colour.as_u32() as std::os::raw::c_ulong,
            )
        }
    }
}

impl WxWidget for HyperlinkCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.ptr as *mut ffi::wxd_Window_t
    }
}

impl WxEvtHandler for HyperlinkCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.ptr as *mut ffi::wxd_EvtHandler_t
    }
}

impl RawWxProps for HyperlinkCtrl {
    type RawWxPtr = ffi::wxd_HyperlinkCtrl_t;
    fn raw_wx_ptr(&self) -> *mut Self::RawWxPtr {
        self.ptr
    }
}

impl Drop for HyperlinkCtrl {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            self.ptr = ptr::null_mut();
        }
    }
}

// --- HyperlinkCtrlBuilder --- //

pub struct HyperlinkCtrlBuilder {
    parent_ptr: *mut ffi::wxd_Window_t,
    id: Id,
    label: String,
    url: String,
    pos: Point,
    size: Size,
    style: i64,
}

impl Default for HyperlinkCtrlBuilder {
    fn default() -> Self {
        Self {
            parent_ptr: ptr::null_mut(),
            id: ID_ANY,
            label: String::new(),
            url: String::new(),
            pos: DEFAULT_POSITION,
            size: DEFAULT_SIZE,
            style: 0x0002,
        }
    }
}

impl HyperlinkCtrlBuilder {
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn with_url(mut self, url: &str) -> Self {
        self.url = url.to_string();
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

    pub fn build(self) -> HyperlinkCtrl {
        let c_label = CString::new(self.label).expect("CString::new failed for label");
        let c_url = CString::new(self.url).expect("CString::new failed for url");
        let raw_ptr = unsafe {
            ffi::wxd_HyperlinkCtrl_Create(
                self.parent_ptr,
                self.id,
                c_label.as_ptr(),
                c_url.as_ptr(),
                self.pos.x,
                self.pos.y,
                self.size.width,
                self.size.height,
                self.style as c_long,
            )
        };
        if raw_ptr.is_null() {
            panic!("Failed to create wxHyperlinkCtrl");
        }
        HyperlinkCtrl::from_ptr(raw_ptr)
    }
}
