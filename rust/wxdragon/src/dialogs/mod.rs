// Remove: use crate::prelude::*;
use crate::event::WindowEvents;
use crate::window::Window;
use std::marker::PhantomData;
use wxdragon_sys as ffi;

pub mod colour_dialog;
pub mod dir_dialog;
pub mod file_dialog;
pub mod font_dialog;
pub mod message_dialog;
pub mod multi_choice_dialog;
pub mod progress_dialog;
pub mod single_choice_dialog;
pub mod text_entry_dialog;

// --- Dialog --- (Base struct for dialogs)
/// Represents a wxDialog.
///
/// # Lifetime Management
/// Dialog instances are typically shown modally and should be destroyed after use.
/// Call the `.destroy()` method (available via the `WxWidget` trait) when the dialog
/// is no longer needed to ensure proper cleanup.
#[derive(Clone)]
pub struct Dialog {
    window: Window, // Composition: Dialog uses a Window internally
    // Store parent pointer to manage drop behavior
    #[allow(dead_code)]
    parent_ptr: *mut ffi::wxd_Window_t,
    _marker: PhantomData<()>,
}

impl Dialog {
    /// Creates a new Dialog from a raw pointer.
    /// # Safety
    /// The pointer must be a valid pointer to a wxDialog.
    pub unsafe fn from_ptr(ptr: *mut ffi::wxd_Dialog_t) -> Self {
        Dialog {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
            parent_ptr: std::ptr::null_mut(),
            _marker: PhantomData,
        }
    }

    /// Creates a Dialog wrapper for an XRC-managed object.
    /// This dialog will not be destroyed when dropped as it's managed by XRC.
    pub fn from_xrc_ptr(ptr: *mut ffi::wxd_Dialog_t) -> Self {
        Dialog {
            window: unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) },
            parent_ptr: std::ptr::null_mut(),
            _marker: PhantomData,
        }
    }

    /// Shows the dialog modally.
    /// Returns an integer value which is usually one of the standard dialog return codes
    /// (e.g., ID_OK, ID_CANCEL, ID_YES, ID_NO).
    pub fn show_modal(&self) -> i32 {
        unsafe { ffi::wxd_Dialog_ShowModal(self.window.as_ptr() as *mut ffi::wxd_Dialog_t) }
    }

    /// Ends the modal dialog with the given return code.
    /// This method should be called from event handlers to close the dialog.
    /// The return code is what will be returned by show_modal().
    pub fn end_modal(&self, ret_code: i32) {
        unsafe {
            ffi::wxd_Dialog_EndModal(self.window.as_ptr() as *mut ffi::wxd_Dialog_t, ret_code)
        }
    }

    /// Returns the raw underlying dialog pointer.
    pub fn as_ptr(&self) -> *mut ffi::wxd_Dialog_t {
        self.window.as_ptr() as *mut ffi::wxd_Dialog_t
    }
}

// Apply common trait implementations for Dialog
implement_widget_traits_with_target!(Dialog, window, Window);

// Implement WindowEvents trait for Dialog
impl WindowEvents for Dialog {}

// XRC Support - enables Dialog to be created from XRC-managed pointers
impl_xrc_support!(Dialog, {
    window,
    parent_ptr: std::ptr::null_mut(),
    _marker: PhantomData
});

// Dialogs are windows
// Remove: impl WindowMethods for Dialog {}

// Dialogs are event handlers -> This comes from WxEvtHandler
// (Already removed EvtHandlerMethods)

// No explicit Drop for Dialog base struct here. Actual dialog instances (like MessageDialog)
// will be wrapped, and their Drop will call wxd_Window_Destroy on the pointer,
// which is appropriate as wxDialog inherits from wxWindow.
