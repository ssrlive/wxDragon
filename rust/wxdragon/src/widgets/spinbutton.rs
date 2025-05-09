//! Safe wrapper for wxSpinButton.

use crate::base::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE, ID_ANY};
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::window::WxWidget;
use std::os::raw::c_int;
use wxdragon_sys as ffi;
use std::ops::{BitOr, BitOrAssign};

// Re-export constants (same as SpinCtrl, potentially defined there or in prelude)
// For clarity, let's redefine them here or ensure they are accessible.

/// Represents a wxSpinButton widget.
#[derive(Clone)]
pub struct SpinButton(pub(crate) *mut ffi::wxd_SpinButton_t);

impl SpinButton {
    /// Creates a new SpinButton builder.
    pub fn builder<W: WxWidget>(parent: &W) -> SpinButtonBuilder {
        SpinButtonBuilder::new(parent)
    }

    // Internal constructor
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_SpinButton_t) -> Self {
        SpinButton(ptr)
    }

    // --- Methods specific to SpinButton ---

    /// Gets the current value.
    pub fn value(&self) -> i32 {
        unsafe { ffi::wxd_SpinButton_GetValue(self.0) }
    }

    /// Sets the value.
    pub fn set_value(&self, value: i32) {
        unsafe { ffi::wxd_SpinButton_SetValue(self.0, value as c_int) };
    }

    /// Sets the allowed range.
    pub fn set_range(&self, min_val: i32, max_val: i32) {
        unsafe { ffi::wxd_SpinButton_SetRange(self.0, min_val as c_int, max_val as c_int) };
    }

    /// Gets the minimum allowed value.
    pub fn min(&self) -> i32 {
        unsafe { ffi::wxd_SpinButton_GetMin(self.0) }
    }

    /// Gets the maximum allowed value.
    pub fn max(&self) -> i32 {
        unsafe { ffi::wxd_SpinButton_GetMax(self.0) }
    }
}

// Implement the core WxWidget trait
impl WxWidget for SpinButton {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.0 as *mut ffi::wxd_Window_t
    }
}

// Implement the event handling trait
impl WxEvtHandler for SpinButton {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.0 as *mut ffi::wxd_EvtHandler_t
    }
}

/// Builder for creating `SpinButton` widgets.
pub struct SpinButtonBuilder {
    parent: *mut ffi::wxd_Window_t,
    id: Id,
    pos: Point,
    size: Size,
    style: SpinButtonStyle,
    // Range is set after creation, but we can add builder methods
    min_val: Option<i32>,
    max_val: Option<i32>,
    initial_val: Option<i32>,
}

impl SpinButtonBuilder {
    /// Creates a new SpinButton builder with default values.
    pub fn new<W: WxWidget>(parent: &W) -> Self {
        SpinButtonBuilder {
            parent: parent.handle_ptr(),
            id: ID_ANY,
            pos: DEFAULT_POSITION,
            size: DEFAULT_SIZE,
            style: SpinButtonStyle::Default,
            min_val: None,
            max_val: None,
            initial_val: None,
        }
    }

    /// Sets the window ID.
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
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
    pub fn with_style(mut self, style: SpinButtonStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the minimum value.
    pub fn with_min_value(mut self, min_val: i32) -> Self {
        self.min_val = Some(min_val);
        self
    }

    /// Sets the maximum allowed value.
    pub fn with_max_value(mut self, max_val: i32) -> Self {
        self.max_val = Some(max_val);
        self
    }

    /// Sets the allowed range.
    pub fn with_range(mut self, min_val: i32, max_val: i32) -> Self {
        self.min_val = Some(min_val);
        self.max_val = Some(max_val);
        self
    }

    /// Sets the initial numeric value.
    pub fn with_initial_value(mut self, initial_val: i32) -> Self {
        self.initial_val = Some(initial_val);
        self
    }

    /// Builds the SpinButton widget.
    pub fn build(self) -> SpinButton {
        let spin_button_ptr = unsafe {
            ffi::wxd_SpinButton_Create(
                self.parent,
                self.id as c_int,
                self.pos.into(),
                self.size.into(),
                self.style.bits() as ffi::wxd_Style_t,
            )
        };
        if spin_button_ptr.is_null() {
            panic!("Failed to create SpinButton");
        }
        let spin_button = unsafe { SpinButton::from_ptr(spin_button_ptr) };

        // Set range and initial value after creation if specified
        let min = self.min_val.unwrap_or(0); // Default min 0
        let max = self.max_val.unwrap_or(100); // Default max 100
        if self.min_val.is_some() || self.max_val.is_some() {
            spin_button.set_range(min, max);
        }
        if let Some(val) = self.initial_val {
            // Clamp initial value to range
            spin_button.set_value(val.clamp(min, max));
        }

        spin_button
    }
}

// --- SpinButtonStyle Enum ---

/// Style flags for `SpinButton`.
/// These flags can be combined using the bitwise OR operator (`|`).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum SpinButtonStyle {
    /// Default style (vertical spin button).
    Default = ffi::WXD_SP_VERTICAL,
    /// Horizontal spin button.
    Horizontal = ffi::WXD_SP_HORIZONTAL,
    /// Allow using arrow keys to change the value.
    ArrowKeys = ffi::WXD_SP_ARROW_KEYS,
    /// The value wraps around when incrementing/decrementing past max/min.
    Wrap = ffi::WXD_SP_WRAP,
}

impl SpinButtonStyle {
    /// Returns the raw integer value of the style.
    pub fn bits(self) -> i64 {
        self as i64
    }
}

impl BitOr for SpinButtonStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for SpinButtonStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe { *self = std::mem::transmute(self.bits() | rhs.bits()); }
    }
}
