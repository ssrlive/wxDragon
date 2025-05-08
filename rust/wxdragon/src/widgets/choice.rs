//! Safe wrapper for wxChoice.

use crate::base::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE};
use crate::event::WxEvtHandler;
use crate::id::{Id, WXD_ID_ANY};
use crate::window::{Window, WxWidget};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wxdragon_sys as ffi;

// --- Constants ---
// Style flags (uses ComboBox flags)
pub const CB_SORT: i64 = ffi::WXD_CB_SORT as i64;
// Special value returned by GetSelection when nothing is selected
pub const NOT_FOUND: i32 = -1; // wxNOT_FOUND is typically -1

// Opaque pointer type from FFI
pub type RawChoice = ffi::wxd_Choice_t;

/// Represents a wxChoice control (dropdown list).
#[derive(Clone)]
pub struct Choice {
    window: Window,
}

impl Choice {
    /// Creates a new `ChoiceBuilder`.
    pub fn builder(parent: &impl WxWidget) -> ChoiceBuilder {
        ChoiceBuilder::new(parent)
    }

    /// Shared implementation called by builder
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        unsafe {
            let ctrl_ptr = ffi::wxd_Choice_Create(
                parent_ptr,
                id,
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t, // Cast style to FFI type (5 args)
            );
            assert!(!ctrl_ptr.is_null(), "wxd_Choice_Create returned null");
            Self::from_ptr(ctrl_ptr)
        }
    }

    // Unsafe constructor from raw pointer
    unsafe fn from_ptr(ptr: *mut RawChoice) -> Self {
        assert!(!ptr.is_null());
        Choice {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Appends an item to the choice control.
    pub fn append(&self, item: &str) {
        let c_item = CString::new(item).expect("Invalid CString for Choice item");
        unsafe {
            ffi::wxd_Choice_Append(self.window.as_ptr() as *mut _, c_item.as_ptr());
        }
    }

    /// Removes all items from the choice control.
    pub fn clear(&self) {
        unsafe {
            ffi::wxd_Choice_Clear(self.window.as_ptr() as *mut _);
        }
    }

    /// Gets the index of the currently selected item.
    /// Returns `None` if no item is selected (matches `NOT_FOUND`).
    pub fn get_selection(&self) -> Option<u32> {
        let selection = unsafe { ffi::wxd_Choice_GetSelection(self.window.as_ptr() as *mut _) };
        if selection == NOT_FOUND {
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
            let len_needed = ffi::wxd_Choice_GetStringSelection(
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
                let len_copied = ffi::wxd_Choice_GetStringSelection(
                    self.window.as_ptr() as *mut _,
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

    /// Selects the item at the given index.
    pub fn set_selection(&self, index: u32) {
        unsafe {
            ffi::wxd_Choice_SetSelection(self.window.as_ptr() as *mut _, index as i32);
        }
    }

    /// Gets the string at the specified index.
    /// Returns `None` if the index is out of bounds.
    pub fn get_string(&self, index: u32) -> Option<String> {
        unsafe {
            let mut buffer: [c_char; 1024] = [0; 1024];
            let len_needed = ffi::wxd_Choice_GetString(
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
                let len_copied = ffi::wxd_Choice_GetString(
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

    /// Gets the number of items in the choice control.
    pub fn get_count(&self) -> u32 {
        unsafe { ffi::wxd_Choice_GetCount(self.window.as_ptr() as *mut _) }
    }
}

// --- Choice Builder ---

/// Builder pattern for creating `Choice` widgets.
pub struct ChoiceBuilder<'a> {
    parent: &'a dyn WxWidget,
    id: Id,
    pos: Option<Point>,
    size: Option<Size>,
    style: i64,
    choices: Vec<String>,
}

impl<'a> ChoiceBuilder<'a> {
    /// Creates a new builder.
    pub fn new(parent: &'a dyn WxWidget) -> Self {
        Self {
            parent,
            id: WXD_ID_ANY as Id,
            pos: None,
            size: None,
            style: 0,
            choices: Vec::new(),
        }
    }

    /// Sets the window identifier.
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    /// Sets the position.
    pub fn with_pos(mut self, x: i32, y: i32) -> Self {
        self.pos = Some(Point { x, y });
        self
    }

    /// Sets the size.
    pub fn with_size(mut self, width: i32, height: i32) -> Self {
        self.size = Some(Size { width, height });
        self
    }

    /// Sets the window style flags (e.g., `CB_SORT`).
    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    /// Sets the initial items in the choice control.
    pub fn with_choices(mut self, choices: &[&str]) -> Self {
        self.choices = choices.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Builds the `Choice`.
    pub fn build(self) -> Choice {
        let parent_ptr = self.parent.handle_ptr();
        let pos = self.pos.unwrap_or(DEFAULT_POSITION);
        let size = self.size.unwrap_or(DEFAULT_SIZE);
        let choice_ctrl = Choice::new_impl(parent_ptr, self.id, pos, size, self.style);

        // Append initial choices if any
        for choice_str in &self.choices {
            choice_ctrl.append(choice_str);
        }

        choice_ctrl
    }
}

// Implement WxWidget trait
impl WxWidget for Choice {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

// Implement Drop (no-op for child widgets)
impl Drop for Choice {
    fn drop(&mut self) {}
}

// Allow Choice to be used where a Window is expected via Deref
impl std::ops::Deref for Choice {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

// Implement WxEvtHandler trait
impl WxEvtHandler for Choice {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.window.get_event_handler_ptr()
    }
}
