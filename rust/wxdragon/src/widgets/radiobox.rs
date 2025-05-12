use crate::geometry::{Point, Size};
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use wxdragon_sys as ffi;

// Constants from wxWidgets for RadioBox
// Values populated by const_extractor via ffi
// pub const RA_SPECIFY_COLS: i64 = ffi::WXD_RA_SPECIFY_COLS;
// pub const RA_SPECIFY_ROWS: i64 = ffi::WXD_RA_SPECIFY_ROWS;
// wxRB_GROUP, wxRB_SINGLE seem less relevant for RadioBox itself, more for RadioButton
// Default style includes wxRA_SPECIFY_COLS

/// Represents a wxRadioBox control.
#[derive(Clone)]
pub struct RadioBox {
    window: Window,
}

impl RadioBox {
    /// Creates a new `RadioBoxBuilder`.
    pub fn builder<'a>(parent: &'a dyn WxWidget, choices: &'a [&'a str]) -> RadioBoxBuilder<'a> {
        // Create a new builder with the parent and convert choices to Strings
        let mut builder = RadioBoxBuilder::new(parent);
        builder.choices = choices.iter().map(|&s| s.to_string()).collect();
        builder
    }

    /// Creates a `RadioBox` from a raw pointer.
    /// # Safety
    /// The caller must ensure the pointer is valid and represents a `wxRadioBox`.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_RadioBox_t) -> Self {
        RadioBox {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Low-level constructor used by the builder.
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        label: &str,
        choices: &[&str],
        major_dimension: i32,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        assert!(!parent_ptr.is_null(), "RadioBox requires a parent");
        let c_label = CString::new(label).expect("CString::new failed for label");

        let c_choices: Vec<CString> = choices
            .iter()
            .map(|&s| CString::new(s).expect("CString::new failed for choice"))
            .collect();
        let c_choices_ptrs: Vec<*const c_char> = c_choices.iter().map(|cs| cs.as_ptr()).collect();

        let ptr = unsafe {
            ffi::wxd_RadioBox_Create(
                parent_ptr,
                id,
                c_label.as_ptr(),
                pos.into(),
                size.into(),
                choices.len() as i32,
                c_choices_ptrs.as_ptr(),
                major_dimension,
                style as ffi::wxd_Style_t,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create wxRadioBox");
        }
        unsafe { RadioBox::from_ptr(ptr) }
    }

    pub fn get_selection(&self) -> i32 {
        unsafe { ffi::wxd_RadioBox_GetSelection(self.as_ptr()) }
    }

    pub fn set_selection(&self, n: i32) {
        unsafe { ffi::wxd_RadioBox_SetSelection(self.as_ptr(), n) }
    }

    pub fn get_string(&self, n: i32) -> String {
        unsafe {
            let required_len_p1 = ffi::wxd_RadioBox_GetString(self.as_ptr(), n, ptr::null_mut(), 0);
            if required_len_p1 <= 0 {
                return String::new();
            }
            let capacity = required_len_p1 as usize;
            let mut buffer: Vec<u8> = Vec::with_capacity(capacity);
            let success_code = ffi::wxd_RadioBox_GetString(
                self.as_ptr(),
                n,
                buffer.as_mut_ptr() as *mut c_char,
                capacity as i32,
            );

            if success_code == 0 {
                // Use CStr to find the length, including potential embedded nulls handled correctly.
                let c_str = CStr::from_ptr(buffer.as_ptr() as *const c_char);
                String::from_utf8_lossy(c_str.to_bytes()).into_owned()
            } else {
                String::new() // Error occurred during copy
            }
        }
    }

    pub fn get_count(&self) -> u32 {
        unsafe { ffi::wxd_RadioBox_GetCount(self.as_ptr()) }
    }

    pub fn enable_item(&self, n: i32, enable: bool) -> bool {
        unsafe { ffi::wxd_RadioBox_EnableItem(self.as_ptr(), n, enable) }
    }

    pub fn is_item_enabled(&self, n: i32) -> bool {
        unsafe { ffi::wxd_RadioBox_IsItemEnabled(self.as_ptr(), n) }
    }

    pub fn show_item(&self, n: i32, show: bool) -> bool {
        unsafe { ffi::wxd_RadioBox_ShowItem(self.as_ptr(), n, show) }
    }

    pub fn is_item_shown(&self, n: i32) -> bool {
        unsafe { ffi::wxd_RadioBox_IsItemShown(self.as_ptr(), n) }
    }

    /// Returns the raw wxRadioBox pointer.
    fn as_ptr(&self) -> *mut ffi::wxd_RadioBox_t {
        self.window.as_ptr() as *mut _
    }
}

// Apply common trait implementations
implement_widget_traits_with_target!(RadioBox, window, Window);

// Use the widget_builder macro for RadioBox
widget_builder!(
    name: RadioBox,
    parent_type: &'a dyn WxWidget,
    style_type: RadioBoxStyle,
    fields: {
        label: String = String::new(),
        choices: Vec<String> = Vec::new(),
        major_dimension: i32 = 0
    },
    build_impl: |slf| {
        // Convert Vec<String> to Vec<&str> for the new_impl function
        let choices_refs: Vec<&str> = slf.choices.iter().map(|s| s.as_str()).collect();
        
        RadioBox::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            &slf.label,
            &choices_refs,
            slf.major_dimension,
            slf.pos,
            slf.size,
            slf.style.bits(),
        )
    }
);

// Define the RadioBoxStyle enum using the widget_style_enum macro
widget_style_enum!(
    name: RadioBoxStyle,
    doc: "Style flags for RadioBox widgets.",
    variants: {
        Default: 0, "Default layout (wxWidgets decides based on major dimension).",
        SpecifyCols: ffi::WXD_RA_SPECIFY_COLS, "Arrange items in columns primarily.",
        SpecifyRows: ffi::WXD_RA_SPECIFY_ROWS, "Arrange items in rows primarily."
    },
    default_variant: Default
);
