//! Safe wrapper for wxStaticText.

use crate::prelude::*; // Use prelude
use crate::window::{Window, WxWidget};
use std::ffi::{CStr, CString};
use std::ops::{BitOr, BitOrAssign};
use std::os::raw::c_char;
use wxdragon_sys as ffi; // ADDED for enum bitwise operations

/// Represents a wxStaticText control.
#[derive(Clone)] // Allow cloning the wrapper
pub struct StaticText {
    window: Window, // Composition: StaticText IS a Window
}

// --- StaticText Builder ---

/// Builder pattern for creating `StaticText` widgets.
pub struct StaticTextBuilder<'a> {
    parent: &'a dyn WxWidget,
    id: Id,
    label: String,
    pos: Point,
    size: Size,
    style: StaticTextStyle, // MODIFIED: Use StaticTextStyle enum
}

impl<'a> StaticTextBuilder<'a> {
    /// Creates a new builder.
    pub fn new(parent: &'a dyn WxWidget) -> Self {
        Self {
            parent,
            id: ffi::WXD_ID_ANY as i32, // Use WXD_ID_ANY (i64) cast to Id (i32)
            label: String::new(),
            pos: Point { x: -1, y: -1 }, // Explicit default
            size: Size {
                width: -1,
                height: -1,
            }, // Explicit default
            style: StaticTextStyle::Default, // MODIFIED: Default style
        }
    }

    /// Sets the window identifier.
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    /// Sets the text label.
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

    /// Sets the window style flags (e.g., alignment flags like `wxALIGN_CENTER_HORIZONTAL`).
    pub fn with_style(mut self, style: StaticTextStyle) -> Self {
        // MODIFIED: Parameter is StaticTextStyle
        self.style = style;
        self
    }

    /// Builds the `StaticText`.
    ///
    /// # Panics
    /// Panics if static text creation fails in the underlying C++ layer.
    pub fn build(self) -> StaticText {
        StaticText::new(
            self.parent,
            self.id,
            self.label,
            self.pos,
            self.size,
            self.style.bits(), // MODIFIED: Use .bits() to get i64 value
        )
        .expect("Failed to create StaticText widget")
    }
}

// --- StaticText Implementation ---

impl StaticText {
    /// Creates a new `StaticTextBuilder`.
    pub fn builder(parent: &dyn WxWidget) -> StaticTextBuilder {
        StaticTextBuilder::new(parent)
    }

    /// Creates a new StaticText (low-level constructor used by builder).
    fn new(
        parent: &dyn WxWidget,
        id: Id,
        label: String,
        pos: Point,
        size: Size,
        style: i64, // Keep i64 here as it's the raw value passed to FFI
    ) -> Option<Self> {
        let c_label = CString::new(label).ok()?;
        unsafe {
            let parent_ptr = parent.handle_ptr();
            if parent_ptr.is_null() {
                return None;
            }
            let ptr = ffi::wxd_StaticText_Create(
                parent_ptr as *mut _,
                id,
                c_label.as_ptr(),
                pos.into(),
                size.into(),
                style.try_into().unwrap(),
            );
            if ptr.is_null() {
                None
            } else {
                Some(StaticText {
                    window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
                })
            }
        }
    }

    /// Sets the text control's label.
    pub fn set_label(&self, label: &str) {
        let c_label = CString::new(label).unwrap_or_default();
        unsafe {
            ffi::wxd_StaticText_SetLabel(
                self.window.as_ptr() as *mut ffi::wxd_StaticText_t,
                c_label.as_ptr(),
            );
        }
    }

    /// Gets the text control's label.
    pub fn get_label(&self) -> String {
        let mut buffer: [c_char; 256] = [0; 256];
        let len_needed = unsafe {
            ffi::wxd_StaticText_GetLabel(
                self.window.as_ptr() as *mut ffi::wxd_StaticText_t,
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
            let mut vec_buffer: Vec<c_char> = vec![0; len_needed as usize];
            let len_needed_2 = unsafe {
                ffi::wxd_StaticText_GetLabel(
                    self.window.as_ptr() as *mut ffi::wxd_StaticText_t,
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
                String::new()
            }
        } else {
            String::new()
        }
    }
}

// Implement WxWidget for StaticText.
impl WxWidget for StaticText {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

/// Drop behavior for StaticText.
/// As a child widget, its C++ object is managed by the parent.
impl Drop for StaticText {
    fn drop(&mut self) {
        // No-op: Parent wxWindow is responsible for destroying the C++ object.
    }
}

// Allow StaticText to be used where a Window is expected via Deref.
impl std::ops::Deref for StaticText {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

// --- StaticTextStyle Enum ---

/// Style flags for `StaticText`.
///
/// These flags can be combined using the bitwise OR operator (`|`).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum StaticTextStyle {
    /// Default style (left-aligned, auto-resizing).
    Default = ffi::WXD_ALIGN_LEFT, // WXD_ALIGN_LEFT is typically 0
    /// Align text to the right.
    AlignRight = ffi::WXD_ALIGN_RIGHT,
    /// Align text to the center horizontally.
    AlignCenterHorizontal = ffi::WXD_ALIGN_CENTRE_HORIZONTAL, // Using WXD_ALIGN_CENTRE_HORIZONTAL for clarity
                                                              // /// Align text to the center (combines horizontal and vertical, though vertical might not apply well here).
                                                              // AlignCenter = ffi::WXD_ALIGN_CENTRE, // WXD_ALIGN_CENTRE usually means both horizontal and vertical
                                                              // /// Do not automatically resize the control to fit its contents.
                                                              // NoAutoResize = ffi::WXD_ST_NO_AUTORESIZE, // Not yet available
}

impl StaticTextStyle {
    /// Returns the raw integer value of the style.
    pub fn bits(self) -> i64 {
        self as i64
    }
}

impl BitOr for StaticTextStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for StaticTextStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe {
            *self = std::mem::transmute(self.bits() | rhs.bits());
        }
    }
}
