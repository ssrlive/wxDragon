//! wxTreeCtrl wrapper
//!
//! The `TreeCtrl` widget provides a tree control for displaying hierarchical data.
//! This module also includes `TreeItemData` for associating custom data with tree items.
//!
//! # Examples
//!
//! ```rust,no_run
//! use wxdragon::prelude::*;
//! use wxdragon::widgets::treectrl::{TreeCtrl, TreeCtrlStyle};
//!
//! // Create custom data to associate with tree items
//! #[derive(Clone)]
//! struct PersonData {
//!     name: String,
//!     age: u32,
//!     role: String,
//! }
//!
//! fn create_tree_with_data(parent: &dyn WxWidget) -> TreeCtrl {
//!     // Create a tree control
//!     let tree = TreeCtrl::builder(parent)
//!         .with_style(TreeCtrlStyle::HasButtons | TreeCtrlStyle::LinesAtRoot)
//!         .build();
//!
//!     // Add root with associated data
//!     let ceo = PersonData {
//!         name: "John Smith".to_string(),
//!         age: 52,
//!         role: "CEO".to_string(),
//!     };
//!     let root = tree.add_root_with_data("Company", ceo).unwrap();
//!
//!     // Add child with different data type
//!     tree.append_item_with_data(&root, "Budget", 1000000).unwrap();
//!
//!     // Add another child with string data
//!     tree.append_item_with_data(
//!         &root,
//!         "Mission",
//!         "To create amazing products".to_string()
//!     ).unwrap();
//!
//!     // Later, when handling selection events:
//!     // if let Some(item_id) = tree.get_selection() {
//!     //     if let Some(item_data) = tree.get_item_data(&item_id) {
//!     //         if let Some(person) = item_data.downcast_ref::<PersonData>() {
//!     //             println!("Selected person: {}", person.name);
//!     //         } else if let Some(budget) = item_data.downcast_ref::<i32>() {
//!     //             println!("Selected budget: ${}", budget);
//!     //         } else if let Some(text) = item_data.downcast_ref::<String>() {
//!     //             println!("Selected text: {}", text);
//!     //         } else {
//!     //             println!("Data type: {}", item_data.get_type_name());
//!     //         }
//!     //     }
//!     // }
//!
//!     tree
//! }
//! ```

use crate::event::WxEvtHandler;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::any::Any;
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

/// TreeItemData allows associating arbitrary data with TreeCtrl items.
///
/// This safe wrapper around wxTreeItemData enables storing any Rust type
/// that implements `Any + Send` with tree items. The data can later be
/// retrieved and downcast to the original type.
///
/// # Examples
///
/// ```rust,no_run
/// use wxdragon::prelude::*;
/// use wxdragon::widgets::treectrl::TreeCtrl;
///
/// // Custom data type
/// #[derive(Clone)]
/// struct PersonData {
///     name: String,
///     age: u32,
/// }
///
/// // Create a tree and add items with data
/// let tree = TreeCtrl::builder(&panel).build();
///
/// // Add item with PersonData
/// let person = PersonData {
///     name: "Alice".to_string(),
///     age: 30,
/// };
/// let item = tree.add_root_with_data("Alice", person).unwrap();
///
/// // Later, retrieve and use the data
/// if let Some(data) = tree.get_item_data(&item) {
///     // Check type and downcast
///     if let Some(person) = data.downcast_ref::<PersonData>() {
///         println!("Name: {}, Age: {}", person.name, person.age);
///     }
///     
///     // Or check type using is<T>()
///     if data.is::<PersonData>() {
///         println!("This is a person entry");
///     }
///     
///     // Get friendly type name
///     println!("Data type: {}", data.get_type_name());
/// }
/// ```
///
/// # Data Type Support
///
/// TreeItemData can store any Rust type that implements `Clone + Any + Send`.
/// When retrieving data, the following happens:
///
/// - Common types like String, integers, floats, bool, etc. can be directly accessed
/// - Custom types that implement Clone can be accessed if they match the expected type
/// - For types that can't be cloned or don't match, type information is still available
///
/// Use `downcast_ref<T>()` to access the data with the correct type, and `is<T>()`
/// to check if the data is of a specific type.
pub struct TreeItemData {
    ptr: *mut ffi::WXD_TreeItemData_t,
    data: Option<Box<dyn Any + Send>>,
    /// Flag indicating if the TreeItemData owns the C++ pointer and should free it
    owns_ptr: bool,
    /// Raw pointer to the original data for direct access (unsafe)
    raw_data_ptr: *const Box<dyn Any + Send>,
}

