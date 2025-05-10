use crate::prelude::*;
use crate::id::ID_ANY;
// REMOVE: use crate::defs::{wxFileCtrlNameStr, ALL_FILES_PATTERN_STR};
use std::ffi::CString;
use wxdragon_sys as ffi; // Added FFI import
use crate::window::{Window, WxWidget}; // Added Window and WxWidget import
use crate::event::WxEvtHandler; // Added WxEvtHandler import
use std::ops::{Deref, DerefMut, BitOr, BitOrAssign}; // ADDED BitOr, BitOrAssign
use std::default::Default; // ADDED Default

// Default name for FileCtrl widget
fn wx_file_ctrl_name_str() -> &'static str {
    "wxFileCtrl"
}

// Default wildcard pattern for FileCtrl
const ALL_FILES_PATTERN: &str = "*.*";

// wxFileCtrl style flags
// TODO: These should be derived from `wxdragon_sys::WXD_FC_XXX` constants once generated.
// REMOVED: pub const FC_OPEN: i64 = ffi::WXD_FC_OPEN;
// REMOVED: pub const FC_SAVE: i64 = ffi::WXD_FC_SAVE;
// REMOVED: pub const FC_MULTIPLE: i64 = ffi::WXD_FC_MULTIPLE;
// REMOVED: pub const FC_NOSHOWHIDDEN: i64 = ffi::WXD_FC_NOSHOWHIDDEN;
// REMOVED: pub const FC_DEFAULT_STYLE: i64 = ffi::WXD_FC_DEFAULT_STYLE; // Typically wxFC_OPEN

/// Window style flags for `FileCtrl`.
///
/// These flags can be combined using the bitwise OR operator (`|`).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum FileCtrlStyle {
    /// For opening files. This is often the default.
    Open = ffi::WXD_FC_OPEN,
    /// For saving files.
    Save = ffi::WXD_FC_SAVE,
    /// Allow multiple files to be selected.
    Multiple = ffi::WXD_FC_MULTIPLE,
    /// Don't show hidden files.
    NoShowHidden = ffi::WXD_FC_NOSHOWHIDDEN,
}

impl FileCtrlStyle {
    /// Returns the raw integer value of the style.
    pub fn bits(self) -> i64 {
        self as i64
    }

    /// The default style for `FileCtrl`.
    pub const DEFAULT: FileCtrlStyle = FileCtrlStyle::Open;
}

impl Default for FileCtrlStyle {
    fn default() -> Self {
        FileCtrlStyle::DEFAULT
    }
}

impl BitOr for FileCtrlStyle {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for FileCtrlStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe {
            *self = std::mem::transmute(self.bits() | rhs.bits());
        }
    }
}

#[derive(Clone)]
pub struct FileCtrl {
    window: Window, // Composition: FileCtrl IS a Window
    #[allow(dead_code)] // parent_ptr is used for Drop logic if implemented later
    parent_ptr: *mut ffi::wxd_Window_t,
}

// REMOVED: impl_widget_type!(FileCtrl, wxd_FileCtrl_t);
// REMOVED: impl_control_type!(FileCtrl);
// REMOVED: impl_window_type!(FileCtrl);
// REMOVED: impl_evthandler_type!(FileCtrl);

impl FileCtrl {
    pub fn builder(parent: &dyn WxWidget) -> FileCtrlBuilder { // Changed to &dyn WxWidget
        FileCtrlBuilder::new(parent)
    }

    // Add specific FileCtrl methods here later if needed, e.g.,
    // pub fn get_path(&self) -> Option<String> { ... }
    // pub fn set_path(&self, path: &str) { ... }
    // pub fn get_filename(&self) -> Option<String> { ... }
    // etc.
}

pub struct FileCtrlBuilder {
    parent_ptr: *mut ffi::wxd_Window_t, // Changed to raw pointer
    id: i32,
    default_directory: String,
    default_filename: String,
    wild_card: String,
    style: FileCtrlStyle, // CHANGED to FileCtrlStyle
    pos: Point,
    size: Size,
    name: String,
}

impl FileCtrlBuilder {
    pub fn new(parent: &dyn WxWidget) -> Self { // Changed to &dyn WxWidget
        FileCtrlBuilder {
            parent_ptr: parent.handle_ptr(), // Use handle_ptr()
            id: ID_ANY as i32, // Explicitly cast ID_ANY to i32
            default_directory: String::new(), // Empty often means current directory
            default_filename: String::new(),
            wild_card: ALL_FILES_PATTERN.to_string(), // Use module-local const
            style: FileCtrlStyle::DEFAULT, // Use the defined default style
            pos: Point::default(),
            size: Size::default(),
            name: wx_file_ctrl_name_str().to_string(), // Use module-local function
        }
    }

    pub fn with_id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    pub fn with_default_directory(mut self, dir: &str) -> Self {
        self.default_directory = dir.to_string();
        self
    }

    pub fn with_default_filename(mut self, filename: &str) -> Self {
        self.default_filename = filename.to_string();
        self
    }

    pub fn with_wild_card(mut self, wildcard: &str) -> Self {
        self.wild_card = wildcard.to_string();
        self
    }

    pub fn with_style(mut self, style: FileCtrlStyle) -> Self { // CHANGED to FileCtrlStyle
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

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn build(self) -> FileCtrl {
        assert!(!self.parent_ptr.is_null(), "FileCtrl requires a parent");
        let c_default_dir = CString::new(self.default_directory).expect("CString::new failed for default_directory");
        let c_default_filename = CString::new(self.default_filename).expect("CString::new failed for default_filename");
        let c_wild_card = CString::new(self.wild_card).expect("CString::new failed for wild_card");
        let c_name = CString::new(self.name).expect("CString::new failed for name");

        let raw_ptr = unsafe {
            ffi::wxd_FileCtrl_Create(
                self.parent_ptr, // Use raw pointer
                self.id,
                c_default_dir.as_ptr(),
                c_default_filename.as_ptr(),
                c_wild_card.as_ptr(),
                self.style.bits(), // CHANGED to use bits()
                self.pos.x,
                self.pos.y,
                self.size.width,
                self.size.height,
                c_name.as_ptr(),
            )
        };
        if raw_ptr.is_null() {
            panic!("Failed to create wxFileCtrl via FFI");
        }
        // Cast the concrete FileCtrl pointer to the base Window pointer for the wrapper
        let window = unsafe { Window::from_ptr(raw_ptr as *mut ffi::wxd_Window_t) };
        FileCtrl {
            window,
            parent_ptr: self.parent_ptr,
        }
    }
}

// Implement WxWidget for FileCtrl
impl WxWidget for FileCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

// Implement Deref to allow FileCtrl to be used where a Window is expected.
impl Deref for FileCtrl {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl DerefMut for FileCtrl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}

// Implement WxEvtHandler for FileCtrl
impl WxEvtHandler for FileCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.window.get_event_handler_ptr()
    }
}

impl Drop for FileCtrl {
    fn drop(&mut self) {
        // Child widgets like FileCtrl are typically managed by their parent in wxWidgets.
        // The `Window` wrapper's Drop logic handles unbinding events.
    }
}

// Associated constants for wxFileCtrl specific styles would go here,
// ideally derived from wxdragon_sys once they are generated.
// pub const FC_OPEN: i64 = wxdragon_sys::WXD_FC_OPEN; // Example 