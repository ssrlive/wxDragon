/// Background mode constants for device contexts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundMode {
    /// Transparent background mode
    Transparent,
    /// Solid background mode
    Solid,
}

impl BackgroundMode {
    /// Convert to the raw FFI value
    pub fn to_raw(&self) -> i32 {
        match self {
            BackgroundMode::Transparent => wxdragon_sys::WXD_TRANSPARENT as i32,
            BackgroundMode::Solid => wxdragon_sys::WXD_SOLID as i32,
        }
    }
}

widget_style_enum!(
    name: PenStyle,
    doc: "Style flags for DC pen.",
    variants: {
        Solid: wxdragon_sys::WXD_PENSTYLE_SOLID as i64, "Solid line style.",
        Dot: wxdragon_sys::WXD_PENSTYLE_DOT as i64, "Dotted line style.",
        LongDash: wxdragon_sys::WXD_PENSTYLE_LONG_DASH as i64, "Long dashed line style.",
        ShortDash: wxdragon_sys::WXD_PENSTYLE_SHORT_DASH as i64, "Short dashed line style.",
        DotDash: wxdragon_sys::WXD_PENSTYLE_DOT_DASH as i64, "Dot and dash line style.",
        Transparent: wxdragon_sys::WXD_PENSTYLE_TRANSPARENT as i64, "Transparent pen.",
        Stipple: wxdragon_sys::WXD_PENSTYLE_STIPPLE as i64, "Stippled pen.",
        UserDash: wxdragon_sys::WXD_PENSTYLE_USER_DASH as i64, "User-defined dash pattern.",
        BDiagonalHatch: wxdragon_sys::WXD_PENSTYLE_BDIAGONAL_HATCH as i64, "Backward diagonal hatch pattern.",
        CrossDiagHatch: wxdragon_sys::WXD_PENSTYLE_CROSSDIAG_HATCH as i64, "Cross-diagonal hatch pattern.",
        FDiagonalHatch: wxdragon_sys::WXD_PENSTYLE_FDIAGONAL_HATCH as i64, "Forward diagonal hatch pattern.",
        CrossHatch: wxdragon_sys::WXD_PENSTYLE_CROSS_HATCH as i64, "Cross hatch pattern.",
        HorizontalHatch: wxdragon_sys::WXD_PENSTYLE_HORIZONTAL_HATCH as i64, "Horizontal hatch pattern.",
        VerticalHatch: wxdragon_sys::WXD_PENSTYLE_VERTICAL_HATCH as i64, "Vertical hatch pattern."
    },
    default_variant: Solid
);

widget_style_enum!(
    name: BrushStyle,
    doc: "Style flags for DC brush.",
    variants: {
        Solid: wxdragon_sys::WXD_BRUSHSTYLE_SOLID as i64, "Solid brush.",
        Transparent: wxdragon_sys::WXD_BRUSHSTYLE_TRANSPARENT as i64, "Transparent brush.",
        BDiagonalHatch: wxdragon_sys::WXD_BRUSHSTYLE_BDIAGONAL_HATCH as i64, "Backward diagonal hatch pattern.",
        CrossDiagHatch: wxdragon_sys::WXD_BRUSHSTYLE_CROSSDIAG_HATCH as i64, "Cross-diagonal hatch pattern.",
        FDiagonalHatch: wxdragon_sys::WXD_BRUSHSTYLE_FDIAGONAL_HATCH as i64, "Forward diagonal hatch pattern.",
        CrossHatch: wxdragon_sys::WXD_BRUSHSTYLE_CROSS_HATCH as i64, "Cross hatch pattern.",
        HorizontalHatch: wxdragon_sys::WXD_BRUSHSTYLE_HORIZONTAL_HATCH as i64, "Horizontal hatch pattern.",
        VerticalHatch: wxdragon_sys::WXD_BRUSHSTYLE_VERTICAL_HATCH as i64, "Vertical hatch pattern.",
        Stipple: wxdragon_sys::WXD_BRUSHSTYLE_STIPPLE as i64, "Stippled brush.",
        StippleMaskOpaque: wxdragon_sys::WXD_BRUSHSTYLE_STIPPLE_MASK_OPAQUE as i64, "Stippled brush with opaque mask.",
        StippleMask: wxdragon_sys::WXD_BRUSHSTYLE_STIPPLE_MASK as i64, "Stippled brush with mask."
    },
    default_variant: Solid
);

pub mod client_dc;
pub mod paint_dc;
pub mod window_dc;
pub mod memory_dc;
pub mod screen_dc;

pub use client_dc::ClientDC;
pub use paint_dc::PaintDC;
pub use window_dc::WindowDC;
pub use memory_dc::MemoryDC;
pub use screen_dc::ScreenDC;

