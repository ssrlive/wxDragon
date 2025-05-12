use crate::geometry::{Point, Size};
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::widget_builder;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use std::ops::{BitOr, BitOrAssign};
use wxdragon_sys as ffi;

// Re-export specific CheckBox constants if needed later

/// Represents a wxCheckBox.
#[derive(Clone)]
pub struct CheckBox {
    window: Window,
}

impl CheckBox {
    /// Creates a new `CheckBoxBuilder` for constructing a checkbox.
    pub fn builder(parent: &dyn WxWidget) -> CheckBoxBuilder {
        CheckBoxBuilder::new(parent)
    }

    /// Low-level constructor used by the builder.
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        label: &str,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        let label_c = CString::new(label).unwrap_or_default();
        let ctrl_ptr = unsafe {
            ffi::wxd_CheckBox_Create(
                parent_ptr,
                id,
                label_c.as_ptr(),
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t,
            )
        };
        assert!(!ctrl_ptr.is_null(), "wxd_CheckBox_Create returned null");
        unsafe { Self::from_ptr(ctrl_ptr) }
    }

    /// Returns `true` if the checkbox is checked, `false` otherwise.
    pub fn is_checked(&self) -> bool {
        unsafe { ffi::wxd_CheckBox_IsChecked(self.window.as_ptr() as *mut ffi::wxd_CheckBox_t) }
    }

    /// Sets the checkbox to the given state.
    pub fn set_value(&self, value: bool) {
        unsafe {
            ffi::wxd_CheckBox_SetValue(self.window.as_ptr() as *mut ffi::wxd_CheckBox_t, value);
        }
    }

    // Private unsafe constructor from raw pointer
    unsafe fn from_ptr(ptr: *mut ffi::wxd_CheckBox_t) -> Self {
        assert!(!ptr.is_null());
        CheckBox {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }
}

// Use the widget_builder macro to generate the CheckBoxBuilder implementation
widget_builder!(
    name: CheckBox,
    parent_type: &'a dyn WxWidget,
    style_type: CheckBoxStyle,
    fields: {},
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        CheckBox::new_impl(
            parent_ptr,
            slf.id,
            &slf.label,
            slf.pos,
            slf.size,
            slf.style.bits(),
        )
    }
);

// Implement WxWidget trait
impl WxWidget for CheckBox {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

// Implement Drop (no-op for child widgets)
impl Drop for CheckBox {
    fn drop(&mut self) {}
}

// Allow CheckBox to be used where a Window is expected via Deref
impl std::ops::Deref for CheckBox {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

// Implement WxEvtHandler for CheckBox by delegating to the inner Window
impl WxEvtHandler for CheckBox {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.window.get_event_handler_ptr()
    }
}

// --- CheckBoxStyle Enum ---

/// Style flags for `CheckBox`.
///
/// These flags can be combined using the bitwise OR operator (`|`).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum CheckBoxStyle {
    /// Default style (2-state, label on the right).
    Default = ffi::WXD_CHK_2STATE,
    /// Three-state checkbox. The third state is "undetermined".
    ThreeState = ffi::WXD_CHK_3STATE,
    /// Allows the user to set the checkbox to the third state (undetermined).
    /// Only applicable if `ThreeState` is also used.
    AllowUserThirdState = ffi::WXD_CHK_ALLOW_3RD_STATE_FOR_USER,
    /// Align label to the right of the checkbox (checkbox on the left).
    /// This is usually the default layout.
    AlignLeft = 0, // Standard behavior, no specific flag needed to achieve this if others aren't set.
    /// Align label to the left of the checkbox (checkbox on the right).
    AlignRight = ffi::WXD_ALIGN_RIGHT,
}

impl CheckBoxStyle {
    /// Returns the raw integer value of the style.
    pub fn bits(self) -> i64 {
        self as i64
    }
}

impl BitOr for CheckBoxStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for CheckBoxStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe {
            *self = std::mem::transmute(self.bits() | rhs.bits());
        }
    }
}
