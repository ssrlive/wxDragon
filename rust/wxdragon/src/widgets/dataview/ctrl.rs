//! DataViewCtrl implementation.

use crate::{Id, Point, Size, Window, WxWidget, WxEvtHandler};
// These macros are exported at the crate root
use crate::{widget_style_enum, widget_builder, implement_widget_traits_with_target};
use wxdragon_sys as ffi;

use super::{DataViewColumn, DataViewModel, DataViewTextRenderer, DataViewToggleRenderer, DataViewProgressRenderer};

// Define style enum for DataViewCtrl using the macro
widget_style_enum!(
    name: DataViewStyle,
    doc: "Style flags for DataViewCtrl widgets.",
    variants: {
        Single: ffi::WXD_DV_SINGLE, "Single-selection mode.",
        Multiple: ffi::WXD_DV_MULTIPLE, "Multiple-selection mode.",
        RowLines: ffi::WXD_DV_ROW_LINES, "Display row dividers.",
        HorizontalRules: ffi::WXD_DV_HORIZ_RULES, "Display horizontal rules.",
        VerticalRules: ffi::WXD_DV_VERT_RULES, "Display vertical rules.",
        VariableLineHeight: ffi::WXD_DV_VARIABLE_LINE_HEIGHT, "Enable variable line height.",
        NoHeader: ffi::WXD_DV_NO_HEADER, "Hide column headers."
    },
    default_variant: Single
);

/// Represents a wxWidgets DataViewCtrl in Rust.
///
/// DataViewCtrl is a control that displays data in a tabular or tree-like format,
/// with customizable renderers and a flexible data model.
///
/// # Features
///
/// - Displays data in a customizable grid or tree format
/// - Supports multiple column types (text, toggle, progress, etc.)
/// - Configurable selection modes (single or multiple)
/// - Row/column highlighting and styling options
///
/// # Example
///
/// ```
/// use wxdragon::{DataViewCtrl, DataViewStyle, Panel, Window, Id};
///
/// let panel = Panel::builder(&frame)
///     .build();
///
/// let data_view = DataViewCtrl::builder(&panel)
///     .with_id(Id::new(100))
///     .with_style(DataViewStyle::RowLines | DataViewStyle::VerticalRules)
///     .build();
/// ```
pub struct DataViewCtrl {
    window: Window,
}

impl DataViewCtrl {
    /// Creates a builder for configuring and constructing a DataViewCtrl.
    pub fn builder(parent: &dyn WxWidget) -> DataViewCtrlBuilder {
        DataViewCtrlBuilder::new(parent)
    }

    fn new_impl(parent_ptr: *mut ffi::wxd_Window_t, id: i32, pos: Point,
               size: Size, style: i64) -> Self {
        let handle = unsafe {
            ffi::wxd_DataViewCtrl_Create(
                parent_ptr,
                id as i64,
                &pos as *const Point as *const ffi::wxd_Point,
                &size as *const Size as *const ffi::wxd_Size,
                style,
            )
        };

        let window = unsafe { Window::from_ptr(handle) };
        Self { window }
    }

    /// Associates a data model with this DataViewCtrl.
    ///
    /// The model provides the data that will be displayed in the control.
    /// 
    /// # Returns
    /// 
    /// `true` if the model was successfully associated, `false` otherwise.
    pub fn associate_model<M: DataViewModel>(&self, model: &M) -> bool {
        let model_ptr = model.as_raw();
        unsafe { ffi::wxd_DataViewCtrl_AssociateModel(self.window.handle_ptr(), model_ptr) }
    }

    /// Appends a column to the control.
    /// 
    /// # Returns
    /// 
    /// `true` if the column was successfully appended, `false` otherwise.
    pub fn append_column(&self, column: &DataViewColumn) -> bool {
        unsafe { ffi::wxd_DataViewCtrl_AppendColumn(self.window.handle_ptr(), column.as_raw()) }
    }

    /// Prepends a column to the control.
    /// 
    /// # Returns
    /// 
    /// `true` if the column was successfully prepended, `false` otherwise.
    pub fn prepend_column(&self, column: &DataViewColumn) -> bool {
        unsafe { ffi::wxd_DataViewCtrl_PrependColumn(self.window.handle_ptr(), column.as_raw()) }
    }

