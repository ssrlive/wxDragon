use crate::base::{Point, Size, DEFAULT_POSITION, ID_ANY};
use crate::id::Id;
// use crate::app::WxdApp; // Assuming App is the entry point
// REMOVED: use crate::event::{EventType, WxEvtHandler, Event}; // Keep needed event imports
use crate::menus::MenuBar; // ADDED: Import MenuBar
use crate::widgets::statusbar::StatusBar; // ADDED Import
use crate::widgets::toolbar::{ToolBar, ToolBarStyle}; // Added ToolBarStyle
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use std::ptr;
use wxdragon_sys as ffi;
// REMOVED: use std::ops::Deref;
use std::default::Default;
use std::marker::PhantomData;
use std::ops::{BitOr, BitOrAssign};
use std::os::raw::c_int; // Import c_long and c_int // ADDED for enum bitwise operations

/// Represents a wxFrame.
#[derive(Clone)]
pub struct Frame {
    window: Window, // Composition: Frame uses a Window internally
    // Store parent pointer to manage drop behavior
    parent_ptr: *mut ffi::wxd_Window_t,
    _marker: PhantomData<()>,
}

// --- Frame Builder ---

/// Builder pattern for creating `Frame` widgets.
// Cannot derive Default because of raw pointer field `parent_ptr`
pub struct FrameBuilder {
    // Fields store values directly, initialized by Default
    parent_ptr: *mut ffi::wxd_Window_t, // Optional parent, defaults to null
    id: Id,
    title: String,
    pos: Point,
    size: Size,
    style: FrameStyle,
    // name: String, // Removed name for now
}

// Manual implementation of Default
impl Default for FrameBuilder {
    fn default() -> Self {
        Self {
            parent_ptr: ptr::null_mut(),
            id: ID_ANY,                          // Use ID_ANY from base (already i32)
            title: "wxDragon Frame".to_string(), // Default title
            pos: DEFAULT_POSITION,               // Default position
            size: Size {
                width: 500,
                height: 400,
            }, // Specific default size for Frame
            style: FrameStyle::DEFAULT,
            // name: String::new(),
        }
    }
}

impl FrameBuilder {
    /// Sets the optional parent window.
    pub fn with_parent(mut self, parent: &impl WxWidget) -> Self {
        self.parent_ptr = parent.handle_ptr();
        self
    }

    /// Sets the window identifier.
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    /// Sets the frame title.
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Sets the position.
    pub fn with_position(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    /// Sets the size.
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Sets the window style flags.
    pub fn with_style(mut self, style: FrameStyle) -> Self {
        self.style = style;
        self
    }

    /// Builds the `Frame`.
    ///
    /// # Panics
    /// Panics if frame creation fails in the underlying C++ layer.
    pub fn build(self) -> Frame {
        let c_title = CString::new(self.title).expect("CString::new failed for title");

        let ptr = unsafe {
            ffi::wxd_Frame_Create(
                self.parent_ptr,
                self.id,
                c_title.as_ptr(),
                self.pos.into(),
                self.size.into(),
                self.style.bits() as ffi::wxd_Style_t,
            )
        };

        if ptr.is_null() {
            panic!("Failed to create wxFrame: wxWidgets returned a null pointer.");
        } else {
            Frame {
                window: unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) },
                parent_ptr: self.parent_ptr,
                _marker: PhantomData,
            }
        }
    }
}

// --- Frame Implementation ---

impl Frame {
    /// Creates a new `FrameBuilder` for constructing a frame.
    pub fn builder() -> FrameBuilder {
        FrameBuilder::default()
    }

    /// Sets the frame's title.
    pub fn set_title(&self, title: &str) {
        let title_c = CString::new(title).expect("CString::new failed");
        unsafe {
            ffi::wxd_Frame_SetTitle(
                self.window.as_ptr() as *mut ffi::wxd_Frame_t,
                title_c.as_ptr(),
            );
        }
    }

    /// Centers the frame on the screen or parent. (wxWidgets `Centre` method)
    pub fn centre(&self) {
        unsafe {
            ffi::wxd_Frame_Centre(
                self.window.as_ptr() as *mut ffi::wxd_Frame_t,
                (ffi::WXD_ALIGN_CENTRE as i32).try_into().unwrap(),
            );
        }
    }

    /// Centers the frame on the screen. (wxWidgets `CenterOnScreen` method)
    pub fn center_on_screen(&self) {
        unsafe { ffi::wxd_Frame_CenterOnScreen(self.window.as_ptr() as *mut ffi::wxd_Frame_t) }
    }

