//! wxListCtrl wrapper

use crate::event::WxEvtHandler;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use std::os::raw::{c_int, c_long, c_void};
use wxdragon_sys as ffi;

// --- ListCtrl Styles ---
widget_style_enum!(
    name: ListCtrlStyle,
    doc: "Style flags for ListCtrl widget.",
    variants: {
        Default: 0, "Default list control style.",
        SingleSel: ffi::WXD_LC_SINGLE_SEL, "Single selection (default is multiple).",
        SortAscending: ffi::WXD_LC_SORT_ASCENDING, "Sort in ascending order.",
        SortDescending: ffi::WXD_LC_SORT_DESCENDING, "Sort in descending order.",
        Virtual: ffi::WXD_LC_VIRTUAL, "The application provides items text on demand.",
        EditLabels: ffi::WXD_LC_EDIT_LABELS, "Labels can be edited for in-place renaming.",
        
        // View styles
        Icon: ffi::WXD_LC_ICON, "Large icon view.",
        SmallIcon: ffi::WXD_LC_SMALL_ICON, "Small icon view.",
        List: ffi::WXD_LC_LIST, "List view showing items on a single line.",
        Report: ffi::WXD_LC_REPORT, "Multicolumn report view (detail view).",
        
        // Alignment styles
        AlignTop: ffi::WXD_LC_ALIGN_TOP, "Align icons with the top (default).",
        AlignLeft: ffi::WXD_LC_ALIGN_LEFT, "Align icons with the left.",
        
        // Behavior styles
        AutoArrange: ffi::WXD_LC_AUTOARRANGE, "Icons arrange themselves.",
        HRules: ffi::WXD_LC_HRULES, "Horizontal rules in report mode.",
        VRules: ffi::WXD_LC_VRULES, "Vertical rules in report mode.",
        NoHeader: ffi::WXD_LC_NO_HEADER, "No header in report mode.",
        NoSort: ffi::WXD_LC_NO_SORT_HEADER, "No sorting when clicking on headers."
    },
    default_variant: Default
);

// --- ListColumnFormat Enum (for LIST_FORMAT_... constants) ---
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum ListColumnFormat {
    /// Align column content to the left
    Left = ffi::WXD_LIST_FORMAT_LEFT as i32,
    /// Align column content to the right
    Right = ffi::WXD_LIST_FORMAT_RIGHT as i32,
    /// Align column content to the center
    Centre = ffi::WXD_LIST_FORMAT_CENTRE as i32,
}

impl ListColumnFormat {
    /// Returns the raw integer value of the format
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}

impl Default for ListColumnFormat {
    fn default() -> Self {
        ListColumnFormat::Left
    }
}

// --- ListItemState (for LIST_STATE_... constants) ---
widget_style_enum!(
    name: ListItemState,
    doc: "Item state flags for ListCtrl items.",
    variants: {
        None: 0, "No state (used for clearing states).",
        Selected: ffi::WXD_LIST_STATE_SELECTED, "Item is selected.",
        Focused: ffi::WXD_LIST_STATE_FOCUSED, "Item has focus.", 
        Disabled: ffi::WXD_LIST_STATE_DISABLED, "Item is disabled.",
        DropHilited: ffi::WXD_LIST_STATE_DROPHILITED, "Item is highlighted as a drop target."
    },
    default_variant: None
);

// --- ListNextItemFlag Enum (for LIST_NEXT_... constants) ---
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum ListNextItemFlag {
    /// All items, no geometric restriction
    All = ffi::WXD_LIST_NEXT_ALL as i32,
    /// Item above current one
    Above = ffi::WXD_LIST_NEXT_ABOVE as i32,
    /// Item below current one
    Below = ffi::WXD_LIST_NEXT_BELOW as i32,
    /// Item to the left of current one
    Left = ffi::WXD_LIST_NEXT_LEFT as i32,
    /// Item to the right of current one
    Right = ffi::WXD_LIST_NEXT_RIGHT as i32,
}

impl ListNextItemFlag {
    /// Returns the raw integer value of the flag
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}

impl Default for ListNextItemFlag {
    fn default() -> Self {
        ListNextItemFlag::All
    }
}

// --- ImageList Type Constants ---
/// Constants for image list types
pub mod image_list_type {
    /// Normal sized images (typically for Icon view)
    pub const NORMAL: i32 = 0;
    /// Small sized images (typically for Report/List view)
    pub const SMALL: i32 = 1;
    /// State images (for checkboxes)
    pub const STATE: i32 = 2;
}

/// A control for displaying and manipulating multiple items
///
/// The ListCtrl can display items in various formats including:
/// - List view (one column)
/// - Report view (multiple columns with headers)
/// - Icon view (large or small icons)
#[derive(Clone)]
pub struct ListCtrl {
    window: Window,
}

