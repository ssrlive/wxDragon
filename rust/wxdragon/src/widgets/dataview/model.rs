//! DataViewModel implementation.

use std::ffi::CString;
use wxdragon_sys as ffi;

use super::Variant;

/// Trait for DataView models.
///
/// A DataViewModel provides the data that is displayed in a DataViewCtrl.
pub trait DataViewModel {
    /// Gets the raw pointer to the native wxDataViewModel.
    fn as_raw(&self) -> *mut ffi::wxd_DataViewModel_t;
}

/// A simple list-based data model for DataViewCtrl.
///
/// DataViewListModel provides a tabular data model that can be used with
/// DataViewCtrl without having to implement a custom model.
pub struct DataViewListModel {
    handle: *mut ffi::wxd_DataViewModel_t,
    column_count: usize,
}

impl DataViewListModel {
    /// Creates a new list model.
    pub fn new() -> Self {
        let handle = unsafe { ffi::wxd_DataViewListModel_Create() };
        Self { 
            handle,
            column_count: 0 
        }
    }

    /// Appends a column to the model.
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the column
    ///
    /// # Returns
    ///
    /// `true` if the column was successfully appended, `false` otherwise.
    pub fn append_column(&mut self, name: &str) -> bool {
        let name_cstr = CString::new(name).unwrap();
        let result = unsafe { ffi::wxd_DataViewListModel_AppendColumn(self.handle, name_cstr.as_ptr()) };
        if result {
            self.column_count += 1;
        }
        result
    }

    /// Appends a new row to the model.
    ///
    /// # Returns
    ///
    /// `true` if the row was successfully appended, `false` otherwise.
    /// 
    /// Note: You must add at least one column before adding rows.
    pub fn append_row(&self) -> bool {
        if self.column_count == 0 {
            eprintln!("Error: Cannot append row. No columns defined yet. Use append_column() first.");
            return false;
        }
        unsafe { ffi::wxd_DataViewListModel_AppendRow(self.handle) }
    }

    /// Sets a value in the model at the specified row and column.
    ///
    /// # Parameters
    ///
    /// * `row` - The row index
    /// * `col` - The column index
    /// * `value` - The value to set
    ///
    /// # Returns
    ///
    /// `true` if the value was successfully set, `false` otherwise.
    fn internal_set_value(&self, row: usize, col: usize, value: &Variant) -> bool {
        if col >= self.column_count {
            eprintln!("Error: Column index {} is out of bounds (column count: {})", col, self.column_count);
            return false;
        }
        unsafe { ffi::wxd_DataViewListModel_SetValue(self.handle, row as u64, col as u64, value.as_raw()) }
    }

    /// Sets a value in the model at the specified row and column.
    ///
    /// This method provides a more convenient way to set values by accepting any type
    /// that can be converted to a Variant, such as strings, booleans, integers, etc.
    ///
    /// # Parameters
    ///
    /// * `row` - The row index
    /// * `col` - The column index
    /// * `value` - The value to set (will be converted to a Variant)
    ///
    /// # Returns
    ///
    /// `true` if the value was successfully set, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// model.set_value(0, 0, "001");
    /// model.set_value(0, 1, "John Smith");
    /// model.set_value(0, 2, true);
    /// model.set_value(0, 3, 92);
    /// ```
    pub fn set_value<T: Into<Variant>>(&self, row: usize, col: usize, value: T) -> bool {
        let variant = value.into();
        self.internal_set_value(row, col, &variant)
    }

    /// Gets the number of columns in this model.
    pub fn get_column_count(&self) -> usize {
        self.column_count
    }
}

impl DataViewModel for DataViewListModel {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewModel_t {
        self.handle
    }
}

impl Default for DataViewListModel {
    fn default() -> Self {
        Self::new()
    }
} 