    /// Shows the frame.
    pub fn show(&self, show: bool) {
        unsafe {
            ffi::wxd_Frame_Show(self.window.as_ptr() as *mut ffi::wxd_Frame_t, show);
        }
    }

    // Method to set the menu bar
    pub fn set_menu_bar(&self, menubar: MenuBar) {
        let menubar_ptr = unsafe { menubar.as_ptr() };
        // Forget wrapper as Frame takes ownership
        std::mem::forget(menubar);
        unsafe {
            ffi::wxd_Frame_SetMenuBar(self.window.as_ptr() as *mut ffi::wxd_Frame_t, menubar_ptr);
        }
    }

    // Method to close the frame
    pub fn close(&self) {
        unsafe {
            // false = don't force close, allow events like EVT_CLOSE_WINDOW
            ffi::wxd_Frame_Close(self.window.as_ptr() as *mut ffi::wxd_Frame_t, false);
        }
    }

    /// Sets the frame's status bar.
    pub fn set_existing_status_bar(&self, status_bar: Option<&StatusBar>) {
        let sb_ptr = status_bar.map_or(std::ptr::null_mut(), |sb| sb.as_ptr() as *mut _);
        unsafe {
            ffi::wxd_Frame_SetStatusBar(self.window.as_ptr() as *mut ffi::wxd_Frame_t, sb_ptr);
        }
    }

    /// Creates and assigns a toolbar to the frame.
    /// Returns `Some(ToolBar)` if successful, `None` otherwise.
    pub fn create_tool_bar(&self, style: Option<ToolBarStyle>, id: Id) -> Option<ToolBar> {
        let style_bits = style
            .map(|s| s.bits())
            .unwrap_or(ToolBarStyle::default().bits()); // Use ToolBarStyle default bits if None

        let tb_ptr = unsafe {
            ffi::wxd_Frame_CreateToolBar(
                self.window.as_ptr() as *mut _,
                style_bits as ffi::wxd_Style_t, // Use bits()
                id,
            )
        };
        if tb_ptr.is_null() {
            None
        } else {
            Some(unsafe { ToolBar::from_ptr(tb_ptr) })
        }
    }

    /// Creates a status bar for the frame.
    pub fn create_status_bar(&self, number: i32, style: i64, id: Id, name: &str) -> StatusBar {
        unsafe {
            let name_c = CString::new(name).unwrap_or_default();
            let statbar_ptr = ffi::wxd_Frame_CreateStatusBar(
                self.window.as_ptr() as *mut ffi::wxd_Frame_t,
                number as c_int,
                style as ffi::wxd_Style_t,
                id,
                name_c.as_ptr(),
            );
            StatusBar::from_ptr(statbar_ptr)
        }
    }

    // New safe wrapper methods for wxFrame
    pub fn set_status_text(&self, text: &str, number: i32) {
        let c_text = CString::new(text).expect("CString::new for status text failed");
        unsafe {
            ffi::wxd_Frame_SetStatusText(
                self.window.as_ptr() as *mut ffi::wxd_Frame_t,
                c_text.as_ptr(),
                number,
            )
        }
    }

    pub fn get_title(&self) -> String {
        unsafe {
            let c_title_ptr =
                ffi::wxd_Frame_GetTitle(self.window.as_ptr() as *mut ffi::wxd_Frame_t);
            if c_title_ptr.is_null() {
                return String::new(); // Should ideally not happen if C returns empty string for null frame
            }
            // CString::from_raw takes ownership and will free the memory.
            CString::from_raw(c_title_ptr)
                .into_string()
                .unwrap_or_else(|_| String::from("Error converting title"))
        }
    }

    pub fn iconize(&self, iconize: bool) {
        unsafe { ffi::wxd_Frame_Iconize(self.window.as_ptr() as *mut ffi::wxd_Frame_t, iconize) }
    }

    pub fn is_iconized(&self) -> bool {
        unsafe { ffi::wxd_Frame_IsIconized(self.window.as_ptr() as *mut ffi::wxd_Frame_t) }
    }

    pub fn maximize(&self, maximize: bool) {
        unsafe { ffi::wxd_Frame_Maximize(self.window.as_ptr() as *mut ffi::wxd_Frame_t, maximize) }
    }

    pub fn is_maximized(&self) -> bool {
        unsafe { ffi::wxd_Frame_IsMaximized(self.window.as_ptr() as *mut ffi::wxd_Frame_t) }
    }
}

// Implement WxWidget for Frame.
impl WxWidget for Frame {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.as_ptr()
    }
    // get_id is provided by Deref<Target=Window> -> Window::get_id
    // show, set_sizer, etc. are handled directly or via Deref
}

