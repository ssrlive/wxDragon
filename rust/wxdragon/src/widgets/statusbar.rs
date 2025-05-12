//!
//! Safe wrapper for wxStatusBar.

// use crate::window::Window; // Unused
use crate::widgets::frame::Frame; // Parent must be a Frame
                                  // use crate::base::Size; // Unused
use crate::id::ID_ANY; // ADDED for ID_ANY constant
use crate::id::Id; // ADDED for Id type alias
use crate::window::WxWidget; // Import trait for handle_ptr()
use std::ffi::CString;
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// --- Constants (Styles) ---
// Add relevant wxStatusBar styles here if needed, e.g.:
// pub const STB_SIZEGRIP: i64 = wxdragon_sys::WXD_STB_SIZEGRIP;
// pub const STB_SHOW_TIPS: i64 = wxdragon_sys::WXD_STB_SHOW_TIPS;
// ... Need to add to const_extractor ...

/// Represents a wxStatusBar attached to a Frame.
/// Note: The StatusBar itself is typically managed by the Frame.
#[derive(Clone)]
pub struct StatusBar {
    ptr: *mut ffi::wxd_StatusBar_t, // Keep a raw pointer
}

impl StatusBar {
    /// Creates a new StatusBar builder.
    /// The parent *must* be a `Frame`.
    pub fn builder(parent: &Frame) -> StatusBarBuilder {
        StatusBarBuilder::new(parent)
    }

    /// Creates a new StatusBar wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_StatusBar_t` pointer owned by a `Frame`.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_StatusBar_t) -> Self {
        // TODO: Consider if we need to INCREMENT a ref count or similar
        // if multiple Rust wrappers could point to the same C++ status bar?
        // For now, assume Frame::create_status_bar gives us the one true pointer.
        Self { ptr }
    }

    /// Returns the raw underlying status bar pointer.
    /// # Safety
    /// The pointer is managed by the parent Frame.
    pub fn as_ptr(&self) -> *mut ffi::wxd_StatusBar_t {
        self.ptr
    }

    /// Checks if the underlying pointer is null.
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    /// Sets the number of fields in the status bar.
    pub fn set_fields_count(&self, count: usize) {
        if !self.is_null() {
            unsafe {
                ffi::wxd_StatusBar_SetFieldsCount(self.ptr, count as c_int);
            }
        }
    }

    /// Sets the text for a specific field.
    pub fn set_status_text(&self, text: &str, field_index: usize) {
        if !self.is_null() {
            let c_text = CString::new(text).unwrap_or_default();
            unsafe {
                ffi::wxd_StatusBar_SetStatusText(self.ptr, c_text.as_ptr(), field_index as c_int);
            }
        }
    }

    /// Sets the widths of the status bar fields.
    /// `widths`: A slice containing the width for each field.
    ///   - Positive values are absolute widths.
    ///   - Negative values are proportional widths (-1, -2 means ratio 1:2).
    ///   - A width of 0 makes the field flexible.
    pub fn set_status_widths(&self, widths: &[i32]) {
        if !self.is_null() && !widths.is_empty() {
            unsafe {
                ffi::wxd_StatusBar_SetStatusWidths(
                    self.ptr,
                    widths.len() as c_int,
                    widths.as_ptr(),
                );
            }
        }
    }

    /// Pushes text onto the stack for a field. Reverts on PopStatusText.
    pub fn push_status_text(&self, text: &str, field_index: usize) {
        if !self.is_null() {
            let c_text = CString::new(text).unwrap_or_default();
            unsafe {
                ffi::wxd_StatusBar_PushStatusText(self.ptr, c_text.as_ptr(), field_index as c_int);
            }
        }
    }

    /// Pops the last pushed text from the stack for a field.
    pub fn pop_status_text(&self, field_index: usize) {
        if !self.is_null() {
            unsafe {
                ffi::wxd_StatusBar_PopStatusText(self.ptr, field_index as c_int);
            }
        }
    }
}

// --- Builder Pattern ---

/// Builder for creating and configuring a `StatusBar`.
pub struct StatusBarBuilder {
    parent_frame: Frame, // Must be a Frame
    id: Id,              // CHANGED to Id
    style: i64,
    // Configuration options applied after creation
    fields_count: Option<usize>,
    status_widths: Option<Vec<i32>>,
    initial_texts: Option<Vec<(usize, String)>>, // (index, text)
}

impl StatusBarBuilder {
    /// Creates a new StatusBar builder.
    pub fn new(parent: &Frame) -> Self {
        Self {
            parent_frame: parent.clone(), // Clone the Frame wrapper
            id: ID_ANY as i32,                   // Use ID_ANY from base (already i32)
            style: 0,                     // Default style (e.g., wxSTB_DEFAULT_STYLE)
            fields_count: None,
            status_widths: None,
            initial_texts: None,
        }
    }

    /// Sets the window ID.
    pub fn with_id(mut self, id: Id) -> Self {
        // CHANGED to Id
        self.id = id;
        self
    }

    /// Sets the style flags.
    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    /// Sets the initial number of fields.
    pub fn with_fields_count(mut self, count: usize) -> Self {
        self.fields_count = Some(count);
        self
    }

    /// Sets the initial status widths.
    pub fn with_status_widths(mut self, widths: Vec<i32>) -> Self {
        self.status_widths = Some(widths);
        self
    }

    /// Adds initial text for a specific field.
    pub fn add_initial_text(mut self, field_index: usize, text: &str) -> Self {
        let texts = self.initial_texts.get_or_insert_with(Vec::new);
        // Remove existing text for this index if present
        texts.retain(|(idx, _)| *idx != field_index);
        texts.push((field_index, text.to_string()));
        self
    }

    /// Creates the `StatusBar` and attaches it to the parent `Frame`.
    /// Returns the `StatusBar` wrapper.
    /// Panics if creation fails (FFI returns null) or parent frame is invalid.
    pub fn build(self) -> StatusBar {
        if self.parent_frame.is_null() {
            panic!("Cannot create StatusBar with a null parent frame");
        }

        let status_bar_ptr = unsafe {
            ffi::wxd_StatusBar_Create(
                self.parent_frame.handle_ptr(), // Pass frame ptr
                self.id,
                self.style as ffi::wxd_Style_t, // Cast style
            )
        };

        if status_bar_ptr.is_null() {
            panic!("Failed to create wxStatusBar via FFI");
        }

        let status_bar = unsafe { StatusBar::from_ptr(status_bar_ptr) };

        // Apply configurations
        if let Some(count) = self.fields_count {
            status_bar.set_fields_count(count);
        }
        if let Some(widths) = self.status_widths {
            status_bar.set_status_widths(&widths);
        }
        if let Some(texts) = self.initial_texts {
            for (index, text) in texts {
                status_bar.set_status_text(&text, index);
            }
        }

        // Attach the status bar to the frame (Frame takes ownership)
        unsafe {
            ffi::wxd_Frame_SetStatusBar(
                self.parent_frame.handle_ptr() as *mut ffi::wxd_Frame_t, // Cast to Frame pointer
                status_bar.as_ptr(),
            );
        }

        // Frame now owns the status bar. The Rust wrapper just holds a non-owning pointer.
        // We do NOT call drop/destroy on the status bar pointer from Rust.

        status_bar // Return the wrapper struct
    }
}

// No Drop implementation needed for StatusBar wrapper, as the Frame owns the actual widget.
