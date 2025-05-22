use crate::event::WxEvtHandler;
use crate::font::Font;
use crate::geometry::{Point, Size};
use crate::sizers::WxSizer;
use wxdragon_sys as ffi;

/// Represents a pointer to any wxDragon window object.
/// This is typically used as a base struct or in trait objects.
/// Note: Deliberately NOT Copy or Clone, as it represents unique FFI resource ownership.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Window(pub(crate) *mut ffi::wxd_Window_t);

impl Window {
    /// Creates a new Window wrapper from a raw pointer.
    /// # Safety
    /// The caller must ensure the pointer is valid and manages its lifetime correctly.
    /// This is typically called internally by concrete widget constructors.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Self(ptr)
    }

    /// Returns the raw underlying pointer.
    /// Temporary: Made public for FFI callbacks until safe event handling is done.
    pub fn as_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.0
    }

    /// Checks if the underlying pointer is null.
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Sets the window's sizer.
    /// Takes ownership of the Sizer object (caller should `std::mem::forget` it).
    /// `delete_old_sizer`: If true, the previous sizer (if any) is deleted.
    pub fn set_sizer(&self, sizer: impl WxSizer, delete_old_sizer: bool) {
        let window_ptr = self.handle_ptr();
        let sizer_ptr = sizer.as_sizer_ptr(); // Get pointer before potential forget
        if !window_ptr.is_null() && !sizer_ptr.is_null() {
            // Forget the Rust wrapper BEFORE passing pointer ownership to C++
            std::mem::forget(sizer);
            unsafe {
                ffi::wxd_Window_SetSizer(window_ptr, sizer_ptr, delete_old_sizer);
            }
        }
        // Sizer ownership is transferred to C++.
    }

    /// Sets the window's sizer and calls `Fit()` on the window.
    /// Takes ownership of the Sizer object (caller should `std::mem::forget` it).
    /// `delete_old_sizer`: If true, the previous sizer (if any) is deleted.
    pub fn set_sizer_and_fit(&self, sizer: impl WxSizer, delete_old_sizer: bool) {
        let window_ptr = self.handle_ptr();
        let sizer_ptr = sizer.as_sizer_ptr(); // Get pointer before potential forget
        if !window_ptr.is_null() && !sizer_ptr.is_null() {
            // Forget the Rust wrapper BEFORE passing pointer ownership to C++
            std::mem::forget(sizer);
            unsafe {
                ffi::wxd_Window_SetSizerAndFit(window_ptr, sizer_ptr, delete_old_sizer);
            }
        }
        // Sizer ownership is transferred to C++.
    }
}

/// Trait for common wxWidget operations.
pub trait WxWidget {
    /// Returns the raw underlying window pointer.
    /// Unsafe because the lifetime is not tied to self.
    /// Primarily for internal use or passing back to FFI.
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t;

    /// Returns the window ID for this widget.
    fn get_id(&self) -> i32 {
        let handle = self.handle_ptr();
        if handle.is_null() {
            ffi::WXD_ID_ANY as i32 // Return ID_ANY if handle is null
        } else {
            // Call the new C API function to get the real ID
            unsafe { ffi::wxd_Window_GetId(handle) }
        }
    }

    /// Adjusts the window size to fit its contents or its sizer.
    fn fit(&self) {
        let handle = self.handle_ptr();
        if !handle.is_null() {
            unsafe { ffi::wxd_Window_Fit(handle) };
        }
    }

    /// Gets the window's sizer-calculated best size.
    fn get_best_size(&self) -> crate::geometry::Size {
        let handle = self.handle_ptr();
        if handle.is_null() {
            crate::geometry::Size {
                width: -1,
                height: -1,
            }
        } else {
            let c_size = unsafe { ffi::wxd_Window_GetBestSize(handle) };
            crate::geometry::Size {
                width: c_size.width,
                height: c_size.height,
            }
        }
    }

    /// Shows or hides the widget.
    fn show(&self, show: bool) {
        let handle = self.handle_ptr();
        if !handle.is_null() {
            unsafe { ffi::wxd_Window_Show(handle, show) };
        }
    }

    /// Sets the window's background color.
    fn set_background_color(&self, color: crate::color::Colour) {
        let window_ptr = self.handle_ptr();
        if !window_ptr.is_null() {
            unsafe {
                ffi::wxd_Window_SetBackgroundColor(window_ptr, color.into());
            }
        }
    }

    /// Sets the window's minimum size.
    fn set_min_size(&self, size: crate::geometry::Size) {
        let window_ptr = self.handle_ptr();
        if !window_ptr.is_null() {
            unsafe {
                ffi::wxd_Window_SetMinSize(window_ptr, size.into());
            }
        }
    }

