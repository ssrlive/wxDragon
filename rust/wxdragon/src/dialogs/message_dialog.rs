use super::DialogPtr; // Import DialogPtr from parent module
use crate::dialogs::Dialog; // Use the Dialog from the parent module
use crate::window::WxWidget; // Keep this
use std::ffi::CString;
use wxdragon_sys as ffi;

// Opaque C pointer for wxMessageDialog
pub type MessageDialogPtr = *mut ffi::wxd_MessageDialog;

// --- MessageDialog ---
#[derive(Clone)]
pub struct MessageDialog {
    // ptr: MessageDialogPtr, // This would be specific
    dialog_base: Dialog, // Composition: MessageDialog IS A Dialog
}

impl MessageDialog {
    /// Creates a new MessageDialog from a raw pointer.
    /// # Safety
    /// The pointer must be a valid pointer to a wxMessageDialog.
    pub unsafe fn from_ptr(ptr: MessageDialogPtr) -> Self {
        MessageDialog {
            dialog_base: Dialog::from_ptr(ptr as DialogPtr),
        }
    }

    pub fn builder<'a>(
        parent: Option<&'a dyn WxWidget>,
        message: &str,
        caption: &str,
    ) -> MessageDialogBuilder<'a> {
        MessageDialogBuilder::new(parent, message, caption)
    }

    /// Shows the dialog modally.
    /// Returns an integer value which is usually one of the standard dialog return codes
    /// (e.g., ID_OK, ID_CANCEL, ID_YES, ID_NO).
    pub fn show_modal(&self) -> i32 {
        self.dialog_base.show_modal()
    }

    pub fn as_ptr(&self) -> MessageDialogPtr {
        self.dialog_base.as_ptr() as MessageDialogPtr
    }
}

impl WxWidget for MessageDialog {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.dialog_base.handle_ptr()
    }
}

// MessageDialogs are Windows (via Dialog base)

// MessageDialogs are EvtHandlers (via Dialog base)

impl Drop for MessageDialog {
    fn drop(&mut self) {
        if !self.handle_ptr().is_null() {
            // Ensure Rust knows this pointer is managed by Rust for this instance.
            // If this instance is a simple wrapper not owning the underlying object,
            // (e.g. from a GetParent call), then this Drop might be too aggressive.
            // For dialogs created via builder, they are owned.
            unsafe {
                // Check if it's already being destroyed by wxWidgets or a parent
                // For dialogs shown modally, they are often auto-destroyed or parented such that
                // wxWidgets handles their deletion. However, explicit Destroy is safer if we created it.
                // The current C API relies on wxd_Window_Destroy.
                ffi::wxd_Window_Destroy(self.handle_ptr());
            }
        }
    }
}

// --- MessageDialogBuilder ---
pub struct MessageDialogBuilder<'a> {
    parent: Option<&'a dyn WxWidget>,
    message: String,
    caption: String,
    style: i64, // wxDialogStyle flags
}

impl<'a> MessageDialogBuilder<'a> {
    pub fn new(parent: Option<&'a dyn WxWidget>, message: &str, caption: &str) -> Self {
        MessageDialogBuilder {
            parent,
            message: message.to_string(),
            caption: caption.to_string(),
            style: 0, // Default: wxOK. TODO: Add actual wxOK constant.
        }
    }

    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    // Convenience methods for common styles might be added later, e.g.:
    // pub fn ok(mut self) -> Self { self.style |= wxOK; self }
    // pub fn yes_no(mut self) -> Self { self.style = wxYES_NO; self }

    pub fn build(self) -> MessageDialog {
        let c_message = CString::new(self.message).expect("CString::new failed for message");
        let c_caption = CString::new(self.caption).expect("CString::new failed for caption");
        let parent_ptr = self.parent.map_or(std::ptr::null_mut(), |p| p.handle_ptr());

        let ptr = unsafe {
            ffi::wxd_MessageDialog_Create(
                parent_ptr,
                c_message.as_ptr(),
                c_caption.as_ptr(),
                self.style as ffi::wxd_Style_t,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create wxMessageDialog");
        }
        unsafe { MessageDialog::from_ptr(ptr) }
    }
}

// Dialog Style Constants (subset for common wxMessageDialog styles)
// These should map to the WXD_ style constants from wxdragon_sys
// which are generated from wxWidgets constants.

pub const OK: i64 = ffi::WXD_OK;
pub const CANCEL: i64 = ffi::WXD_CANCEL;
pub const YES_NO: i64 = ffi::WXD_YES_NO;
pub const YES: i64 = ffi::WXD_YES;
pub const NO: i64 = ffi::WXD_NO;

pub const ICON_NONE: i64 = ffi::WXD_ICON_NONE;
pub const ICON_EXCLAMATION: i64 = ffi::WXD_ICON_EXCLAMATION;
pub const ICON_WARNING: i64 = ffi::WXD_ICON_WARNING; // Same as ICON_EXCLAMATION
pub const ICON_HAND: i64 = ffi::WXD_ICON_HAND;
pub const ICON_ERROR: i64 = ffi::WXD_ICON_ERROR; // Same as ICON_HAND
pub const ICON_QUESTION: i64 = ffi::WXD_ICON_QUESTION;
pub const ICON_INFORMATION: i64 = ffi::WXD_ICON_INFORMATION;
pub const ICON_AUTH_NEEDED: i64 = ffi::WXD_ICON_AUTH_NEEDED;

// Dialog positioning/behavior (though MessageDialog often centers by default)
pub const CENTRE: i64 = ffi::WXD_CENTRE;

// Standard Dialog Return IDs (already available via id.rs, but useful context here)
// pub const ID_OK: i32 = ffi::WXD_ID_OK;
// pub const ID_CANCEL: i32 = ffi::WXD_ID_CANCEL;
// pub const ID_YES: i32 = ffi::WXD_ID_YES;
// pub const ID_NO: i32 = ffi::WXD_ID_NO;
