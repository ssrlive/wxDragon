/* This is a new file */
//! Safe wrapper for wxFontPickerCtrl.

use std::ffi::c_longlong;
use wxdragon_sys as ffi;

use crate::event::WxEvtHandler;
use crate::font::Font;
use crate::implement_widget_traits_with_target;
use crate::prelude::*;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};

// --- Style enum using macro ---
widget_style_enum!(
    name: FontPickerCtrlStyle,
    doc: "Style flags for FontPickerCtrl widgets.",
    variants: {
        Default: ffi::WXD_FNTP_DEFAULT_STYLE, "Default style, includes `UseTextCtrl`.",
        UseTextCtrl: ffi::WXD_FNTP_USE_TEXTCTRL, "Use a text control to display the font description.",
        FontDescAsLabel: ffi::WXD_FNTP_FONTDESC_AS_LABEL, "Show the font description (e.g., \"Times New Roman Bold 10\") as the label.",
        UseFontForLabel: ffi::WXD_FNTP_USEFONT_FOR_LABEL, "Use the selected font itself to draw the label."
    },
    default_variant: Default
);

// --- FontPickerCtrl ---
#[derive(Clone)]
pub struct FontPickerCtrl {
    window: Window, // Embed Window
}

impl FontPickerCtrl {
    /// Creates a new FontPickerCtrlBuilder.
    pub fn builder(parent: &dyn WxWidget) -> FontPickerCtrlBuilder {
        FontPickerCtrlBuilder::new(parent)
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
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_FontPickerCtrl_t) -> Self {
        FontPickerCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }
}

// Use the widget_builder macro to generate the FontPickerCtrlBuilder implementation
widget_builder!(
    name: FontPickerCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: FontPickerCtrlStyle,
    fields: {
        initial_font: Option<Font> = None
    },
    build_impl: |slf| {
        assert!(!slf.parent.handle_ptr().is_null(), "FontPickerCtrl requires a parent");

        let initial_font_ptr = slf
            .initial_font
            .as_ref()
            .map_or(std::ptr::null(), |f| f.as_ptr());

        let ptr = unsafe {
            ffi::wxd_FontPickerCtrl_Create(
                slf.parent.handle_ptr(),
                slf.id,
                initial_font_ptr,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as c_longlong,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create FontPickerCtrl: FFI returned null pointer.");
        } else {
            unsafe { FontPickerCtrl::from_ptr(ptr) }
        }
    }
);

// Use the implement_widget_traits_with_target macro to implement traits
implement_widget_traits_with_target!(FontPickerCtrl, window, Window);
