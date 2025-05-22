//! DataViewItem implementation.

use wxdragon_sys as ffi;

/// Represents an item in a DataViewCtrl.
///
/// This struct is a wrapper around a pointer to a C++ wxDataViewItem object.
/// It owns the C++ object when returned from an FFI call that allocates it (e.g., via FromWxDVI helpers).
/// When a DataViewItem is passed from Rust to C++, its internal pointer is used, but C++
/// does not take ownership of the Rust-side `DataViewItem` or the wxDataViewItem it points to.
#[derive(Debug)]
#[repr(C)]
pub struct DataViewItem {
    // This id is a *mut wxDataViewItem (cast to void*) that Rust owns if created by FromWxDVI.
    id: *mut std::ffi::c_void,
}

impl DataViewItem {
    /// Creates a new DataViewItem with a null ID.
    ///
    /// A null DataViewItem is not valid and cannot be used with a DataViewCtrl.
    pub fn new_null() -> Self {
        Self {
            id: std::ptr::null_mut(),
        }
    }

    /// Creates a new DataViewItem with the given numeric ID.
    ///
    /// This is a safe way to create items with simple numeric IDs.
    pub fn from_number(id: usize) -> Self {
        Self {
            id: id as *mut std::ffi::c_void,
        }
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

    /// Creates a new invalid `DataViewItem`.
    /// This is often used to represent the root item or no specific item.
    pub fn new_invalid() -> Self {
        Self {
            id: std::ptr::null_mut(),
        }
    }
}

impl Drop for DataViewItem {
    fn drop(&mut self) {
        if !self.id.is_null() {
            // This item was created by an FFI call (e.g., FromWxDVI) that allocated a wxDataViewItem on the heap.
            // Rust is responsible for releasing this memory.
            unsafe { ffi::wxd_DataViewItem_Release(self.as_raw()) };
            // Set to null to prevent double free if this Drop is somehow called again (though not typical for owned types).
            self.id = std::ptr::null_mut();
        }
    }
}

// It's important that DataViewItem is not Clone or Copy by default if it manages ownership via Drop.
// If cloning is needed, it would require manual implementation (e.g., an explicit clone method that calls
// a C++ FFI function to duplicate the wxDataViewItem if that's meaningful, or by using Rc/Arc if shared ownership
// within Rust is desired, though that doesn't map directly to the C++ object lifecycle here).
// For now, treating it as a unique owner is safest.
