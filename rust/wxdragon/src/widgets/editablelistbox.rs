use std::ffi::CString;

use wxdragon_sys as ffi;

use crate::implement_widget_traits_with_target;
use crate::prelude::*;
use crate::utils::WxdArrayString;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use crate::Id;

/// An editable listbox is a listbox with buttons to add, remove, and reorder items in the list.
pub struct EditableListBox {
    window: Window,
}

// Style flags for EditableListBox
widget_style_enum!(
    name: EditableListBoxStyle,
    doc: "Style flags for EditableListBox widget.",
    variants: {
        Default: 0, "Default style with no special behavior.",
        AllowNew: ffi::WXD_EL_ALLOW_NEW, "Enable the New button.",
        AllowEdit: ffi::WXD_EL_ALLOW_EDIT, "Enable the Edit button.",
        AllowDelete: ffi::WXD_EL_ALLOW_DELETE, "Enable the Delete button.",
        NoReorder: ffi::WXD_EL_NO_REORDER, "Disable the Up/Down buttons.",
        DefaultStyle: ffi::WXD_EL_DEFAULT_STYLE, "Default style (AllowNew | AllowEdit | AllowDelete)."
    },
    default_variant: DefaultStyle
);

// Implement widget traits
implement_widget_traits_with_target!(EditableListBox, window, Window);

impl EditableListBox {
    /// Create a new EditableListBox with default settings.
    ///
    /// This is a convenience method that creates a builder, sets default values,
    /// and immediately builds the EditableListBox.
    ///
    /// # Arguments
    ///
    /// * `parent` - The parent window
    /// * `label` - The label shown at the top of the listbox
    pub fn new(parent: &dyn WxWidget, label: &str) -> Self {
        Self::builder(parent)
            .with_label(label)
            .build()
    }

    /// Create a builder for configuring and creating an EditableListBox.
    pub fn builder(parent: &dyn WxWidget) -> EditableListBoxBuilder {
        EditableListBoxBuilder::new(parent)
    }

    /// Internal implementation for creating an EditableListBox directly.
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: i32,
        label: &str,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        let label_c = CString::new(label).unwrap_or_default();
        
        let ptr = unsafe {
            ffi::wxd_EditableListBox_New(
                parent_ptr,
                id,
                label_c.as_ptr(),
                pos.x,
                pos.y,
                size.width,
                size.height,
                style,
            )
        };
        
        assert!(!ptr.is_null(), "Failed to create EditableListBox");
        
        EditableListBox {
            window: unsafe { Window::from_ptr(ptr) },
        }
    }

    /// Get all strings in the listbox.
    pub fn get_strings(&self) -> Vec<String> {
        let array_str_ptr = unsafe {
            ffi::wxd_EditableListBox_CopyStringsToArrayString(self.window.handle_ptr())
        };

        if array_str_ptr.is_null() {
            return Vec::new();
        }

        let wxd_array_string = unsafe { WxdArrayString::from_ptr(array_str_ptr, true) };
        wxd_array_string.get_strings()
    }

    /// Set all strings in the listbox.
    pub fn set_strings(&self, strings: &[&str]) {
        let c_strings: Vec<CString> = strings.iter()
            .map(|s| CString::new(*s).unwrap_or_default())
            .collect();
        
        let mut c_ptrs: Vec<*const i8> = c_strings.iter()
            .map(|s| s.as_ptr())
            .collect();
        
        unsafe {
            ffi::wxd_EditableListBox_SetStrings(
                self.window.handle_ptr(),
                c_ptrs.as_mut_ptr(),
                c_strings.len() as i32,
            )
        }
    }

    /// Add a string to the listbox.
    pub fn add_string(&self, string: &str) {
        let c_string = CString::new(string).unwrap_or_default();
        
        unsafe {
            ffi::wxd_EditableListBox_AddString(
                self.window.handle_ptr(),
                c_string.as_ptr(),
            )
        }
    }

    /// Get the underlying ListBox control.
    pub fn get_list_ctrl(&self) -> Window {
        let ptr = unsafe {
            ffi::wxd_EditableListBox_GetListCtrl(
                self.window.handle_ptr()
            )
        };
        
        // We don't take ownership, just a reference
        unsafe { Window::from_ptr(ptr) }
    }
}

// Builder for EditableListBox
widget_builder!(
    name: EditableListBox,
    parent_type: &'a dyn WxWidget,
    style_type: EditableListBoxStyle,
    fields: {
        label: String = String::new()
    },
    build_impl: |slf| {
        EditableListBox::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            &slf.label,
            slf.pos,
            slf.size,
            slf.style.bits()
        )
    }
); 