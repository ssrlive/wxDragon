use crate::event::WxEvtHandler;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use wxdragon_sys as ffi;

use std::ffi::CString;
use std::os::raw::{c_char, c_longlong};
use std::ptr;

// --- Style enum using macro ---
widget_style_enum!(
    name: SearchCtrlStyle,
    doc: "Style flags for SearchCtrl",
    variants: {
        Default: 0, "Default style.",
        ProcessEnter: ffi::WXD_TE_PROCESS_ENTER, "Process Enter key press."
    },
    default_variant: Default
);

// --- SearchCtrl --- //

#[derive(Clone)]
pub struct SearchCtrl {
    window: Window,
}

impl SearchCtrl {
    pub fn builder(parent: &dyn WxWidget) -> SearchCtrlBuilder {
        SearchCtrlBuilder::new(parent)
    }

    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_SearchCtrl_t) -> Self {
        SearchCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Get the raw underlying search ctrl pointer.
    pub fn as_ptr(&self) -> *mut ffi::wxd_SearchCtrl_t {
        self.window.handle_ptr() as *mut ffi::wxd_SearchCtrl_t
    }

    pub fn show_search_button(&self, show: bool) {
        unsafe { ffi::wxd_SearchCtrl_ShowSearchButton(self.as_ptr(), show) }
    }

    pub fn is_search_button_visible(&self) -> bool {
        unsafe { ffi::wxd_SearchCtrl_IsSearchButtonVisible(self.as_ptr()) }
    }

    pub fn show_cancel_button(&self, show: bool) {
        unsafe { ffi::wxd_SearchCtrl_ShowCancelButton(self.as_ptr(), show) }
    }

    pub fn is_cancel_button_visible(&self) -> bool {
        unsafe { ffi::wxd_SearchCtrl_IsCancelButtonVisible(self.as_ptr()) }
    }

    pub fn set_value(&self, value: &str) {
        let c_value = CString::new(value).expect("CString::new failed for value");
        unsafe {
            ffi::wxd_TextCtrl_SetValue(self.as_ptr() as *mut ffi::wxd_TextCtrl_t, c_value.as_ptr())
        }
    }

    pub fn get_value(&self) -> String {
        let len_needed = unsafe {
            ffi::wxd_TextCtrl_GetValue(
                self.as_ptr() as *mut ffi::wxd_TextCtrl_t,
                ptr::null_mut(),
                0,
            )
        };

        if len_needed <= 0 {
            return String::new();
        }
        let buffer_size = len_needed as usize;
        let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size);

        let actual_len = unsafe {
            ffi::wxd_TextCtrl_GetValue(
                self.as_ptr() as *mut ffi::wxd_TextCtrl_t,
                buffer.as_mut_ptr() as *mut c_char,
                len_needed,
            )
        };

        if actual_len <= 0 {
            return String::new();
        }
        unsafe {
            buffer.set_len(usize::max(0, (actual_len - 1) as usize));
        }
        String::from_utf8(buffer).unwrap_or_default()
    }
}

// Apply common trait implementations
implement_widget_traits_with_target!(SearchCtrl, window, Window);

// Use the widget_builder macro to generate the SearchCtrlBuilder implementation
widget_builder!(
    name: SearchCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: SearchCtrlStyle,
    fields: {
        value: String = String::new()
    },
    build_impl: |slf| {
        let c_value = CString::new(slf.value.as_str()).expect("CString::new failed for value");
        let raw_ptr = unsafe {
            ffi::wxd_SearchCtrl_Create(
                slf.parent.handle_ptr(),
                slf.id,
                c_value.as_ptr(),
                slf.pos.x,
                slf.pos.y,
                slf.size.width,
                slf.size.height,
                slf.style.bits() as c_longlong,
            )
        };

        if raw_ptr.is_null() {
            panic!("Failed to create wxSearchCtrl");
        }

        unsafe { SearchCtrl::from_ptr(raw_ptr) }
    }
);
