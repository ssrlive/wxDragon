use std::ffi::CString;
use std::os::raw::{c_int, c_long};

use crate::prelude::*;
use wxdragon_sys as ffi;

// wxAuiToolBar styles (to be sourced from wxdragon_sys once generated)
pub const AUI_TB_TEXT: i64 = ffi::WXD_AUI_TB_TEXT; // wxAUI_TB_TEXT
pub const AUI_TB_NO_TOOLTIPS: i64 = ffi::WXD_AUI_TB_NO_TOOLTIPS; // wxAUI_TB_NO_TOOLTIPS
pub const AUI_TB_NO_AUTORESIZE: i64 = ffi::WXD_AUI_TB_NO_AUTORESIZE; // wxAUI_TB_NO_AUTORESIZE
pub const AUI_TB_GRIPPER: i64 = ffi::WXD_AUI_TB_GRIPPER; // wxAUI_TB_GRIPPER
pub const AUI_TB_OVERFLOW: i64 = ffi::WXD_AUI_TB_OVERFLOW; // wxAUI_TB_OVERFLOW
pub const AUI_TB_VERTICAL: i64 = ffi::WXD_AUI_TB_VERTICAL; // wxAUI_TB_VERTICAL
pub const AUI_TB_HORZ_LAYOUT: i64 = ffi::WXD_AUI_TB_HORZ_LAYOUT; // wxAUI_TB_HORZ_LAYOUT
pub const AUI_TB_HORIZONTAL: i64 = ffi::WXD_AUI_TB_HORIZONTAL; // wxAUI_TB_HORIZONTAL (alias for wxAUI_TB_TOP)

pub const AUI_TB_DEFAULT_STYLE: i64 = AUI_TB_GRIPPER | AUI_TB_OVERFLOW;

// Corresponds to WXDItemKindCEnum in C
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum ItemKind {
    Normal = 0,    // WXD_ITEM_NORMAL
    Check = 1,     // WXD_ITEM_CHECK
    Radio = 2,     // WXD_ITEM_RADIO
    Separator = 3, // WXD_ITEM_SEPARATOR
}
impl Default for ItemKind {
    fn default() -> Self {
        ItemKind::Normal
    }
}

#[derive(Clone)]
pub struct AuiToolBar {
    ptr: *mut ffi::wxd_AuiToolBar_t,
}

impl AuiToolBar {
    pub fn builder<P: WxWidget>(parent: &P) -> AuiToolBarBuilder {
        AuiToolBarBuilder::new(parent)
    }

    // Internal constructor from raw pointer
    fn from_ptr(ptr: *mut ffi::wxd_AuiToolBar_t) -> Self {
        AuiToolBar { ptr }
    }

    pub fn add_tool(&self, tool_id: i32, label: &str, short_help_string: &str, kind: ItemKind) {
        let c_label = CString::new(label).unwrap_or_default();
        let c_short_help = CString::new(short_help_string).unwrap_or_default();
        unsafe {
            ffi::wxd_AuiToolBar_AddTool(
                self.ptr,
                tool_id as c_int,
                c_label.as_ptr(),
                // Bitmaps are currently omitted in C API
                c_short_help.as_ptr(),
                kind as ffi::WXDItemKindCEnum,
            );
        }
    }

    pub fn add_label(&self, tool_id: i32, label: &str, width: i32) {
        let c_label = CString::new(label).unwrap_or_default();
        unsafe {
            ffi::wxd_AuiToolBar_AddLabel(
                self.ptr,
                tool_id as c_int,
                c_label.as_ptr(),
                width as c_int,
            );
        }
    }

    pub fn add_control<C: WxWidget>(&self, control: &C, label: &str) {
        let c_label = CString::new(label).unwrap_or_default();
        unsafe {
            ffi::wxd_AuiToolBar_AddControl(
                self.ptr,
                control.handle_ptr() as *mut ffi::wxd_Control_t,
                c_label.as_ptr(),
            );
        }
    }

    pub fn add_separator(&self) {
        unsafe {
            ffi::wxd_AuiToolBar_AddSeparator(self.ptr);
        }
    }

    pub fn add_spacer(&self, pixels: i32) {
        unsafe {
            ffi::wxd_AuiToolBar_AddSpacer(self.ptr, pixels as c_int);
        }
    }

