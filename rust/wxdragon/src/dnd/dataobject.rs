//! Data objects for drag and drop operations.

use crate::utils::WxdArrayString;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wxdragon_sys as ffi;

/// Trait that all data objects used in drag and drop must implement.
pub trait DataObject {
    /// Gets the raw pointer to the underlying data object.
    fn as_data_object_ptr(&self) -> *mut ffi::wxd_DataObject_t;
}

/// A data object that contains text data.
pub struct TextDataObject {
    ptr: *mut ffi::wxd_TextDataObject_t,
}

impl TextDataObject {
    /// Creates a new TextDataObject with the given text.
    pub fn new(text: &str) -> Self {
        let text_cstring = CString::new(text).unwrap_or_default();
        let ptr = unsafe { ffi::wxd_TextDataObject_Create(text_cstring.as_ptr()) };
        TextDataObject { ptr }
    }

    /// Gets the text contained in this data object.
    pub fn get_text(&self) -> String {
        let mut buffer = [0 as c_char; 2048];
        let len = unsafe {
            ffi::wxd_TextDataObject_GetText(self.ptr, buffer.as_mut_ptr(), buffer.len() as i32)
        };

        if len <= 0 {
            return String::new();
        }

        unsafe {
            CStr::from_ptr(buffer.as_ptr())
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Sets the text contained in this data object.
    pub fn set_text(&mut self, text: &str) {
        let text_cstring = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_TextDataObject_SetText(self.ptr, text_cstring.as_ptr());
        }
    }
}

impl DataObject for TextDataObject {
    fn as_data_object_ptr(&self) -> *mut ffi::wxd_DataObject_t {
        self.ptr as *mut ffi::wxd_DataObject_t
    }
}

impl Drop for TextDataObject {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ffi::wxd_TextDataObject_Destroy(self.ptr);
            }
            self.ptr = std::ptr::null_mut();
        }
    }
}

/// A data object that contains file paths.
pub struct FileDataObject {
    ptr: *mut ffi::wxd_FileDataObject_t,
}

impl FileDataObject {
    /// Creates a new empty FileDataObject.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_FileDataObject_Create() };
        FileDataObject { ptr }
    }

    /// Gets the filenames contained in this data object.
    pub fn get_filenames(&self) -> Vec<String> {
        // Create a WxdArrayString to hold the filenames
        let array_string = WxdArrayString::new();

        let _count =
            unsafe { ffi::wxd_FileDataObject_GetFilenames(self.ptr, array_string.as_ptr()) };

        // Convert the WxdArrayString to a Vec<String>
        array_string.into_vec()
    }

    /// Adds a file to this data object.
    pub fn add_file(&mut self, filename: &str) {
        let filename_cstring = CString::new(filename).unwrap_or_default();
        unsafe {
            ffi::wxd_FileDataObject_AddFile(self.ptr, filename_cstring.as_ptr());
        }
    }
}

impl DataObject for FileDataObject {
    fn as_data_object_ptr(&self) -> *mut ffi::wxd_DataObject_t {
        self.ptr as *mut ffi::wxd_DataObject_t
    }
}

impl Drop for FileDataObject {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ffi::wxd_FileDataObject_Destroy(self.ptr);
            }
            self.ptr = std::ptr::null_mut();
        }
    }
}

impl Default for FileDataObject {
    fn default() -> Self {
        Self::new()
    }
}