impl TreeItemData {
    /// Creates a new TreeItemData instance with the given data.
    pub fn new<T: Clone + 'static + Send>(data: T) -> Self {
        let boxed_data: Box<dyn Any + Send> = Box::new(data);
        let data_ptr = Box::into_raw(Box::new(boxed_data));

        let ptr = unsafe { ffi::wxd_TreeItemData_Create(data_ptr as *mut _) };

        TreeItemData {
            ptr,
            data: None,     // We've moved the data into C++ land
            owns_ptr: true, // We created this pointer, so we own it
            raw_data_ptr: data_ptr,
        }
    }

    /// Creates a new TreeItemData without any associated data.
    pub fn empty() -> Self {
        let ptr = unsafe { ffi::wxd_TreeItemData_Create(ptr::null_mut()) };

        TreeItemData {
            ptr,
            data: None,
            owns_ptr: true,
            raw_data_ptr: ptr::null(),
        }
    }

    /// Returns the raw pointer to the underlying wxTreeItemData.
    pub(crate) fn as_ptr(&self) -> *mut ffi::WXD_TreeItemData_t {
        self.ptr
    }

    /// Attempts to downcast the contained data to a specific type.
    ///
    /// This method allows you to retrieve data stored in a tree item
    /// if you know the exact type. Returns None if the data is not
    /// of the expected type.
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        // First try if we have local data
        if let Some(data) = &self.data {
            if let Some(val) = data.downcast_ref::<T>() {
                return Some(val);
            }
        }

        // If we don't have local data, but we have a raw pointer to the original data,
        // try to access it directly (this is unsafe but necessary for complex types)
        if !self.raw_data_ptr.is_null() {
            unsafe {
                let orig_data = &**self.raw_data_ptr;
                if let Some(val) = orig_data.downcast_ref::<T>() {
                    return Some(val);
                }
            }
        }

        None
    }

    /// Checks if the contained data is of type T.
    pub fn is<T: 'static>(&self) -> bool {
        // First check local data
        if let Some(data) = &self.data {
            if data.is::<T>() {
                return true;
            }
        }

        // Then check raw pointer
        if !self.raw_data_ptr.is_null() {
            unsafe {
                let orig_data = &**self.raw_data_ptr;
                return orig_data.is::<T>();
            }
        }

        false
    }

    /// Gets the original type information if this is a retrieved tree item
    pub fn get_type_info(&self) -> Option<&TypeInfo> {
        self.data
            .as_ref()
            .and_then(|data| data.downcast_ref::<TypeInfo>())
    }

    /// Gets a friendly type name of the contained data.
    ///
    /// This method returns a simplified type name without namespace prefixes,
    /// making it more readable for display purposes.
    pub fn get_type_name(&self) -> String {
        // First check if we have local data
        if let Some(data) = &self.data {
            if let Some(type_info) = data.downcast_ref::<TypeInfo>() {
                // Return the stored type name from TypeInfo
                return simplify_type_name(&type_info.type_name);
            }

            // For known types, return friendly names
            if data.is::<String>() {
                return "String".to_string();
            } else if data.is::<i32>() {
                return "Integer (i32)".to_string();
            } else if data.is::<i64>() {
                return "Integer (i64)".to_string();
            } else if data.is::<u32>() {
                return "Unsigned Integer (u32)".to_string();
            } else if data.is::<u64>() {
                return "Unsigned Integer (u64)".to_string();
            } else if data.is::<f32>() {
                return "Float (f32)".to_string();
            } else if data.is::<f64>() {
                return "Float (f64)".to_string();
            } else if data.is::<bool>() {
                return "Boolean".to_string();
            } else if data.is::<()>() {
                return "Unit (empty)".to_string();
            } else if data.is::<Vec<String>>() {
                return "Vector of Strings".to_string();
            } else if data.is::<Vec<i32>>() {
                return "Vector of Integers".to_string();
            }

            // Default case - get the type name from the type_id
            return simplify_type_name(&std::any::type_name_of_val(&**data));
        }

        // If we don't have local data, but we have a raw pointer to the original data,
        // try to access it directly
        if !self.raw_data_ptr.is_null() {
            unsafe {
                let orig_data = &**self.raw_data_ptr;
                return simplify_type_name(&std::any::type_name_of_val(orig_data));
            }
        }

        "No data".to_string()
    }

    /// Creates a TreeItemData from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be a valid WXD_TreeItemData_t pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::WXD_TreeItemData_t) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            // Retrieve the client data pointer from C++
            let data_ptr = ffi::wxd_TreeItemData_GetClientData(ptr);

            // Store the original data pointer for direct access
            let raw_data_ptr = data_ptr as *const Box<dyn Any + Send>;

            // Create the data to return
            let data: Option<Box<dyn Any + Send>> = if !data_ptr.is_null() {
                // The pointer points to a Box<dyn Any + Send> that we stored
                let boxed_data_ptr = data_ptr as *const Box<dyn Any + Send>;

                if !boxed_data_ptr.is_null() {
                    // For primitive types, we can clone them directly
                    let boxed_data = &**boxed_data_ptr;

                    // Clone primitive types for local access
                    if let Some(s) = boxed_data.downcast_ref::<String>() {
                        Some(Box::new(s.clone()))
                    } else if let Some(i) = boxed_data.downcast_ref::<i32>() {
                        Some(Box::new(*i))
                    } else if let Some(i) = boxed_data.downcast_ref::<i64>() {
                        Some(Box::new(*i))
                    } else if let Some(i) = boxed_data.downcast_ref::<u32>() {
                        Some(Box::new(*i))
                    } else if let Some(i) = boxed_data.downcast_ref::<u64>() {
                        Some(Box::new(*i))
                    } else if let Some(i) = boxed_data.downcast_ref::<usize>() {
                        Some(Box::new(*i))
                    } else if let Some(i) = boxed_data.downcast_ref::<isize>() {
                        Some(Box::new(*i))
                    } else if let Some(f) = boxed_data.downcast_ref::<f32>() {
                        Some(Box::new(*f))
                    } else if let Some(f) = boxed_data.downcast_ref::<f64>() {
                        Some(Box::new(*f))
                    } else if let Some(b) = boxed_data.downcast_ref::<bool>() {
                        Some(Box::new(*b))
                    } else if let Some(c) = boxed_data.downcast_ref::<char>() {
                        Some(Box::new(*c))
                    } else if let Some(_) = boxed_data.downcast_ref::<()>() {
                        Some(Box::new(()))
                    } else {
                        // For complex types, we'll provide type info for debugging
                        // but we'll rely on raw_data_ptr for actual access
                        let type_id = boxed_data.type_id();
                        let type_name = std::any::type_name_of_val(boxed_data);

                        Some(Box::new(TypeInfo {
                            type_id,
                            type_name: type_name.to_string(),
                        }))
                    }
                } else {
                    // Null inner pointer, create a placeholder
                    let empty_box: Box<dyn Any + Send> = Box::new(());
                    Some(empty_box)
                }
            } else {
                // No data attached
                let empty_box: Box<dyn Any + Send> = Box::new(());
                Some(empty_box)
            };

            Some(TreeItemData {
                ptr,
                data,
                owns_ptr: false, // We received this pointer from TreeCtrl, it owns it
                raw_data_ptr,
            })
        }
    }
}

