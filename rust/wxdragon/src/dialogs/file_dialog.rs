use crate::dialogs::Dialog;
use crate::geometry::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE};
use crate::widget_style_enum;
use crate::window::WxWidget;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use wxdragon_sys as ffi;

// Define FileDialogStyle enum using the widget_style_enum macro
widget_style_enum!(
    name: FileDialogStyle,
    doc: "Style flags for FileDialog.",
    variants: {
        Open: ffi::WXD_FD_OPEN, "Creates an open file dialog (cannot be combined with Save).",
        Save: ffi::WXD_FD_SAVE, "Creates a save file dialog (cannot be combined with Open).",
        OverwritePrompt: ffi::WXD_FD_OVERWRITE_PROMPT, "For save dialog only: prompt for a confirmation if a file with the same name already exists.",
        FileMustExist: ffi::WXD_FD_FILE_MUST_EXIST, "For open dialog only: the user may only select files that actually exist.",
        Multiple: ffi::WXD_FD_MULTIPLE, "For open dialog only: allows selecting multiple files.",
        ChangeDir: ffi::WXD_FD_CHANGE_DIR, "Change the current working directory to the directory where the file(s) chosen by the user are.",
        Preview: ffi::WXD_FD_PREVIEW, "Show the preview of the selected files (currently only supported by wxGTK)."
    },
    default_variant: Open
);

// Opaque C pointer type
pub type FileDialogPtr = *mut ffi::wxd_FileDialog_t;

// Helper struct to manage wxd_ArrayString_t from FFI
struct WxdArrayString {
    // Keep this private to the module
    ptr: *mut ffi::wxd_ArrayString_t,
}

impl WxdArrayString {
    fn new() -> Self {
        let ptr = unsafe { ffi::wxd_ArrayString_Create() };
        assert!(!ptr.is_null(), "Failed to create wxd_ArrayString");
        WxdArrayString { ptr }
    }

    fn get_count(&self) -> usize {
        unsafe { ffi::wxd_ArrayString_GetCount(self.ptr) as usize }
    }

    fn get_string(&self, index: usize) -> Option<String> {
        unsafe {
            let mut buffer: [c_char; 1024] = [0; 1024]; // Initial buffer
            let len_needed = ffi::wxd_ArrayString_GetString(
                self.ptr,
                index as i32,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );

            if len_needed < 0 {
                return None; // Error or invalid index
            }

            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                // Fit in initial buffer
                let c_str = CStr::from_ptr(buffer.as_ptr());
                Some(c_str.to_string_lossy().into_owned())
            } else {
                // Allocate exact size
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_ArrayString_GetString(
                    self.ptr,
                    index as i32,
                    vec_buffer.as_mut_ptr() as *mut c_char,
                    vec_buffer.len() as i32,
                );
                if len_copied == len_needed {
                    vec_buffer.pop(); // Remove null terminator
                    String::from_utf8(vec_buffer).ok()
                } else {
                    None // Error on second call
                }
            }
        }
    }

    fn into_vec(self) -> Vec<String> {
        let count = self.get_count();
        let mut vec = Vec::with_capacity(count);
        for i in 0..count {
            if let Some(s) = self.get_string(i) {
                vec.push(s);
            } else {
                // Handle error getting string? Push empty or skip?
                // For now, push an empty string to maintain index correspondence if needed elsewhere.
                vec.push(String::new());
            }
        }
        vec // WxdArrayString is dropped here, freeing the C++ object
    }

    // Raw pointer access if needed (use with caution)
    fn as_ptr(&self) -> *mut ffi::wxd_ArrayString_t {
        self.ptr
    }
}

impl Drop for WxdArrayString {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ffi::wxd_ArrayString_Free(self.ptr);
            }
        }
    }
}

// --- FileDialog ---
#[derive(Clone)] // Cloning FileDialog clones the underlying Dialog pointer
pub struct FileDialog {
    dialog_base: Dialog,
}

