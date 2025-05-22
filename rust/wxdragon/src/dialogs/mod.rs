// Remove: use crate::prelude::*;
use crate::window::WxWidget;
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

// Opaque C pointer for wxDialog
pub type DialogPtr = *mut ffi::wxd_Dialog;

// --- Dialog --- (Base struct for dialogs)
#[derive(Clone)]
pub struct Dialog {
    ptr: DialogPtr,
}

impl Dialog {
    /// Creates a new Dialog from a raw pointer.
    /// # Safety
    /// The pointer must be a valid pointer to a wxDialog.
    pub unsafe fn from_ptr(ptr: DialogPtr) -> Self {
        Dialog { ptr }
    }

    /// Shows the dialog modally.
    /// Returns an integer value which is usually one of the standard dialog return codes
    /// (e.g., ID_OK, ID_CANCEL, ID_YES, ID_NO).
    pub fn show_modal(&self) -> i32 {
        unsafe { ffi::wxd_Dialog_ShowModal(self.ptr) }
    }

    pub fn as_ptr(&self) -> DialogPtr {
        self.ptr
    }
}

impl WxWidget for Dialog {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.ptr as *mut ffi::wxd_Window_t
    }
}

// Dialogs are windows
// Remove: impl WindowMethods for Dialog {}

// Dialogs are event handlers -> This comes from WxEvtHandler
// (Already removed EvtHandlerMethods)

// No explicit Drop for Dialog base struct here. Actual dialog instances (like MessageDialog)
// will be wrapped, and their Drop will call wxd_Window_Destroy on the pointer,
// which is appropriate as wxDialog inherits from wxWindow.
