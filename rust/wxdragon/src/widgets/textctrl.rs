use crate::prelude::*;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use wxdragon_sys as ffi;
// Remove unresolved Style import
// use ffi::Style;
use std::ops::Drop;
use std::ops::{BitOr, BitOrAssign};
use std::os::raw::c_char;

// Comment out unresolved constant import
// pub use ffi::TE_PROCESS_ENTER;
// pub const wxTE_PASSWORD: i64 = ffi::wxTE_PASSWORD; // Re-export if needed

// --- TextCtrl Style Flags ---
// REMOVED: pub type Style = i64; // Based on ffi::wxd_Style_t which is long
// Add more styles as needed
// REMOVED: pub const TE_PROCESS_ENTER: Style = ffi::WXD_TE_PROCESS_ENTER;

#[derive(Clone)]
pub struct TextCtrl {
    window: Window,
}

impl TextCtrl {
    pub fn builder(parent: &dyn WxWidget) -> TextCtrlBuilder {
        TextCtrlBuilder::new(parent)
    }

    fn new(
        parent: &dyn WxWidget,
        id: Id,
        value: &str,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Option<Self> {
        let c_value = CString::new(value).ok()?;
        unsafe {
            let parent_ptr = parent.handle_ptr();
            if parent_ptr.is_null() {
                return None;
            }
            let ctrl_ptr = ffi::wxd_TextCtrl_Create(
                parent_ptr as *mut _,
                id,
                c_value.as_ptr(),
                pos.into(),
                size.into(),
                style.try_into().unwrap(),
            );
            if ctrl_ptr.is_null() {
                None
            } else {
                let window = Window::from_ptr(ctrl_ptr as *mut ffi::wxd_Window_t);
                Some(TextCtrl { window })
            }
        }
    }

    pub fn set_value(&self, value: &str) {
        unsafe {
            let c_value = CString::new(value).unwrap_or_default();
            ffi::wxd_TextCtrl_SetValue(
                self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t,
                c_value.as_ptr(),
            );
        }
    }

    pub fn get_value(&self) -> String {
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 1024];
            let len = ffi::wxd_TextCtrl_GetValue(
                self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );
            if len >= 0 {
                let byte_slice =
                    std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
                String::from_utf8_lossy(byte_slice).to_string()
            } else {
                String::new()
            }
        }
    }
}

// Builder Pattern
#[derive(Clone)]
pub struct TextCtrlBuilder<'a> {
    parent: &'a dyn WxWidget,
    id: Id,
    value: String,
    pos: Point,
    size: Size,
    style: TextCtrlStyle,
}

impl<'a> TextCtrlBuilder<'a> {
    pub fn new(parent: &'a dyn WxWidget) -> Self {
        Self {
            parent,
            id: crate::id::ID_ANY as i32,
            value: String::new(),
            pos: Point { x: -1, y: -1 },
            size: Size {
                width: -1,
                height: -1,
            },
            style: TextCtrlStyle::Default,
        }
    }

    /// Sets the window identifier.
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    /// Sets the initial text value.
    pub fn with_value(mut self, value: &str) -> Self {
        self.value = value.to_string();
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

    /// Sets the window style flags.
    pub fn with_style(mut self, style: TextCtrlStyle) -> Self {
        self.style = style;
        self
    }

    pub fn build(self) -> TextCtrl {
        TextCtrl::new(
            self.parent,
            self.id,
            &self.value,
            self.pos,
            self.size,
            self.style.bits(),
        )
        .expect("Failed to create TextCtrl widget")
    }
}

// Implement WxWidget trait
impl WxWidget for TextCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

// Implement Drop (likely no-op like Button)
impl Drop for TextCtrl {
    fn drop(&mut self) {
        // No-op: Parent wxWindow is responsible for destroying the C++ object.
    }
}

// Allow TextCtrl to be used where a Window is expected
impl std::ops::Deref for TextCtrl {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

// --- TextCtrlStyle Enum ---

/// Style flags for `TextCtrl`.
///
/// These flags can be combined using the bitwise OR operator (`|`).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum TextCtrlStyle {
    /// Default style (single line, editable, left-aligned).
    Default = 0,
    /// Multi-line text control.
    MultiLine = ffi::WXD_TE_MULTILINE,
    /// Password entry control (displays characters as asterisks).
    Password = ffi::WXD_TE_PASSWORD,
    /// Read-only text control.
    ReadOnly = ffi::WXD_TE_READONLY,
    /// For rich text content (implies multiline). Use with care, may require specific handling.
    Rich = ffi::WXD_TE_RICH,
    /// For more advanced rich text content (implies multiline). Use with care.
    Rich2 = ffi::WXD_TE_RICH2,
    /// Automatically detect and make URLs clickable.
    AutoUrl = ffi::WXD_TE_AUTO_URL,
    /// Generate an event when Enter key is pressed.
    ProcessEnter = ffi::WXD_TE_PROCESS_ENTER,
    // /// Align text to the left (default).
    // AlignLeft = ffi::WXD_TE_LEFT, // Not yet available
    // /// Align text to the center.
    // AlignCenter = ffi::WXD_TE_CENTER, // Not yet available
    // /// Align text to the right.
    // AlignRight = ffi::WXD_TE_RIGHT, // Not yet available
    // /// Don't hide selection when focus is lost.
    // NoHideSelection = ffi::WXD_TE_NOHIDESEL, // Not yet available
    // /// Generate an event when Tab key is pressed (usually for single line controls).
    // ProcessTab = ffi::WXD_TE_PROCESS_TAB, // Not yet available
}

impl TextCtrlStyle {
    /// Returns the raw integer value of the style.
    pub fn bits(self) -> i64 {
        self as i64
    }
}

impl BitOr for TextCtrlStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for TextCtrlStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe {
            *self = std::mem::transmute(self.bits() | rhs.bits());
        }
    }
}