    pub fn add_stretch_spacer(&self, proportion: i32) {
        unsafe {
            ffi::wxd_AuiToolBar_AddStretchSpacer(self.ptr, proportion as c_int);
        }
    }

    pub fn realize(&self) {
        unsafe {
            ffi::wxd_AuiToolBar_Realize(self.ptr);
        }
    }

    pub fn set_tool_bitmap_size(&self, size: Size) {
        unsafe {
            ffi::wxd_AuiToolBar_SetToolBitmapSize(self.ptr, size.into());
        }
    }

    pub fn get_tool_bitmap_size(&self) -> Size {
        let ffi_size = unsafe { ffi::wxd_AuiToolBar_GetToolBitmapSize(self.ptr) };
        Size::from(ffi_size)
    }

    pub fn set_overflow_visible(&self, visible: bool) {
        unsafe {
            ffi::wxd_AuiToolBar_SetOverflowVisible(self.ptr, visible);
        }
    }

    pub fn get_overflow_visible(&self) -> bool {
        unsafe { ffi::wxd_AuiToolBar_GetOverflowVisible(self.ptr) }
    }

    pub fn set_gripper_visible(&self, visible: bool) {
        unsafe {
            ffi::wxd_AuiToolBar_SetGripperVisible(self.ptr, visible);
        }
    }

    pub fn get_gripper_visible(&self) -> bool {
        unsafe { ffi::wxd_AuiToolBar_GetGripperVisible(self.ptr) }
    }

    pub fn set_tool_drop_down(&self, tool_id: i32, dropdown: bool) {
        unsafe {
            ffi::wxd_AuiToolBar_SetToolDropDown(self.ptr, tool_id as c_int, dropdown);
        }
    }

    pub fn get_tool_drop_down(&self, tool_id: i32) -> bool {
        unsafe { ffi::wxd_AuiToolBar_GetToolDropDown(self.ptr, tool_id as c_int) }
    }

    pub fn enable_tool(&self, tool_id: i32, enable: bool) {
        unsafe {
            ffi::wxd_AuiToolBar_EnableTool(self.ptr, tool_id as c_int, enable);
        }
    }

    pub fn get_tool_enabled(&self, tool_id: i32) -> bool {
        unsafe { ffi::wxd_AuiToolBar_GetToolEnabled(self.ptr, tool_id as c_int) }
    }

    pub fn get_tool_count(&self) -> i32 {
        unsafe { ffi::wxd_AuiToolBar_GetToolCount(self.ptr) as i32 }
    }

    pub fn clear_tools(&self) {
        unsafe {
            ffi::wxd_AuiToolBar_ClearTools(self.ptr);
        }
    }

    pub fn delete_tool(&self, tool_id: i32) -> bool {
        unsafe { ffi::wxd_AuiToolBar_DeleteTool(self.ptr, tool_id as c_int) }
    }
}

impl WxWidget for AuiToolBar {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.ptr as *mut ffi::wxd_Window_t
    }
}

impl WxEvtHandler for AuiToolBar {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.ptr as *mut ffi::wxd_EvtHandler_t
    }
}

pub struct AuiToolBarBuilder {
    parent_ptr: *mut ffi::wxd_Window_t,
    id: i32,
    pos: Point,
    size: Size,
    style: i64,
}

impl AuiToolBarBuilder {
    pub fn new<P: WxWidget>(parent: &P) -> Self {
        AuiToolBarBuilder {
            parent_ptr: parent.handle_ptr(),
            id: ID_ANY as i32,
            pos: Point::default(),
            size: Size::default(),
            style: AUI_TB_DEFAULT_STYLE,
        }
    }

    pub fn with_id(mut self, id: i32) -> Self {
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

    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    pub fn build(self) -> AuiToolBar {
        let ptr = unsafe {
            ffi::wxd_AuiToolBar_Create(
                self.parent_ptr,
                self.id as c_int,
                self.pos.into(),
                self.size.into(),
                self.style as c_long,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create AuiToolBar: wxWidgets returned a null pointer.");
        } else {
            AuiToolBar::from_ptr(ptr)
        }
    }
}
