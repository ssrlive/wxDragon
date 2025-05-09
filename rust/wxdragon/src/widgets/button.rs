use crate::event::WxEvtHandler;
use crate::prelude::*; // Use prelude
use crate::window::{Window, WxWidget}; // Make sure WxEvtHandler is imported
                                       // Remove specific imports covered by prelude
                                       // use crate::{Id, Point, Size};
use std::default::Default;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wxdragon_sys as ffi; // Import Default
use std::ops::{BitOr, BitOrAssign}; // ADDED for enum bitwise operations

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
    style: ButtonStyle, // MODIFIED: Use ButtonStyle enum
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
            style: ButtonStyle::Default, // MODIFIED: Default style
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
    pub fn with_style(mut self, style: ButtonStyle) -> Self { // MODIFIED: Parameter is ButtonStyle
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
                self.style.bits().try_into().unwrap(), // MODIFIED: Use .bits() to get i64 value
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

// Implement WxWidget for Button by delegating to the composed Window.
impl WxWidget for Button {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr() // Delegate to Window's WxWidget impl
    }
}

// Implement Deref to allow Button to be used where a Window is expected.
impl std::ops::Deref for Button {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl std::ops::DerefMut for Button {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}

// Implement WxEvtHandler for Button by delegating to the composed Window's WxEvtHandler implementation.
impl WxEvtHandler for Button {
    /// # Safety
    /// Inherits safety requirements from `Window::get_event_handler_ptr`.
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.window.get_event_handler_ptr()
    }
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
        // Child widgets like Button are typically managed by their parent in wxWidgets.
        // The `Window` wrapper's Drop logic (via WxdCleaner) handles unbinding events.
        // No explicit call to ffi::wxd_Window_Destroy is needed here for child widgets.
        // If this button were somehow a top-level, unparented button (unlikely for wxButton),
        // then specific destruction logic might be needed. For now, assume it's a child.
    }
}

// --- ButtonStyle Enum ---

/// Style flags for `Button`.
///
/// These flags can be combined using the bitwise OR operator (`|`).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum ButtonStyle {
    /// Default style (no specific alignment, standard border).
    Default = 0,
    /// Align label to the left.
    Left = ffi::WXD_BU_LEFT,
    /// Align label to the top.
    Top = ffi::WXD_BU_TOP,
    /// Align label to the right.
    Right = ffi::WXD_BU_RIGHT,
    /// Align label to the bottom.
    Bottom = ffi::WXD_BU_BOTTOM,
    /// Button size will be adjusted to exactly fit the label.
    ExactFit = ffi::WXD_BU_EXACTFIT,
    /// Do not display the label string (useful for buttons with only an image).
    NoText = ffi::WXD_BU_NOTEXT,
    /// No border.
    BorderNone = ffi::WXD_BORDER_NONE,
    /// A simple border (rarely used for buttons, which have a default look).
    BorderSimple = ffi::WXD_BORDER_SIMPLE,
}

impl ButtonStyle {
    /// Returns the raw integer value of the style.
    pub fn bits(self) -> i64 {
        self as i64
    }
}

impl BitOr for ButtonStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for ButtonStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe { *self = std::mem::transmute(self.bits() | rhs.bits()); }
    }
}
