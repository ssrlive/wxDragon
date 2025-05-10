//! wxTreeCtrl wrapper

use crate::base::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE};
use crate::event::WxEvtHandler;
use crate::id::{Id, ID_ANY};
use crate::window::WxWidget;
use std::ffi::CString;
use std::ops::{BitOr, BitOrAssign};
use std::default::Default;
use std::ptr;
use wxdragon_sys as ffi;

/// Window style flags for `TreeCtrl`.
///
/// These flags can be combined using the bitwise OR operator (`|`).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum TreeCtrlStyle {
    /// Default style. Combines `HasButtons` and `LinesAtRoot`.
    DefaultStyle = ffi::WXD_TR_DEFAULT_STYLE as i64,
    /// Use buttons to show expand/collapse state.
    HasButtons = ffi::WXD_TR_HAS_BUTTONS as i64,
    /// Use lines to show hierarchy at the root level.
    LinesAtRoot = ffi::WXD_TR_LINES_AT_ROOT as i64,
    /// Don't show any lines.
    NoLines = ffi::WXD_TR_NO_LINES as i64,
    /// Only allow a single item to be selected.
    Single = ffi::WXD_TR_SINGLE as i64,
    /// Hide the root item, making its children appear as top-level items.
    HideRoot = ffi::WXD_TR_HIDE_ROOT as i64,
    /// Allow editing of item labels.
    EditLabels = ffi::WXD_TR_EDIT_LABELS as i64,
    // Add other TR_ styles as needed, e.g., TR_FULL_ROW_HIGHLIGHT, TR_MULTIPLE, etc.
    // TR_NO_BUTTONS = ffi::WXD_TR_NO_BUTTONS as i64, (if available)
    // TR_ROW_LINES = ffi::WXD_TR_ROW_LINES as i64, (if available)
    // TR_TWIST_BUTTONS = ffi::WXD_TR_TWIST_BUTTONS as i64, (if available)
}

impl TreeCtrlStyle {
    /// Returns the raw integer value of the style.
    pub fn bits(self) -> i64 {
        self as i64
    }

    /// The default style for `TreeCtrl`.
    pub const DEFAULT: TreeCtrlStyle = TreeCtrlStyle::DefaultStyle;
}

impl Default for TreeCtrlStyle {
    fn default() -> Self {
        TreeCtrlStyle::DEFAULT
    }
}

impl BitOr for TreeCtrlStyle {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for TreeCtrlStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe {
            *self = std::mem::transmute(self.bits() | rhs.bits());
        }
    }
}

// Represents the opaque wxTreeItemId used by wxWidgets.
// This struct owns the pointer returned by the C++ FFI functions
// and is responsible for freeing it via wxd_TreeItemId_Free.
#[derive(Debug)] // Add Debug for easier inspection
pub struct TreeItemId {
    ptr: *mut ffi::WXD_TreeItemId_t,
}

impl TreeItemId {
    // Creates a new TreeItemId from a raw pointer.
    // Assumes ownership of the pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::WXD_TreeItemId_t) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(TreeItemId { ptr })
        }
    }

    // Checks if the underlying wxTreeItemId is valid.
    pub fn is_ok(&self) -> bool {
        unsafe { ffi::wxd_TreeItemId_IsOk(self.ptr) }
    }

    // Returns the raw pointer - use with caution.
    pub(crate) fn as_ptr(&self) -> *mut ffi::WXD_TreeItemId_t {
        self.ptr
    }
}

impl Drop for TreeItemId {
    fn drop(&mut self) {
        // Only free if the pointer is not null.
        if !self.ptr.is_null() {
            unsafe {
                // Tell the C++ side to free the WXD_TreeItemId_t struct.
                ffi::wxd_TreeItemId_Free(self.ptr);
            }
            // Optional: Nullify the pointer after freeing, although Rust won't access it again.
            // self.ptr = ptr::null_mut();
        }
    }
}

// TODO: Implement Clone if needed (requires C API function)

// Represents the wxTreeCtrl widget.
#[derive(Clone)]
pub struct TreeCtrl {
    ptr: *mut ffi::wxd_TreeCtrl_t, // Keep the direct pointer
}

