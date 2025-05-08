//! Safe wrapper for wxPanel.

use crate::base::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE};
use crate::event::WxEvtHandler;
use crate::id::{Id, WXD_ID_ANY};
use crate::window::{Window, WxWidget};
use std::ops::{Deref, DerefMut};
use wxdragon_sys as ffi;

// --- Constants ---
pub const TAB_TRAVERSAL: i64 = wxdragon_sys::WXD_TAB_TRAVERSAL;

// Opaque pointer type from FFI
pub type RawPanel = ffi::wxd_Panel_t;

/// Represents a wxPanel widget.
/// Panels are windows within a frame (or other window) that can contain other widgets.
#[derive(Clone)]
pub struct Panel {
    window: Window, // Embed the generic Window
}

impl Panel {
    /// Creates a new builder for a Panel.
    pub fn builder(parent: &impl WxWidget) -> PanelBuilder {
        PanelBuilder::new(parent)
    }

    /// Creates a new Panel wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_Panel_t` pointer.
    /// Ownership is typically managed by the parent window in wxWidgets.
    pub(crate) unsafe fn from_ptr(ptr: *mut RawPanel) -> Self {
        assert!(!ptr.is_null());
        Panel {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Returns the raw underlying panel pointer.
    pub fn as_ptr(&self) -> *mut ffi::wxd_Panel_t {
        self.window.0 as *mut ffi::wxd_Panel_t
    }

    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        unsafe {
            let panel_ptr = ffi::wxd_Panel_Create(
                parent_ptr,
                id,
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t,
            );
            Self::from_ptr(panel_ptr)
        }
    }
}

// --- Builder Pattern ---

/// Builder for creating a `Panel`.
pub struct PanelBuilder<'a> {
    parent: &'a dyn WxWidget,
    id: Id,
    pos: Option<Point>,
    size: Option<Size>,
    style: i64, // Keep as i64
}

impl<'a> PanelBuilder<'a> {
    /// Creates a new Panel builder.
    /// `parent`: The parent window.
    /// `id`: Window identifier. Use `wxID_ANY` for automatic ID.
    pub fn new(parent: &'a dyn WxWidget) -> Self {
        Self {
            parent,
            id: WXD_ID_ANY as Id,
            pos: None,
            size: None,
            style: 0, // Default style (i64), e.g. ffi::WXD_TAB_TRAVERSAL as i64;
        }
    }

    /// Sets the window ID.
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    /// Sets the position.
    pub fn with_pos(mut self, x: i32, y: i32) -> Self {
        self.pos = Some(Point { x, y });
        self
    }

    /// Sets the size.
    pub fn with_size(mut self, width: i32, height: i32) -> Self {
        self.size = Some(Size { width, height });
        self
    }

    /// Sets the style flags.
    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    /// Creates the `Panel`.
    /// Returns `None` if the panel creation fails (e.g., null pointer returned).
    pub fn build(self) -> Panel {
        let parent_ptr = self.parent.handle_ptr();
        let pos = self.pos.unwrap_or(DEFAULT_POSITION);
        let size = self.size.unwrap_or(DEFAULT_SIZE);
        Panel::new_impl(parent_ptr, self.id, pos, size, self.style)
    }
}

// --- Trait Implementations ---

impl WxWidget for Panel {
    /// Returns the underlying window pointer.
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

impl Deref for Panel {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl DerefMut for Panel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}

impl WxEvtHandler for Panel {
    /// # Safety
    /// Inherits safety requirements from `Window::get_event_handler_ptr`.
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        // Panel inherits from Window, so the pointer is compatible.
        self.window.handle_ptr() as *mut ffi::wxd_EvtHandler_t
    }
}

// No Drop implementation needed here: wxWidgets manages the lifetime of child windows.
// The Panel will be destroyed when its parent (e.g., the Frame) is destroyed.
