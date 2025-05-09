// ! Safe wrapper for wxToggleButton.

use crate::base::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE};
use crate::id::{Id, ID_ANY};
use crate::window::{Window, WxWidget};
use std::ffi::{CStr, CString};
use std::ops::{BitOr, BitOrAssign};
use std::os::raw::c_char;
use wxdragon_sys as ffi;

/// Represents a wxToggleButton control.
#[derive(Clone)]
pub struct ToggleButton {
    window: Window,
}

impl ToggleButton {
    /// Creates a new `ToggleButtonBuilder`.
    pub fn builder(parent: &dyn WxWidget) -> ToggleButtonBuilder {
        ToggleButtonBuilder::new(parent)
    }

    /// Low-level constructor used by the builder.
    fn new(
        parent: &dyn WxWidget,
        id: Id,
        label: &str,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Option<Self> {
        unsafe {
            let parent_ptr = parent.handle_ptr();
            if parent_ptr.is_null() {
                return None;
            }
            let c_label = CString::new(label).ok()?;
            let ctrl_ptr = ffi::wxd_ToggleButton_Create(
                parent_ptr as *mut _,
                id,
                c_label.as_ptr(),
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t,
            );
            if ctrl_ptr.is_null() {
                None
            } else {
                Some(ToggleButton {
                    window: Window::from_ptr(ctrl_ptr as *mut ffi::wxd_Window_t),
                })
            }
        }
    }

    /// Gets the current state of the toggle button (true if pressed/down, false if not).
    pub fn get_value(&self) -> bool {
        unsafe { ffi::wxd_ToggleButton_GetValue(self.window.as_ptr() as *mut _) }
    }

    /// Sets the state of the toggle button.
    pub fn set_value(&self, state: bool) {
        unsafe { ffi::wxd_ToggleButton_SetValue(self.window.as_ptr() as *mut _, state) }
    }

    /// Sets the button label.
    pub fn set_label(&self, label: &str) {
        let c_label = CString::new(label).expect("Invalid CString for ToggleButton label");
        unsafe {
            ffi::wxd_ToggleButton_SetLabel(self.window.as_ptr() as *mut _, c_label.as_ptr());
        }
    }

    /// Gets the button label.
    pub fn get_label(&self) -> String {
        unsafe {
            let mut buffer: [c_char; 1024] = [0; 1024];
            let len_needed = ffi::wxd_ToggleButton_GetLabel(
                self.window.as_ptr() as *mut _,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );

            if len_needed < 0 {
                return String::new(); // Error
            }

            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                CStr::from_ptr(buffer.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            } else {
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_ToggleButton_GetLabel(
                    self.window.as_ptr() as *mut _,
                    vec_buffer.as_mut_ptr() as *mut c_char,
                    vec_buffer.len() as i32,
                );
                if len_copied == len_needed {
                    vec_buffer.pop(); // Remove null terminator
                    String::from_utf8(vec_buffer).unwrap_or_default()
                } else {
                    String::new() // Error on second call
                }
            }
        }
    }
}

// --- ToggleButton Builder ---

/// Builder pattern for creating `ToggleButton` widgets.
#[derive(Clone)]
pub struct ToggleButtonBuilder<'a> {
    parent: &'a dyn WxWidget,
    id: Id,
    label: String,
    pos: Option<Point>,
    size: Option<Size>,
    style: ToggleButtonStyle,
}

impl<'a> ToggleButtonBuilder<'a> {
    /// Creates a new builder.
    pub fn new(parent: &'a dyn WxWidget) -> Self {
        Self {
            parent,
            id: ID_ANY as Id,
            label: String::new(),
            pos: None,
            size: None,
            style: ToggleButtonStyle::Default,
        }
    }

    /// Sets the window identifier.
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    /// Sets the button label.
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

    /// Sets the window style flags (use constants from `togglebutton` module).
    pub fn with_style(mut self, style: ToggleButtonStyle) -> Self {
        self.style = style;
        self
    }

    /// Builds the `ToggleButton`.
    pub fn build(self) -> ToggleButton {
        let pos = self.pos.unwrap_or(DEFAULT_POSITION);
        let size = self.size.unwrap_or(DEFAULT_SIZE);
        ToggleButton::new(
            self.parent,
            self.id,
            &self.label,
            pos,
            size,
            self.style.bits(),
        )
        .expect("Failed to create ToggleButton")
    }
}

// --- Trait Implementations ---

impl WxWidget for ToggleButton {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

impl Drop for ToggleButton {
    fn drop(&mut self) {
        // Window's Drop handles cleanup via WxdCleaner
    }
}

impl std::ops::Deref for ToggleButton {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

// --- ToggleButtonStyle Enum ---

/// Style flags for `ToggleButton`.
/// These flags can be combined using the bitwise OR operator (`|`).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum ToggleButtonStyle {
    /// Default style (no specific alignment, standard border).
    Default = 0,
    /// Align label to the left.
    Left = ffi::WXD_BU_LEFT,
    /// Align label to the top.
    Top = ffi::WXD_BU_TOP,
    /// Align label to the right.
    Right = ffi::WXD_BU_RIGHT,
    /// Align label to the bottom.
    Bottom = ffi::WXD_BU_BOTTOM,
    /// Button size will be adjusted to exactly fit the label.
    ExactFit = ffi::WXD_BU_EXACTFIT,
    /// Do not display the label string (useful for buttons with only an image).
    NoText = ffi::WXD_BU_NOTEXT,
    /// No border.
    BorderNone = ffi::WXD_BORDER_NONE,
    // /// A simple border (rarely used for buttons, which have a default look).
    // BorderSimple = ffi::WXD_BORDER_SIMPLE, // Typically not used for ToggleButton
}

impl ToggleButtonStyle {
    /// Returns the raw integer value of the style.
    pub fn bits(self) -> i64 {
        self as i64
    }
}

impl BitOr for ToggleButtonStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for ToggleButtonStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe {
            *self = std::mem::transmute(self.bits() | rhs.bits());
        }
    }
}