/// Implement Drop to manage *Rust* resources.
/// The underlying C++ window is managed by wxWidgets itself.
impl Drop for Frame {
    fn drop(&mut self) {
        // For top-level windows, wxWidgets handles destruction.
        // Our previous cleanup_handlers_recursive call is now unnecessary
        // because wxWidgets will destroy the client data (WxdBindingMapClientData)
        // which will trigger its destructor to drop Rust closures.
        // If this Frame instance represents a child window, its C++ object
        // will be deleted by its parent anyway.
        // Therefore, this Drop implementation might become empty unless
        // the Frame struct itself holds other Rust resources that need dropping.
        if self.parent_ptr.is_null() {
            // println!("Top-level Frame wrapper dropped (C++ cleanup by wxWidgets)");
        } else {
            // println!("Child Frame wrapper dropped (C++ cleanup by parent)");
        }
    }
}

// Allow Frame to be used where a Window is expected (e.g., as a parent)
// This also provides WxEvtHandler implementation via Window.
impl std::ops::Deref for Frame {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

// --- FrameStyle Enum ---

/// Window style flags for `Frame`.
///
/// These flags can be combined using the bitwise OR operator (`|`).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum FrameStyle {
    /// Includes `wxCAPTION`, `wxRESIZE_BORDER`, `wxSYSTEM_MENU`, `wxMINIMIZE_BOX`, `wxMAXIMIZE_BOX`, `wxCLOSE_BOX`.
    /// This is the default style.
    Default = ffi::WXD_DEFAULT_FRAME_STYLE,
    /// Displays a title bar.
    Caption = ffi::WXD_CAPTION,
    /// Displays a resizeable border.
    ResizeBorder = ffi::WXD_RESIZE_BORDER,
    /// Displays a system menu.
    SystemMenu = ffi::WXD_SYSTEM_MENU,
    /// Displays a close box.
    CloseBox = ffi::WXD_CLOSE_BOX,
    /// Displays a maximize box.
    MaximizeBox = ffi::WXD_MAXIMIZE_BOX,
    /// Displays a minimize box.
    MinimizeBox = ffi::WXD_MINIMIZE_BOX,
    /// Stays on top of other windows.
    StayOnTop = ffi::WXD_STAY_ON_TOP,
    // /// Tool window style (typically a thin border and title bar).
    // ToolWindow = ffi::WXD_FRAME_TOOL_WINDOW, // Not yet available in generated constants
    // /// No taskbar button (Windows only).
    // NoTaskbar = ffi::WXD_FRAME_NO_TASKBAR, // Not yet available in generated constants
    // /// Equivalent to StayOnTop for frames.
    // FloatOnParent = ffi::WXD_FRAME_FLOAT_ON_PARENT, // Not yet available in generated constants
    // /// Clip children to the frame.
    // ClipChildren = ffi::WXD_CLIP_CHILDREN, // Not yet available in generated constants
}

impl FrameStyle {
    /// Returns the raw integer value of the style.
    pub fn bits(self) -> i64 {
        self as i64
    }

    /// A commonly used default set of styles.
    /// Combines `Caption`, `ResizeBorder`, `SystemMenu`, `MinimizeBox`, `MaximizeBox`, `CloseBox`.
    pub const DEFAULT: FrameStyle = FrameStyle::Default; // Keep for clarity if Default is complex
}

impl BitOr for FrameStyle {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        // Safety: We are combining valid i64 repr flags. The resulting i64 is still a valid style combination.
        // It's possible to create a combination not directly represented by an enum variant,
        // which is standard for bitflags.
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for FrameStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        // Safety: Similar to BitOr.
        unsafe {
            *self = std::mem::transmute(self.bits() | rhs.bits());
        }
    }
}

// Ensure FrameBuilder uses the new enum
// Modify FrameBuilder struct definition
// ... search for FrameBuilder struct { ... style: i64 ...
// ... replace with style: FrameStyle ...

// Modify FrameBuilder::default()
// ... search for style: ffi::WXD_DEFAULT_FRAME_STYLE
// ... replace with style: FrameStyle::DEFAULT

// Modify FrameBuilder::with_style()
// ... search for pub fn with_style(mut self, style: i64)
// ... replace with pub fn with_style(mut self, style: FrameStyle)

// Modify FrameBuilder::build()
// ... search for self.style as ffi::wxd_Style_t
// ... replace with self.style.bits() as ffi::wxd_Style_t
// Ensure to add `use std::ops::{BitOr, BitOrAssign};` at the top of the file.