impl ListCtrl {
    /// Creates a new ListCtrl builder.
    pub fn builder(parent: &dyn WxWidget) -> ListCtrlBuilder {
        ListCtrlBuilder::new(parent)
    }

    /// Internal implementation used by the builder.
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        assert!(!parent_ptr.is_null(), "ListCtrl requires a parent");

        let ptr =
            unsafe { ffi::wxd_ListCtrl_Create(parent_ptr, id, pos.into(), size.into(), style) };

        if ptr.is_null() {
            panic!("Failed to create ListCtrl: FFI returned null pointer.");
        }

        unsafe { Self::from_ptr(ptr) }
    }

    /// Creates a ListCtrl from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be a valid wxd_ListCtrl_t pointer.
    unsafe fn from_ptr(ptr: *mut ffi::wxd_ListCtrl_t) -> Self {
        ListCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Returns the ListCtrl pointer from the window pointer.
    #[inline]
    fn as_list_ctrl_ptr(&self) -> *mut ffi::wxd_ListCtrl_t {
        self.handle_ptr() as *mut ffi::wxd_ListCtrl_t
    }

    /// Inserts a column at the specified position.
    pub fn insert_column(
        &self,
        col: i64,
        heading: &str,
        format: ListColumnFormat,
        width: i32,
    ) -> i32 {
        let c_heading = CString::new(heading).unwrap_or_default();
        unsafe {
            ffi::wxd_ListCtrl_InsertColumn(
                self.as_list_ctrl_ptr(),
                col as c_long,
                c_heading.as_ptr(),
                format as c_int,
                width,
            )
        }
    }

    /// Sets the width of the specified column.
    pub fn set_column_width(&self, col: i64, width: i32) -> bool {
        unsafe { ffi::wxd_ListCtrl_SetColumnWidth(self.as_list_ctrl_ptr(), col as c_long, width) }
    }

    /// Gets the width of the specified column.
    pub fn get_column_width(&self, col: i64) -> i32 {
        unsafe { ffi::wxd_ListCtrl_GetColumnWidth(self.as_list_ctrl_ptr(), col as c_long) }
    }

    /// Gets the number of columns in the list control.
    pub fn get_column_count(&self) -> i32 {
        unsafe { ffi::wxd_ListCtrl_GetColumnCount(self.as_list_ctrl_ptr()) }
    }

    /// Inserts a simple item (label only) at the specified index.
    pub fn insert_item(&self, index: i64, label: &str) -> i32 {
        let c_label = CString::new(label).unwrap_or_default();
        unsafe {
            ffi::wxd_ListCtrl_InsertItem_Simple(
                self.as_list_ctrl_ptr(),
                index as c_long,
                c_label.as_ptr(),
            )
        }
    }

    /// Sets the text of an item (label in column 0).
    pub fn set_item_text(&self, index: i64, text: &str) {
        let c_text = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_ListCtrl_SetItemText(self.as_list_ctrl_ptr(), index as c_long, c_text.as_ptr())
        }
    }

    /// Gets the text of an item in the specified column.
    pub fn get_item_text(&self, index: i64, col: i32) -> String {
        unsafe {
            let needed_len = ffi::wxd_ListCtrl_GetItemText(
                self.as_list_ctrl_ptr(),
                index as c_long,
                col,
                std::ptr::null_mut(),
                0,
            );
            if needed_len <= 0 {
                return String::new();
            }
            let mut buffer: Vec<u8> = Vec::with_capacity(needed_len as usize);
            let actual_len = ffi::wxd_ListCtrl_GetItemText(
                self.as_list_ctrl_ptr(),
                index as c_long,
                col,
                buffer.as_mut_ptr() as *mut i8,
                needed_len as i32,
            );
            if actual_len <= 0 {
                return String::new();
            }
            buffer.set_len(actual_len as usize);
            String::from_utf8_lossy(&buffer).into_owned()
        }
    }

    /// Gets the number of items in the list control.
    pub fn get_item_count(&self) -> i32 {
        unsafe { ffi::wxd_ListCtrl_GetItemCount(self.as_list_ctrl_ptr()) }
    }

    /// Sets the state of an item using the ListItemState enum.
    /// 
    /// # Arguments
    /// * `item` - The index of the item.
    /// * `state` - The state flags to set or clear.
    /// * `state_mask` - The state flags to modify (only bits set in mask will be changed).
    /// 
    /// # Example
    /// ```
    /// // To select an item:
    /// list_ctrl.set_item_state(0, ListItemState::Selected, ListItemState::Selected);
    /// // To deselect an item:
    /// list_ctrl.set_item_state(0, ListItemState::default(), ListItemState::Selected);
    /// ```
    pub fn set_item_state(&self, item: i64, state: ListItemState, state_mask: ListItemState) -> bool {
        unsafe {
            ffi::wxd_ListCtrl_SetItemState(
                self.as_list_ctrl_ptr(),
                item as c_long,
                state.bits() as c_long,
                state_mask.bits() as c_long,
            )
        }
    }

    /// Gets the state of an item using the ListItemState enum.
    /// 
    /// # Arguments
    /// * `item` - The index of the item.
    /// * `state_mask` - The specific state flag to check.
    /// 
    /// # Returns
    /// Returns true if the state specified by state_mask is set, false otherwise.
    /// 
    /// # Example
    /// ```
    /// // Check if an item is selected:
    /// let is_selected = list_ctrl.get_item_state(0, ListItemState::Selected);
    /// ```
    pub fn get_item_state(&self, item: i64, state_mask: ListItemState) -> bool {
        let state = unsafe {
            ffi::wxd_ListCtrl_GetItemState(
                self.as_list_ctrl_ptr(),
                item as c_long,
                state_mask.bits() as c_long,
            )
        };
        state != 0
    }

    /// Gets the next item based on geometry and state.
    pub fn get_next_item(
        &self,
        item: i64,
        geometry: ListNextItemFlag,
        state: ListItemState,
    ) -> i32 {
        unsafe {
            ffi::wxd_ListCtrl_GetNextItem(
                self.as_list_ctrl_ptr(),
                item as c_long,
                geometry as c_int,
                state.bits() as c_int,
            )
        }
    }

    /// Gets the first selected item in the list control.
    pub fn get_first_selected_item(&self) -> i32 {
        self.get_next_item(-1, ListNextItemFlag::All, ListItemState::Selected)
    }

    /// Deletes the specified item.
    pub fn delete_item(&self, item: i64) -> bool {
        unsafe { ffi::wxd_ListCtrl_DeleteItem(self.as_list_ctrl_ptr(), item as c_long) }
    }

    /// Deletes all items from the list control.
    pub fn delete_all_items(&self) -> bool {
        unsafe { ffi::wxd_ListCtrl_DeleteAllItems(self.as_list_ctrl_ptr()) }
    }

    /// Deletes all items and columns from the list control.
    pub fn clear_all(&self) -> bool {
        unsafe { ffi::wxd_ListCtrl_ClearAll(self.as_list_ctrl_ptr()) }
    }

    /// Gets the number of selected items.
    pub fn get_selected_item_count(&self) -> i32 {
        unsafe { ffi::wxd_ListCtrl_GetSelectedItemCount(self.as_list_ctrl_ptr()) }
    }

    /// Ensures that the specified item is visible.
    pub fn ensure_visible(&self, item: i64) -> bool {
        unsafe { ffi::wxd_ListCtrl_EnsureVisible(self.as_list_ctrl_ptr(), item as c_long) }
    }

    /// Determines which item, if any, is at the specified point.
    /// Returns a tuple (item_index, flags, subitem_index).
    pub fn hit_test(&self, point: Point) -> (i32, i32, i32) {
        let mut flags: i32 = 0;
        let mut subitem: c_long = 0;
        let item = unsafe {
            ffi::wxd_ListCtrl_HitTest(
                self.as_list_ctrl_ptr(),
                point.into(),
                &mut flags as *mut i32,
                &mut subitem as *mut c_long,
            )
        };
        (item, flags, subitem as i32)
    }

    /// Starts editing the label of the specified item.
    /// 
    /// # Returns
    /// Returns the TextCtrl that will be used to edit the label.
    /// The caller does not own this TextCtrl; it will be deleted automatically
    /// when editing is finished.
    pub fn edit_label(&self, item: i64) -> crate::widgets::textctrl::TextCtrl {
        let ptr = unsafe { ffi::wxd_ListCtrl_EditLabel(self.as_list_ctrl_ptr(), item as c_long) };
        
        if ptr.is_null() {
            panic!("Failed to start editing item label: FFI returned null pointer.");
        }
        
        unsafe { crate::widgets::textctrl::TextCtrl::from_ptr(ptr as *mut ffi::wxd_TextCtrl_t) }
    }

    // --- Item Data Methods ---

    /// Associates integer data with an item.
    pub fn set_item_data(&self, item: i64, data: i64) -> bool {
        unsafe { ffi::wxd_ListCtrl_SetItemData(self.as_list_ctrl_ptr(), item as c_long, data as c_long) }
    }

    /// Retrieves integer data associated with an item.
    pub fn get_item_data(&self, item: i64) -> i64 {
        unsafe { ffi::wxd_ListCtrl_GetItemData(self.as_list_ctrl_ptr(), item as c_long) as i64 }
    }

    // --- Item Appearance Methods ---

    /// Sets the background color of an item.
    pub fn set_item_background_colour(&self, item: i64, colour: &crate::color::Colour) {
        unsafe { 
            ffi::wxd_ListCtrl_SetItemBackgroundColour(
                self.as_list_ctrl_ptr(), 
                item as c_long, 
                (*colour).into()
            ) 
        }
    }

    /// Sets the text color of an item.
    pub fn set_item_text_colour(&self, item: i64, colour: &crate::color::Colour) {
        unsafe { 
            ffi::wxd_ListCtrl_SetItemTextColour(
                self.as_list_ctrl_ptr(), 
                item as c_long, 
                (*colour).into()
            ) 
        }
    }

    /// Gets the background color of an item.
    pub fn get_item_background_colour(&self, item: i64) -> crate::color::Colour {
        unsafe { 
            let c_colour = ffi::wxd_ListCtrl_GetItemBackgroundColour(self.as_list_ctrl_ptr(), item as c_long);
            crate::color::Colour::from(c_colour)
        }
    }

    /// Gets the text color of an item.
    pub fn get_item_text_colour(&self, item: i64) -> crate::color::Colour {
        unsafe { 
            let c_colour = ffi::wxd_ListCtrl_GetItemTextColour(self.as_list_ctrl_ptr(), item as c_long);
            crate::color::Colour::from(c_colour)
        }
    }

    // --- Column Management Methods ---

    /// Sets the custom order of columns.
    /// 
    /// By default, the columns in a list control appear in order of their indices (0, 1, 2, ...).
    /// This method allows you to set a custom visual order for the columns.
    pub fn set_columns_order(&self, orders: &[i32]) -> bool {
        unsafe {
            ffi::wxd_ListCtrl_SetColumnsOrder(
                self.as_list_ctrl_ptr(),
                orders.len() as c_int,
                orders.as_ptr() as *mut c_int
            )
        }
    }

    /// Gets the custom order of all columns.
    /// 
    /// Returns a vector of column indices in their current display order.
    pub fn get_columns_order(&self) -> Vec<i32> {
        unsafe {
            let mut count: c_int = 0;
            let ptr = ffi::wxd_ListCtrl_GetColumnsOrder(self.as_list_ctrl_ptr(), &mut count as *mut c_int);
            
            if ptr.is_null() || count <= 0 {
                return Vec::new();
            }
            
            let mut result = Vec::with_capacity(count as usize);
            for i in 0..count {
                result.push(*ptr.offset(i as isize));
            }
            
            // Free the memory allocated by the C function
            libc::free(ptr as *mut c_void);
            
            result
        }
    }

    /// Gets the position in which the given column is currently displayed.
    /// 
    /// Returns the position where the column is currently shown, or -1 if an error occurred.
    pub fn get_column_order(&self, col: i32) -> i32 {
        unsafe { ffi::wxd_ListCtrl_GetColumnOrder(self.as_list_ctrl_ptr(), col) }
    }

    /// Gets the column index at the given display position.
    /// 
    /// Returns the index of the column which is shown at the specified position, or -1 if an error occurred.
    pub fn get_column_index_from_order(&self, pos: i32) -> i32 {
        unsafe { ffi::wxd_ListCtrl_GetColumnIndexFromOrder(self.as_list_ctrl_ptr(), pos) }
    }

    // --- Virtual List Support Methods ---

    /// Sets the number of items in a virtual list control.
    /// 
    /// Must be used with a list control created with the `ListCtrlStyle::Virtual` style.
    pub fn set_item_count(&self, count: i64) {
        unsafe { ffi::wxd_ListCtrl_SetItemCount(self.as_list_ctrl_ptr(), count as c_long) }
    }

    /// Refreshes a single item in a virtual list control.
    pub fn refresh_item(&self, item: i64) {
        unsafe { ffi::wxd_ListCtrl_RefreshItem(self.as_list_ctrl_ptr(), item as c_long) }
    }

    /// Refreshes a range of items in a virtual list control.
    pub fn refresh_items(&self, item_from: i64, item_to: i64) {
        unsafe { ffi::wxd_ListCtrl_RefreshItems(self.as_list_ctrl_ptr(), item_from as c_long, item_to as c_long) }
    }
}

// Implement common widget traits
implement_widget_traits_with_target!(ListCtrl, window, Window);

// Use the widget_builder macro for ListCtrl
widget_builder!(
    name: ListCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: ListCtrlStyle,
    fields: {},
    build_impl: |slf| {
        ListCtrl::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            slf.pos,
            slf.size,
            slf.style.bits()
        )
    }
);
