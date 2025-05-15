//!
//! Safe wrapper for wxToolBar.

use crate::bitmap::Bitmap;
use crate::id::Id;
use crate::menus::ItemKind; // Reuse ItemKind for tool types
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use std::ops::{BitOr, BitOrAssign};
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// --- ToolBarStyle Enum ---
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum ToolBarStyle {
    Default = ffi::WXD_TB_HORIZONTAL, // Default to Horizontal (e.g., value 4)
    Vertical = ffi::WXD_TB_VERTICAL,
    Text = ffi::WXD_TB_TEXT,           // Show text labels
    NoIcons = ffi::WXD_TB_NOICONS,     // Show text only, no icons
    NoDivider = ffi::WXD_TB_NODIVIDER, // No divider between groups
    Flat = ffi::WXD_TB_FLAT,           // Flat toolbar look
    Dockable = ffi::WXD_TB_DOCKABLE,   // Can be dragged and docked
                                       // Add other styles like TB_RIGHT, TB_BOTTOM if FFI constants exist
}

impl ToolBarStyle {
    pub fn bits(self) -> i64 {
        self as i64
    }
}

impl Default for ToolBarStyle {
    fn default() -> Self {
        ToolBarStyle::Default
    }
}

impl BitOr for ToolBarStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for ToolBarStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = unsafe { std::mem::transmute(self.bits() | rhs.bits()) };
    }
}

/// Represents a wxToolBar control.
/// Toolbars generate `EventType::MENU` events on their parent window when a tool is clicked.
pub struct ToolBar {
    window: Window, // Composition: ToolBar IS a Window (and Control)
}

impl ToolBar {
    /// Creates a `ToolBar` wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_ToolBar_t` pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_ToolBar_t) -> Self {
        ToolBar {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Low-level function to add a tool.
    /// Prefer using `add_tool`, `add_check_tool`, `add_radio_tool` etc.
    /// Returns true if the tool was added successfully (C++ returns non-null ptr).
    /// # Safety
    /// Requires valid toolbar pointer and CString conversions must succeed.
    fn add_tool_raw(
        &self,
        tool_id: Id,
        label: &str,
        bitmap: &Bitmap,
        bitmap_disabled: Option<&Bitmap>,
        kind: ItemKind,
        short_help: &str,
        long_help: &str,
    ) -> bool {
        let c_label = CString::new(label).unwrap_or_default();
        let c_short_help = CString::new(short_help).unwrap_or_default();
        let c_longlong_help = CString::new(long_help).unwrap_or_default();
        let bmp_disabled_ptr = bitmap_disabled.map_or(std::ptr::null_mut(), |bmp| bmp.as_ptr());

        unsafe {
            let tool_ptr = ffi::wxd_ToolBar_AddTool(
                self.window.as_ptr() as *mut _,
                tool_id,
                c_label.as_ptr(),
                bitmap.as_ptr(),
                bmp_disabled_ptr,
                kind as c_int,
                c_short_help.as_ptr(),
                c_longlong_help.as_ptr(),
            );
            !tool_ptr.is_null()
        }
    }

    /// Adds a normal tool to the toolbar.
    ///
    /// # Arguments
    /// * `tool_id` - ID for the tool, used in event handling.
    /// * `label` - Label shown if `TB_TEXT` style is used.
    /// * `bitmap` - The bitmap for the tool's normal state.
    /// * `short_help` - Short help string (tooltip).
    pub fn add_tool(&self, tool_id: Id, label: &str, bitmap: &Bitmap, short_help: &str) -> bool {
        self.add_tool_raw(
            tool_id,
            label,
            bitmap,
            None,
            ItemKind::Normal,
            short_help,
            "",
        )
    }

    /// Adds a check tool (toggle tool) to the toolbar.
    pub fn add_check_tool(
        &self,
        tool_id: Id,
        label: &str,
        bitmap: &Bitmap,
        short_help: &str,
    ) -> bool {
        self.add_tool_raw(
            tool_id,
            label,
            bitmap,
            None,
            ItemKind::Check,
            short_help,
            "",
        )
    }

    /// Adds a radio tool to the toolbar.
    /// Radio tools require grouping with separators or other radio tools.
    pub fn add_radio_tool(
        &self,
        tool_id: Id,
        label: &str,
        bitmap: &Bitmap,
        short_help: &str,
    ) -> bool {
        self.add_tool_raw(
            tool_id,
            label,
            bitmap,
            None,
            ItemKind::Radio,
            short_help,
            "",
        )
    }

    /// Adds a separator.
    pub fn add_separator(&self) {
        unsafe {
            ffi::wxd_ToolBar_AddSeparator(self.window.as_ptr() as *mut _);
        }
    }

    /// Adds an arbitrary control (like a `Choice` or `TextCtrl`) to the toolbar.
    /// The control should have the toolbar as its parent.
    pub fn add_control<W: WxWidget>(&self, control: &W) {
        unsafe {
            ffi::wxd_ToolBar_AddControl(self.window.as_ptr() as *mut _, control.handle_ptr());
        }
    }

    /// Must be called after adding tools to finalize the toolbar layout.
    /// Returns true if successful.
    pub fn realize(&self) -> bool {
        unsafe { ffi::wxd_ToolBar_Realize(self.window.as_ptr() as *mut _) }
    }

    /// Enables or disables a tool.
    pub fn enable_tool(&self, tool_id: Id, enable: bool) {
        unsafe {
            ffi::wxd_ToolBar_EnableTool(self.window.as_ptr() as *mut _, tool_id, enable);
        }
    }

    /// Toggles the state of a check or radio tool.
    pub fn toggle_tool(&self, tool_id: Id, toggle: bool) {
        unsafe {
            ffi::wxd_ToolBar_ToggleTool(self.window.as_ptr() as *mut _, tool_id, toggle);
        }
    }

    /// Checks if a tool is enabled.
    pub fn is_tool_enabled(&self, tool_id: Id) -> bool {
        unsafe { ffi::wxd_ToolBar_IsToolEnabled(self.window.as_ptr() as *mut _, tool_id) }
    }

    /// Gets the state of a check or radio tool.
    pub fn get_tool_state(&self, tool_id: Id) -> bool {
        unsafe { ffi::wxd_ToolBar_GetToolState(self.window.as_ptr() as *mut _, tool_id) }
    }

    /// Sets the short help string (tooltip) for a tool.
    pub fn set_tool_short_help(&self, tool_id: Id, help_string: &str) {
        let c_help = CString::new(help_string).unwrap_or_default();
        unsafe {
            ffi::wxd_ToolBar_SetToolShortHelp(
                self.window.as_ptr() as *mut _,
                tool_id,
                c_help.as_ptr(),
            );
        }
    }
}

// --- Trait Implementations ---

impl WxWidget for ToolBar {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

// ToolBar doesn't handle events directly, the parent (Frame) does via EventType::MENU
// So, no WxEvtHandler impl needed for ToolBar itself.

// No Drop needed, wxToolBar is a Window managed by its parent frame.
