//! Safe wrapper for wxComboBox.

use crate::geometry::{Point, Size};
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::window::{Window, WxWidget};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wxdragon_sys as ffi;

// Value for GetSelection when nothing selected
pub const NOT_FOUND: i32 = -1;

// Opaque pointer type from FFI
pub type RawComboBox = ffi::wxd_ComboBox_t;

/// Represents a wxComboBox control (dropdown list + text entry).
#[derive(Clone)]
pub struct ComboBox {
    window: Window,
}

impl ComboBox {
    /// Creates a new `ComboBoxBuilder`.
    pub fn builder(parent: &dyn WxWidget) -> ComboBoxBuilder<'_> {
        ComboBoxBuilder::new(parent)
    }

    /// Appends an item to the combobox list.
    pub fn append(&self, item: &str) {
        let c_item = CString::new(item).expect("Invalid CString for ComboBox item");
        unsafe {
            ffi::wxd_ComboBox_Append(self.window.as_ptr() as *mut _, c_item.as_ptr());
        }
    }

    /// Removes all items from the combobox list.
    /// Does not clear the text entry field value.
    pub fn clear(&self) {
        unsafe {
            ffi::wxd_ComboBox_Clear(self.window.as_ptr() as *mut _);
        }
    }

    /// Gets the index of the selected item in the list.
    /// Returns `None` if no item is selected or if the text doesn't match an item.
    pub fn get_selection(&self) -> Option<u32> {
        let selection = unsafe { ffi::wxd_ComboBox_GetSelection(self.window.as_ptr() as *mut _) };
        if selection == NOT_FOUND {
            None
        } else {
            Some(selection as u32)
        }
    }

    /// Gets the string value of the currently selected item in the list.
    /// Returns `None` if no item is selected.
    pub fn get_string_selection(&self) -> Option<String> {
        // Reuse the get_string_from_ffi helper
        get_string_from_ffi(|buffer, len| unsafe {
            ffi::wxd_ComboBox_GetStringSelection(self.window.as_ptr() as *mut _, buffer, len)
        })
    }

    /// Selects the item at the given index in the list.
    /// This also updates the text entry field to the selected string.
    pub fn set_selection(&self, index: u32) {
        unsafe {
            ffi::wxd_ComboBox_SetSelection(self.window.as_ptr() as *mut _, index as i32);
        }
    }

    /// Gets the string at the specified index in the list.
    /// Returns `None` if the index is out of bounds.
    pub fn get_string(&self, index: u32) -> Option<String> {
        get_string_from_ffi(|buffer, len| unsafe {
            ffi::wxd_ComboBox_GetString(self.window.as_ptr() as *mut _, index as i32, buffer, len)
        })
    }

    /// Gets the number of items in the combobox list.
    pub fn get_count(&self) -> u32 {
        unsafe { ffi::wxd_ComboBox_GetCount(self.window.as_ptr() as *mut _) }
    }

    /// Gets the current text value from the text entry field.
    pub fn get_value(&self) -> String {
        get_string_from_ffi(|buffer, len| unsafe {
            ffi::wxd_ComboBox_GetValue(self.window.as_ptr() as *mut _, buffer, len)
        })
        .unwrap_or_default() // Should always return a string, even if empty
    }

    /// Sets the text value in the text entry field.
    pub fn set_value(&self, value: &str) {
        let c_value = CString::new(value).expect("Invalid CString for ComboBox value");
        unsafe {
            ffi::wxd_ComboBox_SetValue(self.window.as_ptr() as *mut _, c_value.as_ptr());
        }
    }
}

// --- Style enum using macro ---
widget_style_enum!(
    name: ComboBoxStyle,
    doc: "Style flags for ComboBox widget.",
    variants: {
        Default: ffi::WXD_CB_DROPDOWN, "Default style: a regular dropdown combo box.",
        Simple: ffi::WXD_CB_SIMPLE, "A simple combo box with a permanently displayed list.",
        Sort: ffi::WXD_CB_SORT, "The list of items is kept sorted alphabetically.",
        ReadOnly: ffi::WXD_CB_READONLY, "The text field is read-only (user can only select from the list).",
        ProcessEnter: ffi::WXD_TE_PROCESS_ENTER, "Process the Enter key, generating a TEXT_ENTER event."
    },
    default_variant: Default
);

// --- Builder pattern using macro ---
widget_builder!(
    name: ComboBox,
    parent_type: &'a dyn WxWidget,
    style_type: ComboBoxStyle,
    fields: {
        value: String = String::new(),
        choices: Vec<String> = Vec::new()
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        assert!(!parent_ptr.is_null(), "ComboBox requires a parent");
        
        let c_value = CString::new(slf.value.as_str()).expect("Invalid CString for ComboBox value");
        
        unsafe {
            let ctrl_ptr = ffi::wxd_ComboBox_Create(
                parent_ptr,
                slf.id,
                c_value.as_ptr(),
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
            );
            
            if ctrl_ptr.is_null() {
                panic!("Failed to create ComboBox widget");
            } else {
                let window = Window::from_ptr(ctrl_ptr as *mut ffi::wxd_Window_t);
                let combo = ComboBox { window };
                
                // Append initial choices
                for item in &slf.choices {
                    combo.append(item);
                }
                
                combo
            }
        }
    }
);

// Add a convenience method to handle &[&str] choices
impl<'a> ComboBoxBuilder<'a> {
    /// Sets the initial items in the dropdown list from string slices.
    pub fn with_string_choices(mut self, choices: &[&str]) -> Self {
        self.choices = choices.iter().map(|s| s.to_string()).collect();
        self
    }
}

// --- Widget traits implementation using macro ---
implement_widget_traits_with_target!(ComboBox, window, Window);

// --- Helper Function --- (Consider moving to a common utils module)

/// Helper function to safely get a String from an FFI function
/// that follows the C pattern of returning buffer length needed
/// and writing to a provided buffer.
pub(crate) fn get_string_from_ffi<F>(ffi_getter: F) -> Option<String>
where
    F: Fn(*mut c_char, i32) -> i32,
{
    unsafe {
        let mut buffer: [c_char; 1024] = [0; 1024];
        let len_needed = ffi_getter(buffer.as_mut_ptr(), buffer.len() as i32);

        if len_needed < 0 {
            return None; // Indicates error
        }
        if len_needed == 0 {
            return Some(String::new()); // Empty string
        }

        let len_needed_usize = len_needed as usize;
        if len_needed_usize < buffer.len() {
            let c_str = CStr::from_ptr(buffer.as_ptr());
            Some(c_str.to_string_lossy().into_owned())
        } else {
            let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
            let len_copied = ffi_getter(
                vec_buffer.as_mut_ptr() as *mut c_char,
                vec_buffer.len() as i32,
            );
            if len_copied == len_needed {
                vec_buffer.pop(); // Remove null terminator
                String::from_utf8(vec_buffer).ok()
            } else {
                eprintln!("get_string_from_ffi: Length mismatch on second call");
                None // Error on second call
            }
        }
    }
}
