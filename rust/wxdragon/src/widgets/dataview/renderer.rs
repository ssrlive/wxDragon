//! DataViewRenderer implementation.

use super::{DataViewAlign, DataViewCellMode, VariantType};
use std::ffi::CString;
use wxdragon_sys as ffi;

/// Base trait for DataView renderers.
///
/// DataViewRenderer objects are responsible for drawing the data in the columns
/// of a DataViewCtrl and potentially handling user interaction.
pub trait DataViewRenderer {
    /// Gets the raw pointer to the native wxDataViewRenderer.
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t;
}

/// A text renderer for DataViewCtrl columns.
///
/// DataViewTextRenderer displays text data in a column.
pub struct DataViewTextRenderer {
    handle: *mut ffi::wxd_DataViewRenderer_t,
}

impl DataViewTextRenderer {
    /// Creates a new text renderer.
    ///
    /// # Parameters
    ///
    /// * `variant_type` - The type of data this renderer can display
    /// * `mode` - The cell mode (inert, activatable, or editable)
    /// * `align` - The text alignment
    pub fn new(variant_type: VariantType, mode: DataViewCellMode, align: DataViewAlign) -> Self {
        let variant_type_str = variant_type.to_type_string();
        let variant_type_cstr = CString::new(variant_type_str).unwrap();
        let handle = unsafe {
            ffi::wxd_DataViewTextRenderer_Create(
                variant_type_cstr.as_ptr(),
                mode.bits(),
                align.bits(),
            )
        };
        Self { handle }
    }
}

impl DataViewRenderer for DataViewTextRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.handle
    }
}

/// A toggle (checkbox) renderer for DataViewCtrl columns.
///
/// DataViewToggleRenderer displays boolean data as a checkbox.
pub struct DataViewToggleRenderer {
    handle: *mut ffi::wxd_DataViewRenderer_t,
}

impl DataViewToggleRenderer {
    /// Creates a new toggle renderer.
    ///
    /// # Parameters
    ///
    /// * `variant_type` - The type of data this renderer can display (typically Bool)
    /// * `mode` - The cell mode (typically activatable for toggles)
    /// * `align` - The alignment of the checkbox
    pub fn new(variant_type: VariantType, mode: DataViewCellMode, align: DataViewAlign) -> Self {
        let variant_type_str = variant_type.to_type_string();
        let variant_type_cstr = CString::new(variant_type_str).unwrap();
        let handle = unsafe {
            ffi::wxd_DataViewToggleRenderer_Create(
                variant_type_cstr.as_ptr(),
                mode.bits(),
                align.bits(),
            )
        };
        Self { handle }
    }
}

impl DataViewRenderer for DataViewToggleRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.handle
    }
}

/// A progress bar renderer for DataViewCtrl columns.
///
/// DataViewProgressRenderer displays numeric data as a progress bar.
pub struct DataViewProgressRenderer {
    handle: *mut ffi::wxd_DataViewRenderer_t,
}

impl DataViewProgressRenderer {
    /// Creates a new progress renderer.
    ///
    /// # Parameters
    ///
    /// * `variant_type` - The type of data this renderer can display (typically Int32)
    /// * `mode` - The cell mode (typically inert for progress bars)
    pub fn new(variant_type: VariantType, mode: DataViewCellMode) -> Self {
        let variant_type_str = variant_type.to_type_string();
        let variant_type_cstr = CString::new(variant_type_str).unwrap();
        let handle = unsafe {
            ffi::wxd_DataViewProgressRenderer_Create(
                variant_type_cstr.as_ptr(),
                mode.bits(),
                0, // align - ignored for progress renderer
            )
        };
        Self { handle }
    }
}

impl DataViewRenderer for DataViewProgressRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.handle
    }
}

/// An icon-text renderer for DataViewCtrl columns.
///
/// DataViewIconTextRenderer displays an icon followed by text.
pub struct DataViewIconTextRenderer {
    handle: *mut ffi::wxd_DataViewRenderer_t,
}

impl DataViewIconTextRenderer {
    /// Creates a new icon-text renderer.
    ///
    /// # Parameters
    ///
    /// * `variant_type` - The type of data this renderer can display (e.g., String for text part)
    /// * `mode` - The cell mode
    /// * `align` - The alignment
    pub fn new(variant_type: VariantType, mode: DataViewCellMode, align: DataViewAlign) -> Self {
        let variant_type_str = variant_type.to_type_string();
        let variant_type_cstr =
            CString::new(variant_type_str).unwrap_or_else(|_| CString::new("string").unwrap());
        let handle = unsafe {
            ffi::wxd_DataViewIconTextRenderer_Create(
                variant_type_cstr.as_ptr(),
                mode.bits(),
                align.bits(),
            )
        };
        Self { handle }
    }
}

impl DataViewRenderer for DataViewIconTextRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.handle
    }
}

/// A bitmap renderer for DataViewCtrl columns.
///
/// DataViewBitmapRenderer displays a bitmap in a cell.
pub struct DataViewBitmapRenderer {
    handle: *mut ffi::wxd_DataViewRenderer_t,
}

