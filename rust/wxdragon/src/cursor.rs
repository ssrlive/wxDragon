use crate::geometry::Point;
use crate::bitmap::Bitmap;
use std::ffi::CString;
use wxdragon_sys as ffi;

/// Stock cursor types available in wxWidgets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum StockCursor {
    None = ffi::wxd_StockCursor_WXD_CURSOR_NONE,
    Arrow = ffi::wxd_StockCursor_WXD_CURSOR_ARROW,
    RightArrow = ffi::wxd_StockCursor_WXD_CURSOR_RIGHT_ARROW,
    Bullseye = ffi::wxd_StockCursor_WXD_CURSOR_BULLSEYE,
    Char = ffi::wxd_StockCursor_WXD_CURSOR_CHAR,
    Cross = ffi::wxd_StockCursor_WXD_CURSOR_CROSS,
    Hand = ffi::wxd_StockCursor_WXD_CURSOR_HAND,
    IBeam = ffi::wxd_StockCursor_WXD_CURSOR_IBEAM,
    LeftButton = ffi::wxd_StockCursor_WXD_CURSOR_LEFT_BUTTON,
    Magnifier = ffi::wxd_StockCursor_WXD_CURSOR_MAGNIFIER,
    MiddleButton = ffi::wxd_StockCursor_WXD_CURSOR_MIDDLE_BUTTON,
    NoEntry = ffi::wxd_StockCursor_WXD_CURSOR_NO_ENTRY,
    PaintBrush = ffi::wxd_StockCursor_WXD_CURSOR_PAINT_BRUSH,
    Pencil = ffi::wxd_StockCursor_WXD_CURSOR_PENCIL,
    PointLeft = ffi::wxd_StockCursor_WXD_CURSOR_POINT_LEFT,
    PointRight = ffi::wxd_StockCursor_WXD_CURSOR_POINT_RIGHT,
    QuestionArrow = ffi::wxd_StockCursor_WXD_CURSOR_QUESTION_ARROW,
    RightButton = ffi::wxd_StockCursor_WXD_CURSOR_RIGHT_BUTTON,
    SizeNESW = ffi::wxd_StockCursor_WXD_CURSOR_SIZENESW,
    SizeNS = ffi::wxd_StockCursor_WXD_CURSOR_SIZENS,
    SizeNWSE = ffi::wxd_StockCursor_WXD_CURSOR_SIZENWSE,
    SizeWE = ffi::wxd_StockCursor_WXD_CURSOR_SIZEWE,
    Sizing = ffi::wxd_StockCursor_WXD_CURSOR_SIZING,
    SprayCan = ffi::wxd_StockCursor_WXD_CURSOR_SPRAYCAN,
    Wait = ffi::wxd_StockCursor_WXD_CURSOR_WAIT,
    Watch = ffi::wxd_StockCursor_WXD_CURSOR_WATCH,
    Blank = ffi::wxd_StockCursor_WXD_CURSOR_BLANK,
    Default = ffi::wxd_StockCursor_WXD_CURSOR_DEFAULT,
    ArrowWait = ffi::wxd_StockCursor_WXD_CURSOR_ARROWWAIT,
}

/// Bitmap file types supported for cursor creation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum BitmapType {
    Invalid = ffi::wxd_BitmapType_WXD_BITMAP_TYPE_INVALID,
    Bmp = ffi::wxd_BitmapType_WXD_BITMAP_TYPE_BMP,
    Ico = ffi::wxd_BitmapType_WXD_BITMAP_TYPE_ICO,
    Cur = ffi::wxd_BitmapType_WXD_BITMAP_TYPE_CUR,
    Xbm = ffi::wxd_BitmapType_WXD_BITMAP_TYPE_XBM,
    Xpm = ffi::wxd_BitmapType_WXD_BITMAP_TYPE_XPM,
    Png = ffi::wxd_BitmapType_WXD_BITMAP_TYPE_PNG,
    Jpeg = ffi::wxd_BitmapType_WXD_BITMAP_TYPE_JPEG,
    Gif = ffi::wxd_BitmapType_WXD_BITMAP_TYPE_GIF,
    Ani = ffi::wxd_BitmapType_WXD_BITMAP_TYPE_ANI,
    Any = ffi::wxd_BitmapType_WXD_BITMAP_TYPE_ANY,
}

/// Represents a cursor object.
/// 
/// A cursor is a small bitmap usually used for denoting where the mouse pointer is,
/// with a picture that might indicate the interpretation of a mouse click.
/// 
/// # Examples
/// 
/// ```rust
/// use wxdragon::prelude::*;
/// 
/// // Create a stock cursor
/// let hand_cursor = Cursor::from_stock(StockCursor::Hand);
/// 
/// // Create a cursor from a file
/// let custom_cursor = Cursor::from_file("my_cursor.cur", BitmapType::Cur, 8, 8);
/// 
/// // Use the cursor on a window
/// window.set_cursor(Some(&hand_cursor));
/// ```
#[derive(Debug)]
pub struct Cursor(pub(crate) *mut ffi::wxd_Cursor_t);