impl TreeCtrl {
    // Unsafe constructor from raw pointer
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_TreeCtrl_t) -> Self {
        assert!(!ptr.is_null(), "Cannot create TreeCtrl from null pointer");
        TreeCtrl { ptr }
    }

    pub fn builder(parent: &impl WxWidget) -> TreeCtrlBuilder {
        TreeCtrlBuilder::new(parent)
    }

    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        let ctrl_ptr = unsafe {
            ffi::wxd_TreeCtrl_Create(
                parent_ptr,
                id,
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t, // Cast to FFI type
            )
        };
        assert!(!ctrl_ptr.is_null(), "wxd_TreeCtrl_Create returned null");
        unsafe { Self::from_ptr(ctrl_ptr) } // Use unsafe from_ptr
    }

    /// Adds the root item to the tree control.
    /// Returns the new item ID, or None if creation failed.
    /// Note: Ignores image and data parameters for now (matches C++ stub).
    pub fn add_root(&self, text: &str) -> Option<TreeItemId> {
        let c_text = CString::new(text).ok()?;
        // Pass -1 for image/selImage, nullptr for data, as per C++ stub
        let item_ptr = unsafe {
            ffi::wxd_TreeCtrl_AddRoot(self.ptr, c_text.as_ptr(), -1, -1, ptr::null_mut())
        };
        unsafe { TreeItemId::from_ptr(item_ptr) }
    }

    /// Appends an item to the given parent item.
    /// Returns the new item ID, or None if creation failed.
    /// Note: Ignores image and data parameters for now (matches C++ stub).
    pub fn append_item(&self, parent: &TreeItemId, text: &str) -> Option<TreeItemId> {
        let c_text = CString::new(text).ok()?;
        // Pass -1 for image/selImage, nullptr for data, as per C++ stub
        let item_ptr = unsafe {
            ffi::wxd_TreeCtrl_AppendItem(
                self.ptr,
                parent.as_ptr(),
                c_text.as_ptr(),
                -1,
                -1,
                ptr::null_mut(),
            )
        };
        unsafe { TreeItemId::from_ptr(item_ptr) }
    }

    /// Deletes the specified item and all its children.
    /// Note: The passed TreeItemId becomes invalid after this call,
    /// but Rust's ownership rules mean it will still be dropped (calling Free).
    pub fn delete(&self, item: TreeItemId) {
        unsafe {
            ffi::wxd_TreeCtrl_Delete(self.ptr, item.as_ptr());
        }
        // item is consumed and will be dropped here
    }

    /// Gets the currently selected item.
    /// Returns None if no item is selected or on error.
    pub fn get_selection(&self) -> Option<TreeItemId> {
        let item_ptr = unsafe { ffi::wxd_TreeCtrl_GetSelection(self.ptr) };
        unsafe { TreeItemId::from_ptr(item_ptr) }
    }

    /// Selects the given item.
    pub fn select_item(&self, item: &TreeItemId) {
        unsafe {
            ffi::wxd_TreeCtrl_SelectItem(self.ptr, item.as_ptr());
        }
    }

    // Add other safe methods here, e.g., get_item_text, expand, etc.
}

// Note: No Drop implementation for TreeCtrl.
// Destruction is managed by the parent wxWindow or wxApp,
// and cleanup (like dropping event closures) relies on the notifier
// attached in wxd_TreeCtrl_Create.

// Builder for TreeCtrl
pub struct TreeCtrlBuilder<'a> {
    parent: &'a dyn WxWidget,
    id: Id,
    pos: Point,
    size: Size,
    style: TreeCtrlStyle,
}

impl<'a> TreeCtrlBuilder<'a> {
    pub fn new(parent: &'a dyn WxWidget) -> Self {
        Self {
            parent,
            id: ID_ANY as Id,
            pos: DEFAULT_POSITION,
            size: DEFAULT_SIZE,
            style: TreeCtrlStyle::DEFAULT,
        }
    }

    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
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

    pub fn with_style(mut self, style: TreeCtrlStyle) -> Self {
        self.style = style;
        self
    }

    pub fn build(self) -> TreeCtrl {
        let parent_ptr = self.parent.handle_ptr();
        let pos = self.pos;
        let size = self.size;
        TreeCtrl::new_impl(parent_ptr, self.id, pos, size, self.style.bits())
    }
}

// Delegate WxWidget and WxEvtHandler to the inner Window
impl WxWidget for TreeCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.ptr as *mut ffi::wxd_Window_t
    }
}

impl WxEvtHandler for TreeCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.ptr as *mut ffi::wxd_EvtHandler_t
    }
}

// TODO: Implement methods and traits
// TODO: Implement methods and traits
