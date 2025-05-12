//!
//! Safe wrapper for wxNotebook.

use crate::geometry::{Point, Size};
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use std::os::raw::c_int;
use wxdragon_sys as ffi;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;

// --- Style enum using macro ---
widget_style_enum!(
    name: NotebookStyle,
    doc: "Window style flags for Notebook",
    variants: {
        Default: ffi::WXD_NB_DEFAULT, "Default style, tabs at the top.",
        Top: ffi::WXD_NB_TOP, "Place tabs at the top.",
        Bottom: ffi::WXD_NB_BOTTOM, "Place tabs at the bottom.",
        Left: ffi::WXD_NB_LEFT, "Place tabs on the left.",
        Right: ffi::WXD_NB_RIGHT, "Place tabs on the right.",
        FixedWidth: ffi::WXD_NB_FIXEDWIDTH, "Display all tabs in a single row, possibly with arrows if too many.",
        Multiline: ffi::WXD_NB_MULTILINE, "Allow multiple lines of tabs.",
        NoPageTheme: ffi::WXD_NB_NOPAGETHEME, "Under MSW, don't draw the page theme (allows pages to have individual colours)."
    },
    default_variant: Default
);

/// Represents a wxNotebook widget.
#[derive(Clone)]
pub struct Notebook {
    window: Window,
}

impl Notebook {
    /// Creates a new Notebook builder.
    pub fn builder(parent: &dyn WxWidget) -> NotebookBuilder {
        NotebookBuilder::new(parent)
    }

    // Internal constructor
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_Notebook_t) -> Self {
        Notebook {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
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
                self.window.as_ptr() as *mut ffi::wxd_Notebook_t,
                page.handle_ptr(), // Get the window handle of the page
                c_text.as_ptr(),
                select,
            )
        }
    }

    /// Gets the index of the currently selected page.
    /// Returns `wxNOT_FOUND` (-1) if no page is selected.
    pub fn selection(&self) -> i32 {
        unsafe { ffi::wxd_Notebook_GetSelection(self.window.as_ptr() as *mut ffi::wxd_Notebook_t) }
    }

    /// Sets the selection to the given page index.
    /// Returns the index of the previously selected page.
    pub fn set_selection(&self, page: usize) -> i32 {
        unsafe { ffi::wxd_Notebook_SetSelection(self.window.as_ptr() as *mut ffi::wxd_Notebook_t, page as c_int) }
    }

    // Add other methods like GetPageCount, GetPageText, ChangeSelection, etc. if needed.
}

// Apply common trait implementations for Notebook
implement_widget_traits_with_target!(Notebook, window, Window);

// Use the widget_builder macro to generate the NotebookBuilder implementation
widget_builder!(
    name: Notebook,
    parent_type: &'a dyn WxWidget,
    style_type: NotebookStyle,
    fields: {},
    build_impl: |slf| {
        let notebook_ptr = unsafe {
            ffi::wxd_Notebook_Create(
                slf.parent.handle_ptr(),
                slf.id as c_int,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
            )
        };
        if notebook_ptr.is_null() {
            panic!("Failed to create Notebook");
        }
        unsafe { Notebook::from_ptr(notebook_ptr) }
    }
);
