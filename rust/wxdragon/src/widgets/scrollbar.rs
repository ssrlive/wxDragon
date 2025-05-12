//!
//! Safe wrapper for wxScrollBar.
//!

use crate::geometry::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE};
use crate::event::WxEvtHandler;
use crate::window::{Window, WxWidget};
use crate::id::ID_ANY;
use std::default::Default;
use std::marker::PhantomData;
use std::ops::{BitOr, BitOrAssign};
use std::ops::{Deref, DerefMut};
use std::os::raw::c_int;
use wxdragon_sys as ffi;

/// Represents a wxScrollBar widget.
#[derive(Clone)]
pub struct ScrollBar {
    window: Window,
}

impl ScrollBar {
    /// Creates a new ScrollBar builder.
    pub fn builder<W: WxWidget>(parent: &W) -> ScrollBarBuilder<W> {
        let mut builder = ScrollBarBuilder::default();
        builder.parent_ptr = parent.handle_ptr();
        builder.parent_type = PhantomData;
        builder
    }

    /// Creates a ScrollBar from a raw pointer.
    /// # Safety
    /// The caller must ensure the pointer is valid and relates to a wxScrollBar.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_ScrollBar_t) -> Self {
        ScrollBar {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Sets the scrollbar properties.
    pub fn set_scrollbar(
        &self,
        position: i32,
        thumb_size: i32,
        range: i32,
        page_size: i32,
        refresh: bool,
    ) {
        unsafe {
            ffi::wxd_ScrollBar_SetScrollbar(
                self.window.as_ptr() as *mut ffi::wxd_ScrollBar_t,
                position as c_int,
                thumb_size as c_int,
                range as c_int,
                page_size as c_int,
                refresh,
            );
        }
    }

    /// Gets the current position of the scrollbar thumb.
    pub fn thumb_position(&self) -> i32 {
        unsafe {
            ffi::wxd_ScrollBar_GetThumbPosition(self.window.as_ptr() as *mut ffi::wxd_ScrollBar_t)
        }
    }

    // TODO: Add GetThumbSize, GetPageSize, GetRange if needed via FFI calls.
}

impl WxWidget for ScrollBar {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

impl WxEvtHandler for ScrollBar {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.window.handle_ptr() as *mut ffi::wxd_EvtHandler_t
    }
}

impl Deref for ScrollBar {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl DerefMut for ScrollBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}

/// Builder for creating `ScrollBar` widgets.
#[derive(Clone)]
pub struct ScrollBarBuilder<'a, P: WxWidget + 'a> {
    parent_ptr: *mut ffi::wxd_Window_t,
    id: i32,
    pos: Point,
    size: Size,
    style: ScrollBarStyle,
    name: String,
    parent_type: PhantomData<&'a P>,
}

impl<'a, P: WxWidget> Default for ScrollBarBuilder<'a, P> {
    fn default() -> Self {
        ScrollBarBuilder {
            parent_ptr: std::ptr::null_mut(),
            id: ID_ANY as i32,
            pos: DEFAULT_POSITION,
            size: DEFAULT_SIZE,
            style: ScrollBarStyle::Default,
            name: String::from("scrollBar"),
            parent_type: PhantomData,
        }
    }
}

impl<'a, P: WxWidget> ScrollBarBuilder<'a, P> {
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

    pub fn with_style(mut self, style: ScrollBarStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn build(self) -> ScrollBar {
        unsafe {
            let c_name =
                std::ffi::CString::new(self.name.as_str()).expect("CString::new failed for name");
            // Call the FFI function assuming it exists (will be checked at link time)
            let ptr = ffi::wxd_ScrollBar_Create(
                self.parent_ptr,
                self.id,
                self.pos.into(),
                self.size.into(),
                self.style.bits() as ffi::wxd_Style_t,
                c_name.as_ptr(),
            );
            if ptr.is_null() {
                panic!("wxd_ScrollBar_Create returned null");
            }
            ScrollBar::from_ptr(ptr)
        }
    }
}

// --- ScrollBarStyle Enum ---

/// Style flags for `ScrollBar`.
///
/// These flags can be combined using the bitwise OR operator (`|`).
/// Note: ScrollBar styles are typically exclusive (either Horizontal or Vertical).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum ScrollBarStyle {
    /// Default style (horizontal).
    Default = ffi::WXD_SB_HORIZONTAL,
    /// Vertical scrollbar.
    Vertical = ffi::WXD_SB_VERTICAL,
}

impl ScrollBarStyle {
    /// Returns the raw integer value of the style.
    pub fn bits(self) -> i64 {
        self as i64
    }
}

// BitOr and BitOrAssign might not be strictly necessary if styles are exclusive,
// but added for consistency with other style enums.
impl BitOr for ScrollBarStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        // While typically exclusive, allow bitwise combination for potential future use
        // or if a combination makes sense in some wxWidgets port/version.
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for ScrollBarStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe {
            *self = std::mem::transmute(self.bits() | rhs.bits());
        }
    }
}
