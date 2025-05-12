//! Color definitions for wxDragon.
//!
//! This module provides the Colour struct for representing RGBA colors,
//! along with common color constants and conversion utilities.

use wxdragon_sys as ffi;

/// Represents an RGBA color.
///
/// A color with red, green, blue, and alpha components.
/// The alpha value ranges from 0 (transparent) to 255 (opaque).
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8, // Alpha component (0-255, 255 is opaque)
}

impl Colour {
    /// Creates a new color with the given RGBA values.
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Colour { r, g, b, a }
    }

    /// Creates a new color with full opacity.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Colour { r, g, b, a: 255 }
    }

    /// Create from a u32 in RGBA order (e.g., 0xRRGGBBAA)
    pub fn from_u32(val: u32) -> Self {
        Colour {
            r: ((val >> 24) & 0xFF) as u8,
            g: ((val >> 16) & 0xFF) as u8,
            b: ((val >> 8) & 0xFF) as u8,
            a: (val & 0xFF) as u8,
        }
    }

    /// Convert to u32 in RGBA order
    pub fn as_u32(&self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | (self.a as u32)
    }
    
    /// Returns a darker version of this color
    pub fn darker(&self, factor: f32) -> Self {
        let factor = factor.max(0.0).min(1.0);
        Colour {
            r: (self.r as f32 * factor) as u8,
            g: (self.g as f32 * factor) as u8,
            b: (self.b as f32 * factor) as u8,
            a: self.a,
        }
    }
    
    /// Returns a lighter version of this color
    pub fn lighter(&self, factor: f32) -> Self {
        let factor = factor.max(0.0).min(1.0);
        Colour {
            r: (self.r as f32 + (255.0 - self.r as f32) * factor) as u8,
            g: (self.g as f32 + (255.0 - self.g as f32) * factor) as u8,
            b: (self.b as f32 + (255.0 - self.b as f32) * factor) as u8,
            a: self.a,
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

/// Common color constants
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
    
    pub const TRANSPARENT: Colour = Colour {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    
    pub const GRAY: Colour = Colour {
        r: 128,
        g: 128,
        b: 128,
        a: 255,
    };
    
    pub const LIGHT_GRAY: Colour = Colour {
        r: 192,
        g: 192,
        b: 192,
        a: 255,
    };
    
    pub const DARK_GRAY: Colour = Colour {
        r: 64,
        g: 64,
        b: 64,
        a: 255,
    };
} 