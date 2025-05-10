use crate::base::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE, ID_ANY};
use crate::event::{Event, EventType, WxEvtHandler};
use crate::id::Id;
use crate::window::{Window, WxWidget};
use std::default::Default;
use std::ffi::CString;
use std::marker::PhantomData;
use std::os::raw::{c_int, c_long};
use wxdragon_sys as ffi;
use std::ops::{BitOr, BitOrAssign};

// --- ListCtrlStyle Enum (for LC_... constants) ---
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum ListCtrlStyle {
    List = ffi::WXD_LC_LIST,
    Report = ffi::WXD_LC_REPORT,
    Icon = ffi::WXD_LC_ICON,
    SmallIcon = ffi::WXD_LC_SMALL_ICON,
    AlignTop = ffi::WXD_LC_ALIGN_TOP,
    AlignLeft = ffi::WXD_LC_ALIGN_LEFT,
    AutoArrange = ffi::WXD_LC_AUTOARRANGE,
    EditLabels = ffi::WXD_LC_EDIT_LABELS,
    NoHeader = ffi::WXD_LC_NO_HEADER,
    SingleSel = ffi::WXD_LC_SINGLE_SEL,
    SortAscending = ffi::WXD_LC_SORT_ASCENDING,
    SortDescending = ffi::WXD_LC_SORT_DESCENDING,
    HRules = ffi::WXD_LC_HRULES,
    VRules = ffi::WXD_LC_VRULES,
}

impl ListCtrlStyle {
    pub fn bits(self) -> i64 {
        self as i64
    }
}

impl Default for ListCtrlStyle {
    fn default() -> Self {
        ListCtrlStyle::Report
    }
}

impl BitOr for ListCtrlStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for ListCtrlStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe { *self = std::mem::transmute(self.bits() | rhs.bits()); }
    }
}

// --- ListColumnFormat Enum (for LIST_FORMAT_... constants) ---
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum ListColumnFormat {
    Left = ffi::WXD_LIST_FORMAT_LEFT as i32,
    Right = ffi::WXD_LIST_FORMAT_RIGHT as i32,
    Centre = ffi::WXD_LIST_FORMAT_CENTRE as i32,
}

impl ListColumnFormat {
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
    Selected = ffi::WXD_LIST_STATE_SELECTED,
    Focused = ffi::WXD_LIST_STATE_FOCUSED,
    Disabled = ffi::WXD_LIST_STATE_DISABLED,
    DropHilited = ffi::WXD_LIST_STATE_DROPHILITED,
}

impl ListItemState {
    pub fn bits(self) -> i64 {
        self as i64
    }
}

// BitOr and BitOrAssign for ListItemState allow combining states
impl BitOr for ListItemState {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for ListItemState {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe { *self = std::mem::transmute(self.bits() | rhs.bits()); }
    }
}

// --- ListNextItemFlag Enum (for LIST_NEXT_... constants) ---
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum ListNextItemFlag {
    All = ffi::WXD_LIST_NEXT_ALL as i32,
    Above = ffi::WXD_LIST_NEXT_ABOVE as i32,
    Below = ffi::WXD_LIST_NEXT_BELOW as i32,
    Left = ffi::WXD_LIST_NEXT_LEFT as i32,
    Right = ffi::WXD_LIST_NEXT_RIGHT as i32,
}

impl ListNextItemFlag {
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}

impl Default for ListNextItemFlag {
    fn default() -> Self {
        ListNextItemFlag::All
    }
}

#[derive(Clone)]
pub struct ListCtrl {
    ptr: *mut ffi::wxd_ListCtrl_t,
    window: Window,
}

impl ListCtrl {
    pub fn builder(parent: &impl WxWidget) -> ListCtrlBuilder<Window> {
        let mut builder = ListCtrlBuilder::<Window>::default();
        builder.parent_ptr = parent.handle_ptr();
        builder
    }

    /// Inserts a column at the specified position.
    pub fn insert_column(&self, col: i64, heading: &str, format: ListColumnFormat, width: i32) -> i32 {
        let c_heading = CString::new(heading).unwrap();
        unsafe {
            ffi::wxd_ListCtrl_InsertColumn(
                self.ptr,
                col as c_long,
                c_heading.as_ptr(),
                format as c_int,
                width,
            )
        }
    }

    /// Sets the width of the specified column.
    pub fn set_column_width(&self, col: i64, width: i32) -> bool {
        unsafe { ffi::wxd_ListCtrl_SetColumnWidth(self.ptr, col as c_long, width) }
    }

    /// Gets the width of the specified column.
    pub fn get_column_width(&self, col: i64) -> i32 {
        unsafe { ffi::wxd_ListCtrl_GetColumnWidth(self.ptr, col as c_long) }
    }

    pub fn get_column_count(&self) -> i32 {
        unsafe { ffi::wxd_ListCtrl_GetColumnCount(self.ptr) }
    }

    /// Inserts a simple item (label only) at the specified index.
    pub fn insert_item(&self, index: i64, label: &str) -> i32 {
        let c_label = CString::new(label).unwrap();
        unsafe { ffi::wxd_ListCtrl_InsertItem_Simple(self.ptr, index as c_long, c_label.as_ptr()) }
    }

    /// Sets the text of an item (label in column 0).
    pub fn set_item_text(&self, index: i64, text: &str) {
        let c_text = CString::new(text).unwrap_or_default();
        unsafe { ffi::wxd_ListCtrl_SetItemText(self.ptr, index as c_long, c_text.as_ptr()) }
    }

