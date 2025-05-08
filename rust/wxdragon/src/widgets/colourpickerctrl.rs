//! Safe wrapper for wxColourPickerCtrl.

use crate::base::{Point, Size, DEFAULT_POSITION, ID_ANY};
use crate::defs::Style;
use crate::event::WxEvtHandler; // EventType and Event itself will be used via Event::get_colour
use crate::id::Id;
use crate::window::{Window, WxWidget};
// use std::ffi::CString; // Not strictly needed for ColourPickerCtrl itself unless styles have string names
use std::default::Default;
use wxdragon_sys as ffi; // Import Default

// --- Colour Struct ---
// Can be moved to base.rs or a new colour.rs if generally useful.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)] // Matches ffi::wxd_Colour_t layout
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8, // Alpha component (0-255, 255 is opaque)
}

impl Colour {
    /// Creates a new Colour with full opacity.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Colour { r, g, b, a: 255 }
    }

    /// Creates a new Colour with specified alpha.
    pub fn new_with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Colour { r, g, b, a }
    }
}

impl From<ffi::wxd_Colour_t> for Colour {
    fn from(c: ffi::wxd_Colour_t) -> Self {
        Colour {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
        }
    }
}

impl From<Colour> for ffi::wxd_Colour_t {
    fn from(c: Colour) -> Self {
        ffi::wxd_Colour_t {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
        }
    }
}

// Some common colours (optional, but can be handy)
pub mod colours {
    use super::Colour;
    pub const BLACK: Colour = Colour {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const WHITE: Colour = Colour {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const RED: Colour = Colour {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const GREEN: Colour = Colour {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const BLUE: Colour = Colour {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const YELLOW: Colour = Colour {
        r: 255,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const CYAN: Colour = Colour {
        r: 0,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const MAGENTA: Colour = Colour {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };
}

// --- ColourPickerCtrl Widget ---

// wxColourPickerCtrl specific style flags (from wx/clrpicker.h)
// Example: pub const CP_DEFAULT_STYLE: Style = 0;
// pub const CP_USE_ALPHA: Style = 0x0008; // wxCLRP_USE_ALPHA - if this is a defined style for creation
// Constants like WXD_CP_USE_ALPHA would be generated from const_extractor if available
// For now, assume standard window styles are sufficient or specific picker styles are passed as long.

#[derive(Clone)]
pub struct ColourPickerCtrl {
    window: Window,
}

impl ColourPickerCtrl {
    pub fn builder<'a>(parent: &'a dyn WxWidget) -> ColourPickerCtrlBuilder<'a> {
        ColourPickerCtrlBuilder::new(parent)
    }

    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_ColourPickerCtrl_t) -> Self {
        ColourPickerCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    pub fn get_colour(&self) -> Colour {
        let c_colour = unsafe { ffi::wxd_ColourPickerCtrl_GetColour(self.as_ptr()) };
        Colour::from(c_colour)
    }

    pub fn set_colour(&self, colour: Colour) {
        unsafe { ffi::wxd_ColourPickerCtrl_SetColour(self.as_ptr(), colour.into()) };
    }

    fn as_ptr(&self) -> *mut ffi::wxd_ColourPickerCtrl_t {
        self.window.as_ptr() as *mut _
    }
}

// --- Builder ---

pub struct ColourPickerCtrlBuilder<'a> {
    parent: &'a dyn WxWidget, // wxColourPickerCtrl usually requires a parent.
    id: Id,
    initial_colour: Colour,
    pos: Point,
    size: Size,
    style: Style,
}

impl<'a> ColourPickerCtrlBuilder<'a> {
    pub fn new(parent: &'a dyn WxWidget) -> Self {
        ColourPickerCtrlBuilder {
            parent,
            id: ID_ANY,
            initial_colour: colours::BLACK,
            pos: DEFAULT_POSITION,
            size: Size::new(80, -1),
            style: 0,
        }
    }

    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    pub fn with_initial_colour(mut self, colour: Colour) -> Self {
        self.initial_colour = colour;
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

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn build(self) -> ColourPickerCtrl {
        let parent_ptr = self.parent.handle_ptr();
        let ptr = unsafe {
            ffi::wxd_ColourPickerCtrl_Create(
                parent_ptr,
                self.id,
                self.initial_colour.into(),
                self.pos.into(),
                self.size.into(),
                self.style as ffi::wxd_Style_t,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create wxColourPickerCtrl");
        }
        unsafe { ColourPickerCtrl::from_ptr(ptr) }
    }
}

// --- Trait Implementations ---

impl WxWidget for ColourPickerCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.as_ptr()
    }
}

impl WxEvtHandler for ColourPickerCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.window.get_event_handler_ptr()
    }
}

// No explicit Drop needed for ColourPickerCtrl; Window handles destruction.
