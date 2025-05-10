//!
//! Safe wrapper for wxNotebook.

use crate::base::{Point, Size, ID_ANY};
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::window::WxWidget;
use std::ffi::CString;
use std::os::raw::c_int;
use wxdragon_sys as ffi;
use std::marker::PhantomData;
use std::ops::{BitOr, BitOrAssign};
use std::default::Default;

/// Window style flags for `Notebook`.
///
/// These flags can be combined using the bitwise OR operator (`|`).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum NotebookStyle {
    /// Default style, tabs at the top.
    Default = ffi::WXD_NB_DEFAULT, // Note: WXD_NB_DEFAULT is often WXD_NB_TOP
    /// Place tabs at the top.
    Top = ffi::WXD_NB_TOP,
    /// Place tabs at the bottom.
    Bottom = ffi::WXD_NB_BOTTOM,
    /// Place tabs on the left.
    Left = ffi::WXD_NB_LEFT,
    /// Place tabs on the right.
    Right = ffi::WXD_NB_RIGHT,
    /// Display all tabs in a single row, possibly with arrows if too many.
    FixedWidth = ffi::WXD_NB_FIXEDWIDTH,
    /// Allow multiple lines of tabs.
    Multiline = ffi::WXD_NB_MULTILINE,
    /// Under MSW, don't draw the page theme (allows pages to have individual colours).
    NoPageTheme = ffi::WXD_NB_NOPAGETHEME,
}

impl NotebookStyle {
    /// Returns the raw integer value of the style.
    pub fn bits(self) -> i64 {
        self as i64
    }

    /// The default style for `Notebook`.
    pub const DEFAULT: NotebookStyle = NotebookStyle::Default;
}

impl Default for NotebookStyle {
    fn default() -> Self {
        NotebookStyle::DEFAULT
    }
}

impl BitOr for NotebookStyle {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for NotebookStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe {
            *self = std::mem::transmute(self.bits() | rhs.bits());
        }
    }
}

/// Represents a wxNotebook widget.
#[derive(Clone)]
pub struct Notebook(pub(crate) *mut ffi::wxd_Notebook_t);

impl Notebook {
    /// Creates a new Notebook builder.
    pub fn builder<W: WxWidget>(parent: &W) -> NotebookBuilder {
        NotebookBuilder::new(parent)
    }

    // Internal constructor
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_Notebook_t) -> Self {
        Notebook(ptr)
    }

    /// Adds a new page to the notebook.
    ///
    /// # Arguments
    /// * `page` - The window to be added as a page.
    /// * `text` - The text for the page's tab.
    /// * `select` - If `true`, selects the page after adding it.
    ///
    /// Returns `true` if the page was added successfully.
    pub fn add_page<W: WxWidget>(&self, page: &W, text: &str, select: bool) -> bool {
        let c_text = CString::new(text).expect("CString::new failed");
        unsafe {
            ffi::wxd_Notebook_AddPage(
                self.0,
                page.handle_ptr(), // Get the window handle of the page
                c_text.as_ptr(),
                select,
            )
        }
    }

    /// Gets the index of the currently selected page.
    /// Returns `wxNOT_FOUND` (-1) if no page is selected.
    pub fn selection(&self) -> i32 {
        unsafe { ffi::wxd_Notebook_GetSelection(self.0) }
    }

    /// Sets the selection to the given page index.
    /// Returns the index of the previously selected page.
    pub fn set_selection(&self, page: usize) -> i32 {
        unsafe { ffi::wxd_Notebook_SetSelection(self.0, page as c_int) }
    }

    // Add other methods like GetPageCount, GetPageText, ChangeSelection, etc. if needed.
}

// Implement the core WxWidget trait
impl WxWidget for Notebook {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.0 as *mut ffi::wxd_Window_t
    }
}

// Implement the event handling trait
impl WxEvtHandler for Notebook {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.0 as *mut ffi::wxd_EvtHandler_t
    }
}

/// Builder for creating `Notebook` widgets.
pub struct NotebookBuilder<'a> {
    parent_ptr: *mut ffi::wxd_Window_t,
    id: Id,
    pos: Point,
    size: Size,
    style: NotebookStyle,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> NotebookBuilder<'a> {
    /// Creates a new Notebook builder with default values.
    pub fn new(parent: &'a dyn WxWidget) -> Self {
        NotebookBuilder {
            parent_ptr: parent.handle_ptr(),
            id: ID_ANY,
            pos: Point::default(),
            size: Size::default(),
            style: NotebookStyle::DEFAULT,
            _phantom: PhantomData,
        }
    }

    /// Sets the window ID.
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    /// Sets the position.
    pub fn with_pos(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    /// Sets the size.
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Sets the style flags.
    pub fn with_style(mut self, style: NotebookStyle) -> Self {
        self.style = style;
        self
    }

    /// Builds the Notebook widget.
    pub fn build(self) -> Notebook {
        let notebook_ptr = unsafe {
            ffi::wxd_Notebook_Create(
                self.parent_ptr,
                self.id as c_int,
                self.pos.into(),
                self.size.into(),
                self.style.bits() as ffi::wxd_Style_t,
            )
        };
        if notebook_ptr.is_null() {
            panic!("Failed to create Notebook");
        }
        unsafe { Notebook::from_ptr(notebook_ptr) }
    }
}
