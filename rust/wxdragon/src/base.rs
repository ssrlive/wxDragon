//! Basic geometry types (Point, Size).

use wxdragon_sys as ffi;

// Use repr(C) to ensure memory layout compatibility with FFI types.
// Derive common traits.

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Creates a new Point.
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl From<Point> for ffi::wxd_Point {
    fn from(p: Point) -> Self {
        ffi::wxd_Point { x: p.x, y: p.y }
    }
}

impl From<ffi::wxd_Point> for Point {
    fn from(p: ffi::wxd_Point) -> Self {
        Point { x: p.x, y: p.y }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    /// Creates a new Size.
    pub fn new(width: i32, height: i32) -> Self {
        Size { width, height }
    }
}

impl From<Size> for ffi::wxd_Size {
    fn from(s: Size) -> Self {
        ffi::wxd_Size {
            width: s.width,
            height: s.height,
        }
    }
}

impl From<ffi::wxd_Size> for Size {
    fn from(s: ffi::wxd_Size) -> Self {
        Size {
            width: s.width,
            height: s.height,
        }
    }
}

/// Represents a rectangle with position (x, y) and dimensions (width, height).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

// FFI Conversion: Rect into wxdragon_sys::wxd_Rect
impl From<Rect> for wxdragon_sys::wxd_Rect {
    fn from(rect: Rect) -> Self {
        wxdragon_sys::wxd_Rect {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        }
    }
}

// Optional: wxdragon_sys::wxd_Rect into Rect (if needed elsewhere)
impl From<wxdragon_sys::wxd_Rect> for Rect {
    fn from(wxd_rect: wxdragon_sys::wxd_Rect) -> Self {
        Self {
            x: wxd_rect.x,
            y: wxd_rect.y,
            width: wxd_rect.width,
            height: wxd_rect.height,
        }
    }
}

// Common Constants

/// Matches wxID_ANY (-1)
pub const ID_ANY: i32 = -1;
/// Matches wxDefaultPosition (-1, -1)
pub const DEFAULT_POSITION: Point = Point { x: -1, y: -1 };
/// Matches wxDefaultSize (-1, -1)
pub const DEFAULT_SIZE: Size = Size {
    width: -1,
    height: -1,
};

// --- ADDED: RawWxProps Trait ---
/// Trait to get the raw FFI pointer of a specific widget type.
pub trait RawWxProps {
    type RawWxPtr; // Associated type for the specific FFI pointer (e.g., wxd_Button_t)
    fn raw_wx_ptr(&self) -> *mut Self::RawWxPtr;
}

// NEW: Colour struct for RGBA colours
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8, // Alpha component (0-255, 255 is opaque)
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Colour { r, g, b, a }
    }

    // Create from a u32 in RGBA order (e.g., 0xRRGGBBAA)
    pub fn from_u32(val: u32) -> Self {
        Colour {
            r: ((val >> 24) & 0xFF) as u8,
            g: ((val >> 16) & 0xFF) as u8,
            b: ((val >> 8) & 0xFF) as u8,
            a: (val & 0xFF) as u8,
        }
    }

    // Convert to u32 in RGBA order
    pub fn as_u32(&self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | (self.a as u32)
    }
}
