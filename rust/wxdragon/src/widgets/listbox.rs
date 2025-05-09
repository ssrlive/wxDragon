//! Safe wrapper for wxListBox.

use crate::base::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE};
use crate::event::WxEvtHandler;
use crate::id::{Id, ID_ANY};
use crate::window::{Window, WxWidget};
use std::ffi::{CStr, CString};
use std::ops::{BitOr, BitOrAssign};
use std::os::raw::c_char;
use wxdragon_sys as ffi;

// --- Constants ---
// Style flags
// REMOVED: pub const LB_SINGLE: i64 = ffi::WXD_LB_SINGLE;
// REMOVED: pub const LB_MULTIPLE: i64 = ffi::WXD_LB_MULTIPLE;
// REMOVED: pub const LB_EXTENDED: i64 = ffi::WXD_LB_EXTENDED;
// REMOVED: pub const LB_SORT: i64 = ffi::WXD_LB_SORT;
// REMOVED: pub const LB_ALWAYS_SB: i64 = ffi::WXD_LB_ALWAYS_SB;
// REMOVED: pub const LB_HSCROLL: i64 = ffi::WXD_LB_HSCROLL;
// Special value returned by GetSelection when nothing is selected
pub const NOT_FOUND: i32 = -1; // wxNOT_FOUND is typically -1

// Opaque pointer type from FFI
pub type RawListBox = ffi::wxd_ListBox_t;

/// Represents a wxListBox control.
#[derive(Clone)]
pub struct ListBox {
    window: Window,
}

impl ListBox {
    /// Creates a new `ListBoxBuilder`.
    pub fn builder(parent: &impl WxWidget) -> ListBoxBuilder {
        ListBoxBuilder::new(parent)
    }

