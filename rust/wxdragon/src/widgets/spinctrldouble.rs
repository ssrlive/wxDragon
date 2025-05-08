use crate::base::{Point, RawWxProps, Size, ID_ANY};
use crate::event::WxEvtHandler;
use crate::window::WxWidget;
use wxdragon_sys as ffi;

use std::ffi::CString;
use std::os::raw::c_long;
use std::ptr;

// --- SpinCtrlDouble --- //

#[derive(Debug, Clone)]
pub struct SpinCtrlDouble {
    ptr: *mut ffi::wxd_SpinCtrlDouble_t,
}

impl SpinCtrlDouble {
    pub fn builder(parent: &impl WxWidget) -> SpinCtrlDoubleBuilder {
        SpinCtrlDoubleBuilder::new(parent)
    }

    fn from_ptr(ptr: *mut ffi::wxd_SpinCtrlDouble_t) -> Self {
        SpinCtrlDouble { ptr }
    }

    pub fn get_value(&self) -> f64 {
        unsafe { ffi::wxd_SpinCtrlDouble_GetValue(self.ptr) }
    }

    pub fn set_value(&self, value: f64) {
        unsafe { ffi::wxd_SpinCtrlDouble_SetValue(self.ptr, value) }
    }

    pub fn set_range(&self, min_val: f64, max_val: f64) {
        unsafe { ffi::wxd_SpinCtrlDouble_SetRange(self.ptr, min_val, max_val) }
    }

    pub fn get_min(&self) -> f64 {
        unsafe { ffi::wxd_SpinCtrlDouble_GetMin(self.ptr) }
    }

    pub fn get_max(&self) -> f64 {
        unsafe { ffi::wxd_SpinCtrlDouble_GetMax(self.ptr) }
    }

    pub fn set_increment(&self, inc: f64) {
        unsafe { ffi::wxd_SpinCtrlDouble_SetIncrements(self.ptr, inc) } // C API is SetIncrements
    }

    pub fn get_increment(&self) -> f64 {
        unsafe { ffi::wxd_SpinCtrlDouble_GetIncrement(self.ptr) }
    }

    pub fn set_digits(&self, digits: u32) {
        unsafe { ffi::wxd_SpinCtrlDouble_SetDigits(self.ptr, digits) }
    }

    pub fn get_digits(&self) -> u32 {
        unsafe { ffi::wxd_SpinCtrlDouble_GetDigits(self.ptr) }
    }
}

impl WxWidget for SpinCtrlDouble {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.ptr as *mut ffi::wxd_Window_t
    }
}

impl WxEvtHandler for SpinCtrlDouble {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.ptr as *mut ffi::wxd_EvtHandler_t
    }
}

impl RawWxProps for SpinCtrlDouble {
    type RawWxPtr = ffi::wxd_SpinCtrlDouble_t;
    fn raw_wx_ptr(&self) -> *mut Self::RawWxPtr {
        self.ptr
    }
}

impl Drop for SpinCtrlDouble {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            self.ptr = ptr::null_mut();
        }
    }
}

// --- SpinCtrlDoubleBuilder --- //

pub struct SpinCtrlDoubleBuilder {
    parent_ptr: *mut ffi::wxd_Window_t,
    id: i32,
    value_str: String, // Initial text value
    pos: Point,
    size: Size,
    style: i64,
    min_val: f64,
    max_val: f64,
    initial_val: f64, // Initial numeric value
    increment: f64,
}

impl SpinCtrlDoubleBuilder {
    fn new(parent: &impl WxWidget) -> Self {
        SpinCtrlDoubleBuilder {
            parent_ptr: parent.handle_ptr(),
            id: ID_ANY,
            value_str: String::new(),
            pos: Point { x: -1, y: -1 },
            size: Size {
                width: -1,
                height: -1,
            },
            style: 0, // Default will be wxSP_ARROW_KEYS in C++ if 0
            min_val: 0.0,
            max_val: 100.0,
            initial_val: 0.0,
            increment: 1.0,
        }
    }

    pub fn with_id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    /// Sets the initial string value in the text part of the control.
    pub fn with_value_str(mut self, value_str: &str) -> Self {
        self.value_str = value_str.to_string();
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

    pub fn with_range(mut self, min_val: f64, max_val: f64) -> Self {
        self.min_val = min_val;
        self.max_val = max_val;
        self
    }

    /// Sets the initial numeric value of the control.
    pub fn with_initial_value(mut self, initial_val: f64) -> Self {
        self.initial_val = initial_val;
        self
    }

    pub fn with_increment(mut self, increment: f64) -> Self {
        self.increment = increment;
        self
    }

    pub fn build(self) -> SpinCtrlDouble {
        let c_value_str = CString::new(self.value_str).expect("CString::new failed for value_str");
        let raw_ptr = unsafe {
            ffi::wxd_SpinCtrlDouble_Create(
                self.parent_ptr,
                self.id,
                c_value_str.as_ptr(),
                self.pos.x,
                self.pos.y,
                self.size.width,
                self.size.height,
                self.style as c_long,
                self.min_val,
                self.max_val,
                self.initial_val,
                self.increment,
            )
        };
        if raw_ptr.is_null() {
            panic!("Failed to create wxSpinCtrlDouble");
        }
        SpinCtrlDouble::from_ptr(raw_ptr)
    }
}
