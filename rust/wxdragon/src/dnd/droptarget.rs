//! Target for drop operations.

use crate::prelude::WxWidget;
use std::boxed::Box;
use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use wxdragon_sys as ffi;

/// A drop target handles text data dropped via drag and drop.
pub struct TextDropTarget {
    ptr: *mut ffi::wxd_TextDropTarget_t,
}

impl TextDropTarget {
    /// Creates a new TextDropTarget and associates it with the given window.
    ///
    /// # Parameters
    ///
    /// * `window` - The window that will receive dropped text data.
    /// * `callback` - A closure that will be called when text is dropped.
    ///   The closure receives the text, x and y coordinates, and returns
    ///   whether the drop was accepted.
    pub fn new<W: WxWidget, F>(window: &W, callback: F) -> Self
    where
        F: FnMut(&str, i32, i32) -> bool + 'static,
    {
        // Box and get raw pointer to the callback closure
        let boxed_callback: Box<Box<dyn FnMut(&str, i32, i32) -> bool + 'static>> =
            Box::new(Box::new(callback));
        let user_data = Box::into_raw(boxed_callback) as *mut c_void;

        // Create the drop target with our callback trampoline
        let ptr = unsafe {
            ffi::wxd_TextDropTarget_Create(
                window.handle_ptr(),
                on_drop_text_trampoline as *mut c_void,
                user_data,
            )
        };

        TextDropTarget { ptr }
    }
}

impl Drop for TextDropTarget {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ffi::wxd_TextDropTarget_Destroy(self.ptr);
            }
            self.ptr = std::ptr::null_mut();
        }
    }
}

/// A drop target handles file data dropped via drag and drop.
pub struct FileDropTarget {
    ptr: *mut ffi::wxd_FileDropTarget_t,
}

impl FileDropTarget {
    /// Creates a new FileDropTarget and associates it with the given window.
    ///
    /// # Parameters
    ///
    /// * `window` - The window that will receive dropped file data.
    /// * `callback` - A closure that will be called when files are dropped.
    ///   The closure receives the filenames, x and y coordinates, and returns
    ///   whether the drop was accepted.
    pub fn new<W: WxWidget, F>(window: &W, callback: F) -> Self
    where
        F: FnMut(Vec<String>, i32, i32) -> bool + 'static,
    {
        // Box and get raw pointer to the callback closure
        let boxed_callback: Box<Box<dyn FnMut(Vec<String>, i32, i32) -> bool + 'static>> =
            Box::new(Box::new(callback));
        let user_data = Box::into_raw(boxed_callback) as *mut c_void;

        // Create the drop target with our callback trampoline
        let ptr = unsafe {
            ffi::wxd_FileDropTarget_Create(
                window.handle_ptr(),
                on_drop_files_trampoline as *mut c_void,
                user_data,
            )
        };

        FileDropTarget { ptr }
    }
}

impl Drop for FileDropTarget {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ffi::wxd_FileDropTarget_Destroy(self.ptr);
            }
            self.ptr = std::ptr::null_mut();
        }
    }
}

// --- Callback trampolines ---

// Called by wxWidgets when text is dropped
#[allow(clippy::not_unsafe_ptr_arg_deref)]
extern "C" fn on_drop_text_trampoline(
    text: *const c_char,
    x: i32,
    y: i32,
    closure_ptr: *mut c_void,
) -> bool {
    if text.is_null() || closure_ptr.is_null() {
        return false;
    }

    let text_str = unsafe { CStr::from_ptr(text).to_string_lossy().into_owned() };

    // Get the boxed callback and call it
    let callback = unsafe { &mut **(closure_ptr as *mut Box<dyn FnMut(&str, i32, i32) -> bool>) };

    callback(&text_str, x, y)
}

// Called by wxWidgets when files are dropped
#[allow(clippy::not_unsafe_ptr_arg_deref)]
extern "C" fn on_drop_files_trampoline(
    filenames_ptr: *const ffi::wxd_ArrayString_t,
    x: i32,
    y: i32,
    closure_ptr: *mut c_void,
) -> bool {
    if filenames_ptr.is_null() || closure_ptr.is_null() {
        return false;
    }

    // Extract filenames from wxArrayString
    let mut filenames = Vec::<String>::new();
    unsafe {
        let count = ffi::wxd_ArrayString_GetCount(filenames_ptr as *mut _);
        filenames.reserve(count as usize);

        for i in 0..count {
            let mut buffer = vec![0u8; 2048]; // Buffer for path
            let len = ffi::wxd_ArrayString_GetString(
                filenames_ptr as *mut _,
                i,
                buffer.as_mut_ptr() as *mut i8,
                buffer.len() as i32,
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

    // Get the boxed callback and call it
    let callback =
        unsafe { &mut **(closure_ptr as *mut Box<dyn FnMut(Vec<String>, i32, i32) -> bool>) };

    callback(filenames, x, y)
}