impl Cursor {
    /// Creates a cursor from a stock cursor type.
    /// 
    /// # Arguments
    /// * `cursor_type` - The stock cursor type to create
    /// 
    /// # Returns
    /// A new `Cursor` instance, or `None` if creation failed
    /// 
    /// # Examples
    /// ```rust
    /// let cursor = Cursor::from_stock(StockCursor::Hand);
    /// let wait_cursor = Cursor::from_stock(StockCursor::Wait);
    /// ```
    pub fn from_stock(cursor_type: StockCursor) -> Option<Self> {
        let ptr = unsafe { ffi::wxd_Cursor_CreateStock(cursor_type as u32) };
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }
    
    /// Creates a cursor from a file.
    /// 
    /// # Arguments
    /// * `filename` - Path to the cursor file
    /// * `bitmap_type` - The type of bitmap file
    /// * `hotspot_x` - X coordinate of the cursor hotspot
    /// * `hotspot_y` - Y coordinate of the cursor hotspot
    /// 
    /// # Returns
    /// A new `Cursor` instance, or `None` if creation failed
    /// 
    /// # Examples
    /// ```rust
    /// let cursor = Cursor::from_file("cursor.cur", BitmapType::Cur, 8, 8);
    /// let png_cursor = Cursor::from_file("cursor.png", BitmapType::Png, 16, 16);
    /// ```
    pub fn from_file(filename: &str, bitmap_type: BitmapType, hotspot_x: i32, hotspot_y: i32) -> Option<Self> {
        let c_filename = CString::new(filename).ok()?;
        let ptr = unsafe {
            ffi::wxd_Cursor_CreateFromFile(
                c_filename.as_ptr(),
                bitmap_type as u32,
                hotspot_x,
                hotspot_y,
            )
        };
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }
    
    /// Creates a cursor from raw bitmap data.
    /// 
    /// # Arguments
    /// * `bits` - Raw bitmap data (1 bit per pixel)
    /// * `width` - Width of the cursor in pixels
    /// * `height` - Height of the cursor in pixels
    /// * `hotspot_x` - X coordinate of the cursor hotspot
    /// * `hotspot_y` - Y coordinate of the cursor hotspot
    /// * `mask_bits` - Optional mask data (1 bit per pixel for transparency)
    /// 
    /// # Returns
    /// A new `Cursor` instance, or `None` if creation failed
    /// 
    /// # Safety
    /// The `bits` slice must contain at least `(width * height + 7) / 8` bytes.
    /// If `mask_bits` is provided, it must also contain the same amount of data.
    pub fn from_data(
        bits: &[u8],
        width: i32,
        height: i32,
        hotspot_x: i32,
        hotspot_y: i32,
        mask_bits: Option<&[u8]>,
    ) -> Option<Self> {
        if width <= 0 || height <= 0 {
            return None;
        }
        
        let expected_size = ((width * height + 7) / 8) as usize;
        if bits.len() < expected_size {
            return None;
        }
        
        if let Some(mask) = mask_bits {
            if mask.len() < expected_size {
                return None;
            }
        }
        
        let mask_ptr = mask_bits.map(|m| m.as_ptr()).unwrap_or(std::ptr::null());
        let ptr = unsafe {
            ffi::wxd_Cursor_CreateFromData(
                bits.as_ptr(),
                width,
                height,
                hotspot_x,
                hotspot_y,
                mask_ptr,
            )
        };
        
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }
    
    /// Creates a cursor from a bitmap/image.
    /// 
    /// # Arguments
    /// * `bitmap` - The bitmap to convert to a cursor
    /// 
    /// # Returns
    /// A new `Cursor` instance, or `None` if creation failed
    pub fn from_bitmap(bitmap: &Bitmap) -> Option<Self> {
        let ptr = unsafe { ffi::wxd_Cursor_CreateFromImage(bitmap.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }
    
    /// Creates a copy of this cursor.
    /// 
    /// # Returns
    /// A new `Cursor` instance that is a copy of this one, or `None` if copying failed
    pub fn copy(&self) -> Option<Self> {
        let ptr = unsafe { ffi::wxd_Cursor_Copy(self.0) };
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }
    
    /// Returns true if the cursor is valid and can be used.
    /// 
    /// # Returns
    /// `true` if the cursor is valid, `false` otherwise
    pub fn is_ok(&self) -> bool {
        unsafe { ffi::wxd_Cursor_IsOk(self.0) }
    }
    
    /// Gets the hotspot coordinates of the cursor.
    /// 
    /// The hotspot is the point at which the mouse is actually considered to be
    /// when this cursor is used.
    /// 
    /// # Returns
    /// A `Point` containing the hotspot coordinates, or (-1, -1) if not available
    pub fn get_hotspot(&self) -> Point {
        let point = unsafe { ffi::wxd_Cursor_GetHotSpot(self.0) };
        Point::new(point.x, point.y)
    }
    
    /// Gets the native handle of the cursor (platform-specific).
    /// 
    /// # Returns
    /// A raw pointer to the native cursor handle, or null if not available
    /// 
    /// # Safety
    /// The returned pointer should not be used to modify the cursor and may
    /// only be valid for the lifetime of this `Cursor` instance.
    pub unsafe fn get_handle(&self) -> *mut std::ffi::c_void {
        ffi::wxd_Cursor_GetHandle(self.0)
    }
    
    /// Sets the native handle of the cursor (platform-specific).
    /// 
    /// # Arguments
    /// * `handle` - Raw pointer to the native cursor handle
    /// 
    /// # Safety
    /// The caller must ensure the handle is valid and compatible with the current platform.
    /// This function is only supported on Windows.
    pub unsafe fn set_handle(&self, handle: *mut std::ffi::c_void) {
        ffi::wxd_Cursor_SetHandle(self.0, handle);
    }
    
    /// Returns the raw underlying pointer.
    /// 
    /// # Safety
    /// This is intended for internal use by wxDragon and should not be used directly.
    pub(crate) fn as_ptr(&self) -> *mut ffi::wxd_Cursor_t {
        self.0
    }
    
    /// Creates a new Cursor wrapper from a raw pointer.
    /// 
    /// # Safety
    /// The caller must ensure the pointer is valid and manages its lifetime correctly.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_Cursor_t) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }
    
    /// Checks if the underlying pointer is null.
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

impl Drop for Cursor {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                ffi::wxd_Cursor_Destroy(self.0);
            }
        }
    }
}

