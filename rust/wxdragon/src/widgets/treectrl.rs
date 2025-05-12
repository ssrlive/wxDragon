//! wxTreeCtrl wrapper

use crate::event::WxEvtHandler;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use std::ptr;
use wxdragon_sys as ffi;

// --- TreeCtrl Styles ---
widget_style_enum!(
    name: TreeCtrlStyle,
    doc: "Style flags for TreeCtrl widget.",
    variants: {
        Default: ffi::WXD_TR_DEFAULT_STYLE as i64, "Default style. Combines `HasButtons` and `LinesAtRoot`.",
        HasButtons: ffi::WXD_TR_HAS_BUTTONS as i64, "Use buttons to show expand/collapse state.",
        LinesAtRoot: ffi::WXD_TR_LINES_AT_ROOT as i64, "Use lines to show hierarchy at the root level.",
        NoLines: ffi::WXD_TR_NO_LINES as i64, "Don't show any lines.",
        Single: ffi::WXD_TR_SINGLE as i64, "Only allow a single item to be selected.",
        HideRoot: ffi::WXD_TR_HIDE_ROOT as i64, "Hide the root item, making its children appear as top-level items.",
        EditLabels: ffi::WXD_TR_EDIT_LABELS as i64, "Allow editing of item labels."
        // Add other TR_ styles as needed, e.g., TR_FULL_ROW_HIGHLIGHT, TR_MULTIPLE, etc.
        // TR_NO_BUTTONS = ffi::WXD_TR_NO_BUTTONS as i64, (if available)
        // TR_ROW_LINES = ffi::WXD_TR_ROW_LINES as i64, (if available)
        // TR_TWIST_BUTTONS = ffi::WXD_TR_TWIST_BUTTONS as i64, (if available)
    },
    default_variant: Default
);

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
        }
    }
}

// Represents the wxTreeCtrl widget.
#[derive(Clone)]
pub struct TreeCtrl {
    window: Window,
}

impl TreeCtrl {
    /// Creates a new TreeCtrl builder.
    pub fn builder(parent: &dyn WxWidget) -> TreeCtrlBuilder {
        TreeCtrlBuilder::new(parent)
    }

    /// Creates a new TreeCtrl wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_TreeCtrl_t` pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_TreeCtrl_t) -> Self {
        TreeCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Internal implementation used by the builder.
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        assert!(!parent_ptr.is_null(), "TreeCtrl parent cannot be null");
        
        let ctrl_ptr = unsafe {
            ffi::wxd_TreeCtrl_Create(
                parent_ptr,
                id,
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t,
            )
        };
        
        if ctrl_ptr.is_null() {
            panic!("Failed to create wxTreeCtrl");
        }
        
        unsafe { Self::from_ptr(ctrl_ptr) }
    }

    /// Returns the raw underlying TreeCtrl pointer.
    fn as_ptr(&self) -> *mut ffi::wxd_TreeCtrl_t {
        self.window.as_ptr() as *mut ffi::wxd_TreeCtrl_t
    }

    /// Adds the root item to the tree control.
    /// Returns the new item ID, or None if creation failed.
    /// Note: Ignores image and data parameters for now (matches C++ stub).
    pub fn add_root(&self, text: &str) -> Option<TreeItemId> {
        let c_text = CString::new(text).unwrap_or_default();
        // Pass -1 for image/selImage, nullptr for data, as per C++ stub
        let item_ptr = unsafe {
            ffi::wxd_TreeCtrl_AddRoot(self.as_ptr(), c_text.as_ptr(), -1, -1, ptr::null_mut())
        };
        unsafe { TreeItemId::from_ptr(item_ptr) }
    }

    /// Appends an item to the given parent item.
    /// Returns the new item ID, or None if creation failed.
    /// Note: Ignores image and data parameters for now (matches C++ stub).
    pub fn append_item(&self, parent: &TreeItemId, text: &str) -> Option<TreeItemId> {
        let c_text = CString::new(text).unwrap_or_default();
        // Pass -1 for image/selImage, nullptr for data, as per C++ stub
        let item_ptr = unsafe {
            ffi::wxd_TreeCtrl_AppendItem(
                self.as_ptr(),
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
            ffi::wxd_TreeCtrl_Delete(self.as_ptr(), item.as_ptr());
        }
        // item is consumed and will be dropped here
    }

    /// Gets the currently selected item.
    /// Returns None if no item is selected or on error.
    pub fn get_selection(&self) -> Option<TreeItemId> {
        let item_ptr = unsafe { ffi::wxd_TreeCtrl_GetSelection(self.as_ptr()) };
        unsafe { TreeItemId::from_ptr(item_ptr) }
    }

    /// Selects the given item.
    pub fn select_item(&self, item: &TreeItemId) {
        unsafe {
            ffi::wxd_TreeCtrl_SelectItem(self.as_ptr(), item.as_ptr());
        }
    }

    // Add other safe methods here, e.g., get_item_text, expand, etc.
}

// Apply common trait implementations for this widget
implement_widget_traits_with_target!(TreeCtrl, window, Window);

// Use the widget_builder macro for TreeCtrl
widget_builder!(
    name: TreeCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: TreeCtrlStyle,
    fields: {},
    build_impl: |slf| {
        TreeCtrl::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            slf.pos,
            slf.size,
            slf.style.bits()
        )
    }
);