    /// Low-level constructor used by the builder.
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        unsafe {
            let ctrl_ptr = ffi::wxd_ListBox_Create(
                parent_ptr,
                id,
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t, // Pass 5 args
            );
            assert!(!ctrl_ptr.is_null(), "wxd_ListBox_Create returned null");
            Self::from_ptr(ctrl_ptr)
        }
    }

    unsafe fn from_ptr(ptr: *mut RawListBox) -> Self {
        assert!(!ptr.is_null());
        ListBox {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Appends an item to the list box.
    pub fn append(&self, item: &str) {
        let c_item = CString::new(item).expect("Invalid CString for ListBox item");
        unsafe {
            ffi::wxd_ListBox_Append(self.window.as_ptr() as *mut _, c_item.as_ptr());
        }
    }

    /// Removes all items from the list box.
    pub fn clear(&self) {
        unsafe {
            ffi::wxd_ListBox_Clear(self.window.as_ptr() as *mut _);
        }
    }

    /// Gets the index of the currently selected item.
    /// Returns `None` if no item is selected (matches `NOT_FOUND`).
    /// Note: For multi-selection list boxes, this returns the *first* selected item.
    pub fn get_selection(&self) -> Option<u32> {
        let selection = unsafe { ffi::wxd_ListBox_GetSelection(self.window.as_ptr() as *mut _) };
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
            // Allocate a buffer first, like in Event::get_string
            let mut buffer: [c_char; 1024] = [0; 1024];
            let len_needed = ffi::wxd_ListBox_GetStringSelection(
                self.window.as_ptr() as *mut _,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );

            if len_needed < 0 {
                return None; // Indicates error or no selection
            }

            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                // String fit in the initial buffer
                let c_str = CStr::from_ptr(buffer.as_ptr());
                Some(c_str.to_string_lossy().into_owned())
            } else {
                // Buffer was too small, allocate exact size + null terminator
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_ListBox_GetStringSelection(
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
    /// For single-selection list boxes, `select = true` selects the item.
    /// For multi-selection list boxes, `select = true` toggles the selection.
    pub fn set_selection(&self, index: u32, select: bool) {
        unsafe {
            ffi::wxd_ListBox_SetSelection(self.window.as_ptr() as *mut _, index as i32, select);
        }
    }

    /// Gets the string at the specified index.
    /// Returns `None` if the index is out of bounds.
    pub fn get_string(&self, index: u32) -> Option<String> {
        unsafe {
            // Allocate buffer first
            let mut buffer: [c_char; 1024] = [0; 1024];
            let len_needed = ffi::wxd_ListBox_GetString(
                self.window.as_ptr() as *mut _,
                index as i32,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );

            if len_needed < 0 {
                return None; // Indicates error or invalid index
            }

            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                let c_str = CStr::from_ptr(buffer.as_ptr());
                Some(c_str.to_string_lossy().into_owned())
            } else {
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_ListBox_GetString(
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
        unsafe { ffi::wxd_ListBox_GetCount(self.window.as_ptr() as *mut _) }
    }
}

// --- ListBox Builder ---

/// Builder pattern for creating `ListBox` widgets.
#[derive(Clone)]
pub struct ListBoxBuilder<'a> {
    parent: &'a dyn WxWidget,
    id: Id,
    pos: Option<Point>,
    size: Option<Size>,
    style: ListBoxStyle,
    choices: Vec<String>,
}

impl<'a> ListBoxBuilder<'a> {
    /// Creates a new builder.
    pub fn new(parent: &'a dyn WxWidget) -> Self {
        Self {
            parent,
            id: ID_ANY as Id,
            pos: None,
            size: None,
            style: ListBoxStyle::Default,
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

    /// Sets the window style flags.
    pub fn with_style(mut self, style: ListBoxStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the initial items in the list box.
    pub fn with_choices(mut self, choices: &[&str]) -> Self {
        self.choices = choices.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Builds the `ListBox`.
    pub fn build(self) -> ListBox {
        let parent_ptr = self.parent.handle_ptr();
        let pos = self.pos.unwrap_or(DEFAULT_POSITION);
        let size = self.size.unwrap_or(DEFAULT_SIZE);
        // Call new_impl which now matches the 5-arg FFI
        let list_box = ListBox::new_impl(parent_ptr, self.id, pos, size, self.style.bits());

        // Append initial choices if any
        for choice_str in &self.choices {
            list_box.append(choice_str);
        }

        list_box
    }
}

// Implement WxWidget trait
impl WxWidget for ListBox {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

// Implement Drop (no-op for child widgets)
impl Drop for ListBox {
    fn drop(&mut self) {}
}

// Allow ListBox to be used where a Window is expected via Deref
impl std::ops::Deref for ListBox {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

// Implement WxEvtHandler trait
impl WxEvtHandler for ListBox {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.window.get_event_handler_ptr()
    }
}

// --- ListBoxStyle Enum ---

/// Style flags for `ListBox`.
///
/// These flags can be combined using the bitwise OR operator (`|`).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum ListBoxStyle {
    /// Default style (single selection).
    Default = ffi::WXD_LB_SINGLE,
    /// Multiple selection list: any number of items can be selected.
    Multiple = ffi::WXD_LB_MULTIPLE,
    /// Extended selection list: allows using Shift and Ctrl keys for selection.
    Extended = ffi::WXD_LB_EXTENDED,
    /// The items in the listbox are kept sorted in alphabetical order.
    Sort = ffi::WXD_LB_SORT,
    /// Always show a vertical scrollbar.
    AlwaysScrollbar = ffi::WXD_LB_ALWAYS_SB,
    /// Create a horizontal scrollbar if contents are too wide (requires explicit sizing).
    HorizontalScrollbar = ffi::WXD_LB_HSCROLL,
}

impl ListBoxStyle {
    /// Returns the raw integer value of the style.
    pub fn bits(self) -> i64 {
        self as i64
    }
}

impl BitOr for ListBoxStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for ListBoxStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe {
            *self = std::mem::transmute(self.bits() | rhs.bits());
        }
    }
}