impl DataViewBitmapRenderer {
    /// Creates a new bitmap renderer.
    ///
    /// # Parameters
    ///
    /// * `mode` - The cell mode
    /// * `align` - The alignment
    pub fn new(mode: DataViewCellMode, align: DataViewAlign) -> Self {
        // Bitmap renderer always uses the "bitmap" type
        let variant_type_cstr = CString::new("bitmap").unwrap();
        let handle = unsafe {
            ffi::wxd_DataViewBitmapRenderer_Create(
                variant_type_cstr.as_ptr(),
                mode.bits(),
                align.bits(),
            )
        };
        Self { handle }
    }
}

impl DataViewRenderer for DataViewBitmapRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.handle
    }
}

/// A date renderer for DataViewCtrl columns.
///
/// DataViewDateRenderer displays date values.
pub struct DataViewDateRenderer {
    handle: *mut ffi::wxd_DataViewRenderer_t,
}

impl DataViewDateRenderer {
    /// Creates a new date renderer.
    ///
    /// # Parameters
    ///
    /// * `variant_type` - The type of data this renderer can display (typically DateTime)
    /// * `mode` - The cell mode
    /// * `align` - The alignment
    pub fn new(variant_type: VariantType, mode: DataViewCellMode, align: DataViewAlign) -> Self {
        let variant_type_str = variant_type.to_type_string();
        let variant_type_cstr = CString::new(variant_type_str).unwrap();
        let handle = unsafe {
            ffi::wxd_DataViewDateRenderer_Create(
                variant_type_cstr.as_ptr(),
                mode.bits(),
                align.bits(),
            )
        };
        Self { handle }
    }
}

impl DataViewRenderer for DataViewDateRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.handle
    }
}

/// A spin renderer for DataViewCtrl columns.
///
/// DataViewSpinRenderer displays a spin control for numeric data.
pub struct DataViewSpinRenderer {
    handle: *mut ffi::wxd_DataViewRenderer_t,
}

impl DataViewSpinRenderer {
    /// Creates a new spin renderer.
    ///
    /// # Parameters
    ///
    /// * `variant_type` - The type of data this renderer can display (typically Int32)
    /// * `mode` - The cell mode
    /// * `align` - The alignment
    /// * `min` - Minimum value
    /// * `max` - Maximum value
    /// * `inc` - Increment value
    pub fn new(
        variant_type: VariantType,
        mode: DataViewCellMode,
        align: DataViewAlign,
        min: i32,
        max: i32,
        inc: i32,
    ) -> Self {
        let variant_type_str = variant_type.to_type_string();
        let variant_type_cstr = CString::new(variant_type_str).unwrap();
        let handle = unsafe {
            ffi::wxd_DataViewSpinRenderer_Create(
                variant_type_cstr.as_ptr(),
                mode.bits(),
                align.bits(),
                min,
                max,
                inc,
            )
        };
        Self { handle }
    }
}

impl DataViewRenderer for DataViewSpinRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.handle
    }
}

/// A choice renderer for DataViewCtrl columns.
///
/// DataViewChoiceRenderer displays a dropdown with choices.
pub struct DataViewChoiceRenderer {
    handle: *mut ffi::wxd_DataViewRenderer_t,
}

impl DataViewChoiceRenderer {
    /// Creates a new choice renderer.
    ///
    /// # Parameters
    ///
    /// * `variant_type` - The type of data this renderer can display (typically String)
    /// * `choices` - A list of choices to display in the dropdown
    /// * `mode` - The cell mode
    /// * `align` - The alignment
    pub fn new(
        variant_type: VariantType,
        choices: &[&str],
        mode: DataViewCellMode,
        align: DataViewAlign,
    ) -> Self {
        // Convert choices to a comma-separated string
        let choices_str = choices.join(",");
        let choices_cstr = CString::new(choices_str).unwrap();

        let variant_type_str = variant_type.to_type_string();
        let variant_type_cstr = CString::new(variant_type_str).unwrap();

        let handle = unsafe {
            ffi::wxd_DataViewChoiceRenderer_Create(
                variant_type_cstr.as_ptr(),
                choices_cstr.as_ptr(),
                mode.bits(),
                align.bits(),
            )
        };

        Self { handle }
    }
}

impl DataViewRenderer for DataViewChoiceRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.handle
    }
}

/// A check-icon-text renderer for DataViewCtrl columns.
///
/// DataViewCheckIconTextRenderer displays a checkbox, an icon, and text.
pub struct DataViewCheckIconTextRenderer {
    handle: *mut ffi::wxd_DataViewRenderer_t,
}

impl DataViewCheckIconTextRenderer {
    /// Creates a new check-icon-text renderer.
    ///
    /// # Parameters
    ///
    /// * `mode` - The cell mode
    /// * `align` - The alignment
    pub fn new(mode: DataViewCellMode, align: DataViewAlign) -> Self {
        // This renderer uses a special variant type
        let variant_type_cstr = CString::new("wxDataViewCheckIconText").unwrap();
        let handle = unsafe {
            ffi::wxd_DataViewCheckIconTextRenderer_Create(
                variant_type_cstr.as_ptr(),
                mode.bits(),
                align.bits(),
            )
        };
        Self { handle }
    }
}

impl DataViewRenderer for DataViewCheckIconTextRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.handle
    }
}