    /// Refreshes the window, optionally erasing the background and specifying a rectangle.
    fn refresh(&self, erase_background: bool, rect: Option<&crate::geometry::Rect>) {
        let window_ptr = self.handle_ptr();
        if !window_ptr.is_null() {
            let c_rect_storage: wxdragon_sys::wxd_Rect;
            let c_rect_ptr: *const wxdragon_sys::wxd_Rect;
            if let Some(r_borrowed) = rect {
                // Need to convert from &crate::base::Rect to wxdragon_sys::wxd_Rect
                // Assuming crate::base::Rect has an into() or from() for wxdragon_sys::wxd_Rect
                // or we construct it manually if fields are compatible.
                // Let's assume Rect has x, y, width, height and `into()` exists
                c_rect_storage = (*r_borrowed).into();
                c_rect_ptr = &c_rect_storage as *const _;
            } else {
                c_rect_ptr = std::ptr::null();
            }
            unsafe {
                // Ensure eraseBackground is passed as int (0 or 1)
                ffi::wxd_Window_Refresh(
                    window_ptr,
                    if erase_background { 1 } else { 0 },
                    c_rect_ptr,
                );
            }
        }
    }

    /// Sets the tooltip string for this widget.
    fn set_tooltip(&self, tip: &str) {
        let handle = self.handle_ptr();
        if !handle.is_null() {
            match std::ffi::CString::new(tip) {
                Ok(c_tip) => unsafe {
                    ffi::wxd_Window_SetToolTip(handle, c_tip.as_ptr());
                },
                Err(_) => {
                    // Handle CString creation error, perhaps log it or do nothing
                    // For now, do nothing if the string can't be converted (e.g., contains null bytes)
                }
            }
        }
    }

    /// Explicitly destroys the underlying wxWidgets object.
    /// After calling this, the widget wrapper should not be used further.
    /// This is useful for dynamically creating and destroying widgets.
    fn destroy(&self) {
        let handle = self.handle_ptr();
        if !handle.is_null() {
            // It's important that the Rust wrapper doesn't try to access
            // the object after this. The object will be scheduled for deletion.
            unsafe { ffi::wxd_Window_Destroy(handle) };
            // Note: We might want to nullify the internal pointer in the specific widget's struct
            // if the struct allows mutable access to itself after destroy is called, though
            // typically after destroy(), the Rust wrapper instance should be dropped or not used.
        }
    }

    fn set_font(&self, font: &Font) {
        // Create a new Font object owned by the widget to avoid ownership issues
        let font_copy = font.to_owned();
        unsafe {
            ffi::wxd_Window_SetFont(self.handle_ptr(), font_copy.as_ptr());
        }
        // Intentionally leak the font as the C++ side now owns it
        std::mem::forget(font_copy);
    }

    /// Gets the font currently used for this widget.
    ///
    /// Returns `Some(Font)` if a valid font is found, or `None` if no font is set or the widget handle is invalid.
    fn get_font(&self) -> Option<crate::font::Font> {
        let handle = self.handle_ptr();
        if handle.is_null() {
            return None;
        }

        let font_ptr = unsafe { ffi::wxd_Window_GetFont(handle) };
        if font_ptr.is_null() {
            return None;
        }

        // Create a Font object that takes ownership of the returned font pointer
        Some(unsafe { crate::font::Font::from_ptr(font_ptr, true) })
    }

    /// Enables or disables the widget.
    ///
    /// A disabled widget does not receive user input and is usually visually distinct.
    fn enable(&self, enable: bool) {
        let handle = self.handle_ptr();
        if !handle.is_null() {
            unsafe { ffi::wxd_Window_Enable(handle, enable) }
        }
    }

    /// Returns `true` if the widget is enabled, `false` otherwise.
    fn is_enabled(&self) -> bool {
        let handle = self.handle_ptr();
        if !handle.is_null() {
            unsafe { ffi::wxd_Window_IsEnabled(handle) }
        } else {
            false // If handle is null, widget can't be enabled
        }
    }

    /// Gets the window's position relative to its parent.
    fn get_position(&self) -> Point {
        let handle = self.handle_ptr();
        if handle.is_null() {
            Point { x: -1, y: -1 }
        } else {
            let pos = unsafe { ffi::wxd_Window_GetPosition(handle) };
            Point { x: pos.x, y: pos.y }
        }
    }

    /// Gets the window's size.
    fn get_size(&self) -> Size {
        let handle = self.handle_ptr();
        if handle.is_null() {
            Size {
                width: -1,
                height: -1,
            }
        } else {
            let size = unsafe { ffi::wxd_Window_GetSize(handle) };
            Size {
                width: size.width,
                height: size.height,
            }
        }
    }

