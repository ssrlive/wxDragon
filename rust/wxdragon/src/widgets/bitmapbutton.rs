//!
//! Safe wrapper for wxBitmapButton.

use crate::base::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE, ID_ANY};
use crate::bitmap::Bitmap;
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::window::{Window, WxWidget};
use std::default::Default;
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
        let mut builder = BitmapButtonBuilder::default();
        builder.parent_ptr = parent.handle_ptr();
        builder
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
pub struct BitmapButtonBuilder {
    parent_ptr: *mut ffi::wxd_Window_t,
    id: Id,
    bitmap: Option<*mut ffi::wxd_Bitmap_t>, // Bitmap is required, Option used to enforce setting via with_bitmap
    pos: Point,
    size: Size, // Default size is calculated in build() if not set
    style: i64,
}

// Manual Default implementation
impl Default for BitmapButtonBuilder {
    fn default() -> Self {
        Self {
            parent_ptr: std::ptr::null_mut(),
            id: ID_ANY,            // Use ID_ANY from base (i32)
            bitmap: None,          // Must be set via with_bitmap
            pos: DEFAULT_POSITION, // Use base constant
            size: DEFAULT_SIZE,    // Use base constant
            style: 0,
        }
    }
}

impl BitmapButtonBuilder {
    /// Sets the window ID.
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    /// Sets the bitmap label for the button. This is mandatory.
    pub fn with_bitmap(mut self, bitmap: &Bitmap) -> Self {
        self.bitmap = Some(bitmap.as_ptr()); // Use the (currently crate-public) as_ptr method
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

    /// Creates the `BitmapButton`.
    /// Panics if `with_bitmap` was not called or parent not set.
    pub fn build(self) -> BitmapButton {
        assert!(!self.parent_ptr.is_null(), "BitmapButton requires a parent");
        let bitmap_ptr = self
            .bitmap
            .expect("Bitmap must be set for BitmapButton using with_bitmap");

        // Default Size Calculation
        let mut final_size = self.size;
        // Check against DEFAULT_SIZE constant from base
        if final_size == DEFAULT_SIZE {
            // ... (size calculation logic as before) ...
            let bmp_width = unsafe { ffi::wxd_Bitmap_GetWidth(bitmap_ptr) };
            let bmp_height = unsafe { ffi::wxd_Bitmap_GetHeight(bitmap_ptr) };
            if bmp_width > 0 && bmp_height > 0 {
                const PADDING_X: i32 = 10;
                const PADDING_Y: i32 = 6;
                final_size = Size::new(bmp_width + PADDING_X, bmp_height + PADDING_Y);
            } else {
                final_size = Size::new(30, 30);
            }
        }

        let button_ptr = unsafe {
            ffi::wxd_BitmapButton_Create(
                self.parent_ptr,                // Use raw pointer
                self.id as c_int,               // Use id
                bitmap_ptr,                     // Use unwrapped bitmap pointer
                self.pos.into(),                // Use pos
                final_size.into(),              // Use calculated or user-provided size
                self.style as ffi::wxd_Style_t, // Use style
            )
        };

        if button_ptr.is_null() {
            panic!("Failed to create BitmapButton");
        }
        unsafe { BitmapButton::from_ptr(button_ptr) }
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
