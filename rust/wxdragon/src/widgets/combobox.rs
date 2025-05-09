//! Safe wrapper for wxComboBox.

use crate::base::{Point, Size};
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::window::{Window, WxWidget};
use std::default::Default;
use std::ffi::{CStr, CString};
use std::ops::{BitOr, BitOrAssign};
use std::os::raw::c_char;
use wxdragon_sys as ffi;

// --- Constants ---
// Style flags
// REMOVED: pub const CB_SIMPLE: i64 = ffi::WXD_CB_SIMPLE;
// REMOVED: pub const CB_SORT: i64 = ffi::WXD_CB_SORT;
// REMOVED: pub const CB_READONLY: i64 = ffi::WXD_CB_READONLY;
// REMOVED: pub const CB_DROPDOWN: i64 = ffi::WXD_CB_DROPDOWN;
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
    pub fn builder(parent: &dyn WxWidget) -> ComboBoxBuilder {
        let mut builder = ComboBoxBuilder::default();
        builder.parent_ptr = parent.handle_ptr();
        builder
    }

    /// Low-level constructor used by the builder's `build` method.
    fn new(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        value: &str,
        pos: Point,
        size: Size,
        style: i64,
        choices: &[&str],
    ) -> Option<Self> {
        let c_value = CString::new(value).ok()?;
        unsafe {
            if parent_ptr.is_null() {
                return None;
            }
            let ctrl_ptr = ffi::wxd_ComboBox_Create(
                parent_ptr,
                id,
                c_value.as_ptr(),
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t,
            );
            if ctrl_ptr.is_null() {
                None
            } else {
                let window = Window::from_ptr(ctrl_ptr as *mut ffi::wxd_Window_t);
                let combo = ComboBox { window };
                // Append initial choices
                for item in choices {
                    combo.append(item);
                }
                Some(combo)
            }
        }
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

// --- ComboBox Builder ---

/// Builder pattern for creating `ComboBox` widgets.
pub struct ComboBoxBuilder {
    parent_ptr: *mut ffi::wxd_Window_t,
    id: Id,
    value: String,
    pos: Point,
    size: Size,
    style: ComboBoxStyle,
    choices: Vec<String>,
}

// Manual Default implementation
impl Default for ComboBoxBuilder {
    fn default() -> Self {
        Self {
            parent_ptr: std::ptr::null_mut(),
            id: -1, // Explicit ID_ANY
            value: String::new(),
            pos: Point { x: -1, y: -1 }, // Explicit default
            size: Size {
                width: -1,
                height: -1,
            }, // Explicit default
            style: ComboBoxStyle::Default,
            choices: Vec::new(),
        }
    }
}

impl ComboBoxBuilder {
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
    pub fn with_style(mut self, style: ComboBoxStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the initial items in the dropdown list.
    pub fn with_choices(mut self, choices: &[&str]) -> Self {
        self.choices = choices.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Builds the `ComboBox`.
    pub fn build(self) -> ComboBox {
        assert!(!self.parent_ptr.is_null(), "ComboBox requires a parent");
        let choice_slices: Vec<&str> = self.choices.iter().map(|s| s.as_str()).collect();
        ComboBox::new(
            self.parent_ptr,
            self.id,
            &self.value,
            self.pos,
            self.size,
            self.style.bits(),
            &choice_slices,
        )
        .expect("Failed to create ComboBox widget")
    }
}

// Implement WxWidget trait
impl WxWidget for ComboBox {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

// Implement Drop (no-op for child widgets)
impl Drop for ComboBox {
    fn drop(&mut self) {}
}

// Allow ComboBox to be used where a Window is expected via Deref
impl std::ops::Deref for ComboBox {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

// Implement WxEvtHandler trait
impl WxEvtHandler for ComboBox {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.window.handle_ptr() as *mut _
    }
}

// --- ComboBoxStyle Enum ---

/// Style flags for `ComboBox`.
///
/// These flags can be combined using the bitwise OR operator (`|`).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum ComboBoxStyle {
    /// Default style: a regular dropdown combo box.
    Default = ffi::WXD_CB_DROPDOWN,
    /// A simple combo box with a permanently displayed list.
    Simple = ffi::WXD_CB_SIMPLE,
    /// The list of items is kept sorted alphabetically.
    Sort = ffi::WXD_CB_SORT,
    /// The text field is read-only (user can only select from the list).
    ReadOnly = ffi::WXD_CB_READONLY,
    /// Process the Enter key, generating a TEXT_ENTER event.
    ProcessEnter = ffi::WXD_TE_PROCESS_ENTER,
}

impl ComboBoxStyle {
    /// Returns the raw integer value of the style.
    pub fn bits(self) -> i64 {
        self as i64
    }
}

impl BitOr for ComboBoxStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for ComboBoxStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe {
            *self = std::mem::transmute(self.bits() | rhs.bits());
        }
    }
}

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
