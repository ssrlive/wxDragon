//! Safe wrapper for wxSpinCtrl.

use crate::geometry::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE};
use crate::id::ID_ANY;
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::window::WxWidget;
use std::ffi::CString;
use std::ops::{BitOr, BitOrAssign};
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// Re-export constants from wxdragon-sys

/// Represents a wxSpinCtrl widget.
#[derive(Clone)]
pub struct SpinCtrl(pub(crate) *mut ffi::wxd_SpinCtrl_t);

impl SpinCtrl {
    /// Creates a new SpinCtrl builder.
    pub fn builder<W: WxWidget>(parent: &W) -> SpinCtrlBuilder {
        SpinCtrlBuilder::new(parent)
    }

    // Internal constructor
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_SpinCtrl_t) -> Self {
        SpinCtrl(ptr)
    }

    // --- Methods specific to SpinCtrl ---

    /// Gets the current value.
    pub fn value(&self) -> i32 {
        unsafe { ffi::wxd_SpinCtrl_GetValue(self.0) }
    }

    /// Sets the value.
    pub fn set_value(&self, value: i32) {
        unsafe { ffi::wxd_SpinCtrl_SetValue(self.0, value as c_int) };
    }

    /// Sets the allowed range.
    pub fn set_range(&self, min_val: i32, max_val: i32) {
        unsafe { ffi::wxd_SpinCtrl_SetRange(self.0, min_val as c_int, max_val as c_int) };
    }

    /// Gets the minimum allowed value.
    pub fn min(&self) -> i32 {
        unsafe { ffi::wxd_SpinCtrl_GetMin(self.0) }
    }

    /// Gets the maximum allowed value.
    pub fn max(&self) -> i32 {
        unsafe { ffi::wxd_SpinCtrl_GetMax(self.0) }
    }
}

// Implement the core WxWidget trait
impl WxWidget for SpinCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.0 as *mut ffi::wxd_Window_t
    }
}

// Implement the event handling trait
impl WxEvtHandler for SpinCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.0 as *mut ffi::wxd_EvtHandler_t
    }
}

/// Builder for creating `SpinCtrl` widgets.
pub struct SpinCtrlBuilder {
    parent: *mut ffi::wxd_Window_t,
    id: Id,
    value: String, // Initial value is string in C API
    pos: Point,
    size: Size,
    style: SpinCtrlStyle,
    min_val: i32,
    max_val: i32,
    initial_val: i32,
}

impl SpinCtrlBuilder {
    /// Creates a new SpinCtrl builder with default values.
    pub fn new<W: WxWidget>(parent: &W) -> Self {
        SpinCtrlBuilder {
            parent: parent.handle_ptr(),
            id: ID_ANY as i32,
            value: "0".to_string(), // Default initial string
            pos: DEFAULT_POSITION,
            size: DEFAULT_SIZE,
            style: SpinCtrlStyle::Default,
            min_val: 0,
            max_val: 100,
            initial_val: 0, // Default initial numeric value
        }
    }

    /// Sets the window ID.
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    /// Sets the initial numeric value (overrides initial string value).
    pub fn with_initial_value(mut self, initial_val: i32) -> Self {
        self.initial_val = initial_val;
        // Update string value representation as well for consistency if needed
        self.value = initial_val.to_string();
        self
    }

    /// Sets the minimum allowed value.
    pub fn with_min_value(mut self, min_val: i32) -> Self {
        self.min_val = min_val;
        self
    }

    /// Sets the maximum allowed value.
    pub fn with_max_value(mut self, max_val: i32) -> Self {
        self.max_val = max_val;
        self
    }

    /// Sets the allowed range.
    pub fn with_range(mut self, min_val: i32, max_val: i32) -> Self {
        self.min_val = min_val;
        self.max_val = max_val;
        // Adjust initial value if it's outside the new range
        self.initial_val = self.initial_val.clamp(min_val, max_val);
        self.value = self.initial_val.to_string();
        self
    }

    /// Sets the position.
    pub fn with_pos(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    /// Sets the size.
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Sets the style flags.
    pub fn with_style(mut self, style: SpinCtrlStyle) -> Self {
        self.style = style;
        self
    }

    /// Builds the SpinCtrl widget.
    pub fn build(self) -> SpinCtrl {
        let initial_c_string =
            CString::new(self.value).expect("CString::new failed for SpinCtrl initial value");
        let spin_ctrl_ptr = unsafe {
            ffi::wxd_SpinCtrl_Create(
                self.parent,
                self.id as c_int,
                initial_c_string.as_ptr(),
                self.pos.into(),
                self.size.into(),
                self.style.bits() as ffi::wxd_Style_t,
                self.min_val as c_int,
                self.max_val as c_int,
                self.initial_val as c_int,
            )
        };
        if spin_ctrl_ptr.is_null() {
            panic!("Failed to create SpinCtrl");
        }
        unsafe { SpinCtrl::from_ptr(spin_ctrl_ptr) }
    }
}

// --- SpinCtrlStyle Enum ---

/// Style flags for `SpinCtrl`.
/// These flags can be combined using the bitwise OR operator (`|`).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum SpinCtrlStyle {
    /// Default style (vertical, arrow keys enabled).
    Default = ffi::WXD_SP_VERTICAL | ffi::WXD_SP_ARROW_KEYS,
    /// Horizontal spin control.
    Horizontal = ffi::WXD_SP_HORIZONTAL,
    /// Vertical spin control.
    Vertical = ffi::WXD_SP_VERTICAL,
    /// Allow using arrow keys to change the value.
    ArrowKeys = ffi::WXD_SP_ARROW_KEYS,
    /// The value wraps around when incrementing/decrementing past max/min.
    Wrap = ffi::WXD_SP_WRAP,
    /// Process the Enter key press event (generates a command event).
    ProcessEnter = ffi::WXD_TE_PROCESS_ENTER,
}

impl SpinCtrlStyle {
    /// Returns the raw integer value of the style.
    pub fn bits(self) -> i64 {
        self as i64
    }
}

impl BitOr for SpinCtrlStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for SpinCtrlStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe {
            *self = std::mem::transmute(self.bits() | rhs.bits());
        }
    }
}
