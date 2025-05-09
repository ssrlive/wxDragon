use crate::base::{Point, Size};
use crate::event::WxEvtHandler;
use crate::id::{Id, ID_ANY};
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

// --- CheckBox Builder ---

/// Builder pattern for creating `CheckBox` widgets.
#[derive(Clone)]
pub struct CheckBoxBuilder<'a> {
    parent: &'a dyn WxWidget,
    id: Id,
    label: String,
    pos: Option<Point>,
    size: Option<Size>,
    style: CheckBoxStyle,
}

impl<'a> CheckBoxBuilder<'a> {
    pub fn new(parent: &'a dyn WxWidget) -> Self {
        Self {
            parent,
            id: ID_ANY as Id,
            label: String::new(),
            pos: None,
            size: None,
            style: CheckBoxStyle::Default,
        }
    }

    /// Sets the window identifier.
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    /// Sets the checkbox label.
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    /// Sets the position.
    pub fn with_pos(mut self, pos: Point) -> Self {
        self.pos = Some(pos);
        self
    }

    /// Sets the size.
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the window style flags.
    pub fn with_style(mut self, style: CheckBoxStyle) -> Self {
        self.style = style;
        self
    }

    /// Builds the `CheckBox`.
    pub fn build(self) -> CheckBox {
        let parent_ptr = self.parent.handle_ptr();
        let pos = self.pos.unwrap_or_default();
        let size = self.size.unwrap_or_default();
        CheckBox::new_impl(
            parent_ptr,
            self.id,
            &self.label,
            pos,
            size,
            self.style.bits(),
        )
    }
}

// Implement WxWidget trait
impl WxWidget for CheckBox {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

// Allow CheckBox to be used where a Window is expected (read-only access)
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