    /// Sets the window's size.
    fn set_size(&self, size: Size) {
        let handle = self.handle_ptr();
        if !handle.is_null() {
            unsafe { ffi::wxd_Window_SetSize(handle, size.into()) }
        }
    }

    /// Sets the window's position and size.
    fn set_size_with_pos(&self, x: i32, y: i32, width: i32, height: i32) {
        let handle = self.handle_ptr();
        if !handle.is_null() {
            unsafe {
                ffi::wxd_Window_SetSizeWithPos(
                    handle,
                    x,
                    y,
                    width,
                    height,
                    ffi::WXD_SIZE_AUTO as i32,
                )
            }
        }
    }

    /// Sets the window's client area size (the area inside borders, scrollbars, etc).
    fn set_client_size(&self, size: Size) {
        let handle = self.handle_ptr();
        if !handle.is_null() {
            unsafe { ffi::wxd_Window_SetClientSize(handle, size.into()) }
        }
    }

    /// Gets the client area size.
    fn get_client_size(&self) -> Size {
        let handle = self.handle_ptr();
        if handle.is_null() {
            Size {
                width: -1,
                height: -1,
            }
        } else {
            let size = unsafe { ffi::wxd_Window_GetClientSize(handle) };
            Size {
                width: size.width,
                height: size.height,
            }
        }
    }

    /// Gets the window's minimum size.
    fn get_min_size(&self) -> Size {
        let handle = self.handle_ptr();
        if handle.is_null() {
            Size {
                width: -1,
                height: -1,
            }
        } else {
            let size = unsafe { ffi::wxd_Window_GetMinSize(handle) };
            Size {
                width: size.width,
                height: size.height,
            }
        }
    }

    /// Moves the window to the specified position.
    fn move_window(&self, x: i32, y: i32) {
        let handle = self.handle_ptr();
        if !handle.is_null() {
            unsafe { ffi::wxd_Window_Move(handle, x, y) }
        }
    }

    /// Centers the window relative to its parent.
    fn center(&self) {
        let handle = self.handle_ptr();
        if !handle.is_null() {
            unsafe { ffi::wxd_Window_Center(handle) }
        }
    }

    /// UK spelling alias for center()
    fn centre(&self) {
        self.center()
    }

    /// Converts client coordinates to screen coordinates.
    fn client_to_screen(&self, pt: Point) -> Point {
        let handle = self.handle_ptr();
        if handle.is_null() {
            return pt; // Return the same point if the handle is null
        }

        let result = unsafe { ffi::wxd_Window_ClientToScreen(handle, pt.into()) };
        Point {
            x: result.x,
            y: result.y,
        }
    }

    /// Converts screen coordinates to client coordinates.
    fn screen_to_client(&self, pt: Point) -> Point {
        let handle = self.handle_ptr();
        if handle.is_null() {
            return pt; // Return the same point if the handle is null
        }

        let result = unsafe { ffi::wxd_Window_ScreenToClient(handle, pt.into()) };
        Point {
            x: result.x,
            y: result.y,
        }
    }

    /// Gets the window label (title or text).
    /// Returns `None` if the label is not set, cannot be converted to UTF-8, or an error occurs.
    fn get_label(&self) -> Option<String> {
        let handle = self.handle_ptr();
        if handle.is_null() {
            return None;
        }
        let c_str_ptr = unsafe { ffi::wxd_Window_GetLabel(handle) };
        if c_str_ptr.is_null() {
            return None; // No label set or error in C++
        }
        let rust_string = unsafe {
            // Create a CStr reference to the C string data.
            let c_str = std::ffi::CStr::from_ptr(c_str_ptr);
            // Convert to a Rust String. to_string_lossy().into_owned() handles invalid UTF-8.
            let s = c_str.to_string_lossy().into_owned();
            // IMPORTANT: Free the string allocated by C++ using the provided wxd_free_string function.
            ffi::wxd_free_string(c_str_ptr);
            s
        };
        Some(rust_string)
    }

    // Other common methods (SetSize, GetSize, etc.) can be added here
    // if corresponding wxd_Window_* functions are added to the C API.
}

// Implement the trait for the base Window struct itself.
impl WxWidget for Window {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.0 // Return the raw pointer directly
    }
    // get_id, show use the default trait implementation.
    // set_sizer and set_sizer_and_fit REMOVED from trait
}

// --- Event Handling Implementation ---

impl WxEvtHandler for Window {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.0 as *mut ffi::wxd_EvtHandler_t
    }
}
