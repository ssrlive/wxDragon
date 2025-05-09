//! Safe wrapper for wxBitmapComboBox.

use crate::base::{Point, Size};
use crate::bitmap::Bitmap;
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::widgets::combobox::ComboBoxStyle;
use crate::window::{Window, WxWidget};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use wxdragon_sys as ffi;

/// Represents a wxBitmapComboBox widget.
#[derive(Clone)]
pub struct BitmapComboBox {
    window: Window,
}

impl BitmapComboBox {
    /// Creates a new `BitmapComboBoxBuilder`.
    pub fn builder<'a>(parent: Option<&'a dyn WxWidget>) -> BitmapComboBoxBuilder<'a> {
        BitmapComboBoxBuilder::new(parent)
    }

    /// Creates a `BitmapComboBox` from a raw pointer.
    /// # Safety
    /// The caller must ensure the pointer is valid and represents a `wxBitmapComboBox`.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_BitmapComboBox_t) -> Self {
        BitmapComboBox {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Appends an item with an optional bitmap.
    pub fn append(&self, item: &str, bitmap: Option<&Bitmap>) {
        let c_item = CString::new(item).expect("CString::new failed for item");
        let bmp_ptr = bitmap.map_or(ptr::null_mut(), |b| b.as_ptr());
        unsafe {
            ffi::wxd_BitmapComboBox_Append(self.as_ptr(), c_item.as_ptr(), bmp_ptr);
        }
    }

    /// Removes all items from the control.
    pub fn clear(&self) {
        unsafe { ffi::wxd_BitmapComboBox_Clear(self.as_ptr()) };
    }

    /// Gets the index of the currently selected item or -1 if none.
    pub fn get_selection(&self) -> i32 {
        unsafe { ffi::wxd_BitmapComboBox_GetSelection(self.as_ptr()) }
    }

    /// Sets the selection to the given item index.
    pub fn set_selection(&self, index: i32) {
        unsafe { ffi::wxd_BitmapComboBox_SetSelection(self.as_ptr(), index) };
    }

    /// Gets the string at the specified index.
    pub fn get_string(&self, index: u32) -> String {
        unsafe {
            let required_len_p1 =
                ffi::wxd_BitmapComboBox_GetString(self.as_ptr(), index as i32, ptr::null_mut(), 0);
            if required_len_p1 <= 0 {
                return String::new();
            }
            let capacity = required_len_p1 as usize;
            let mut buffer: Vec<u8> = Vec::with_capacity(capacity);
            let success_code = ffi::wxd_BitmapComboBox_GetString(
                self.as_ptr(),
                index as i32,
                buffer.as_mut_ptr() as *mut c_char,
                capacity as i32,
            );
            if success_code == 0 {
                let c_str = CStr::from_ptr(buffer.as_ptr() as *const c_char);
                String::from_utf8_lossy(c_str.to_bytes()).into_owned()
            } else {
                String::new()
            }
        }
    }

    /// Gets the number of items in the control.
    pub fn get_count(&self) -> u32 {
        unsafe { ffi::wxd_BitmapComboBox_GetCount(self.as_ptr()) }
    }

    /// Sets the text value in the text entry part of the control.
    pub fn set_value(&self, value: &str) {
        let c_value = CString::new(value).expect("CString::new failed for value");
        unsafe { ffi::wxd_BitmapComboBox_SetValue(self.as_ptr(), c_value.as_ptr()) };
    }

    /// Gets the text from the text entry part of the control.
    pub fn get_value(&self) -> String {
        unsafe {
            let required_len_p1 =
                ffi::wxd_BitmapComboBox_GetValue(self.as_ptr(), ptr::null_mut(), 0);
            if required_len_p1 <= 0 {
                return String::new();
            }
            let capacity = required_len_p1 as usize;
            let mut buffer: Vec<u8> = Vec::with_capacity(capacity);
            let success_code = ffi::wxd_BitmapComboBox_GetValue(
                self.as_ptr(),
                buffer.as_mut_ptr() as *mut c_char,
                capacity as i32,
            );
            if success_code == 0 {
                let c_str = CStr::from_ptr(buffer.as_ptr() as *const c_char);
                String::from_utf8_lossy(c_str.to_bytes()).into_owned()
            } else {
                String::new()
            }
        }
    }

    /// Gets the bitmap associated with the item at the specified index.
    /// Returns `None` if the index is invalid or the item has no bitmap.
    pub fn get_item_bitmap(&self, n: u32) -> Option<Bitmap> {
        let bmp_ptr = unsafe { ffi::wxd_BitmapComboBox_GetItemBitmap(self.as_ptr(), n) };
        if bmp_ptr.is_null() {
            None
        } else {
            // The C++ side created a `new wxBitmap`. We take ownership.
            Some(Bitmap(bmp_ptr))
        }
    }

    /// Sets the bitmap for the item at the specified index.
    pub fn set_item_bitmap(&self, n: u32, bitmap: &Bitmap) {
        unsafe { ffi::wxd_BitmapComboBox_SetItemBitmap(self.as_ptr(), n, bitmap.as_ptr()) };
    }

    /// Returns the raw wxBitmapComboBox pointer.
    fn as_ptr(&self) -> *mut ffi::wxd_BitmapComboBox_t {
        self.window.as_ptr() as *mut _
    }
}

// --- Builder ---

pub struct BitmapComboBoxBuilder<'a> {
    parent: Option<&'a dyn WxWidget>,
    id: Id,
    value: &'a str,
    pos: Point,
    size: Size,
    style: ComboBoxStyle,
}

impl<'a> BitmapComboBoxBuilder<'a> {
    pub fn new(parent: Option<&'a dyn WxWidget>) -> Self {
        BitmapComboBoxBuilder {
            parent,
            id: ffi::WXD_ID_ANY as i32,
            value: "",
            pos: Point::default(),
            size: Size::default(),
            style: ComboBoxStyle::Default,
        }
    }

    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    pub fn with_value(mut self, value: &'a str) -> Self {
        self.value = value;
        self
    }

    pub fn with_pos(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn with_style(mut self, style: ComboBoxStyle) -> Self {
        self.style = style;
        self
    }

    pub fn build(self) -> BitmapComboBox {
        let parent_ptr = self.parent.map_or(ptr::null_mut(), |p| p.handle_ptr());
        let c_value = CString::new(self.value).expect("CString::new failed for value");

        let ptr = unsafe {
            ffi::wxd_BitmapComboBox_Create(
                parent_ptr,
                self.id,
                c_value.as_ptr(),
                self.pos.into(),
                self.size.into(),
                self.style.bits() as ffi::wxd_Style_t,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create wxBitmapComboBox");
        }
        unsafe { BitmapComboBox::from_ptr(ptr) }
    }
}

// --- Trait Implementations ---

impl WxWidget for BitmapComboBox {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.as_ptr()
    }
}

impl WxEvtHandler for BitmapComboBox {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.window.get_event_handler_ptr()
    }
}

// No explicit Drop needed for the main struct; Window handles destruction.
// However, Bitmaps returned by GetItemBitmap *are* owned by Rust and need dropping.
// The `Bitmap` struct already implements Drop using wxd_Bitmap_Destroy.