/// Simplifies a Rust type name by removing module paths.
///
/// Converts something like "alloc::string::String" to just "String"
/// or "example::rust::gallery::tabs::treectrl_tab::PersonData" to just "PersonData".
fn simplify_type_name(type_name: &str) -> String {
    type_name
        .rsplit("::")
        .next()
        .unwrap_or(type_name)
        .to_string()
}

/// Information about a type that was stored in a tree item
#[derive(Debug)]
pub struct TypeInfo {
    /// Type ID of the original object
    pub type_id: std::any::TypeId,
    /// Name of the original object's type
    pub type_name: String,
    // We don't store the original pointer because it would make TypeInfo not Send-safe
    // Instead, we just use the type information
}

impl Drop for TreeItemData {
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.owns_ptr {
            unsafe {
                // If we own the data, we need to retrieve and free it first
                if self.data.is_none() {
                    let data_ptr =
                        ffi::wxd_TreeItemData_GetClientData(self.ptr) as *mut Box<dyn Any + Send>;
                    if !data_ptr.is_null() {
                        // Take ownership of the Box and drop it
                        let _ = Box::from_raw(data_ptr);
                    }
                }

                // Now free the TreeItemData itself
                ffi::wxd_TreeItemData_Free(self.ptr);
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
    pub fn add_root(&self, text: &str) -> Option<TreeItemId> {
        let c_text = CString::new(text).unwrap_or_default();
        // Pass -1 for image/selImage, nullptr for data
        let item_ptr = unsafe {
            ffi::wxd_TreeCtrl_AddRoot(self.as_ptr(), c_text.as_ptr(), -1, -1, ptr::null_mut())
        };
        unsafe { TreeItemId::from_ptr(item_ptr) }
    }

    /// Adds the root item to the tree control with associated data.
    ///
    /// This method creates the root item and associates custom data with it.
    /// The data is stored in the tree control and can be retrieved later using
    /// `get_item_data()`.
    ///
    /// # Parameters
    ///
    /// * `text` - The text label for the root item
    /// * `data` - Custom data to associate with the item. Can be any type that
    ///            implements `Clone + 'static + Send`
    ///
    /// # Returns
    ///
    /// * `Some(TreeItemId)` - The ID of the newly created item
    /// * `None` - If item creation failed
    ///
    /// # Ownership
    ///
    /// The tree control takes ownership of the data. When the tree item is deleted,
    /// the associated data will be properly cleaned up.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use wxdragon::prelude::*;
    /// use wxdragon::widgets::treectrl::TreeCtrl;
    ///
    /// #[derive(Clone)]
    /// struct CompanyData { employees: u32, revenue: f64 }
    ///
    /// let tree = TreeCtrl::builder(&panel).build();
    /// let company = CompanyData { employees: 500, revenue: 10000000.0 };
    /// let root = tree.add_root_with_data("ACME Corp", company).unwrap();
    /// ```
    pub fn add_root_with_data<T: Clone + 'static + Send>(
        &self,
        text: &str,
        data: T,
    ) -> Option<TreeItemId> {
        let c_text = CString::new(text).unwrap_or_default();
        let item_data = TreeItemData::new(data);

        let item_ptr = unsafe {
            ffi::wxd_TreeCtrl_AddRoot(
                self.as_ptr(),
                c_text.as_ptr(),
                -1,
                -1,
                item_data.as_ptr() as *mut _,
            )
        };

        // Don't drop the TreeItemData, it's now owned by the tree control
        std::mem::forget(item_data);

        unsafe { TreeItemId::from_ptr(item_ptr) }
    }

    /// Appends an item to the given parent item.
    /// Returns the new item ID, or None if creation failed.
    pub fn append_item(&self, parent: &TreeItemId, text: &str) -> Option<TreeItemId> {
        let c_text = CString::new(text).unwrap_or_default();
        // Pass -1 for image/selImage, nullptr for data
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

    /// Appends an item to the given parent item with associated data.
    ///
    /// This method creates a child item under the specified parent and associates
    /// custom data with it. The data is stored in the tree control and can be
    /// retrieved later using `get_item_data()`.
    ///
    /// # Parameters
    ///
    /// * `parent` - The parent item to which this item will be added
    /// * `text` - The text label for the item
    /// * `data` - Custom data to associate with the item. Can be any type that
    ///            implements `Clone + 'static + Send`
    ///
    /// # Returns
    ///
    /// * `Some(TreeItemId)` - The ID of the newly created item
    /// * `None` - If item creation failed
    ///
    /// # Ownership
    ///
    /// The tree control takes ownership of the data. When the tree item is deleted,
    /// the associated data will be properly cleaned up.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use wxdragon::prelude::*;
    /// use wxdragon::widgets::treectrl::TreeCtrl;
    ///
    /// let tree = TreeCtrl::builder(&panel).build();
    /// let root = tree.add_root("Departments").unwrap();
    ///
    /// // Add child with integer data (budget)
    /// tree.append_item_with_data(&root, "Engineering", 500000).unwrap();
    ///
    /// // Add child with string data (description)
    /// tree.append_item_with_data(&root, "Marketing",
    ///     "Handles all promotional activities".to_string()).unwrap();
    /// ```
    pub fn append_item_with_data<T: Clone + 'static + Send>(
        &self,
        parent: &TreeItemId,
        text: &str,
        data: T,
    ) -> Option<TreeItemId> {
        let c_text = CString::new(text).unwrap_or_default();
        let item_data = TreeItemData::new(data);

        let item_ptr = unsafe {
            ffi::wxd_TreeCtrl_AppendItem(
                self.as_ptr(),
                parent.as_ptr(),
                c_text.as_ptr(),
                -1,
                -1,
                item_data.as_ptr() as *mut _,
            )
        };

        // Don't drop the TreeItemData, it's now owned by the tree control
        std::mem::forget(item_data);

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

    /// Gets the data associated with an item.
    ///
    /// This method retrieves any custom data that was previously associated with
    /// the tree item using `add_root_with_data`, `append_item_with_data`, or
    /// `set_item_data`.
    ///
    /// # Parameters
    ///
    /// * `item` - The tree item to get data from
    ///
    /// # Returns
    ///
    /// * `Some(TreeItemData)` - The data associated with the item, which can be
    ///                          downcast to the original type using `downcast_ref<T>()`
    /// * `None` - If the item has no associated data or on error
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use wxdragon::prelude::*;
    /// use wxdragon::widgets::treectrl::TreeCtrl;
    ///
    /// let tree = TreeCtrl::builder(&panel).build();
    /// let root = tree.add_root_with_data("Root", "Root data".to_string()).unwrap();
    ///
    /// if let Some(data) = tree.get_item_data(&root) {
    ///     if let Some(text) = data.downcast_ref::<String>() {
    ///         println!("Item data: {}", text);
    ///     }
    /// }
    /// ```
    pub fn get_item_data(&self, item: &TreeItemId) -> Option<TreeItemData> {
        let data_ptr = unsafe { ffi::wxd_TreeCtrl_GetItemData(self.as_ptr(), item.as_ptr()) };
        unsafe { TreeItemData::from_ptr(data_ptr) }
    }

    /// Sets data for an item.
    ///
    /// This method associates custom data with an existing tree item. Any previously
    /// associated data will be properly cleaned up.
    ///
    /// # Parameters
    ///
    /// * `item` - The tree item to set data for
    /// * `data` - Custom data to associate with the item. Can be any type that
    ///            implements `Clone + 'static + Send`
    ///
    /// # Returns
    ///
    /// * `true` - If the data was successfully set
    /// * `false` - If setting the data failed
    ///
    /// # Ownership
    ///
    /// The tree control takes ownership of the data. When the tree item is deleted,
    /// or when new data is set, the associated data will be properly cleaned up.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use wxdragon::prelude::*;
    /// use wxdragon::widgets::treectrl::TreeCtrl;
    ///
    /// let tree = TreeCtrl::builder(&panel).build();
    /// let root = tree.add_root("Root").unwrap();
    ///
    /// // Set data for an existing item
    /// tree.set_item_data(&root, "New root data".to_string());
    /// ```
    pub fn set_item_data<T: Clone + 'static + Send>(&self, item: &TreeItemId, data: T) -> bool {
        let item_data = TreeItemData::new(data);
        let result = unsafe {
            ffi::wxd_TreeCtrl_SetItemData(self.as_ptr(), item.as_ptr(), item_data.as_ptr())
        };

        // Don't drop the TreeItemData, it's now owned by the tree control
        if result {
            std::mem::forget(item_data);
        }

        result
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
