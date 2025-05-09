//! Safe wrapper for wxSlider.

use crate::base::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE, ID_ANY};
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::window::WxWidget;
use std::os::raw::c_int;
use wxdragon_sys as ffi;
use std::ops::{BitOr, BitOrAssign};

/// Represents a wxSlider widget.
#[derive(Clone)] // Keep Clone, as it just copies the pointer
pub struct Slider(pub(crate) *mut ffi::wxd_Slider_t);

impl Slider {
    /// Creates a new Slider builder.
    pub fn builder<W: WxWidget>(parent: &W) -> SliderBuilder {
        SliderBuilder::new(parent)
    }

    // Internal constructor
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_Slider_t) -> Self {
        Slider(ptr)
    }

    // --- Methods specific to Slider ---

    /// Gets the current slider value.
    pub fn value(&self) -> i32 {
        unsafe { ffi::wxd_Slider_GetValue(self.0) }
    }

    /// Sets the slider value.
    pub fn set_value(&self, value: i32) {
        unsafe { ffi::wxd_Slider_SetValue(self.0, value as c_int) };
    }

    /// Sets the slider range (minimum and maximum values).
    pub fn set_range(&self, min_value: i32, max_value: i32) {
        unsafe { ffi::wxd_Slider_SetRange(self.0, min_value as c_int, max_value as c_int) };
    }

    /// Gets the minimum slider value.
    pub fn min(&self) -> i32 {
        unsafe { ffi::wxd_Slider_GetMin(self.0) }
    }

    /// Gets the maximum slider value.
    pub fn max(&self) -> i32 {
        unsafe { ffi::wxd_Slider_GetMax(self.0) }
    }
}

// Implement the core WxWidget trait
impl WxWidget for Slider {
    /// Returns the raw underlying window pointer.
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.0 as *mut ffi::wxd_Window_t
    }
}

// Implement the event handling trait
impl WxEvtHandler for Slider {
    /// Returns the raw pointer to the underlying `wxd_EvtHandler_t`.
    /// # Safety
    /// The pointer must be valid.
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.0 as *mut ffi::wxd_EvtHandler_t
    }
}

/// Builder for creating `Slider` widgets.
pub struct SliderBuilder {
    parent: *mut ffi::wxd_Window_t,
    id: Id,
    value: i32,
    min_value: i32,
    max_value: i32,
    pos: Point,
    size: Size,
    style: SliderStyle,
}

impl SliderBuilder {
    /// Creates a new Slider builder with default values.
    pub fn new<W: WxWidget>(parent: &W) -> Self {
        SliderBuilder {
            parent: parent.handle_ptr(),
            id: ID_ANY,
            value: 0,
            min_value: 0,
            max_value: 100,
            pos: DEFAULT_POSITION,
            size: DEFAULT_SIZE,
            style: SliderStyle::Default,
        }
    }

    /// Sets the window ID.
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    /// Sets the initial value.
    pub fn with_value(mut self, value: i32) -> Self {
        self.value = value;
        self
    }

    /// Sets the minimum value.
    pub fn with_min_value(mut self, min_value: i32) -> Self {
        self.min_value = min_value;
        self
    }

    /// Sets the maximum value.
    pub fn with_max_value(mut self, max_value: i32) -> Self {
        self.max_value = max_value;
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
    pub fn with_style(mut self, style: SliderStyle) -> Self {
        self.style = style;
        self
    }

    /// Builds the Slider widget.
    pub fn build(self) -> Slider {
        let slider_ptr = unsafe {
            ffi::wxd_Slider_Create(
                self.parent as *mut ffi::wxd_Window_t,
                self.id as c_int,
                self.value as c_int,
                self.min_value as c_int,
                self.max_value as c_int,
                self.pos.into(),
                self.size.into(),
                self.style.bits() as ffi::wxd_Style_t,
            )
        };
        if slider_ptr.is_null() {
            panic!("Failed to create Slider");
        }
        unsafe { Slider::from_ptr(slider_ptr) }
    }
}

// --- SliderStyle Enum ---

/// Style flags for `Slider`.
/// These flags can be combined using the bitwise OR operator (`|`).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum SliderStyle {
    /// Default style (horizontal slider with no labels or ticks).
    Default = ffi::WXD_SL_HORIZONTAL,
    /// Vertical slider.
    Vertical = ffi::WXD_SL_VERTICAL,
    /// Display tick marks.
    AutoTicks = ffi::WXD_SL_AUTOTICKS,
    /// Display labels (min, max, and current value).
    Labels = ffi::WXD_SL_LABELS,
    /// Display min and max labels only.
    MinMaxLabels = ffi::WXD_SL_MIN_MAX_LABELS,
    /// Display the current value as a label.
    ValueLabel = ffi::WXD_SL_VALUE_LABEL,
    /// Show ticks on both sides of the slider (not always supported or visually distinct).
    BothSides = ffi::WXD_SL_BOTH,
}

impl SliderStyle {
    /// Returns the raw integer value of the style.
    pub fn bits(self) -> i64 {
        self as i64
    }
}

impl BitOr for SliderStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for SliderStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe { *self = std::mem::transmute(self.bits() | rhs.bits()); }
    }
}
