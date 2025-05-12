//! 
//! Safe wrapper for wxTextCtrl.

use crate::event::WxEvtHandler;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use std::os::raw::c_char;
use wxdragon_sys as ffi;

// --- Text Control Styles ---
widget_style_enum!(
    name: TextCtrlStyle,
    doc: "Style flags for TextCtrl widget.",
    variants: {
        Default: 0, "Default style (single line, editable, left-aligned).",
        MultiLine: ffi::WXD_TE_MULTILINE, "Multi-line text control.",
        Password: ffi::WXD_TE_PASSWORD, "Password entry control (displays characters as asterisks).",
        ReadOnly: ffi::WXD_TE_READONLY, "Read-only text control.",
        Rich: ffi::WXD_TE_RICH, "For rich text content (implies multiline). Use with care, may require specific handling.",
        Rich2: ffi::WXD_TE_RICH2, "For more advanced rich text content (implies multiline). Use with care.",
        AutoUrl: ffi::WXD_TE_AUTO_URL, "Automatically detect and make URLs clickable.",
        ProcessEnter: ffi::WXD_TE_PROCESS_ENTER, "Generate an event when Enter key is pressed."
    },
    default_variant: Default
);

/// Represents a wxTextCtrl widget.
#[derive(Clone)]
pub struct TextCtrl {
    window: Window,
}

impl TextCtrl {
    /// Creates a new TextCtrl builder.
    pub fn builder(parent: &dyn WxWidget) -> TextCtrlBuilder {
        TextCtrlBuilder::new(parent)
    }

    /// Creates a new TextCtrl wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_TextCtrl_t` pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_TextCtrl_t) -> Self {
        TextCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Internal implementation used by the builder.
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        value: &str,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        let c_value = CString::new(value).unwrap_or_default();
        
        let ptr = unsafe {
            ffi::wxd_TextCtrl_Create(
                parent_ptr,
                id,
                c_value.as_ptr(),
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t,
            )
        };
        
        if ptr.is_null() {
            panic!("Failed to create TextCtrl widget");
        }
        
        unsafe { TextCtrl::from_ptr(ptr) }
    }

    /// Sets the text value of the control.
    pub fn set_value(&self, value: &str) {
        let c_value = CString::new(value).unwrap_or_default();
        unsafe {
            ffi::wxd_TextCtrl_SetValue(
                self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t,
                c_value.as_ptr(),
            );
        }
    }

    /// Gets the current text value of the control.
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

// Apply common trait implementations for this widget
implement_widget_traits_with_target!(TextCtrl, window, Window);

// Use the widget_builder macro for TextCtrl
widget_builder!(
    name: TextCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: TextCtrlStyle,
    fields: {
        value: String = String::new()
    },
    build_impl: |slf| {
        TextCtrl::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            &slf.value,
            slf.pos,
            slf.size,
            slf.style.bits()
        )
    }
); 