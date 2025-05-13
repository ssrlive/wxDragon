//! Data objects for drag and drop operations.

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
            ffi::wxd_TextDataObject_GetText(
                self.ptr,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            )
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
        let mut array_string = ffi::wxd_ArrayString_t {
            internal_data: std::ptr::null_mut(),
        };

        let count = unsafe { ffi::wxd_FileDataObject_GetFilenames(self.ptr, &mut array_string) };
        
        // Convert the ArrayString to a Vec<String>
        let mut filenames = Vec::with_capacity(count as usize);
        
        unsafe {
            for i in 0..count {
                let mut buffer = vec![0u8; 2048]; // Buffer for path
                let len = ffi::wxd_ArrayString_GetString(
                    &mut array_string,
                    i,
                    buffer.as_mut_ptr() as *mut std::os::raw::c_char,
                    buffer.len() as i32
                );
                
                if len > 0 {
                    buffer.truncate(len as usize);
                    // Convert to UTF-8 String
                    if let Ok(s) = String::from_utf8(buffer) {
                        filenames.push(s);
                    }
                }
            }
        }

        // Clean up the array string
        if !array_string.internal_data.is_null() {
            unsafe {
                ffi::wxd_ArrayString_Free(&mut array_string);
            }
        }

        filenames
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