//!
//! Safe wrapper for wxStaticLine.
//!

use crate::base::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE, ID_ANY};
use crate::window::{Window, WxWidget};
use std::default::Default;
use std::ffi::CString;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use wxdragon_sys as ffi;

// wxStaticLine styles
// wxLI_HORIZONTAL is an alias for wxHORIZONTAL, wxLI_VERTICAL for wxVERTICAL.
// wxStaticLine defaults to wxLI_HORIZONTAL if no style is specified or style is 0.
pub const LI_HORIZONTAL: i64 = ffi::WXD_HORIZONTAL; // Use WXD_HORIZONTAL
pub const LI_VERTICAL: i64 = ffi::WXD_VERTICAL; // Use WXD_VERTICAL

/// Represents a wxStaticLine widget.
pub struct StaticLine {
    window: Window,
}

impl StaticLine {
    pub fn builder<W: WxWidget>(parent: &W) -> StaticLineBuilder<W> {
        let mut builder = StaticLineBuilder::default();
        builder.parent_ptr = parent.handle_ptr();
        builder.parent_type = PhantomData; // Initialize PhantomData
        builder
    }

    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_StaticLine_t) -> Self {
        StaticLine {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }
}

impl WxWidget for StaticLine {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

impl Deref for StaticLine {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl DerefMut for StaticLine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}

/// Builder for creating `StaticLine` widgets.
#[derive(Clone)]
pub struct StaticLineBuilder<'a, P: WxWidget + 'a> {
    parent_ptr: *mut ffi::wxd_Window_t,
    id: i32,
    pos: Point,
    size: Size,
    style: i64,
    name: String,
    parent_type: PhantomData<&'a P>, // To hold the lifetime and type of the parent
}

impl<'a, P: WxWidget> Default for StaticLineBuilder<'a, P> {
    fn default() -> Self {
        StaticLineBuilder {
            parent_ptr: std::ptr::null_mut(), // Will be set by StaticLine::builder
            id: ID_ANY,
            pos: DEFAULT_POSITION,
            size: DEFAULT_SIZE, // wxStaticLine will size itself by default
            style: 0,           // Defaults to LI_HORIZONTAL in wxWidgets if 0
            name: String::from("staticLine"), // Default name if not provided
            parent_type: PhantomData,
        }
    }
}

impl<'a, P: WxWidget> StaticLineBuilder<'a, P> {
    pub fn with_id(mut self, id: i32) -> Self {
        self.id = id;
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

    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn build(self) -> StaticLine {
        unsafe {
            let c_name = CString::new(self.name.as_str()).expect("CString::new failed for name");
            let ptr = ffi::wxd_StaticLine_Create(
                self.parent_ptr as *mut _,
                self.id,
                self.pos.into(),
                self.size.into(),
                self.style as ffi::wxd_Style_t,
                c_name.as_ptr(),
            );
            if ptr.is_null() {
                panic!("wxd_StaticLine_Create returned null");
            }
            StaticLine::from_ptr(ptr)
        }
    }
}
