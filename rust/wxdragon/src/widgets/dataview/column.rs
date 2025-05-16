//! DataViewColumn implementation.

use std::ffi::CString;
use wxdragon_sys as ffi;

use super::{DataViewRenderer, DataViewAlign};

/// A column in a DataViewCtrl.
///
/// DataViewColumn associates a renderer with a model column and handles the display
/// of data in a specific column of the control.
pub struct DataViewColumn {
    handle: *mut ffi::wxd_DataViewColumn_t,
}

impl DataViewColumn {
    /// Creates a new column for a DataViewCtrl.
    ///
    /// # Parameters
    ///
    /// * `title` - The header text for this column
    /// * `renderer` - The renderer that will be used to display data in this column
    /// * `model_column` - The column index in the data model
    /// * `width` - The column width (in pixels)
    /// * `align` - The alignment of the column content
    pub fn new(title: &str, renderer: &dyn DataViewRenderer, model_column: usize, width: i32, align: DataViewAlign) -> Self {
        let title_cstr = CString::new(title).unwrap();
        let handle = unsafe {
            ffi::wxd_DataViewColumn_Create(
                title_cstr.as_ptr(),
                renderer.as_raw(),
                model_column as i64,
                width,
                align.bits(),
            )
        };
        Self { handle }
    }

    /// Gets the raw pointer to the native wxDataViewColumn.
    pub fn as_raw(&self) -> *mut ffi::wxd_DataViewColumn_t {
        self.handle
    }
} 