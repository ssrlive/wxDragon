//! wxListCtrl wrapper

use crate::geometry::{Point, Size};
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::window::{Window, WxWidget};
use crate::widget_style_enum;
use crate::widget_builder;
use crate::implement_widget_traits_with_target;
use std::ffi::CString;
use std::os::raw::{c_int, c_long};
use wxdragon_sys as ffi;

// --- ListCtrl Styles ---
widget_style_enum!(
    name: ListCtrlStyle,
    doc: "Style flags for ListCtrl widget.",
    variants: {
        List: ffi::WXD_LC_LIST, "Display items in a list format, one item per row.",
        Report: ffi::WXD_LC_REPORT, "Display items in a multicolumn report view.",
        Icon: ffi::WXD_LC_ICON, "Display items as large icons.",
        SmallIcon: ffi::WXD_LC_SMALL_ICON, "Display items as small icons.",
        AlignTop: ffi::WXD_LC_ALIGN_TOP, "Align items to the top (icon view only).",
        AlignLeft: ffi::WXD_LC_ALIGN_LEFT, "Align items to the left (icon view only).",
        AutoArrange: ffi::WXD_LC_AUTOARRANGE, "Automatically arrange items.",
        EditLabels: ffi::WXD_LC_EDIT_LABELS, "Allow item labels to be edited.",
        NoHeader: ffi::WXD_LC_NO_HEADER, "Don't display column headers in report mode.",
        SingleSel: ffi::WXD_LC_SINGLE_SEL, "Allow only a single item to be selected.",
        SortAscending: ffi::WXD_LC_SORT_ASCENDING, "Sort items in ascending order.",
        SortDescending: ffi::WXD_LC_SORT_DESCENDING, "Sort items in descending order.",
        HRules: ffi::WXD_LC_HRULES, "Display horizontal rules between rows in report mode.",
        VRules: ffi::WXD_LC_VRULES, "Display vertical rules between columns in report mode."
    },
    default_variant: Report
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

// --- ListItemState Enum (for LIST_STATE_... constants) ---
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum ListItemState {
    /// Item is selected
    Selected = ffi::WXD_LIST_STATE_SELECTED,
    /// Item has focus
    Focused = ffi::WXD_LIST_STATE_FOCUSED,
    /// Item is disabled
    Disabled = ffi::WXD_LIST_STATE_DISABLED,
    /// Item is highlighted as a drop target
    DropHilited = ffi::WXD_LIST_STATE_DROPHILITED,
}

impl ListItemState {
    /// Returns the raw integer value of the state
    pub fn bits(self) -> i64 {
        self as i64
    }
}

// BitOr and BitOrAssign for ListItemState allow combining states
impl std::ops::BitOr for ListItemState {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl std::ops::BitOrAssign for ListItemState {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe { *self = std::mem::transmute(self.bits() | rhs.bits()); }
    }
}

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
        
        let ptr = unsafe {
            ffi::wxd_ListCtrl_Create(
                parent_ptr,
                id,
                pos.into(),
                size.into(),
                style,
            )
        };
        
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
    pub fn insert_column(&self, col: i64, heading: &str, format: ListColumnFormat, width: i32) -> i32 {
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
        unsafe { 
            ffi::wxd_ListCtrl_SetColumnWidth(
                self.as_list_ctrl_ptr(), 
                col as c_long, 
                width
            ) 
        }
    }

    /// Gets the width of the specified column.
    pub fn get_column_width(&self, col: i64) -> i32 {
        unsafe { 
            ffi::wxd_ListCtrl_GetColumnWidth(
                self.as_list_ctrl_ptr(), 
                col as c_long
            ) 
        }
    }

    /// Gets the number of columns in the list control.
    pub fn get_column_count(&self) -> i32 {
        unsafe { 
            ffi::wxd_ListCtrl_GetColumnCount(
                self.as_list_ctrl_ptr()
            ) 
        }
    }

    /// Inserts a simple item (label only) at the specified index.
    pub fn insert_item(&self, index: i64, label: &str) -> i32 {
        let c_label = CString::new(label).unwrap_or_default();
        unsafe { 
            ffi::wxd_ListCtrl_InsertItem_Simple(
                self.as_list_ctrl_ptr(), 
                index as c_long, 
                c_label.as_ptr()
            ) 
        }
    }

    /// Sets the text of an item (label in column 0).
    pub fn set_item_text(&self, index: i64, text: &str) {
        let c_text = CString::new(text).unwrap_or_default();
        unsafe { 
            ffi::wxd_ListCtrl_SetItemText(
                self.as_list_ctrl_ptr(), 
                index as c_long, 
                c_text.as_ptr()
            ) 
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
        unsafe { 
            ffi::wxd_ListCtrl_GetItemCount(
                self.as_list_ctrl_ptr()
            ) 
        }
    }

    /// Sets the state of an item.
    pub fn set_item_state(&self, item: i64, state: i64, state_mask: i64) -> bool {
        unsafe {
            ffi::wxd_ListCtrl_SetItemState(
                self.as_list_ctrl_ptr(),
                item as c_long,
                state as c_long,
                state_mask as c_long,
            )
        }
    }

    /// Gets the state of an item.
    pub fn get_item_state(&self, item: i64, state_mask: i64) -> i32 {
        unsafe { 
            ffi::wxd_ListCtrl_GetItemState(
                self.as_list_ctrl_ptr(), 
                item as c_long, 
                state_mask as c_long
            ) 
        }
    }

    /// Gets the next item based on geometry and state.
    pub fn get_next_item(&self, item: i64, geometry: ListNextItemFlag, state: ListItemState) -> i32 {
        unsafe { 
            ffi::wxd_ListCtrl_GetNextItem(
                self.as_list_ctrl_ptr(), 
                item as c_long, 
                geometry as c_int, 
                state as c_int
            ) 
        }
    }

    /// Gets the first selected item in the list control.
    pub fn get_first_selected_item(&self) -> i32 {
        self.get_next_item(-1, ListNextItemFlag::All, ListItemState::Selected)
    }

    /// Deletes the specified item.
    pub fn delete_item(&self, item: i64) -> bool {
        unsafe { 
            ffi::wxd_ListCtrl_DeleteItem(
                self.as_list_ctrl_ptr(), 
                item as c_long
            ) 
        }
    }

    /// Deletes all items from the list control.
    pub fn delete_all_items(&self) -> bool {
        unsafe { 
            ffi::wxd_ListCtrl_DeleteAllItems(
                self.as_list_ctrl_ptr()
            ) 
        }
    }

    /// Deletes all items and columns from the list control.
    pub fn clear_all(&self) -> bool {
        unsafe { 
            ffi::wxd_ListCtrl_ClearAll(
                self.as_list_ctrl_ptr()
            ) 
        }
    }

    /// Gets the number of selected items.
    pub fn get_selected_item_count(&self) -> i32 {
        unsafe { 
            ffi::wxd_ListCtrl_GetSelectedItemCount(
                self.as_list_ctrl_ptr()
            ) 
        }
    }

    /// Ensures that the specified item is visible.
    pub fn ensure_visible(&self, item: i64) -> bool {
        unsafe { 
            ffi::wxd_ListCtrl_EnsureVisible(
                self.as_list_ctrl_ptr(), 
                item as c_long
            ) 
        }
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
    pub fn edit_label(&self, item: i64) {
        unsafe { 
            ffi::wxd_ListCtrl_EditLabel(
                self.as_list_ctrl_ptr(), 
                item as c_long
            ) 
        }
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
