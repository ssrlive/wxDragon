use crate::prelude::*;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use wxdragon_sys as ffi;
// Remove unresolved Style import
// use ffi::Style;
use std::ops::Drop;
use std::os::raw::c_char;

// Comment out unresolved constant import
// pub use ffi::TE_PROCESS_ENTER;
// pub const wxTE_PASSWORD: i64 = ffi::wxTE_PASSWORD; // Re-export if needed

// --- TextCtrl Style Flags ---
pub type Style = i64; // Based on ffi::wxd_Style_t which is long
                      // Add more styles as needed
pub const TE_PROCESS_ENTER: Style = ffi::WXD_TE_PROCESS_ENTER;

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
    style: i64,
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
            style: 0,
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
    pub fn with_style(mut self, style: i64) -> Self {
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
            self.style,
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
