//! DataViewItem implementation.

use wxdragon_sys as ffi;

/// Represents an item in a DataViewCtrl.
/// 
/// A DataViewItem is used to uniquely identify an item in a 
/// DataViewModel and DataViewCtrl.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataViewItem {
    id: *mut std::ffi::c_void,
}

impl DataViewItem {
    /// Creates a new DataViewItem with a null ID.
    /// 
    /// A null DataViewItem is not valid and cannot be used with a DataViewCtrl.
    pub fn new_null() -> Self {
        Self { id: std::ptr::null_mut() }
    }
    
    /// Creates a new DataViewItem with the given numeric ID.
    /// 
    /// This is a safe way to create items with simple numeric IDs.
    pub fn from_number(id: usize) -> Self {
        Self { id: id as *mut std::ffi::c_void }
    }
    
    /// Checks if the DataViewItem is valid (non-null).
    pub fn is_valid(&self) -> bool {
        !self.id.is_null()
    }
    
    /// Creates a DataViewItem from a raw FFI type.
    /// 
    /// # Safety
    /// This function should only be used with valid wxd_DataViewItem_t values.
    pub(crate) unsafe fn from_raw(raw: ffi::wxd_DataViewItem_t) -> Self {
        Self { id: raw.id }
    }
    
    /// Checks if a raw FFI type represents a valid item.
    pub(crate) fn is_valid_raw(raw: ffi::wxd_DataViewItem_t) -> bool {
        !raw.id.is_null()
    }
    
    /// Converts the DataViewItem to its raw FFI representation.
    pub(crate) fn as_raw(&self) -> ffi::wxd_DataViewItem_t {
        ffi::wxd_DataViewItem_t { id: self.id }
    }
    
    /// Gets the numeric ID if this item was created with from_number().
    /// 
    /// # Returns
    /// The numeric ID or 0 if the item is null or not created with from_number.
    pub fn as_number(&self) -> usize {
        if self.id.is_null() {
            0
        } else {
            self.id as usize
        }
    }
} 