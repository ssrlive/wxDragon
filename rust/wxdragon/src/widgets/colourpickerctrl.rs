use crate::color::{Colour, colours};
use crate::geometry::{Point, Size};
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::implement_widget_traits;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::default::Default;
use wxdragon_sys as ffi;

// --- ColourPickerCtrl Style Enum ---

widget_style_enum!(
    name: ColourPickerCtrlStyle,
    doc: "Style flags for the ColourPickerCtrl widget.",
    variants: {
        Default: ffi::WXD_CLRP_DEFAULT_STYLE, "Default style with no specific options.",
        UseTextCtrl: ffi::WXD_CLRP_USE_TEXTCTRL, "Creates a text control to the left of the picker button which can be used by the user to specify a colour.",
        ShowLabel: ffi::WXD_CLRP_SHOW_LABEL, "Shows the colour in HTML form (AABBCC) as colour button label.",
        ShowAlpha: ffi::WXD_CLRP_SHOW_ALPHA, "Allows selecting opacity in the colour-chooser (effective under wxGTK and wxOSX)."
    },
    default_variant: Default
);

// --- ColourPickerCtrl Widget ---

/// Represents a wxColourPickerCtrl, which allows the user to select a colour.
#[derive(Clone)]
pub struct ColourPickerCtrl {
    window: Window,
}

impl ColourPickerCtrl {
    /// Creates a new `ColourPickerCtrlBuilder` for constructing a colour picker control.
    pub fn builder(parent: &dyn WxWidget) -> ColourPickerCtrlBuilder {
        ColourPickerCtrlBuilder::new(parent)
    }

    /// Gets the currently selected colour.
    pub fn get_colour(&self) -> Colour {
        let c_colour = unsafe { ffi::wxd_ColourPickerCtrl_GetColour(self.window.as_ptr() as *mut _) };
        Colour::from(c_colour)
    }

    /// Sets the currently selected colour.
    pub fn set_colour(&self, colour: Colour) {
        unsafe { ffi::wxd_ColourPickerCtrl_SetColour(self.window.as_ptr() as *mut _, colour.into()) };
    }
}

widget_builder!(
    name: ColourPickerCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: ColourPickerCtrlStyle,
    fields: {
        initial_colour: Colour = colours::BLACK
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        let pos = slf.pos.into();
        let size = slf.size.into();
        let colour = slf.initial_colour.into();
        
        let ptr = unsafe {
            ffi::wxd_ColourPickerCtrl_Create(
                parent_ptr,
                slf.id,
                colour,
                pos,
                size,
                slf.style.bits(),
            )
        };
        
        if ptr.is_null() {
            panic!("Failed to create wxColourPickerCtrl");
        }
        
        ColourPickerCtrl {
            window: unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) },
        }
    }
);

implement_widget_traits!(ColourPickerCtrl, window); 