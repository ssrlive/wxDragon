use crate::base::{Point, RawWxProps, Size, ID_ANY}; // ID_ANY and RawWxProps are in base
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::window::WxWidget;
use wxdragon_sys as ffi; // ADDED for Id type alias
                         // ConvertResult path still TBD, will add if compiler needs it.

use std::ffi::CString;
use std::ops::{BitOr, BitOrAssign};
use std::os::raw::{c_char, c_long};
use std::ptr; // Added for style enum

// --- SearchCtrl --- //

#[derive(Debug, Clone)]
pub struct SearchCtrl {
    ptr: *mut ffi::wxd_SearchCtrl_t,
}

impl SearchCtrl {
    pub fn builder(parent: &impl WxWidget) -> SearchCtrlBuilder {
        SearchCtrlBuilder::new(parent)
    }

    fn from_ptr(ptr: *mut ffi::wxd_SearchCtrl_t) -> Self {
        SearchCtrl { ptr }
    }

    pub fn show_search_button(&self, show: bool) {
        unsafe { ffi::wxd_SearchCtrl_ShowSearchButton(self.ptr, show) }
    }

    pub fn is_search_button_visible(&self) -> bool {
        unsafe { ffi::wxd_SearchCtrl_IsSearchButtonVisible(self.ptr) }
    }

    pub fn show_cancel_button(&self, show: bool) {
        unsafe { ffi::wxd_SearchCtrl_ShowCancelButton(self.ptr, show) }
    }

    pub fn is_cancel_button_visible(&self) -> bool {
        unsafe { ffi::wxd_SearchCtrl_IsCancelButtonVisible(self.ptr) }
    }

    pub fn set_value(&self, value: &str) {
        let c_value = CString::new(value).expect("CString::new failed for value");
        unsafe {
            ffi::wxd_TextCtrl_SetValue(self.ptr as *mut ffi::wxd_TextCtrl_t, c_value.as_ptr())
        }
    }

    pub fn get_value(&self) -> String {
        let len_needed = unsafe {
            ffi::wxd_TextCtrl_GetValue(self.ptr as *mut ffi::wxd_TextCtrl_t, ptr::null_mut(), 0)
        };

        if len_needed <= 0 {
            return String::new();
        }
        let buffer_size = len_needed as usize;
        let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size);

        let actual_len = unsafe {
            ffi::wxd_TextCtrl_GetValue(
                self.ptr as *mut ffi::wxd_TextCtrl_t,
                buffer.as_mut_ptr() as *mut c_char,
                len_needed,
            )
        };

        if actual_len <= 0 {
            return String::new();
        }
        unsafe {
            buffer.set_len((actual_len - 1).max(0) as usize);
        }
        String::from_utf8(buffer).unwrap_or_default()
    }
}

impl WxWidget for SearchCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.ptr as *mut ffi::wxd_Window_t
    }
}

impl WxEvtHandler for SearchCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.ptr as *mut ffi::wxd_EvtHandler_t
    }
}

impl RawWxProps for SearchCtrl {
    // Now using RawWxProps from crate::base
    type RawWxPtr = ffi::wxd_SearchCtrl_t;
    fn raw_wx_ptr(&self) -> *mut Self::RawWxPtr {
        self.ptr
    }
}

impl Drop for SearchCtrl {
    fn drop(&mut self) {
        // No-op: Parent wxWindow is responsible for destroying the C++ object for child controls.
        // Explicitly calling destroy here can lead to double-free errors.
        // If self.ptr is not null, it means the Rust wrapper is being dropped.
        // The C++ object's lifetime is tied to its wxWidgets parent.
        if !self.ptr.is_null() {
            // For debugging, one might log this.
            // println!("SearchCtrl wrapper dropped: {:?}", self.ptr);
            self.ptr = std::ptr::null_mut(); // Nullify the pointer to prevent use after drop of wrapper
        }
    }
}

// --- SearchCtrlStyle Enum ---
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum SearchCtrlStyle {
    Default = 0,
    ProcessEnter = ffi::WXD_TE_PROCESS_ENTER, // Assuming WXD_TE_PROCESS_ENTER is the correct FFI const
}

impl SearchCtrlStyle {
    pub fn bits(self) -> i64 {
        self as i64
    }
}

impl Default for SearchCtrlStyle {
    fn default() -> Self {
        SearchCtrlStyle::Default
    }
}

impl BitOr for SearchCtrlStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for SearchCtrlStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = unsafe { std::mem::transmute(self.bits() | rhs.bits()) };
    }
}

// --- SearchCtrlBuilder --- //

pub struct SearchCtrlBuilder {
    parent_ptr: *mut ffi::wxd_Window_t,
    id: Id,
    value: String,
    pos: Point,
    size: Size,
    style: SearchCtrlStyle,
}

impl SearchCtrlBuilder {
    fn new(parent: &impl WxWidget) -> Self {
        SearchCtrlBuilder {
            parent_ptr: parent.handle_ptr(),
            id: ID_ANY,
            value: String::new(),
            pos: Point { x: -1, y: -1 },
            size: Size {
                width: -1,
                height: -1,
            },
            style: SearchCtrlStyle::default(),
        }
    }

    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    pub fn with_value(mut self, value: &str) -> Self {
        self.value = value.to_string();
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

    pub fn with_style(mut self, style: SearchCtrlStyle) -> Self {
        self.style = style;
        self
    }

    pub fn build(self) -> SearchCtrl {
        let c_value = CString::new(self.value).expect("CString::new failed for value");
        let raw_ptr = unsafe {
            ffi::wxd_SearchCtrl_Create(
                self.parent_ptr,
                self.id,
                c_value.as_ptr(),
                self.pos.x,
                self.pos.y,
                self.size.width,
                self.size.height,
                self.style.bits() as c_long,
            )
        };
        if raw_ptr.is_null() {
            panic!("Failed to create wxSearchCtrl");
        }
        SearchCtrl::from_ptr(raw_ptr)
    }
}
