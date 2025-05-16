//! DataViewRenderer implementation.

use std::ffi::CString;
use crate::Rect;
use wxdragon_sys as ffi;

use super::VariantType;

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
    /// * `variant_type` - The type of data this renderer can display (usually "string")
    /// * `mode` - The cell mode (inert, activatable, or editable)
    /// * `align` - The text alignment
    pub fn new(variant_type: &str, mode: i64, align: i64) -> Self {
        let variant_type_cstr = CString::new(variant_type).unwrap();
        let handle = unsafe {
            ffi::wxd_DataViewTextRenderer_Create(
                variant_type_cstr.as_ptr(),
                mode,
                align,
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
    /// * `variant_type` - The type of data this renderer can display (usually "bool")
    /// * `mode` - The cell mode (typically activatable for toggles)
    /// * `align` - The alignment of the checkbox
    pub fn new(variant_type: &str, mode: i64, align: i64) -> Self {
        let variant_type_cstr = CString::new(variant_type).unwrap();
        let handle = unsafe {
            ffi::wxd_DataViewToggleRenderer_Create(
                variant_type_cstr.as_ptr(),
                mode,
                align,
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
    /// * `variant_type` - The type of data this renderer can display (usually "long")
    /// * `mode` - The cell mode (typically inert for progress bars)
    pub fn new(variant_type: &str, mode: i64) -> Self {
        let variant_type_cstr = CString::new(variant_type).unwrap();
        let handle = unsafe {
            ffi::wxd_DataViewProgressRenderer_Create(
                variant_type_cstr.as_ptr(),
                mode,
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
    /// * `variant_type` - The type of data this renderer can display
    /// * `mode` - The cell mode
    /// * `align` - The text alignment
    pub fn new(variant_type: &str, mode: i64, align: i64) -> Self {
        let variant_type_cstr = CString::new(variant_type).unwrap();
        let handle = unsafe {
            ffi::wxd_DataViewIconTextRenderer_Create(
                variant_type_cstr.as_ptr(),
                mode,
                align,
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

/// Trait for implementing custom rendering logic.
///
/// This trait can be implemented to provide custom rendering for DataViewCtrl cells.
pub trait CustomRendererLogic {
    /// Render the content of a cell.
    ///
    /// # Parameters
    ///
    /// * `rect` - Rectangle defining the cell boundaries
    /// * `row` - Row index
    /// * `column` - Column index
    fn render(&self, rect: &Rect, row: usize, column: usize) -> bool;
}

/// A custom renderer that allows for application-defined rendering of cells.
///
/// This renderer provides the ability to implement custom visualizations of data
/// by implementing the `CustomRendererLogic` trait.
///
/// # Example
///
/// ```rust,no_run
/// use wxdragon::prelude::*;
/// use wxdragon::widgets::dataview::{DataViewCustomRenderer, DataViewColumn, CustomRendererLogic, VariantType};
///
/// // Create a custom renderer implementation
/// struct ProgressArcRenderer;
///
/// impl CustomRendererLogic for ProgressArcRenderer {
///     fn render(&self, rect: &Rect, row: usize, _column: usize) -> bool {
///         // Sample implementation - display progress as text
///         println!("Rendering row {} in rect ({}, {}, {}, {})",
///                  row, rect.x, rect.y, rect.width, rect.height);
///         true
///     }
/// }
///
/// // Create the renderer and add it to a column
/// let renderer = DataViewCustomRenderer::new(VariantType::Double, Box::new(ProgressArcRenderer));
/// let column = DataViewColumn::new("Custom", &renderer, 0, 100, wxdragon_sys::WXD_ALIGN_CENTER);
/// dataview_ctrl.append_column(&column);
/// ```
pub struct DataViewCustomRenderer {
    handle: *mut ffi::wxd_DataViewRenderer_t,
    _callback_box: Box<RendererCallback>,
}

// Internal struct to store renderer data
struct RendererCallback {
    logic: Box<dyn CustomRendererLogic>,
}

impl DataViewCustomRenderer {
    /// Creates a new custom renderer with the specified rendering logic.
    ///
    /// # Parameters
    ///
    /// * `variant_type` - The type of data this renderer can display
    /// * `renderer` - Custom rendering logic implementation
    pub fn new(variant_type: VariantType, renderer: Box<dyn CustomRendererLogic>) -> Self {
        Self::with_mode(variant_type, ffi::WXD_DATAVIEW_CELL_INERT, ffi::WXD_ALIGN_CENTER, renderer)
    }
    
    /// Creates a new custom renderer with the specified mode and alignment.
    ///
    /// # Parameters
    ///
    /// * `variant_type` - The type of data this renderer can display
    /// * `mode` - The cell mode (inert, activatable, or editable)
    /// * `align` - The content alignment
    /// * `renderer` - Custom rendering logic implementation
    pub fn with_mode(
        variant_type: VariantType,
        mode: i64,
        align: i64,
        renderer: Box<dyn CustomRendererLogic>
    ) -> Self {
        // Create callback wrapper
        let callback = Box::new(RendererCallback {
            logic: renderer,
        });
        
        // Get a raw pointer to the callback data
        let callback_ptr = Box::into_raw(callback);
        
        // Convert the variant type to a string
        let type_str = variant_type.to_type_string();
        
        // Create a C string for the variant type
        let variant_type_cstr = CString::new(type_str).unwrap();
        
        // Create the renderer
        let handle = unsafe {
            ffi::wxd_DataViewCustomRenderer_Create(
                variant_type_cstr.as_ptr(),
                mode,
                align,
                Some(custom_render_trampoline),
                callback_ptr as *mut std::os::raw::c_void,
            )
        };
        
        // Recreate the box to maintain ownership
        let callback_box = unsafe { Box::from_raw(callback_ptr) };
        
        Self {
            handle,
            _callback_box: callback_box,
        }
    }
}

// Trampoline function that forwards the call to the Rust implementation
unsafe extern "C" fn custom_render_trampoline(
    user_data: *mut std::os::raw::c_void,
    _dc_ptr: *mut ffi::wxd_DC_t,
    rect_ptr: *mut ffi::wxd_Rect,
    _flags: i64,
    item: i64,
) -> bool {
    if user_data.is_null() || rect_ptr.is_null() {
        return false;
    }
    
    // Extract the struct from the pointer
    let callback = &*(user_data as *const RendererCallback);
    
    // Create a safe rectangle for the callback using the existing Rect struct
    let rect = Rect {
        x: (*rect_ptr).x,
        y: (*rect_ptr).y,
        width: (*rect_ptr).width,
        height: (*rect_ptr).height,
    };
    
    // Calculate the row and column (simplistic approach)
    let row = (item / 10) as usize;
    let column = (item % 10) as usize;
    
    // Call the user's implementation
    callback.logic.render(&rect, row, column)
}

impl DataViewRenderer for DataViewCustomRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.handle
    }
}

impl Drop for DataViewCustomRenderer {
    fn drop(&mut self) {
        // The renderer is owned by the column, so we don't need to free it here
    }
} 