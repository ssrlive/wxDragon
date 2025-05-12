/* This is a new file */
//! Safe wrapper for wxFontPickerCtrl.

use crate::geometry::{Point, Size};
use crate::event::WxEvtHandler;
use crate::font::Font; // Import the safe Font wrapper
use crate::window::Window;
use crate::WxWidget;
use std::ffi::c_long;
use std::ptr;
use wxdragon_sys as ffi;
use std::ops::{BitOr, BitOrAssign};
use std::default::Default;

/// Window style flags for `FontPickerCtrl`.
///
/// These flags can be combined using the bitwise OR operator (`|`).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum FontPickerCtrlStyle {
    /// Default style, includes `UseTextCtrl`.
    Default = ffi::WXD_FNTP_DEFAULT_STYLE,
    /// Use a text control to display the font description.
    UseTextCtrl = ffi::WXD_FNTP_USE_TEXTCTRL,
    /// Show the font description (e.g., "Times New Roman Bold 10") as the label.
    FontDescAsLabel = ffi::WXD_FNTP_FONTDESC_AS_LABEL,
    /// Use the selected font itself to draw the label.
    UseFontForLabel = ffi::WXD_FNTP_USEFONT_FOR_LABEL,
}

impl FontPickerCtrlStyle {
    /// Returns the raw integer value of the style.
    pub fn bits(self) -> i64 {
        self as i64
    }

    /// The default style for `FontPickerCtrl`.
    pub const DEFAULT: FontPickerCtrlStyle = FontPickerCtrlStyle::Default;
}

impl Default for FontPickerCtrlStyle {
    fn default() -> Self {
        FontPickerCtrlStyle::DEFAULT
    }
}

impl BitOr for FontPickerCtrlStyle {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for FontPickerCtrlStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe {
            *self = std::mem::transmute(self.bits() | rhs.bits());
        }
    }
}

// --- FontPickerCtrl ---

#[derive(Clone)]
pub struct FontPickerCtrl {
    window: Window, // Embed Window
}

impl FontPickerCtrl {
    /// Creates a new FontPickerCtrlBuilder.
    pub fn builder<'a>(parent: &'a impl WxWidget) -> FontPickerCtrlBuilder<'a> {
        let mut builder = FontPickerCtrlBuilder::default();
        builder.parent_ptr = parent.handle_ptr();
        builder
    }

    /// Gets the currently selected font.
    /// Returns `None` if no font is selected or the font is invalid.
    pub fn get_selected_font(&self) -> Option<Font> {
        unsafe {
            let font_ptr = ffi::wxd_FontPickerCtrl_GetSelectedFont(
                self.window.as_ptr() as *mut ffi::wxd_FontPickerCtrl_t
            );
            if font_ptr.is_null() {
                None
            } else {
                // Font::from_ptr should take ownership (true) and handle drop via wxd_Font_Destroy
                Some(Font::from_ptr(font_ptr, true))
            }
        }
    }

    /// Sets the currently selected font.
    pub fn set_selected_font(&self, font: &Font) {
        unsafe {
            ffi::wxd_FontPickerCtrl_SetSelectedFont(
                self.window.as_ptr() as *mut ffi::wxd_FontPickerCtrl_t,
                font.as_ptr(), // Font::as_ptr() should provide const wxd_Font_t*
            );
        }
    }

    /// Creates a FontPickerCtrl from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_FontPickerCtrl_t`.
    unsafe fn from_ptr(ptr: *mut ffi::wxd_FontPickerCtrl_t) -> Self {
        FontPickerCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }
}

impl WxWidget for FontPickerCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

impl WxEvtHandler for FontPickerCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.window.get_event_handler_ptr()
    }
}

// Drop is handled by the embedded Window or parentage.

// --- FontPickerCtrlBuilder ---

pub struct FontPickerCtrlBuilder<'a> {
    parent_ptr: *mut ffi::wxd_Window_t,
    id: i32,
    initial_font: Option<Font>, // Store Option<Font> and get its ptr in build()
    pos: Point,
    size: Size,
    style: FontPickerCtrlStyle,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Default for FontPickerCtrlBuilder<'a> {
    fn default() -> Self {
        Self {
            parent_ptr: ptr::null_mut(),
            id: ffi::WXD_ID_ANY as i32,
            initial_font: None,
            pos: Point::default(),
            size: Size::default(),
            style: FontPickerCtrlStyle::DEFAULT,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a> FontPickerCtrlBuilder<'a> {
    pub fn with_id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    pub fn with_initial_font(mut self, font: Font) -> Self {
        self.initial_font = Some(font);
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

    pub fn with_style(mut self, style: FontPickerCtrlStyle) -> Self {
        self.style = style;
        self
    }

    pub fn build(self) -> FontPickerCtrl {
        assert!(
            !self.parent_ptr.is_null(),
            "FontPickerCtrl requires a parent"
        );

        let initial_font_ptr = self
            .initial_font
            .as_ref()
            .map_or(ptr::null(), |f| f.as_ptr());

        let ffi_pos: ffi::wxd_Point = self.pos.into();
        let ffi_size: ffi::wxd_Size = self.size.into();

        let ptr = unsafe {
            ffi::wxd_FontPickerCtrl_Create(
                self.parent_ptr,
                self.id,
                initial_font_ptr,
                ffi_pos,
                ffi_size,
                self.style.bits() as c_long,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create FontPickerCtrl: FFI returned null pointer.");
        } else {
            unsafe { FontPickerCtrl::from_ptr(ptr) }
        }
    }
}
