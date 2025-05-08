// ! Safe wrapper for wxCheckListBox.

use crate::base::{Point, Size};
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::window::{Window, WxWidget};
use std::default::Default; // Import Default
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wxdragon_sys as ffi;

// Re-export constants from listbox module as they are the same
pub mod checklistbox {
    pub use crate::widgets::listbox::{
        LB_ALWAYS_SB, LB_EXTENDED, LB_HSCROLL, LB_MULTIPLE, LB_SINGLE, LB_SORT, NOT_FOUND,
    };
}

// Opaque pointer type from FFI
pub type RawCheckListBox = ffi::wxd_CheckListBox_t;

/// Represents a wxCheckListBox control.
#[derive(Clone)]
pub struct CheckListBox {
    window: Window,
}

impl CheckListBox {
    /// Creates a new `CheckListBoxBuilder`.
    pub fn builder(parent: &dyn WxWidget) -> CheckListBoxBuilder {
        let mut builder = CheckListBoxBuilder::default();
        builder.parent_ptr = parent.handle_ptr();
        builder
    }

    /// Low-level constructor used by the builder's `build` method.
    fn new(
        parent_ptr: *mut ffi::wxd_Window_t, // Take raw ptr
        id: Id,
        pos: Point,
        size: Size,
        style: i64,
        choices: &[&str], // Accept initial choices
    ) -> Option<Self> {
        unsafe {
            if parent_ptr.is_null() {
                return None;
            }
            let ctrl_ptr = ffi::wxd_CheckListBox_Create(
                parent_ptr,
                id,
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t, // Cast to FFI type
            );
            if ctrl_ptr.is_null() {
                None
            } else {
                let window = Window::from_ptr(ctrl_ptr as *mut ffi::wxd_Window_t);
                let clbox = CheckListBox { window };
                // Append initial choices
                for choice in choices {
                    clbox.append(choice);
                }
                Some(clbox)
            }
        }
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
        if selection == checklistbox::NOT_FOUND {
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

// --- CheckListBox Builder ---

/// Builder pattern for creating `CheckListBox` widgets.
//#[derive(Clone)] // Clone requires choices to be Clone
// #[derive(Default)] // Cannot derive Default
pub struct CheckListBoxBuilder {
    parent_ptr: *mut ffi::wxd_Window_t,
    id: Id,
    pos: Point,
    size: Size,
    style: i64,
    choices: Vec<String>, // Store initial choices as Strings
}

// Manual Default implementation
impl Default for CheckListBoxBuilder {
    fn default() -> Self {
        Self {
            parent_ptr: std::ptr::null_mut(),
            id: -1,                      // Explicit ID_ANY
            pos: Point { x: -1, y: -1 }, // Explicit default
            size: Size {
                width: -1,
                height: -1,
            }, // Explicit default
            style: 0,                    // Default style
            choices: Vec::new(),
        }
    }
}

impl CheckListBoxBuilder {
    /// Sets the window identifier.
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
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

    /// Sets the window style flags (use constants from `checklistbox` module).
    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    /// Sets the initial items in the list box.
    pub fn with_choices(mut self, choices: &[&str]) -> Self {
        self.choices = choices.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Builds the `CheckListBox`.
    pub fn build(self) -> CheckListBox {
        assert!(!self.parent_ptr.is_null(), "CheckListBox requires a parent");
        let choice_slices: Vec<&str> = self.choices.iter().map(|s| s.as_str()).collect();

        CheckListBox::new(
            self.parent_ptr, // Pass raw ptr
            self.id,
            self.pos,
            self.size,
            self.style,
            &choice_slices,
        )
        .expect("Failed to create CheckListBox widget")
    }
}

// --- Trait Implementations ---

impl WxWidget for CheckListBox {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

impl Drop for CheckListBox {
    fn drop(&mut self) {
        // Window's Drop implementation handles cleanup via WxdCleaner
        // No specific CheckListBox cleanup needed here
    }
}

impl std::ops::Deref for CheckListBox {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl WxEvtHandler for CheckListBox {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.window.get_event_handler_ptr()
    }
}