    pub fn get_item_text(&self, index: i64, col: i32) -> String {
        unsafe {
            let needed_len = ffi::wxd_ListCtrl_GetItemText(
                self.ptr,
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
                self.ptr,
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

    pub fn get_item_count(&self) -> i32 {
        unsafe { ffi::wxd_ListCtrl_GetItemCount(self.ptr) }
    }

    /// Sets the state of an item.
    pub fn set_item_state(&self, item: i64, state: i64, state_mask: i64) -> bool {
        unsafe {
            ffi::wxd_ListCtrl_SetItemState(
                self.ptr,
                item as c_long,
                state as c_long,
                state_mask as c_long,
            )
        }
    }

    /// Gets the state of an item.
    pub fn get_item_state(&self, item: i64, state_mask: i64) -> i32 {
        unsafe { ffi::wxd_ListCtrl_GetItemState(self.ptr, item as c_long, state_mask as c_long) }
    }

    /// Gets the next item based on geometry and state.
    pub fn get_next_item(&self, item: i64, geometry: ListNextItemFlag, state: ListItemState) -> i32 {
        unsafe { ffi::wxd_ListCtrl_GetNextItem(self.ptr, item as c_long, geometry as c_int, state as c_int) }
    }

    pub fn get_first_selected_item(&self) -> i32 {
        self.get_next_item(-1, ListNextItemFlag::All, ListItemState::Selected)
    }

    /// Deletes the specified item.
    pub fn delete_item(&self, item: i64) -> bool {
        unsafe { ffi::wxd_ListCtrl_DeleteItem(self.ptr, item as c_long) }
    }

    pub fn delete_all_items(&self) -> bool {
        unsafe { ffi::wxd_ListCtrl_DeleteAllItems(self.ptr) }
    }

    pub fn clear_all(&self) -> bool {
        unsafe { ffi::wxd_ListCtrl_ClearAll(self.ptr) }
    }

    pub fn get_selected_item_count(&self) -> i32 {
        unsafe { ffi::wxd_ListCtrl_GetSelectedItemCount(self.ptr) }
    }

    /// Ensures that the specified item is visible.
    pub fn ensure_visible(&self, item: i64) -> bool {
        unsafe { ffi::wxd_ListCtrl_EnsureVisible(self.ptr, item as c_long) }
    }

    /// Determines which item, if any, is at the specified point.
    /// Returns a tuple (item_index, flags, subitem_index).
    pub fn hit_test(&self, point: Point) -> (i32, i32, i32) {
        let mut flags: i32 = 0;
        let mut subitem: c_long = 0;
        let item = unsafe {
            ffi::wxd_ListCtrl_HitTest(
                self.ptr,
                point.into(),
                &mut flags as *mut i32,
                &mut subitem as *mut c_long,
            )
        };
        (item, flags, subitem as i32)
    }

    /// Starts editing the label of the specified item.
    pub fn edit_label(&self, item: i64) {
        unsafe { ffi::wxd_ListCtrl_EditLabel(self.ptr, item as c_long) }
    }

    pub fn bind<F>(&self, event_type: EventType, callback: F)
    where
        F: FnMut(Event) + 'static,
    {
        self.window.bind(event_type, callback)
    }
}

impl WxWidget for ListCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

impl Drop for ListCtrl {
    fn drop(&mut self) {
        // Ownership is managed by the parent window/app in wxWidgets typically
        // No explicit Destroy call needed for ListCtrl if it's a child of a window
        // that gets destroyed, unless specifically detached or created as a top-level like window.
        // For now, assume it's cleaned up by parent like other controls.
    }
}

pub struct ListCtrlBuilder<P: WxWidget + Clone> {
    parent_ptr: *mut ffi::wxd_Window_t,
    id: Id,
    pos: Point,
    size: Size,
    style: ListCtrlStyle,
    _phantom: PhantomData<P>,
}

impl<P: WxWidget + Clone> Default for ListCtrlBuilder<P> {
    fn default() -> Self {
        Self {
            parent_ptr: std::ptr::null_mut(),
            id: ID_ANY,
            pos: DEFAULT_POSITION,
            size: DEFAULT_SIZE,
            style: ListCtrlStyle::default(),
            _phantom: PhantomData,
        }
    }
}

impl<P: WxWidget + Clone> ListCtrlBuilder<P> {
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    pub fn with_pos(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn with_style(mut self, style: ListCtrlStyle) -> Self {
        self.style = style;
        self
    }

    pub fn build(self) -> ListCtrl {
        assert!(!self.parent_ptr.is_null(), "ListCtrl requires a parent");
        let ptr = unsafe {
            ffi::wxd_ListCtrl_Create(
                self.parent_ptr,
                self.id,
                self.pos.into(),
                self.size.into(),
                self.style.bits(),
            )
        };
        if ptr.is_null() {
            panic!("Failed to create ListCtrl: FFI returned null pointer.");
        } else {
            unsafe {
                ListCtrl {
                    ptr,
                    window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
                }
            }
        }
    }
}

// Add specific ListEventData if needed, or ensure current Event can handle it.
// For example, if ListEventData needs methods like get_item_index(), get_column(), get_label().
// For now, assuming base Event with get_id() is used, and specific data access
// will be added to the generic Event<T> system or through specialized event types.
