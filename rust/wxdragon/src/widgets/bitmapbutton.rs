//!
//! Safe wrapper for wxBitmapButton.

use crate::base::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE, ID_ANY};
use crate::bitmap::Bitmap;
use crate::event::WxEvtHandler;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// wxBitmapButton styles (Combine with wxBU_ from Button if needed)
pub const BU_LEFT: i64 = ffi::WXD_BU_LEFT;
pub const BU_TOP: i64 = ffi::WXD_BU_TOP;
pub const BU_RIGHT: i64 = ffi::WXD_BU_RIGHT;
pub const BU_BOTTOM: i64 = ffi::WXD_BU_BOTTOM;
pub const BU_EXACTFIT: i64 = ffi::WXD_BU_EXACTFIT;
pub const BORDER_NONE: i64 = ffi::WXD_BORDER_NONE;
pub const BU_NOTEXT: i64 = ffi::WXD_BU_NOTEXT;

/// Represents a wxBitmapButton widget.
/// This is a button that displays a bitmap instead of a text label.
pub struct BitmapButton {
    window: Window, // Inherits basic window properties
}

impl BitmapButton {
    /// Creates a new BitmapButton builder.
    /// Requires a bitmap to be set using `with_bitmap`.
    pub fn builder<W: WxWidget>(parent: &W) -> BitmapButtonBuilder {
        BitmapButtonBuilder {
            parent_ptr: parent.handle_ptr(),
            label: CString::new("").unwrap(), // Default empty label
            id: ID_ANY,
            pos: DEFAULT_POSITION, // Use constant
            size: DEFAULT_SIZE,    // Use constant
            style: 0i64,           // Use 0 as default style
            bitmap: None,
            bitmap_disabled: None,
            bitmap_focus: None,
            bitmap_hover: None,
            name: CString::new("BitmapButton").unwrap(), // Default name
        }
    }

    /// Creates a new BitmapButton wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_BitmapButton_t` pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_BitmapButton_t) -> Self {
        BitmapButton {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    // TODO: Add methods like SetBitmapLabel, SetBitmapHover etc. if needed
}

// --- Builder Pattern ---

/// Builder for creating `BitmapButton` widgets.
#[derive(Clone)]
pub struct BitmapButtonBuilder {
    parent_ptr: *mut ffi::wxd_Window_t,
    label: CString, // Retained for consistency, though wxBitmapButton doesn't directly use text label
    id: i32,
    pos: Point,
    size: Size,
    style: i64,
    bitmap: Option<Bitmap>,          // To hold the main bitmap
    bitmap_disabled: Option<Bitmap>, // Optional: disabled bitmap
    bitmap_focus: Option<Bitmap>,    // Optional: focus bitmap
    bitmap_hover: Option<Bitmap>,    // Optional: hover bitmap
    name: CString,
}

impl BitmapButtonBuilder {
    /// Sets the window ID.
    pub fn with_id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    /// Sets the bitmap for the normal (default) state. This is mandatory.
    pub fn with_bitmap(mut self, bitmap: &Bitmap) -> Self {
        self.bitmap = Some(bitmap.clone());
        self
    }

    /// Sets the bitmap for the disabled state.
    pub fn with_bitmap_disabled(mut self, bitmap: &Bitmap) -> Self {
        self.bitmap_disabled = Some(bitmap.clone());
        self
    }

    /// Sets the bitmap for when the button has keyboard focus.
    pub fn with_bitmap_focus(mut self, bitmap: &Bitmap) -> Self {
        self.bitmap_focus = Some(bitmap.clone());
        self
    }

    /// Sets the bitmap for when the mouse is hovering over the button.
    pub fn with_bitmap_hover(mut self, bitmap: &Bitmap) -> Self {
        self.bitmap_hover = Some(bitmap.clone());
        self
    }

    /// Sets the position.
    pub fn with_pos(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    /// Sets the size.
    /// By default, the button sizes itself to fit the bitmap.
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Sets the style flags.
    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    /// Sets the label for the button.
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = CString::new(label).unwrap_or_default();
        self
    }

    /// Sets the name for the button.
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = CString::new(name).unwrap_or_default();
        self
    }

    /// Creates the `BitmapButton`.
    /// Panics if `with_bitmap` was not called or parent not set.
    pub fn build(self) -> BitmapButton {
        let main_bitmap = self.bitmap.expect("BitmapButton requires a main bitmap.");
        let bitmap_ptr = main_bitmap.as_ptr();

        let bmp_disabled_ptr = self
            .bitmap_disabled
            .as_ref()
            .map_or(std::ptr::null_mut(), |b| b.as_ptr());
        let bmp_focus_ptr = self
            .bitmap_focus
            .as_ref()
            .map_or(std::ptr::null_mut(), |b| b.as_ptr());
        let bmp_hover_ptr = self
            .bitmap_hover
            .as_ref()
            .map_or(std::ptr::null_mut(), |b| b.as_ptr());

        // For BitmapButton, size is often best derived from the bitmap if not explicitly set.
        let final_size = if self.size.width == -1 && self.size.height == -1 {
            Size::new(main_bitmap.get_width(), main_bitmap.get_height())
        } else {
            self.size
        };

        unsafe {
            let ptr = ffi::wxd_BitmapButton_Create(
                self.parent_ptr,
                self.id as c_int,
                bitmap_ptr,
                self.pos.into(),
                final_size.into(),
                self.style as ffi::wxd_Style_t,
                self.name.as_ptr(),
                bmp_disabled_ptr,
                bmp_focus_ptr,
                bmp_hover_ptr,
            );
            if ptr.is_null() {
                panic!("Failed to create BitmapButton widget");
            } else {
                BitmapButton::from_ptr(ptr)
            }
        }
    }
}

// --- Trait Implementations ---

impl WxWidget for BitmapButton {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

impl Deref for BitmapButton {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl DerefMut for BitmapButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}

impl WxEvtHandler for BitmapButton {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.window.handle_ptr() as *mut ffi::wxd_EvtHandler_t
    }
}

// No Drop needed, wxBitmapButton is a Window managed by its parent.
