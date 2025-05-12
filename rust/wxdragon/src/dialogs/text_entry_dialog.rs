use crate::geometry::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE};
use crate::dialogs::Dialog;
use crate::window::WxWidget;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use wxdragon_sys as ffi;

// --- Constants ---
// Style Flags
// Combine general dialog styles (like CENTRE) with text entry specific ones.
pub use ffi::WXD_CANCEL as CANCEL;
pub use ffi::WXD_CENTRE as CENTRE;
pub use ffi::WXD_OK as OK; // Style flag for OK button // Style flag for Cancel button

// Text Entry Specific Styles
pub use ffi::WXD_TE_PASSWORD as TE_PASSWORD;
pub use ffi::WXD_TE_PROCESS_ENTER as TE_PROCESS_ENTER; // (Less common for dialogs, but possible)

// Default styles often include OK | CANCEL | CENTRE
pub const TEXT_ENTRY_DIALOG_STYLE: i64 = OK | CANCEL | CENTRE;

// Opaque C pointer type
pub type TextEntryDialogPtr = *mut ffi::wxd_TextEntryDialog_t;

// --- TextEntryDialog ---
#[derive(Clone)]
pub struct TextEntryDialog {
    dialog_base: Dialog,
}

impl TextEntryDialog {
    /// Creates a new builder for a TextEntryDialog.
    pub fn builder<'a>(
        parent: Option<&'a dyn WxWidget>,
        message: &str,
        caption: &str,
    ) -> TextEntryDialogBuilder<'a> {
        TextEntryDialogBuilder::new(parent, message, caption)
    }

    /// Creates a new TextEntryDialog wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid pointer to a wxTextEntryDialog.
    pub(crate) unsafe fn from_ptr(ptr: TextEntryDialogPtr) -> Self {
        TextEntryDialog {
            dialog_base: Dialog::from_ptr(ptr as super::DialogPtr),
        }
    }

    fn as_ptr(&self) -> TextEntryDialogPtr {
        self.dialog_base.as_ptr() as TextEntryDialogPtr
    }

    /// Shows the dialog modally.
    /// Returns an integer value which is usually one of the standard dialog return codes
    /// (e.g., ID_OK, ID_CANCEL).
    pub fn show_modal(&self) -> i32 {
        self.dialog_base.show_modal()
    }

    /// Gets the text entered by the user.
    /// Returns `None` if the dialog was cancelled or an error occurred retrieving the value.
    pub fn get_value(&self) -> Option<String> {
        unsafe {
            let mut buffer: [c_char; 1024] = [0; 1024]; // Reasonable buffer size
            let len_needed = ffi::wxd_TextEntryDialog_GetValue(
                self.as_ptr(),
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );

            if len_needed < 0 {
                return None; // Error or dialog cancelled before value retrieved?
            }

            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                let c_str = CStr::from_ptr(buffer.as_ptr());
                Some(c_str.to_string_lossy().into_owned())
            } else {
                // Allocate exact size if needed
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_TextEntryDialog_GetValue(
                    self.as_ptr(),
                    vec_buffer.as_mut_ptr() as *mut c_char,
                    vec_buffer.len() as i32,
                );
                if len_copied == len_needed {
                    vec_buffer.pop();
                    String::from_utf8(vec_buffer).ok()
                } else {
                    None // Error on second call
                }
            }
        }
    }
}

// Implement WxWidget by delegating to the inner Dialog
impl WxWidget for TextEntryDialog {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.dialog_base.handle_ptr()
    }
}

// Implement Drop
impl Drop for TextEntryDialog {
    fn drop(&mut self) {
        // The composed Dialog's Drop will be called automatically,
        // which calls wxd_Window_Destroy on the pointer.
    }
}

// --- TextEntryDialogBuilder ---
pub struct TextEntryDialogBuilder<'a> {
    parent: Option<&'a dyn WxWidget>,
    message: String,
    caption: String,
    default_value: String,
    style: i64,
    pos: Point,
    size: Size, // Often unused, but kept for consistency
}

impl<'a> TextEntryDialogBuilder<'a> {
    pub fn new(parent: Option<&'a dyn WxWidget>, message: &str, caption: &str) -> Self {
        TextEntryDialogBuilder {
            parent,
            message: message.to_string(),
            caption: caption.to_string(),
            default_value: String::new(),
            style: TEXT_ENTRY_DIALOG_STYLE, // Default includes OK/Cancel/Centre
            pos: DEFAULT_POSITION,
            size: DEFAULT_SIZE,
        }
    }

    pub fn with_default_value(mut self, value: &str) -> Self {
        self.default_value = value.to_string();
        self
    }

    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    /// Convenience method to add password style flag.
    pub fn password(mut self) -> Self {
        self.style |= TE_PASSWORD;
        self
    }

    pub fn with_pos(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn build(self) -> TextEntryDialog {
        let c_message = CString::new(self.message).expect("CString::new failed for message");
        let c_caption = CString::new(self.caption).expect("CString::new failed for caption");
        let c_default_value =
            CString::new(self.default_value).expect("CString::new failed for default_value");
        let parent_ptr = self.parent.map_or(ptr::null_mut(), |p| p.handle_ptr());

        let ptr = unsafe {
            ffi::wxd_TextEntryDialog_Create(
                parent_ptr,
                c_message.as_ptr(),
                c_caption.as_ptr(),
                c_default_value.as_ptr(),
                self.style as ffi::wxd_Style_t,
                self.pos.x,
                self.pos.y,
                self.size.width,
                self.size.height,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create wxTextEntryDialog");
        }
        unsafe { TextEntryDialog::from_ptr(ptr) }
    }
}
