use crate::base::{Point, Size};
use crate::event::WxEvtHandler;
use crate::id::{Id, ID_ANY};
use crate::window::{Window, WxWidget};
use std::ffi::CString;
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
    style: i64,
}

impl<'a> CheckBoxBuilder<'a> {
    pub fn new(parent: &'a dyn WxWidget) -> Self {
        Self {
            parent,
            id: ID_ANY as Id,
            label: String::new(),
            pos: None,
            size: None,
            style: 0,
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
    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    /// Builds the `CheckBox`.
    pub fn build(self) -> CheckBox {
        let parent_ptr = self.parent.handle_ptr();
        let pos = self.pos.unwrap_or_default();
        let size = self.size.unwrap_or_default();
        CheckBox::new_impl(parent_ptr, self.id, &self.label, pos, size, self.style)
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