impl FileDialog {
    /// Creates a new builder for a FileDialog.
    pub fn builder<'a>(parent: Option<&'a dyn WxWidget>) -> FileDialogBuilder<'a> {
        FileDialogBuilder::new(parent)
    }

    /// Creates a new FileDialog wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid pointer to a wxFileDialog.
    pub(crate) unsafe fn from_ptr(ptr: FileDialogPtr) -> Self {
        FileDialog {
            dialog_base: Dialog::from_ptr(ptr as super::DialogPtr),
        }
    }

    fn as_ptr(&self) -> FileDialogPtr {
        self.dialog_base.as_ptr() as FileDialogPtr
    }

    /// Shows the dialog modally.
    /// Returns an integer value which is usually one of the standard dialog return codes
    /// (e.g., ID_OK, ID_CANCEL).
    pub fn show_modal(&self) -> i32 {
        self.dialog_base.show_modal()
    }

    /// Gets the full path of the selected file.
    /// Returns `None` if the dialog was cancelled or an error occurred.
    pub fn get_path(&self) -> Option<String> {
        unsafe {
            let mut buffer: [c_char; 2048] = [0; 2048]; // Larger buffer for paths
            let len_needed = ffi::wxd_FileDialog_GetPath(
                self.as_ptr(),
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );

            if len_needed < 0 {
                return None;
            }

            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                let c_str = CStr::from_ptr(buffer.as_ptr());
                Some(c_str.to_string_lossy().into_owned())
            } else {
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_FileDialog_GetPath(
                    self.as_ptr(),
                    vec_buffer.as_mut_ptr() as *mut c_char,
                    vec_buffer.len() as i32,
                );
                if len_copied == len_needed {
                    vec_buffer.pop();
                    String::from_utf8(vec_buffer).ok()
                } else {
                    None
                }
            }
        }
    }

    /// Gets the full paths of the selected files (for multi-select dialogs).
    pub fn get_paths(&self) -> Vec<String> {
        let arr_str = WxdArrayString::new();
        unsafe {
            ffi::wxd_FileDialog_GetPaths(self.as_ptr(), arr_str.as_ptr());
        }
        arr_str.into_vec()
    }

    /// Gets the filename part of the selected file.
    /// Returns `None` if the dialog was cancelled or an error occurred.
    pub fn get_filename(&self) -> Option<String> {
        unsafe {
            let mut buffer: [c_char; 1024] = [0; 1024];
            let len_needed = ffi::wxd_FileDialog_GetFilename(
                self.as_ptr(),
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );

            if len_needed < 0 {
                return None;
            }

            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                let c_str = CStr::from_ptr(buffer.as_ptr());
                Some(c_str.to_string_lossy().into_owned())
            } else {
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_FileDialog_GetFilename(
                    self.as_ptr(),
                    vec_buffer.as_mut_ptr() as *mut c_char,
                    vec_buffer.len() as i32,
                );
                if len_copied == len_needed {
                    vec_buffer.pop();
                    String::from_utf8(vec_buffer).ok()
                } else {
                    None
                }
            }
        }
    }

    /// Gets the filenames of the selected files (for multi-select dialogs).
    pub fn get_filenames(&self) -> Vec<String> {
        let arr_str = WxdArrayString::new();
        unsafe {
            ffi::wxd_FileDialog_GetFilenames(self.as_ptr(), arr_str.as_ptr());
        }
        arr_str.into_vec()
    }

    /// Gets the directory part of the selected path.
    /// Returns `None` if the dialog was cancelled or an error occurred.
    pub fn get_directory(&self) -> Option<String> {
        unsafe {
            let mut buffer: [c_char; 2048] = [0; 2048];
            let len_needed = ffi::wxd_FileDialog_GetDirectory(
                self.as_ptr(),
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );

            if len_needed < 0 {
                return None;
            }

            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                let c_str = CStr::from_ptr(buffer.as_ptr());
                Some(c_str.to_string_lossy().into_owned())
            } else {
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_FileDialog_GetDirectory(
                    self.as_ptr(),
                    vec_buffer.as_mut_ptr() as *mut c_char,
                    vec_buffer.len() as i32,
                );
                if len_copied == len_needed {
                    vec_buffer.pop();
                    String::from_utf8(vec_buffer).ok()
                } else {
                    None
                }
            }
        }
    }

    /// Gets the index of the filter currently selected.
    pub fn get_filter_index(&self) -> i32 {
        unsafe { ffi::wxd_FileDialog_GetFilterIndex(self.as_ptr()) }
    }

    // TODO: Add Setters if needed
    // set_message, set_path, set_directory, set_filename, set_wildcard, set_filter_index
}

// Implement WxWidget by delegating to the inner Dialog
impl WxWidget for FileDialog {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.dialog_base.handle_ptr()
    }
}

// FileDialogs are Windows (via Dialog base)
// FileDialogs are EvtHandlers (via Dialog base)

// Implement Drop
impl Drop for FileDialog {
    fn drop(&mut self) {
        // The composed Dialog's Drop will be called automatically,
        // which calls wxd_Window_Destroy on the pointer.
    }
}

// --- FileDialogBuilder ---
pub struct FileDialogBuilder<'a> {
    parent: Option<&'a dyn WxWidget>,
    message: String,
    default_dir: String,
    default_file: String,
    wildcard: String,
    style: FileDialogStyle,
    pos: Point,
    size: Size, // Often unused for FileDialog, but kept for consistency
}

impl<'a> FileDialogBuilder<'a> {
    pub fn new(parent: Option<&'a dyn WxWidget>) -> Self {
        FileDialogBuilder {
            parent,
            message: "Choose a file".to_string(), // Default message
            default_dir: String::new(),
            default_file: String::new(),
            wildcard: "*.*".to_string(), // Default wildcard
            style: FileDialogStyle::Open,
            pos: DEFAULT_POSITION,
            size: DEFAULT_SIZE,
        }
    }

    pub fn with_message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn with_default_dir(mut self, dir: &str) -> Self {
        self.default_dir = dir.to_string();
        self
    }

    pub fn with_default_file(mut self, file: &str) -> Self {
        self.default_file = file.to_string();
        self
    }

    pub fn with_wildcard(mut self, wildcard: &str) -> Self {
        self.wildcard = wildcard.to_string();
        self
    }

    pub fn with_style(mut self, style: FileDialogStyle) -> Self {
        self.style = style;
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

    pub fn build(self) -> FileDialog {
        let c_message = CString::new(self.message).expect("CString::new failed for message");
        let c_default_dir =
            CString::new(self.default_dir).expect("CString::new failed for default_dir");
        let c_default_file =
            CString::new(self.default_file).expect("CString::new failed for default_file");
        let c_wildcard = CString::new(self.wildcard).expect("CString::new failed for wildcard");
        let parent_ptr = self.parent.map_or(ptr::null_mut(), |p| p.handle_ptr());

        let ptr = unsafe {
            ffi::wxd_FileDialog_Create(
                parent_ptr,
                c_message.as_ptr(),
                c_default_dir.as_ptr(),
                c_default_file.as_ptr(),
                c_wildcard.as_ptr(),
                self.style.bits() as ffi::wxd_Style_t,
                self.pos.x,
                self.pos.y, // Pass position components
                self.size.width,
                self.size.height, // Pass size components
            )
        };
        if ptr.is_null() {
            panic!("Failed to create wxFileDialog");
        }
        unsafe { FileDialog::from_ptr(ptr) }
    }
}