// Make Cursor Send and Sync for multi-threading support
unsafe impl Send for Cursor {}
unsafe impl Sync for Cursor {}

// Global cursor functions

/// Sets the global cursor for the application.
/// 
/// # Arguments
/// * `cursor` - The cursor to set globally, or `None` to reset to default
/// 
/// # Examples
/// ```rust
/// let wait_cursor = Cursor::from_stock(StockCursor::Wait);
/// set_cursor(Some(&wait_cursor));
/// 
/// // Reset to default
/// set_cursor(None);
/// ```
pub fn set_cursor(cursor: Option<&Cursor>) {
    let cursor_ptr = cursor.map(|c| c.as_ptr()).unwrap_or(std::ptr::null_mut());
    unsafe {
        ffi::wxd_SetCursor(cursor_ptr);
    }
}

/// Begins a busy cursor state.
/// 
/// This function sets a wait/busy cursor globally and maintains a stack of busy states.
/// You must call [`end_busy_cursor`] to restore the previous cursor.
/// 
/// # Arguments
/// * `cursor` - Optional custom busy cursor. If `None`, uses the default wait cursor
/// 
/// # Examples
/// ```rust
/// // Use default wait cursor
/// begin_busy_cursor(None);
/// 
/// // Use custom busy cursor
/// let custom_wait = Cursor::from_stock(StockCursor::Watch);
/// begin_busy_cursor(Some(&custom_wait));
/// 
/// // Don't forget to end the busy state
/// end_busy_cursor();
/// ```
pub fn begin_busy_cursor(cursor: Option<&Cursor>) {
    let cursor_ptr = cursor.map(|c| c.as_ptr()).unwrap_or(std::ptr::null_mut());
    unsafe {
        ffi::wxd_BeginBusyCursor(cursor_ptr);
    }
}

/// Ends the current busy cursor state.
/// 
/// This function restores the cursor that was active before the last call to [`begin_busy_cursor`].
/// 
/// # Examples
/// ```rust
/// begin_busy_cursor(None);
/// // ... do some work ...
/// end_busy_cursor();
/// ```
pub fn end_busy_cursor() {
    unsafe {
        ffi::wxd_EndBusyCursor();
    }
}

/// Returns true if a busy cursor is currently active.
/// 
/// # Returns
/// `true` if a busy cursor is currently being displayed, `false` otherwise
/// 
/// # Examples
/// ```rust
/// assert!(!is_busy());
/// begin_busy_cursor(None);
/// assert!(is_busy());
/// end_busy_cursor();
/// assert!(!is_busy());
/// ```
pub fn is_busy() -> bool {
    unsafe { ffi::wxd_IsBusy() }
}

/// Helper struct for automatic busy cursor management.
/// 
/// This struct automatically begins a busy cursor when created and ends it when dropped,
/// ensuring proper cleanup even if an error occurs.
/// 
/// # Examples
/// ```rust
/// {
///     let _busy = BusyCursor::new(None);
///     // Busy cursor is active here
///     // ... do some work ...
/// } // Busy cursor is automatically restored here
/// ```
pub struct BusyCursor {
    _marker: (),
}

impl BusyCursor {
    /// Creates a new busy cursor state.
    /// 
    /// # Arguments
    /// * `cursor` - Optional custom busy cursor. If `None`, uses the default wait cursor
    /// 
    /// # Returns
    /// A `BusyCursor` instance that will automatically restore the cursor when dropped
    pub fn new(cursor: Option<&Cursor>) -> Self {
        begin_busy_cursor(cursor);
        Self { _marker: () }
    }
}

impl Drop for BusyCursor {
    fn drop(&mut self) {
        end_busy_cursor();
    }
} 