// Re-export for convenience
pub use crate::bitmap::Bitmap;
pub use crate::font::Font;
pub use crate::color::Colour;

/// Common trait implemented by all device context types
pub trait DeviceContext {
    /// Get a pointer to the underlying DC
    fn dc_ptr(&self) -> *mut wxdragon_sys::wxd_DC_t;

    /// Clear the device context
    fn clear(&self) {
        unsafe {
            wxdragon_sys::wxd_DC_Clear(self.dc_ptr());
        }
    }

    /// Set the background color of the device context
    fn set_background(&self, colour: Colour) {
        unsafe {
            wxdragon_sys::wxd_DC_SetBackground(self.dc_ptr(), colour.into());
        }
    }

    /// Set the background mode of the device context
    fn set_background_mode(&self, mode: BackgroundMode) {
        unsafe {
            wxdragon_sys::wxd_DC_SetBackgroundMode(self.dc_ptr(), mode.to_raw());
        }
    }

    /// Set the text background color
    fn set_text_background(&self, colour: Colour) {
        unsafe {
            wxdragon_sys::wxd_DC_SetTextBackground(self.dc_ptr(), colour.into());
        }
    }

    /// Set the text foreground color
    fn set_text_foreground(&self, colour: Colour) {
        unsafe {
            wxdragon_sys::wxd_DC_SetTextForeground(self.dc_ptr(), colour.into());
        }
    }

    /// Set the font for text drawing
    fn set_font(&self, font: &Font) {
        unsafe {
            wxdragon_sys::wxd_DC_SetFont(self.dc_ptr(), font.as_ptr());
        }
    }

    /// Set the pen for drawing outlines
    fn set_pen(&self, colour: Colour, width: i32, style: PenStyle) {
        unsafe {
            wxdragon_sys::wxd_DC_SetPen(self.dc_ptr(), colour.into(), width, style.bits() as i32);
        }
    }

    /// Set the brush for filling shapes
    fn set_brush(&self, colour: Colour, style: BrushStyle) {
        unsafe {
            wxdragon_sys::wxd_DC_SetBrush(self.dc_ptr(), colour.into(), style.bits() as i32);
        }
    }

    /// Draw a point at the specified coordinates
    fn draw_point(&self, x: i32, y: i32) {
        unsafe {
            wxdragon_sys::wxd_DC_DrawPoint(self.dc_ptr(), x, y);
        }
    }

    /// Draw a line from (x1, y1) to (x2, y2)
    fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        unsafe {
            wxdragon_sys::wxd_DC_DrawLine(self.dc_ptr(), x1, y1, x2, y2);
        }
    }

    /// Draw a rectangle with the specified dimensions
    fn draw_rectangle(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            wxdragon_sys::wxd_DC_DrawRectangle(self.dc_ptr(), x, y, width, height);
        }
    }

    /// Draw a circle with the specified center and radius
    fn draw_circle(&self, x: i32, y: i32, radius: i32) {
        unsafe {
            wxdragon_sys::wxd_DC_DrawCircle(self.dc_ptr(), x, y, radius);
        }
    }

    /// Draw an ellipse inside the specified rectangle
    fn draw_ellipse(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            wxdragon_sys::wxd_DC_DrawEllipse(self.dc_ptr(), x, y, width, height);
        }
    }

    /// Draw text at the specified position
    fn draw_text(&self, text: &str, x: i32, y: i32) {
        use std::ffi::CString;
        if let Ok(c_text) = CString::new(text) {
            unsafe {
                wxdragon_sys::wxd_DC_DrawText(self.dc_ptr(), c_text.as_ptr(), x, y);
            }
        }
    }

    /// Draw a bitmap at the specified position
    fn draw_bitmap(&self, bitmap: &Bitmap, x: i32, y: i32, transparent: bool) {
        unsafe {
            wxdragon_sys::wxd_DC_DrawBitmap(self.dc_ptr(), bitmap.as_ptr(), x, y, transparent);
        }
    }

    /// Get the size of the device context
    fn get_size(&self) -> (i32, i32) {
        unsafe {
            let size = wxdragon_sys::wxd_DC_GetSize(self.dc_ptr());
            (size.width, size.height)
        }
    }

    /// Get the text extent (width and height) for the specified string
    fn get_text_extent(&self, text: &str) -> (i32, i32) {
        use std::ffi::CString;
        if let Ok(c_text) = CString::new(text) {
            let mut width = 0;
            let mut height = 0;
            unsafe {
                wxdragon_sys::wxd_DC_GetTextExtent(self.dc_ptr(), c_text.as_ptr(), &mut width, &mut height);
            }
            (width, height)
        } else {
            (0, 0)
        }
    }
} 