// ! Safe wrapper for wxCheckListBox.

use crate::geometry::{Point, Size};
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::implement_widget_traits;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wxdragon_sys as ffi;

// Create a style enum for CheckListBox, reusing the values from ListBoxStyle
widget_style_enum!(
    name: CheckListBoxStyle,
    doc: "Style flags for the CheckListBox widget.",
    variants: {
        Default: 0, "Default style.",
        Single: ffi::WXD_LB_SINGLE, "Single-selection list.",
        Multiple: ffi::WXD_LB_MULTIPLE, "Multiple-selection list.",
        Extended: ffi::WXD_LB_EXTENDED, "Extended-selection list.",
        HScroll: ffi::WXD_LB_HSCROLL, "Create horizontal scrollbar if contents are too wide.",
        AlwaysSB: ffi::WXD_LB_ALWAYS_SB, "Always show a vertical scrollbar.",
        Sort: ffi::WXD_LB_SORT, "Sort strings in the list alphabetically."
    },
    default_variant: Default
);

/// Represents a wxCheckListBox control, which combines a ListBox with checkboxes.
#[derive(Clone)]
pub struct CheckListBox {
    window: Window,
}

impl CheckListBox {
    /// Creates a new `CheckListBoxBuilder` for constructing a check list box control.
    pub fn builder(parent: &dyn WxWidget) -> CheckListBoxBuilder {
        CheckListBoxBuilder::new(parent)
    }

    /// Appends an item to the list box.
    pub fn append(&self, item: &str) {
        let c_item = CString::new(item).expect("Invalid CString for CheckListBox item");
        unsafe {
            ffi::wxd_CheckListBox_Append(self.window.as_ptr() as *mut _, c_item.as_ptr());
        }
    }

    /// Removes all items from the list box.
    pub fn clear(&self) {
        unsafe {
            ffi::wxd_CheckListBox_Clear(self.window.as_ptr() as *mut _);
        }
    }

    /// Gets the index of the currently selected item.
    /// Returns `None` if no item is selected (matches `NOT_FOUND`).
    pub fn get_selection(&self) -> Option<u32> {
        let selection =
            unsafe { ffi::wxd_CheckListBox_GetSelection(self.window.as_ptr() as *mut _) };
        if selection == -1 {
            None
        } else {
            Some(selection as u32)
        }
    }

    /// Gets the string value of the currently selected item.
    /// Returns `None` if no item is selected.
    pub fn get_string_selection(&self) -> Option<String> {
        unsafe {
            let mut buffer: [c_char; 1024] = [0; 1024];
            let len_needed = ffi::wxd_CheckListBox_GetStringSelection(
                self.window.as_ptr() as *mut _,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );

            if len_needed < 0 {
                return None; // Error or no selection
            }

            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                let c_str = CStr::from_ptr(buffer.as_ptr());
                Some(c_str.to_string_lossy().into_owned())
            } else {
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_CheckListBox_GetStringSelection(
                    self.window.as_ptr() as *mut _,
                    vec_buffer.as_mut_ptr() as *mut c_char,
                    vec_buffer.len() as i32,
                );
                if len_copied == len_needed {
                    vec_buffer.pop(); // Remove null terminator
                    String::from_utf8(vec_buffer).ok()
                } else {
                    None // Error on second call
                }
            }
        }
    }

    /// Selects or deselects an item at the given index.
    pub fn set_selection(&self, index: u32, select: bool) {
        unsafe {
            ffi::wxd_CheckListBox_SetSelection(
                self.window.as_ptr() as *mut _,
                index as i32,
                select,
            );
        }
    }

    /// Gets the string at the specified index.
    /// Returns `None` if the index is out of bounds.
    pub fn get_string(&self, index: u32) -> Option<String> {
        unsafe {
            let mut buffer: [c_char; 1024] = [0; 1024];
            let len_needed = ffi::wxd_CheckListBox_GetString(
                self.window.as_ptr() as *mut _,
                index as i32,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );

            if len_needed < 0 {
                return None; // Error or invalid index
            }

            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                let c_str = CStr::from_ptr(buffer.as_ptr());
                Some(c_str.to_string_lossy().into_owned())
            } else {
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_CheckListBox_GetString(
                    self.window.as_ptr() as *mut _,
                    index as i32,
                    vec_buffer.as_mut_ptr() as *mut c_char,
                    vec_buffer.len() as i32,
                );
                if len_copied == len_needed {
                    vec_buffer.pop();
                    String::from_utf8(vec_buffer).ok()
                } else {
                    None
                }
            }
        }
    }

    /// Gets the number of items in the list box.
    pub fn get_count(&self) -> u32 {
        unsafe { ffi::wxd_CheckListBox_GetCount(self.window.as_ptr() as *mut _) }
    }

    /// Checks if the item at the given index is checked.
    /// Returns `false` if the index is out of bounds.
    pub fn is_checked(&self, index: u32) -> bool {
        unsafe { ffi::wxd_CheckListBox_IsChecked(self.window.as_ptr() as *mut _, index) }
    }

    /// Sets the checked state of the item at the given index.
    /// Does nothing if the index is out of bounds.
    pub fn check(&self, index: u32, check: bool) {
        unsafe { ffi::wxd_CheckListBox_Check(self.window.as_ptr() as *mut _, index, check) }
    }
}

widget_builder!(
    name: CheckListBox,
    parent_type: &'a dyn WxWidget,
    style_type: CheckListBoxStyle,
    fields: {
        choices: Vec<String> = Vec::new()
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        let pos = slf.pos.into();
        let size = slf.size.into();
        
        // Create the control
        let ctrl_ptr = unsafe {
            ffi::wxd_CheckListBox_Create(
                parent_ptr,
                slf.id,
                pos,
                size,
                slf.style.bits(),
            )
        };
        
        if ctrl_ptr.is_null() {
            panic!("Failed to create CheckListBox widget");
        }
        
        let clbox = CheckListBox {
            window: unsafe { Window::from_ptr(ctrl_ptr as *mut ffi::wxd_Window_t) },
        };

        // Append initial choices
        for choice_str in &slf.choices {
            clbox.append(choice_str);
        }
        
        clbox
    }
);

implement_widget_traits!(CheckListBox, window);
