use crate::event::WxEvtHandler;
use crate::font::Font;
use crate::sizers::WxSizer;
use lazy_static::lazy_static;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;
use wxdragon_sys as ffi;

// Define a type alias for the window pointer used as a key in the map
type WindowPtrKey = usize;

// Static map to store user data associated with window pointers.
// Key: Window pointer address (usize)
// Value: Boxed RefCell containing the dynamic user data.
lazy_static! {
    static ref WINDOW_USER_DATA_MAP: Mutex<HashMap<WindowPtrKey, Box<RefCell<dyn Any + Send + Sync + 'static>>>> = Mutex::new(HashMap::new());
    // Note: Using Send + Sync bounds on Any for Mutex compatibility.
    // If data doesn't need to cross threads (likely for GUI), consider alternatives
    // like a RefCell<HashMap<...>> in thread-local storage if Mutex becomes complex.
}

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
    fn get_best_size(&self) -> crate::base::Size {
        let handle = self.handle_ptr();
        if handle.is_null() {
            crate::base::Size {
                width: -1,
                height: -1,
            }
        } else {
            let c_size = unsafe { ffi::wxd_Window_GetBestSize(handle) };
            crate::base::Size {
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
    fn set_background_color(&self, color: crate::widgets::colourpickerctrl::Colour) {
        let window_ptr = self.handle_ptr();
        if !window_ptr.is_null() {
            unsafe {
                ffi::wxd_Window_SetBackgroundColor(window_ptr, color.into());
            }
        }
    }

    /// Sets the window's minimum size.
    fn set_min_size(&self, size: crate::base::Size) {
        let window_ptr = self.handle_ptr();
        if !window_ptr.is_null() {
            unsafe {
                ffi::wxd_Window_SetMinSize(window_ptr, size.into());
            }
        }
    }

    /// Refreshes the window, optionally erasing the background and specifying a rectangle.
    fn refresh(&self, erase_background: bool, rect: Option<&crate::base::Rect>) {
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
        unsafe {
            ffi::wxd_Window_SetFont(self.handle_ptr(), font.as_ptr());
        }
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

// --- Rust Cleanup Callback ---

/// FFI-callable function notified by the C++ WxdCleanupNotifier destructor.
/// Removes the associated user data from the Rust map.
#[no_mangle]
pub extern "C" fn notify_rust_of_cleanup(win_ptr: *mut ffi::wxd_Window_t) {
    if win_ptr.is_null() {
        return;
    }
    let key = win_ptr as WindowPtrKey;
    // Lock the map and remove the entry. The returned Box is dropped here.
    let mut map = WINDOW_USER_DATA_MAP
        .lock()
        .expect("Failed to lock user data map for cleanup");
    if let Some(_removed_data) = map.remove(&key) {
        // Optional: Log cleanup
        // println!("Cleaned up user data for window {:?}", key);
    } else {
        // Optional: Log if notifier called but no data found (e.g., detached manually)
        // println!("Cleanup notification received for window {:?}, but no data found in map.", key);
    }
}

// --- Window User Data Trait ---

/// Trait for associating arbitrary, type-erased Rust data with a window.
pub trait WindowUserData: WxWidget {
    /// Associates arbitrary Rust data with this widget.
    /// Data must be `Send + Sync + 'static` due to the global Mutex.
    /// Any previously associated data will be dropped.
    fn set_user_data(&self, data: Box<RefCell<dyn Any + Send + Sync + 'static>>) {
        let handle = self.handle_ptr();
        if handle.is_null() {
            return;
        }
        let key = handle as WindowPtrKey;

        // Lock the map and insert the new data.
        // This drops the previous Box<RefCell<...>> if one existed for this key.
        let mut map = WINDOW_USER_DATA_MAP
            .lock()
            .expect("Failed to lock user data map for set");
        let existed_before = map.insert(key, data).is_some();

        // If data didn't exist before, attach the C++ notifier.
        // If it did exist, the notifier should already be attached.
        // Re-attaching might be safe if SetClientObject handles replacing correctly.
        if !existed_before {
            unsafe {
                ffi::wxd_Window_AttachCleanupNotifier(handle);
            }
        }
        // else: Consider if detaching/reattaching is needed on replacement?
        // Current C++ SetClientObject replaces and deletes old, so re-attaching *might* be okay,
        // but let's keep it simple and only attach if it wasn't there before.
    }

    /// Provides temporary immutable access to the associated user data if it exists and matches type T.
    /// Executes the provided closure `func` with a reference `&T` to the data.
    /// Returns `true` if the data existed, matched the type, and the closure was run, `false` otherwise.
    /// The borrow and map lock are released after the closure finishes.
    fn with_borrowed_data<T: Any + 'static, F, R>(&self, func: F) -> bool
    where
        F: FnOnce(&T) -> R,
    {
        let handle = self.handle_ptr();
        if handle.is_null() {
            return false;
        }
        let key = handle as WindowPtrKey;

        let map = WINDOW_USER_DATA_MAP
            .lock()
            .expect("Failed to lock user data map");
        if let Some(boxed_cell) = map.get(&key) {
            if let Ok(borrowed_any) = boxed_cell.try_borrow() {
                if let Some(data_t) = borrowed_any.downcast_ref::<T>() {
                    func(data_t);
                    return true; // Closure executed successfully
                }
            }
        }
        false // Data not found, type mismatch, or already borrowed mutably
    }

    /// Provides temporary mutable access to the associated user data if it exists and matches type T.
    /// Executes the provided closure `func` with a mutable reference `&mut T` to the data.
    /// Returns `true` if the data existed, matched the type, was not already borrowed, and the closure was run, `false` otherwise.
    /// The borrow and map lock are released after the closure finishes.
    fn with_borrowed_data_mut<T: Any + 'static, F, R>(&self, func: F) -> bool
    where
        F: FnOnce(&mut T) -> R,
    {
        let handle = self.handle_ptr();
        if handle.is_null() {
            return false;
        }
        let key = handle as WindowPtrKey;

        let map = WINDOW_USER_DATA_MAP
            .lock()
            .expect("Failed to lock user data map");
        if let Some(boxed_cell) = map.get(&key) {
            if let Ok(mut borrowed_any_mut) = boxed_cell.try_borrow_mut() {
                if let Some(data_t_mut) = borrowed_any_mut.downcast_mut::<T>() {
                    func(data_t_mut);
                    return true; // Closure executed successfully
                }
            }
        }
        false // Data not found, type mismatch, or already borrowed
    }

    /// Takes ownership of the associated user data, removing it from the widget.
    /// Returns `None` if no data is associated.
    fn take_user_data_dyn(&self) -> Option<Box<RefCell<dyn Any + Send + Sync + 'static>>> {
        let handle = self.handle_ptr();
        if handle.is_null() {
            return None;
        }
        let key = handle as WindowPtrKey;

        // Lock the map and remove the data.
        let mut map = WINDOW_USER_DATA_MAP
            .lock()
            .expect("Failed to lock user data map for take");
        let removed_data = map.remove(&key);

        // If data was removed, detach the C++ cleanup notifier.
        if removed_data.is_some() {
            unsafe {
                ffi::wxd_Window_DetachCleanupNotifier(handle);
            }
        }
        removed_data
    }
}

// Implement the trait for the base Window struct
impl WindowUserData for Window {}

// ... rest of window.rs ...