    /// Inserts a column at the specified position.
    /// 
    /// # Returns
    /// 
    /// `true` if the column was successfully inserted, `false` otherwise.
    pub fn insert_column(&self, pos: usize, column: &DataViewColumn) -> bool {
        unsafe { ffi::wxd_DataViewCtrl_InsertColumn(self.window.handle_ptr(), pos as i64, column.as_raw()) }
    }

    /// Selects the specified row.
    /// 
    /// # Returns
    /// 
    /// `true` if the row was successfully selected, `false` otherwise.
    pub fn select_row(&self, row: usize) -> bool {
        unsafe { ffi::wxd_DataViewCtrl_SelectRow(self.window.handle_ptr(), row as i64) }
    }

    /// Gets the currently selected row.
    ///
    /// # Returns
    /// 
    /// An `Option` containing the index of the selected row, or `None` if no row is selected.
    pub fn get_selected_row(&self) -> Option<usize> {
        let row = unsafe { ffi::wxd_DataViewCtrl_GetSelectedRow(self.window.handle_ptr()) };
        if row >= 0 { Some(row as usize) } else { None }
    }

    /// Deselects all currently selected items.
    pub fn unselect_all(&self) {
        unsafe { ffi::wxd_DataViewCtrl_UnselectAll(self.window.handle_ptr()) }
    }
    
    /// Creates and appends a text column to this control.
    ///
    /// This is a convenience method for creating a text renderer column and appending it.
    ///
    /// # Parameters
    ///
    /// * `label` - The header label for the column
    /// * `model_column` - The column index in the data model
    /// * `width` - The column width (in pixels)
    /// * `align` - The text alignment (use constants from `ffi::WXD_ALIGN_*`)
    ///
    /// # Returns
    ///
    /// `true` if the column was successfully appended, `false` otherwise.
    pub fn append_text_column(&self, label: &str, model_column: usize, width: i32, align: i64) -> bool {
        let renderer = DataViewTextRenderer::new("string", ffi::WXD_DATAVIEW_CELL_INERT, align);
        let column = DataViewColumn::new(label, &renderer, model_column, width, align);
        self.append_column(&column)
    }
    
    /// Creates and appends a toggle (checkbox) column to this control.
    ///
    /// This is a convenience method for creating a toggle renderer column and appending it.
    ///
    /// # Parameters
    ///
    /// * `label` - The header label for the column
    /// * `model_column` - The column index in the data model
    /// * `width` - The column width (in pixels)
    /// * `align` - The alignment (use constants from `ffi::WXD_ALIGN_*`)
    ///
    /// # Returns
    ///
    /// `true` if the column was successfully appended, `false` otherwise.
    pub fn append_toggle_column(&self, label: &str, model_column: usize, width: i32, align: i64) -> bool {
        let renderer = DataViewToggleRenderer::new("bool", ffi::WXD_DATAVIEW_CELL_ACTIVATABLE, align);
        let column = DataViewColumn::new(label, &renderer, model_column, width, align);
        self.append_column(&column)
    }
    
    /// Creates and appends a progress bar column to this control.
    ///
    /// This is a convenience method for creating a progress renderer column and appending it.
    ///
    /// # Parameters
    ///
    /// * `label` - The header label for the column
    /// * `model_column` - The column index in the data model
    /// * `width` - The column width (in pixels)
    ///
    /// # Returns
    ///
    /// `true` if the column was successfully appended, `false` otherwise.
    pub fn append_progress_column(&self, label: &str, model_column: usize, width: i32) -> bool {
        let renderer = DataViewProgressRenderer::new("long", ffi::WXD_DATAVIEW_CELL_INERT);
        let column = DataViewColumn::new(label, &renderer, model_column, width, ffi::WXD_ALIGN_CENTER);
        self.append_column(&column)
    }
}

implement_widget_traits_with_target!(DataViewCtrl, window, Window);

widget_builder!(
    name: DataViewCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: DataViewStyle,
    fields: {},
    build_impl: |slf| {
        DataViewCtrl::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            slf.pos,
            slf.size,
            slf.style.bits(),
        )
    }
); 