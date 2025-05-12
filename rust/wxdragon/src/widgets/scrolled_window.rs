//!
//! Safe wrapper for wxScrolledWindow.

use crate::geometry::{Point, Size};
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::id::ID_ANY;
use crate::widgets::panel::Panel; // Inherits from Panel (used for Deref)
use crate::window::WxWidget; // Used for builder parent type constraint
use std::ops::{Deref, DerefMut};
use wxdragon_sys as ffi;

// --- Constants (Styles) ---
// Add relevant wxScrolledWindow styles here if needed, e.g.:
// pub const HSCROLL: i64 = wxdragon_sys::WXD_HSCROLL;
// pub const VSCROLL: i64 = wxdragon_sys::WXD_VSCROLL;
// Need to add WXD_HSCROLL, WXD_VSCROLL to const_extractor if used.

/// Represents a wxScrolledWindow widget.
/// A window that can scroll its contents.
pub struct ScrolledWindow {
    panel: Panel, // Composition: ScrolledWindow "is a" Panel
}

impl ScrolledWindow {
    /// Creates a new builder for a ScrolledWindow.
    pub fn builder<W: WxWidget>(parent: &W) -> ScrolledWindowBuilder {
        ScrolledWindowBuilder::new(parent)
    }

    /// Creates a new ScrolledWindow wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_ScrolledWindow_t` pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_ScrolledWindow_t) -> Self {
        ScrolledWindow {
            panel: Panel::from_ptr(ptr as *mut ffi::wxd_Panel_t),
        }
    }

    /// Returns the raw underlying scrolled window pointer.
    pub fn as_ptr(&self) -> *mut ffi::wxd_ScrolledWindow_t {
        self.panel.as_ptr() as *mut ffi::wxd_ScrolledWindow_t
    }

    /// Sets the scroll rate (pixels per scroll unit).
    pub fn set_scroll_rate(&self, x_step: i32, y_step: i32) {
        unsafe { ffi::wxd_ScrolledWindow_SetScrollRate(self.as_ptr(), x_step, y_step) }
    }

    /// Sets up the scrollbars.
    pub fn set_scrollbars(
        &self,
        pixels_per_unit_x: i32,
        pixels_per_unit_y: i32,
        no_units_x: i32,
        no_units_y: i32,
        x_pos: i32,
        y_pos: i32,
        no_refresh: bool,
    ) {
        unsafe {
            ffi::wxd_ScrolledWindow_SetScrollbars(
                self.as_ptr(),
                pixels_per_unit_x,
                pixels_per_unit_y,
                no_units_x,
                no_units_y,
                x_pos,
                y_pos,
                no_refresh,
            )
        }
    }

    /// Enables or disables scrolling for the specified orientation(s).
    pub fn enable_scrolling(&self, x_scrolling: bool, y_scrolling: bool) {
        unsafe { ffi::wxd_ScrolledWindow_EnableScrolling(self.as_ptr(), x_scrolling, y_scrolling) }
    }

    /// Scrolls the window to the given position (in scroll units).
    pub fn scroll_coords(&self, x: i32, y: i32) {
        unsafe { ffi::wxd_ScrolledWindow_Scroll_Coord(self.as_ptr(), x, y) }
    }

    /// Scrolls the window to the given position (in scroll units).
    pub fn scroll_point(&self, pt: Point) {
        let c_pt = ffi::wxd_Point { x: pt.x, y: pt.y };
        unsafe { ffi::wxd_ScrolledWindow_Scroll_Point(self.as_ptr(), c_pt) }
    }

    /// Gets the size of the scrollable virtual area in pixels.
    pub fn get_virtual_size(&self) -> Size {
        let mut w: i32 = 0;
        let mut h: i32 = 0;
        unsafe { ffi::wxd_ScrolledWindow_GetVirtualSize(self.as_ptr(), &mut w, &mut h) };
        Size {
            width: w,
            height: h,
        }
    }

    /// Gets the number of pixels per scroll unit.
    pub fn get_scroll_pixels_per_unit(&self) -> (i32, i32) {
        let mut x_unit: i32 = 0;
        let mut y_unit: i32 = 0;
        unsafe {
            ffi::wxd_ScrolledWindow_GetScrollPixelsPerUnit(self.as_ptr(), &mut x_unit, &mut y_unit)
        };
        (x_unit, y_unit)
    }
}

// --- Builder Pattern ---

/// Builder for creating a `ScrolledWindow`.
pub struct ScrolledWindowBuilder {
    parent_ptr: *mut ffi::wxd_Window_t, // Store raw parent pointer
    id: Id,
    pos: Point,
    size: Size,
    style: i64,
}

impl ScrolledWindowBuilder {
    /// Creates a new ScrolledWindow builder.
    pub fn new(parent: &impl WxWidget) -> Self {
        Self {
            parent_ptr: parent.handle_ptr(),
            id: ID_ANY as i32,
            pos: Point { x: -1, y: -1 }, // Default position
            size: Size {
                width: -1,
                height: -1,
            }, // Default size
            style: 0, // Default style (consider adding HSCROLL/VSCROLL flags here?)
        }
    }

    /// Sets the window ID.
    pub fn with_id(mut self, id: Id) -> Self {
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
    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    /// Creates the `ScrolledWindow`.
    /// Panics if creation fails (FFI returns null).
    pub fn build(self) -> ScrolledWindow {
        if self.parent_ptr.is_null() {
            panic!("Cannot create ScrolledWindow with a null parent");
        }

        let c_pos = ffi::wxd_Point {
            x: self.pos.x,
            y: self.pos.y,
        };
        let c_size = ffi::wxd_Size {
            width: self.size.width,
            height: self.size.height,
        };

        let ptr = unsafe {
            ffi::wxd_ScrolledWindow_Create(
                self.parent_ptr,
                self.id,
                c_pos,
                c_size,
                self.style as ffi::wxd_Style_t,
            )
        };

        if ptr.is_null() {
            panic!("Failed to create wxScrolledWindow");
        } else {
            unsafe { ScrolledWindow::from_ptr(ptr) }
        }
    }
}

// --- Trait Implementations ---

impl WxWidget for ScrolledWindow {
    /// Returns the underlying window pointer.
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.panel.handle_ptr()
    }
}

impl Deref for ScrolledWindow {
    type Target = Panel;
    fn deref(&self) -> &Self::Target {
        &self.panel
    }
}

impl DerefMut for ScrolledWindow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.panel
    }
}

// ScrolledWindow handles events like any other Window/Panel derivative
impl WxEvtHandler for ScrolledWindow {
    /// # Safety
    /// Inherits safety requirements from `Window::get_event_handler_ptr`.
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.panel.get_event_handler_ptr()
    }
}

// No Drop needed, Panel's base (Window) handles cleanup notifier attachment
