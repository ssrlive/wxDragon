//!
//! Safe wrapper for wxSplitterWindow.

use crate::geometry::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE};
use crate::id::ID_ANY;
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::window::WxWidget;
use std::ops::{BitOr, BitOrAssign};
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// wxSplitterWindow styles (Combine with SP_ from SpinCtrl/Button if appropriate)
// pub const SP_HORIZONTAL: i64 = ffi::WXD_SP_HORIZONTAL;
// pub const SP_VERTICAL: i64 = ffi::WXD_SP_VERTICAL;
// pub const SP_3D: i64 = ffi::WXD_SP_3D;
// pub const SP_BORDER: i64 = ffi::WXD_SP_BORDER;
// pub const SP_PERMIT_UNSPLIT: i64 = ffi::WXD_SP_PERMIT_UNSPLIT;
// pub const SP_LIVE_UPDATE: i64 = ffi::WXD_SP_LIVE_UPDATE;
// pub const SP_NOBORDER: i64 = ffi::WXD_SP_NOBORDER;
// pub const SP_THIN_SASH: i64 = ffi::WXD_SP_THIN_SASH;
// pub const SP_DEFAULT_STYLE: i64 = ffi::WXD_SP_BORDER;

/// Represents a wxSplitterWindow widget.
#[derive(Clone)]
pub struct SplitterWindow(pub(crate) *mut ffi::wxd_SplitterWindow_t);

impl SplitterWindow {
    /// Creates a new SplitterWindow builder.
    pub fn builder<W: WxWidget>(parent: &W) -> SplitterWindowBuilder {
        SplitterWindowBuilder::new(parent)
    }

    // Internal constructor - Revert back to crate-public
    // SAFETY: Caller must ensure ptr is a valid wxd_SplitterWindow_t
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_SplitterWindow_t) -> Self {
        SplitterWindow(ptr)
    }

    /// Initializes the splitter to contain the given window.
    /// Should be called after creation if the splitter is not split initially.
    pub fn initialize<W: WxWidget>(&self, window: &W) {
        unsafe { ffi::wxd_SplitterWindow_Initialize(self.0, window.handle_ptr()) };
    }

    /// Splits the window vertically, putting `window1` on the left and `window2` on the right.
    ///
    /// # Arguments
    /// * `window1` - The window for the left pane.
    /// * `window2` - The window for the right pane.
    /// * `sash_position` - The initial position of the sash. If 0 or negative, a default position is used.
    ///
    /// Returns `true` on success.
    pub fn split_vertically<W1: WxWidget, W2: WxWidget>(
        &self,
        window1: &W1,
        window2: &W2,
        sash_position: i32,
    ) -> bool {
        unsafe {
            ffi::wxd_SplitterWindow_SplitVertically(
                self.0,
                window1.handle_ptr(),
                window2.handle_ptr(),
                sash_position as c_int,
            )
        }
    }

    /// Splits the window horizontally, putting `window1` above `window2`.
    ///
    /// # Arguments
    /// * `window1` - The window for the top pane.
    /// * `window2` - The window for the bottom pane.
    /// * `sash_position` - The initial position of the sash. If 0 or negative, a default position is used.
    ///
    /// Returns `true` on success.
    pub fn split_horizontally<W1: WxWidget, W2: WxWidget>(
        &self,
        window1: &W1,
        window2: &W2,
        sash_position: i32,
    ) -> bool {
        unsafe {
            ffi::wxd_SplitterWindow_SplitHorizontally(
                self.0,
                window1.handle_ptr(),
                window2.handle_ptr(),
                sash_position as c_int,
            )
        }
    }

    /// Unspltis the window.
    ///
    /// # Arguments
    /// * `to_remove` - Optional window to remove. If `None`, the second (right/bottom) window is removed.
    ///
    /// Returns `true` on success.
    pub fn unsplit<W: WxWidget>(&self, to_remove: Option<&W>) -> bool {
        let remove_ptr = to_remove.map_or(std::ptr::null_mut(), |w| w.handle_ptr());
        unsafe { ffi::wxd_SplitterWindow_Unsplit(self.0, remove_ptr) }
    }

    /// Sets the sash position.
    pub fn set_sash_position(&self, position: i32, redraw: bool) {
        unsafe { ffi::wxd_SplitterWindow_SetSashPosition(self.0, position as c_int, redraw) };
    }

    /// Gets the current sash position.
    pub fn sash_position(&self) -> i32 {
        unsafe { ffi::wxd_SplitterWindow_GetSashPosition(self.0) }
    }

    /// Sets the minimum pane size (applies to both panes).
    pub fn set_minimum_pane_size(&self, size: i32) {
        unsafe { ffi::wxd_SplitterWindow_SetMinimumPaneSize(self.0, size as c_int) };
    }
}

// Implement the core WxWidget trait
impl WxWidget for SplitterWindow {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.0 as *mut ffi::wxd_Window_t
    }
}

// Implement the event handling trait
impl WxEvtHandler for SplitterWindow {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.0 as *mut ffi::wxd_EvtHandler_t
    }
}

// --- SplitterWindowStyle Enum ---
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum SplitterWindowStyle {
    Default = ffi::WXD_SP_BORDER, // Default to having a border (e.g. 512)
    Horizontal = ffi::WXD_SP_HORIZONTAL,
    Vertical = ffi::WXD_SP_VERTICAL,
    PermitUnsplit = ffi::WXD_SP_PERMIT_UNSPLIT,
    LiveUpdate = ffi::WXD_SP_LIVE_UPDATE,
    ThinSash = ffi::WXD_SP_THIN_SASH,
}

impl SplitterWindowStyle {
    pub fn bits(self) -> i64 {
        self as i64
    }
}

impl Default for SplitterWindowStyle {
    fn default() -> Self {
        SplitterWindowStyle::Default
    }
}

impl BitOr for SplitterWindowStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for SplitterWindowStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = unsafe { std::mem::transmute(self.bits() | rhs.bits()) };
    }
}

/// Builder for creating `SplitterWindow` widgets.
pub struct SplitterWindowBuilder {
    parent: *mut ffi::wxd_Window_t,
    id: Id,
    pos: Point,
    size: Size,
    style: SplitterWindowStyle,
}

impl SplitterWindowBuilder {
    /// Creates a new SplitterWindow builder with default values.
    pub fn new<W: WxWidget>(parent: &W) -> Self {
        SplitterWindowBuilder {
            parent: parent.handle_ptr(),
            id: ID_ANY as i32,
            pos: DEFAULT_POSITION,
            size: DEFAULT_SIZE,
            style: SplitterWindowStyle::default(),
        }
    }

    /// Sets the window ID.
    pub fn with_id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    /// Sets the position.
    pub fn with_pos(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    /// Sets the size.
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Sets the style flags.
    pub fn with_style(mut self, style: SplitterWindowStyle) -> Self {
        self.style = style;
        self
    }

    /// Builds the SplitterWindow widget.
    /// Note: Call `initialize` or one of the `split_` methods after building.
    pub fn build(self) -> SplitterWindow {
        let splitter_ptr = unsafe {
            ffi::wxd_SplitterWindow_Create(
                self.parent,
                self.id as c_int,
                self.pos.into(),
                self.size.into(),
                self.style.bits() as ffi::wxd_Style_t,
            )
        };
        if splitter_ptr.is_null() {
            panic!("Failed to create SplitterWindow");
        }
        unsafe { SplitterWindow::from_ptr(splitter_ptr) }
    }
}
