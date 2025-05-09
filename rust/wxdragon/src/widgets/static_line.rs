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
use std::ops::{BitOr, BitOrAssign};

// wxStaticLine styles
// wxLI_HORIZONTAL is an alias for wxHORIZONTAL, wxLI_VERTICAL for wxVERTICAL.
// wxStaticLine defaults to wxLI_HORIZONTAL if no style is specified or style is 0.

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
    style: StaticLineStyle,
    name: String,
    parent_type: PhantomData<&'a P>,
}

impl<'a, P: WxWidget> Default for StaticLineBuilder<'a, P> {
    fn default() -> Self {
        StaticLineBuilder {
            parent_ptr: std::ptr::null_mut(), // Will be set by StaticLine::builder
            id: ID_ANY,
            pos: DEFAULT_POSITION,
            size: DEFAULT_SIZE, // wxStaticLine will size itself by default
            style: StaticLineStyle::Default,           // Defaults to LI_HORIZONTAL in wxWidgets if 0
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

    pub fn with_style(mut self, style: StaticLineStyle) -> Self {
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
                self.style.bits() as ffi::wxd_Style_t,
                c_name.as_ptr(),
            );
            if ptr.is_null() {
                panic!("wxd_StaticLine_Create returned null");
            }
            StaticLine::from_ptr(ptr)
        }
    }
}

// --- StaticLineStyle Enum ---

/// Style flags for `StaticLine`.
///
/// These flags are typically exclusive.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum StaticLineStyle {
    /// Default style (horizontal line).
    Default = ffi::WXD_HORIZONTAL,
    /// Vertical line.
    Vertical = ffi::WXD_VERTICAL,
}

impl StaticLineStyle {
    /// Returns the raw integer value of the style.
    pub fn bits(self) -> i64 {
        self as i64
    }
}

// BitOr and BitOrAssign are less relevant here as styles are exclusive,
// but included for consistency.
impl BitOr for StaticLineStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        // This combination might not make sense for StaticLine but allowed by type system.
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for StaticLineStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe { *self = std::mem::transmute(self.bits() | rhs.bits()); }
    }
}
