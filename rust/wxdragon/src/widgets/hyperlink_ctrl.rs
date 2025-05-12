use std::ffi::{CStr, CString};
use std::os::raw::c_long;
use wxdragon_sys as ffi;

use crate::color::Colour;
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::geometry::{Point, Size};
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};

// --- Style enum using macro ---
widget_style_enum!(
    name: HyperlinkCtrlStyle,
    doc: "Style flags for HyperlinkCtrl.",
    variants: {
        Default: 0x0002, "Default style.",
        AlignLeft: 0x0004, "Align the text to the left (default).",
        AlignRight: 0x0008, "Align the text to the right.",
        AlignCentre: 0x0010, "Center the text.",
        NoUnderline: 0x0020, "Don't show the underline below the link."
    },
    default_variant: Default
);

// --- HyperlinkCtrl --- //
#[derive(Clone)]
pub struct HyperlinkCtrl {
    window: Window, // Embed the Window struct
}

impl HyperlinkCtrl {
    /// Creates a new HyperlinkCtrlBuilder.
    pub fn builder(parent: &dyn WxWidget) -> HyperlinkCtrlBuilder {
        HyperlinkCtrlBuilder::new(parent)
    }

    /// Gets the URL associated with the hyperlink.
    pub fn get_url(&self) -> String {
        unsafe {
            let c_str_ptr = ffi::wxd_HyperlinkCtrl_GetURL(
                self.window.as_ptr() as *mut ffi::wxd_HyperlinkCtrl_t
            );
            if c_str_ptr.is_null() {
                String::new()
            } else {
                CStr::from_ptr(c_str_ptr).to_string_lossy().into_owned()
            }
        }
    }

    /// Sets the URL associated with the hyperlink.
    pub fn set_url(&self, url: &str) {
        let c_url = CString::new(url).expect("CString::new failed for url");
        unsafe { 
            ffi::wxd_HyperlinkCtrl_SetURL(
                self.window.as_ptr() as *mut ffi::wxd_HyperlinkCtrl_t,
                c_url.as_ptr()
            ) 
        }
    }

    /// Returns whether the hyperlink has been visited.
    pub fn get_visited(&self) -> bool {
        unsafe { 
            ffi::wxd_HyperlinkCtrl_GetVisited(
                self.window.as_ptr() as *mut ffi::wxd_HyperlinkCtrl_t
            ) 
        }
    }

    /// Sets whether the hyperlink has been visited.
    pub fn set_visited(&self, visited: bool) {
        unsafe { 
            ffi::wxd_HyperlinkCtrl_SetVisited(
                self.window.as_ptr() as *mut ffi::wxd_HyperlinkCtrl_t,
                visited
            ) 
        }
    }

    /// Gets the colour used when the mouse hovers over the hyperlink.
    pub fn get_hover_colour(&self) -> Colour {
        let val = unsafe { 
            ffi::wxd_HyperlinkCtrl_GetHoverColour(
                self.window.as_ptr() as *mut ffi::wxd_HyperlinkCtrl_t
            ) 
        };
        Colour::from_u32(val as u32)
    }

    /// Sets the colour used when the mouse hovers over the hyperlink.
    pub fn set_hover_colour(&self, colour: Colour) {
        unsafe {
            ffi::wxd_HyperlinkCtrl_SetHoverColour(
                self.window.as_ptr() as *mut ffi::wxd_HyperlinkCtrl_t,
                colour.as_u32() as std::os::raw::c_ulong,
            )
        }
    }

    /// Gets the normal colour of the hyperlink.
    pub fn get_normal_colour(&self) -> Colour {
        let val = unsafe { 
            ffi::wxd_HyperlinkCtrl_GetNormalColour(
                self.window.as_ptr() as *mut ffi::wxd_HyperlinkCtrl_t
            ) 
        };
        Colour::from_u32(val as u32)
    }

    /// Sets the normal colour of the hyperlink.
    pub fn set_normal_colour(&self, colour: Colour) {
        unsafe {
            ffi::wxd_HyperlinkCtrl_SetNormalColour(
                self.window.as_ptr() as *mut ffi::wxd_HyperlinkCtrl_t,
                colour.as_u32() as std::os::raw::c_ulong,
            )
        }
    }

    /// Gets the colour of the visited hyperlink.
    pub fn get_visited_colour(&self) -> Colour {
        let val = unsafe { 
            ffi::wxd_HyperlinkCtrl_GetVisitedColour(
                self.window.as_ptr() as *mut ffi::wxd_HyperlinkCtrl_t
            ) 
        };
        Colour::from_u32(val as u32)
    }

    /// Sets the colour of the visited hyperlink.
    pub fn set_visited_colour(&self, colour: Colour) {
        unsafe {
            ffi::wxd_HyperlinkCtrl_SetVisitedColour(
                self.window.as_ptr() as *mut ffi::wxd_HyperlinkCtrl_t,
                colour.as_u32() as std::os::raw::c_ulong,
            )
        }
    }

    /// Creates a HyperlinkCtrl from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_HyperlinkCtrl_t`.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_HyperlinkCtrl_t) -> Self {
        HyperlinkCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }
}

// Use the widget_builder macro to generate the HyperlinkCtrlBuilder implementation
widget_builder!(
    name: HyperlinkCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: HyperlinkCtrlStyle,
    fields: {
        label: String = String::new(),
        url: String = String::new()
    },
    build_impl: |slf| {
        let c_label = CString::new(&slf.label[..]).expect("CString::new failed for label");
        let c_url = CString::new(&slf.url[..]).expect("CString::new failed for url");
        let raw_ptr = unsafe {
            ffi::wxd_HyperlinkCtrl_Create(
                slf.parent.handle_ptr(),
                slf.id,
                c_label.as_ptr(),
                c_url.as_ptr(),
                slf.pos.x,
                slf.pos.y,
                slf.size.width,
                slf.size.height,
                slf.style.bits() as c_long,
            )
        };
        if raw_ptr.is_null() {
            panic!("Failed to create wxHyperlinkCtrl");
        }
        unsafe { HyperlinkCtrl::from_ptr(raw_ptr) }
    }
);

// Apply common trait implementations for HyperlinkCtrl
implement_widget_traits_with_target!(HyperlinkCtrl, window, Window);
