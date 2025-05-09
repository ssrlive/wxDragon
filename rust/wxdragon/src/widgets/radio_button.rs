//! Safe wrapper for wxRadioButton.

use crate::base::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE};
use crate::event::WxEvtHandler;
use crate::id::{Id, ID_ANY};
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use wxdragon_sys as ffi;
// use std::ops::{BitOr, BitOrAssign}; // Commented out as BitOr/BitOrAssign not implemented for RadioButtonStyle

// Opaque pointer type from FFI
pub type RawRadioButton = ffi::wxd_RadioButton_t;

/// Represents a wxRadioButton control.
#[derive(Clone)]
pub struct RadioButton {
    window: Window,
}

impl RadioButton {
    /// Private constructor from raw pointer
    unsafe fn from_ptr(ptr: *mut RawRadioButton) -> Self {
        assert!(!ptr.is_null());
        RadioButton {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Creates a new `RadioButtonBuilder`.
    pub fn builder(parent: &impl WxWidget) -> RadioButtonBuilder {
        RadioButtonBuilder::new(parent)
    }

    /// Shared implementation called by builder
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        label: &str,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        unsafe {
            let label_c = CString::new(label).unwrap_or_default();
            let ctrl_ptr = ffi::wxd_RadioButton_Create(
                parent_ptr,
                id,
                label_c.as_ptr(),
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t,
            );
            assert!(!ctrl_ptr.is_null(), "wxd_RadioButton_Create returned null");
            Self::from_ptr(ctrl_ptr)
        }
    }

    /// Gets the state of the radio button.
    pub fn get_value(&self) -> bool {
        unsafe { ffi::wxd_RadioButton_GetValue(self.window.handle_ptr() as *mut RawRadioButton) }
    }

    /// Sets the state of the radio button.
    /// Note: Setting a radio button to `true` will implicitly set others in the same group to `false`.
    pub fn set_value(&self, value: bool) {
        unsafe {
            ffi::wxd_RadioButton_SetValue(self.window.handle_ptr() as *mut RawRadioButton, value);
        }
    }
}

// Delegate WxWidget and WxEvtHandler to the inner Window
impl WxWidget for RadioButton {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

impl WxEvtHandler for RadioButton {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.window.get_event_handler_ptr()
    }
}

// --- RadioButton Builder ---

/// Builder pattern for creating `RadioButton` widgets.
#[derive(Clone)]
pub struct RadioButtonBuilder<'a> {
    parent: &'a dyn WxWidget,
    id: Id,
    label: String,
    pos: Option<Point>,
    size: Option<Size>,
    style: RadioButtonStyle,
}

impl<'a> RadioButtonBuilder<'a> {
    /// Creates a new builder.
    pub fn new(parent: &'a dyn WxWidget) -> Self {
        Self {
            parent,
            id: ID_ANY as Id,
            label: String::new(),
            pos: None,
            size: None,
            style: RadioButtonStyle::Default,
        }
    }

    /// Sets the window identifier.
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    /// Sets the label.
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    /// Sets the position.
    pub fn with_pos(mut self, x: i32, y: i32) -> Self {
        self.pos = Some(Point { x, y });
        self
    }

    /// Sets the size.
    pub fn with_size(mut self, width: i32, height: i32) -> Self {
        self.size = Some(Size { width, height });
        self
    }

    /// Sets the window style flags.
    pub fn with_style(mut self, style: RadioButtonStyle) -> Self {
        self.style = style;
        self
    }

    /// Marks this radio button as the first in a group.
    /// This typically means subsequent radio buttons (until the next `RB_GROUP` or end of dialog)
    /// belong to the same group.
    pub fn first_in_group(mut self) -> Self {
        self.style = RadioButtonStyle::GroupStart;
        self
    }

    /// Builds the `RadioButton`.
    pub fn build(self) -> RadioButton {
        let parent_ptr = self.parent.handle_ptr();
        let pos = self.pos.unwrap_or(DEFAULT_POSITION);
        let size = self.size.unwrap_or(DEFAULT_SIZE);
        RadioButton::new_impl(parent_ptr, self.id, &self.label, pos, size, self.style.bits())
    }
}

// Implement Drop (no-op for child widgets)
impl Drop for RadioButton {
    fn drop(&mut self) {}
}

// Allow RadioButton to be used where a Window is expected via Deref
impl std::ops::Deref for RadioButton {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

// --- RadioButtonStyle Enum ---

/// Style flags for `RadioButton`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum RadioButtonStyle {
    /// Default style. Represents a standalone radio button or a subsequent button in a group.
    Default = 0,
    /// Marks this radio button as the first in a new group.
    /// Subsequent radio buttons (until the next `GroupStart` or end of dialog)
    /// belong to the same group, where only one can be selected.
    GroupStart = ffi::WXD_RB_GROUP,
    // WXD_RB_SINGLE is usually implied for subsequent buttons in a group
    // or for standalone buttons if not using GroupStart.
    // Explicitly: Single = ffi::WXD_RB_SINGLE, // Not typically set directly by user
}

impl RadioButtonStyle {
    /// Returns the raw integer value of the style.
    pub fn bits(self) -> i64 {
        self as i64
    }
}

// RadioButton styles are not typically combined with bitwise OR by users.
// RB_GROUP is a distinct state. If other combinable flags existed,
// BitOr/BitOrAssign would be more relevant. For now, they are omitted
// to keep the API simple and reflect typical wxRadioButton usage.
// If needed, they can be added:
// impl BitOr for RadioButtonStyle { ... }
// impl BitOrAssign for RadioButtonStyle { ... }
