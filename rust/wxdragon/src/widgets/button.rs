use crate::event::WxEvtHandler;
use crate::prelude::*; // Use prelude
use crate::window::{Window, WxWidget}; // Make sure WxEvtHandler is imported
                                       // Remove specific imports covered by prelude
                                       // use crate::{Id, Point, Size};
use std::default::Default;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wxdragon_sys as ffi; // Import Default

/// Represents a wxButton.
#[derive(Clone)]
pub struct Button {
    window: Window, // Composition: Button IS a Window
    // Store parent pointer to manage drop behavior
    // Allow dead_code because it's used implicitly by the Drop logic.
    #[allow(dead_code)]
    parent_ptr: *mut ffi::wxd_Window_t,
}

// --- Button Builder ---

/// Builder pattern for creating `Button` widgets.
// #[derive(Default, Clone)] // Cannot derive Default with raw pointer
#[derive(Clone)]
pub struct ButtonBuilder {
    parent_ptr: *mut ffi::wxd_Window_t, // Use raw pointer
    id: Id,
    label: String,
    pos: Point,
    size: Size,
    style: i64,
}

// Manual Default implementation
impl Default for ButtonBuilder {
    fn default() -> Self {
        Self {
            parent_ptr: std::ptr::null_mut(),
            id: -1, // Explicitly use -1 (wxID_ANY)
            label: String::new(),
            pos: Point { x: -1, y: -1 }, // Explicit default position
            size: Size {
                width: -1,
                height: -1,
            }, // Explicit default size
            style: 0,
        }
    }
}

impl ButtonBuilder {
    /// Sets the window identifier.
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    /// Sets the button label.
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = label.to_string();
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
    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    /// Builds the `Button` widget.
    ///
    /// # Panics
    /// Panics if button creation fails in the underlying C++ layer.
    pub fn build(self) -> Button {
        assert!(!self.parent_ptr.is_null(), "Button requires a parent");
        let c_label = CString::new(self.label).expect("CString::new failed");

        let ptr = unsafe {
            ffi::wxd_Button_Create(
                self.parent_ptr,
                self.id, // Use directly from builder
                c_label.as_ptr(),
                self.pos.into(),                // Use directly
                self.size.into(),               // Use directly
                self.style.try_into().unwrap(), // Use directly
            )
        };

        if ptr.is_null() {
            panic!("Failed to create Button widget");
        } else {
            // Cast the concrete Button pointer to the base Window pointer for the wrapper
            let window = unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) };
            Button {
                window,
                parent_ptr: self.parent_ptr, // Store parent ptr
            }
        }
    }
}

// --- Button Implementation ---

impl Button {
    /// Creates a new `ButtonBuilder` for constructing a button.
    pub fn builder(parent: &dyn WxWidget) -> ButtonBuilder {
        let mut builder = ButtonBuilder::default();
        builder.parent_ptr = parent.handle_ptr();
        builder
    }

    /// Creates a new Button from a raw window and parent pointer.
    /// This is intended for internal use by other widget wrappers that compose Button.
    pub(crate) fn new_from_composition(window: Window, parent_ptr: *mut ffi::wxd_Window_t) -> Self {
        Self { window, parent_ptr }
    }

    /// Sets the button's label.
    pub fn set_label(&self, label: &str) {
        let c_label = CString::new(label).expect("CString::new failed");
        unsafe {
            ffi::wxd_Button_SetLabel(
                self.window.as_ptr() as *mut ffi::wxd_Button_t,
                c_label.as_ptr(),
            );
        }
    }

    /// Gets the button's label.
    pub fn get_label(&self) -> String {
        let mut buffer: [c_char; 256] = [0; 256]; // Reasonable buffer size
        let len_needed = unsafe {
            ffi::wxd_Button_GetLabel(
                self.window.as_ptr() as *mut ffi::wxd_Button_t,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            )
        };

        if len_needed > 0 && (len_needed as usize) <= buffer.len() {
            unsafe {
                CStr::from_ptr(buffer.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            }
        } else if len_needed > (buffer.len() as i32) {
            // Buffer too small, try again with required size
            let mut vec_buffer: Vec<c_char> = vec![0; len_needed as usize];
            let len_needed_2 = unsafe {
                ffi::wxd_Button_GetLabel(
                    self.window.as_ptr() as *mut ffi::wxd_Button_t,
                    vec_buffer.as_mut_ptr(),
                    vec_buffer.len() as i32,
                )
            };
            if len_needed_2 == len_needed {
                unsafe {
                    CStr::from_ptr(vec_buffer.as_ptr())
                        .to_string_lossy()
                        .into_owned()
                }
            } else {
                // Something went wrong
                String::new()
            }
        } else {
            // Error or empty label
            String::new()
        }
    }
}

// Implement WxWidget for Button.
impl WxWidget for Button {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }

    // Inherit default `show` from trait for now, assuming a generic Window Show
    // might be added to C API later. Or, implement wxd_Window_Show.
    // For Button, Show is usually handled by the parent layout,
    // but an explicit wxd_Button_Show could be added if needed.

    // No need to implement `destroy` here, Drop handles it.
}

/// Implement Drop for Button.
/// NOTE: For child widgets like Button, we assume wxWidgets handles the
/// actual C++ object destruction when the parent is destroyed.
/// Therefore, this Drop implementation does *nothing* to avoid potential
/// double-free or use-after-free issues if the Rust wrapper is dropped
/// while the C++ parent still exists, or if the parent is dropped first.
/// Lifetime management for persistent widgets is handled by `WxdAppHandle::forget_widget`
/// for top-level windows.
impl Drop for Button {
    fn drop(&mut self) {
        // println!("Button wrapper drop (noop for children): {:?}", self.window.as_ptr());
        // No-op: Parent wxWindow is responsible for destroying the C++ object.
        // Calling ffi::wxd_Button_Destroy here would be incorrect for non-top-level widgets.
    }
}

// Allow Button to be used where a Window is expected
impl std::ops::Deref for Button {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

// Restore correct WxEvtHandler implementation
impl WxEvtHandler for Button {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.window.get_event_handler_ptr()
    }
}
