use crate::event::WxEvtHandler;
use crate::prelude::*;
use crate::widgets::button::Button; // Compose Button
use crate::window::{Window, WxWidget};
use std::default::Default;
use std::ffi::CString;
use wxdragon_sys as ffi;

/// Represents a wxCommandLinkButton.
#[derive(Clone)]
pub struct CommandLinkButton {
    button: Button, // Composition: CommandLinkButton IS a Button
}

// --- CommandLinkButton Builder ---

/// Builder pattern for creating `CommandLinkButton` widgets.
#[derive(Clone)]
pub struct CommandLinkButtonBuilder {
    parent_ptr: *mut ffi::wxd_Window_t,
    id: Id,
    main_label: String,
    note: String,
    pos: Point,
    size: Size,
    style: i64,
}

impl Default for CommandLinkButtonBuilder {
    fn default() -> Self {
        Self {
            parent_ptr: std::ptr::null_mut(),
            id: -1, // wxID_ANY
            main_label: String::new(),
            note: String::new(),
            pos: Point { x: -1, y: -1 },
            size: Size { width: -1, height: -1 },
            style: 0,
        }
    }
}

impl CommandLinkButtonBuilder {
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    pub fn with_main_label(mut self, label: &str) -> Self {
        self.main_label = label.to_string();
        self
    }

    pub fn with_note(mut self, note: &str) -> Self {
        self.note = note.to_string();
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

    pub fn build(self) -> CommandLinkButton {
        assert!(!self.parent_ptr.is_null(), "CommandLinkButton requires a parent");
        let c_main_label = CString::new(self.main_label).expect("CString::new for main_label failed");
        let c_note = CString::new(self.note).expect("CString::new for note failed");

        let ptr = unsafe {
            ffi::wxd_CommandLinkButton_Create(
                self.parent_ptr,
                self.id,
                c_main_label.as_ptr(),
                c_note.as_ptr(),
                self.pos.into(),
                self.size.into(),
                self.style.try_into().unwrap(),
            )
        };

        if ptr.is_null() {
            panic!("Failed to create CommandLinkButton widget");
        } else {
            let window = unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) }; // Cast to Window pointer and wrap
            let button = Button::new_from_composition(window, self.parent_ptr);
            CommandLinkButton {
                button,
            }
        }
    }
}

// --- CommandLinkButton Implementation ---

impl CommandLinkButton {
    /// Creates a new `CommandLinkButtonBuilder` for constructing a command link button.
    pub fn builder(parent: &dyn WxWidget) -> CommandLinkButtonBuilder {
        let mut builder = CommandLinkButtonBuilder::default();
        builder.parent_ptr = parent.handle_ptr();
        builder
    }

    /// Sets the note displayed on the button.
    pub fn set_note(&self, note: &str) {
        let c_note = CString::new(note).expect("CString::new for note failed");
        unsafe {
            ffi::wxd_CommandLinkButton_SetNote(
                self.button.handle_ptr() as *mut ffi::wxd_CommandLinkButton_t, // Cast to specific type
                c_note.as_ptr(),
            );
        }
    }
    
    // Getters for main label and note could be added if wxCommandLinkButton provides them
    // or if we decide to store them in the Rust struct. For now, main label uses Button::get_label().
}

// Implement WxWidget for CommandLinkButton by delegating to the composed Button.
impl WxWidget for CommandLinkButton {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.button.handle_ptr() // Delegate to Button's WxWidget impl
    }
}

// Implement Deref to allow CommandLinkButton to be used where a Button (and thus Window) is expected.
impl std::ops::Deref for CommandLinkButton {
    type Target = Button;
    fn deref(&self) -> &Self::Target {
        &self.button
    }
}

impl std::ops::DerefMut for CommandLinkButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.button
    }
}

// Implement WxEvtHandler for CommandLinkButton by delegating to the composed Button.
impl WxEvtHandler for CommandLinkButton {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.button.get_event_handler_ptr() // Delegate
    }
}

// Drop behavior is handled by the composed Button's Drop implementation (which is a no-op for child widgets)
// and ultimately by wxWidgets when the parent is destroyed. 