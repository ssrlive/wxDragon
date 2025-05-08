//! Safe wrappers for wxWidgets events.

use crate::base::Point;
use crate::datetime::DateTime;
use crate::widgets::colourpickerctrl::Colour;
use crate::widgets::treectrl::TreeItemId;
use crate::window::Window;
use std::boxed::Box;
use std::ffi::c_void;
use std::ffi::CStr;
use std::os::raw::c_char;
use wxdragon_sys as ffi;

// Re-export FFI EventType alias and specific EVT_ constants
// pub use ffi::{EventType as EventTypeInt, EVT_COMMAND_BUTTON_CLICKED, EVT_CHECKBOX, EVT_CLOSE_WINDOW, EVT_MENU, EVT_TEXT, EVT_TEXT_ENTER};

// Re-export the stable C enum for use in the safe wrapper
pub use ffi::WXDEventTypeCEnum;

// --- EventType Enum ---

/// Represents a wxDragon event type using stable C enum values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // Ensures memory layout matches the underlying C enum integer type
pub struct EventType(ffi::WXDEventTypeCEnum); // Use the generated C enum type

impl EventType {
    // Constants map directly to the stable C enum values
    pub const COMMAND_BUTTON_CLICKED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_BUTTON_CLICKED);
    pub const CLOSE_WINDOW: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CLOSE_WINDOW);
    pub const CHECKBOX: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CHECKBOX);
    pub const TEXT: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TEXT);
    pub const TEXT_ENTER: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TEXT_ENTER);
    pub const SIZE: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SIZE);
    pub const MENU: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MENU);
    pub const LEFT_DOWN: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LEFT_DOWN);
    pub const LEFT_UP: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LEFT_UP);
    pub const MOTION: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MOTION);
    pub const MOUSEWHEEL: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MOUSEWHEEL);
    pub const KEY_DOWN: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_KEY_DOWN);
    pub const KEY_UP: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_KEY_UP);
    pub const CHAR: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CHAR);
    pub const COMMAND_RADIOBUTTON_SELECTED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_RADIOBUTTON_SELECTED);
    pub const COMMAND_RADIOBOX_SELECTED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_RADIOBOX_SELECTED);
    pub const COMMAND_LISTBOX_SELECTED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_LISTBOX_SELECTED);
    pub const COMMAND_CHOICE_SELECTED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_CHOICE_SELECTED);
    pub const COMMAND_COMBOBOX_SELECTED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_COMBOBOX_SELECTED);
    pub const COMMAND_CHECKLISTBOX_SELECTED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_CHECKLISTBOX_SELECTED);
    pub const COMMAND_TOGGLEBUTTON_CLICKED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_TOGGLEBUTTON_CLICKED);
    // ADDED: TreeCtrl event types
    pub const TREE_BEGIN_LABEL_EDIT: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_BEGIN_LABEL_EDIT);
    pub const TREE_END_LABEL_EDIT: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_END_LABEL_EDIT);
    pub const TREE_SEL_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_SEL_CHANGED);
    pub const TREE_ITEM_ACTIVATED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_ITEM_ACTIVATED);
    // ADDED: Slider event type
    pub const SLIDER: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SLIDER);
    // ADDED: SpinCtrl event type
    pub const SPINCTRL: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPINCTRL);
    // ADDED: SpinButton event types
    pub const SPIN_UP: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPIN_UP);
    pub const SPIN_DOWN: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPIN_DOWN);
    pub const SPIN: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPIN);
    // ADDED: Notebook event type
    pub const NOTEBOOK_PAGE_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_NOTEBOOK_PAGE_CHANGED);
    // ADDED: Splitter event types
    pub const SPLITTER_SASH_POS_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPLITTER_SASH_POS_CHANGED);
    pub const SPLITTER_SASH_POS_CHANGING: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPLITTER_SASH_POS_CHANGING);
    pub const SPLITTER_DOUBLECLICKED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPLITTER_DOUBLECLICKED);
    pub const SPLITTER_UNSPLIT: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPLITTER_UNSPLIT);
    // ADDED: ListCtrl event types
    pub const LIST_ITEM_SELECTED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_ITEM_SELECTED);
    pub const LIST_ITEM_ACTIVATED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_ITEM_ACTIVATED);
    pub const LIST_COL_CLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_COL_CLICK);
    pub const LIST_BEGIN_LABEL_EDIT: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_BEGIN_LABEL_EDIT);
    pub const LIST_END_LABEL_EDIT: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_END_LABEL_EDIT);
    // ADDED: ColourPickerCtrl event type
    pub const COLOURPICKER_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COLOURPICKER_CHANGED);
    // DatePicker Event
    pub const DATE_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATE_CHANGED);
    // Treebook Events (match WXDEventTypeCEnum values)
    pub const TREEBOOK_PAGE_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREEBOOK_PAGE_CHANGED);
    pub const TREEBOOK_PAGE_CHANGING: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREEBOOK_PAGE_CHANGING);
    pub const TREEBOOK_NODE_EXPANDED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREEBOOK_NODE_EXPANDED);
    pub const TREEBOOK_NODE_COLLAPSED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREEBOOK_NODE_COLLAPSED);
    // ADDED: SearchCtrl Event Types
    pub const COMMAND_SEARCHCTRL_SEARCH_BTN: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_SEARCHCTRL_SEARCH_BTN);
    pub const COMMAND_SEARCHCTRL_CANCEL_BTN: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_SEARCHCTRL_CANCEL_BTN);
    pub const COMMAND_HYPERLINK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_HYPERLINK);
    pub const SPINCTRLDOUBLE: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPINCTRLDOUBLE);
    // ADDED: Calendar Control Event Type
    pub const CALENDAR_SEL_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CALENDAR_SEL_CHANGED);
    // ADDED: ScrollBar Events
    pub const SCROLL_TOP: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_TOP);
    pub const SCROLL_BOTTOM: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_BOTTOM);
    pub const SCROLL_LINEUP: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_LINEUP);
    pub const SCROLL_LINEDOWN: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_LINEDOWN);
    pub const SCROLL_PAGEUP: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_PAGEUP);
    pub const SCROLL_PAGEDOWN: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_PAGEDOWN);
    pub const SCROLL_THUMBTRACK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_THUMBTRACK);
    pub const SCROLL_THUMBRELEASE: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_THUMBRELEASE);
    pub const SCROLL_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_CHANGED);
    // Add others as needed

    /// Get the underlying stable C enum value.
    pub(crate) fn as_c_enum(&self) -> ffi::WXDEventTypeCEnum {
        self.0
    }
}

// --- Simple Event Struct ---

/// Represents a wxWidgets event.
/// This struct is a lightweight wrapper around the raw `wxd_Event_t*` pointer.
/// It provides safe methods to access event details.
#[derive(Debug, Clone, Copy)] // Raw pointers are Copy
pub struct Event(pub(crate) *mut ffi::wxd_Event_t);

impl Event {
    /// Creates a new Event wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_Event_t` pointer obtained from wxWidgets.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_Event_t) -> Self {
        Event(ptr)
    }

    /// Gets the raw pointer to the underlying wxWidgets event object.
    pub(crate) fn _as_ptr(&self) -> *mut ffi::wxd_Event_t {
        self.0
    }

    /// Checks if the underlying pointer is null.
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Gets the ID of the event.
    pub fn get_id(&self) -> i32 {
        if self.0.is_null() {
            return ffi::WXD_ID_ANY as i32;
        }
        unsafe { ffi::wxd_Event_GetId(self.0) }
    }

    /// Gets the object (usually a window) that generated the event.
    pub fn get_event_object(&self) -> Option<Window> {
        if self.0.is_null() {
            return None;
        }
        let ptr = unsafe { ffi::wxd_Event_GetEventObject(self.0) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Window::from_ptr(ptr) })
        }
    }

    /// Controls whether the event is processed further.
    pub fn skip(&self, skip: bool) {
        if self.0.is_null() {
            return;
        }
        unsafe { ffi::wxd_Event_Skip(self.0, skip) };
    }

    // --- Common Event Data Accessors ---

    /// Gets the string associated with a command event.
    pub fn get_string(&self) -> Option<String> {
        if self.0.is_null() {
            return None;
        }
        unsafe {
            let mut buffer: [c_char; 1024] = [0; 1024];
            let len_needed =
                ffi::wxd_CommandEvent_GetString(self.0, buffer.as_mut_ptr(), buffer.len() as i32);
            if len_needed < 0 {
                return None;
            }
            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                Some(
                    CStr::from_ptr(buffer.as_ptr())
                        .to_string_lossy()
                        .into_owned(),
                )
            } else {
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_CommandEvent_GetString(
                    self.0,
                    vec_buffer.as_mut_ptr() as *mut c_char,
                    vec_buffer.len() as i32,
                );
                if len_copied == len_needed {
                    vec_buffer.pop();
                    String::from_utf8(vec_buffer).ok()
                } else {
                    None
                }
            }
        }
    }

    /// Checks if a command event represents a "checked" state.
    pub fn is_checked(&self) -> Option<bool> {
        if self.0.is_null() {
            return None;
        }
        Some(unsafe { ffi::wxd_CommandEvent_IsChecked(self.0) })
    }

    /// Gets the mouse position associated with a mouse event.
    pub fn get_position(&self) -> Option<Point> {
        if self.0.is_null() {
            return None;
        }
        let c_point = unsafe { ffi::wxd_MouseEvent_GetPosition(self.0) };
        if c_point.x == -1 && c_point.y == -1 {
            None
        } else {
            Some(Point {
                x: c_point.x,
                y: c_point.y,
            })
        }
    }

    /// Gets the key code associated with a key event.
    pub fn get_key_code(&self) -> Option<i32> {
        if self.0.is_null() {
            return None;
        }
        let key_code = unsafe { ffi::wxd_KeyEvent_GetKeyCode(self.0) };
        if key_code == 0 {
            None
        } else {
            Some(key_code)
        }
    }

    /// Gets the integer value associated with a command event.
    pub fn get_int(&self) -> Option<i32> {
        if self.0.is_null() {
            return None;
        }
        let int_val = unsafe { ffi::wxd_CommandEvent_GetInt(self.0) };
        if int_val == -1 {
            None
        } else {
            Some(int_val)
        }
    }

    // --- Scroll Event Data Accessors ---

    /// Gets the scroll position associated with a scroll event.
    pub fn get_scroll_position(&self) -> Option<i32> {
        if self.0.is_null() {
            return None;
        }
        let pos = unsafe { ffi::wxd_ScrollEvent_GetPosition(self.0) };
        if pos == -1 {
            None
        } else {
            Some(pos)
        } // Assuming -1 indicates error/not applicable
    }

    /// Gets the orientation (wxHORIZONTAL/wxVERTICAL) associated with a scroll event.
    pub fn get_scroll_orientation(&self) -> Option<i32> {
        if self.0.is_null() {
            return None;
        }
        let orient = unsafe { ffi::wxd_ScrollEvent_GetOrientation(self.0) };
        if orient == -1 {
            None
        } else {
            Some(orient)
        } // Assuming -1 indicates error/not applicable
    }

    // --- ColourPicker Event Data Accessor ---
    pub fn get_colour(&self) -> Option<Colour> {
        if self.0.is_null() {
            return None;
        }
        // The C function returns wxd_Colour_t directly.
        // We assume if the event type is correct, the call is valid.
        // No easy way to return None if the underlying event wasn't a ColourPickerEvent
        // without adding more complex type checking or specific event structs.
        let c_colour = unsafe { ffi::wxd_ColourPickerEvent_GetColour(self.0) };
        // Check for a potential default/invalid value returned by C++ if the cast failed internally?
        // E.g., if (c_colour.r == 0 && c_colour.g == 0 && c_colour.b == 0 && c_colour.a == 0) { None } else { Some(Colour::from(c_colour)) }
        // For now, directly convert.
        Some(Colour::from(c_colour))
    }

    // --- TreeEvent Data Accessors ---
    pub fn get_item(&self) -> Option<TreeItemId> {
        if self.0.is_null() {
            return None;
        }
        let item_ptr = unsafe { ffi::wxd_TreeEvent_GetItem(self.0) };
        unsafe { TreeItemId::from_ptr(item_ptr) }
    }

    // --- NotebookEvent Data Accessor ---
    pub fn get_selection(&self) -> Option<i32> {
        let val = unsafe { ffi::wxd_NotebookEvent_GetSelection(self.0) };
        if val == ffi::WXD_NOT_FOUND as i32 {
            None
        } else {
            Some(val)
        }
    }

    /// For book control events (Notebook, Treebook, etc.), gets the old page selection.
    pub fn get_old_selection(&self) -> Option<i32> {
        let val = unsafe { ffi::wxd_NotebookEvent_GetOldSelection(self.0) };
        if val == ffi::WXD_NOT_FOUND as i32 {
            None
        } else {
            Some(val)
        }
    }

    // --- SplitterEvent Data Accessor ---
    pub fn get_sash_position(&self) -> Option<i32> {
        if self.0.is_null() {
            return None;
        }
        Some(unsafe { ffi::wxd_SplitterEvent_GetSashPosition(self.0) })
    }

    // --- ListCtrl Event Data Accessors ---
    pub fn get_item_index(&self) -> i32 {
        // Changed return type to i32
        if self.0.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_ListEvent_GetItemIndex(self.0) } // Should now return i32
    }

    pub fn get_column(&self) -> Option<i32> {
        if self.0.is_null() {
            return None;
        }
        let col = unsafe { ffi::wxd_ListEvent_GetColumn(self.0) };
        if col == -1 {
            None
        } else {
            Some(col)
        }
    }

    // get_label for ListCtrl events (duplicates TreeEvent get_label, but okay on non-generic Event)
    pub fn get_label(&self) -> Option<String> {
        if self.0.is_null() {
            return None;
        }
        // Try ListEvent first, then TreeEvent?
        // Let's assume context implies correct FFI call or FFI handles it.
        // Using wxd_ListEvent_GetLabel here.
        unsafe {
            let mut buffer: [c_char; 1024] = [0; 1024];
            let len_needed =
                ffi::wxd_ListEvent_GetLabel(self.0, buffer.as_mut_ptr(), buffer.len() as i32);
            if len_needed < 0 {
                return None;
            }
            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                Some(
                    CStr::from_ptr(buffer.as_ptr())
                        .to_string_lossy()
                        .into_owned(),
                )
            } else {
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_ListEvent_GetLabel(
                    self.0,
                    vec_buffer.as_mut_ptr() as *mut c_char,
                    vec_buffer.len() as i32,
                );
                if len_copied == len_needed {
                    vec_buffer.pop();
                    String::from_utf8(vec_buffer).ok()
                } else {
                    None
                }
            }
        }
    }

    pub fn is_edit_cancelled(&self) -> Option<bool> {
        if self.0.is_null() {
            return None;
        }
        Some(unsafe { ffi::wxd_ListEvent_IsEditCancelled(self.0) })
    }

    // --- DatePicker / CalendarCtrl Event Data Accessor ---
    // Note: wxDatePickerCtrl and wxCalendarCtrl both use wxDateEvent / wxCalendarEvent
    // which derive from wxCommandEvent. The C function wxd_DatePickerEvent_GetDate
    // and the new wxd_CalendarEvent_GetDate should be compatible or identical in practice for GetDate().
    // We can have a generic get_date() if EventType helps dispatch, or specific ones.
    // For now, adding a specific one for calendar.
    pub fn get_calendar_date(&self) -> Option<DateTime> {
        // SAFETY: Assumes self.0 is a valid wxd_Event_t containing calendar data.
        // Needs corresponding C API function wxd_CalendarEvent_GetDate.
        // let raw_dt = unsafe { ffi::wxd_CalendarEvent_GetDate(self.0) }; // REMOVED
        // If the event doesn't contain date info, we might get default/invalid DateTime.
        // Consider getting date directly from the CalendarCtrl in the handler instead.
        println!("Warning: Event::get_calendar_date() called, but FFI wxd_CalendarEvent_GetDate is removed. Returning None.");
        None // Temporarily return None as FFI is missing
             // if DateTime::is_valid_raw(&raw_dt) {
             //     Some(DateTime::from_ffi(raw_dt))
             // } else {
             //     None
             // }
    }
}

// --- WxEvtHandler Trait (Updated for Simple Event Handling) ---

pub trait WxEvtHandler {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t;

    // UPDATED: Takes simple FnMut(Event)
    fn bind<F>(&self, event_type: EventType, callback: F)
    where
        F: FnMut(Event) + 'static,
    {
        let handler_ptr = unsafe { self.get_event_handler_ptr() };
        if handler_ptr.is_null() {
            /* ... error handling ... */
            return;
        }

        // UPDATED: Box simple FnMut(Event)
        let boxed_callback: Box<Box<dyn FnMut(Event) + 'static>> = Box::new(Box::new(callback));
        let user_data = Box::into_raw(boxed_callback) as *mut c_void;

        type TrampolineFn = unsafe extern "C" fn(*mut c_void, *mut c_void);
        let trampoline_ptr: TrampolineFn = rust_event_handler_trampoline;
        let trampoline_c_void = trampoline_ptr as *mut c_void;

        unsafe {
            ffi::wxd_EvtHandler_Bind(
                handler_ptr,
                event_type.as_c_enum(),
                trampoline_c_void,
                user_data,
            );
        }
    }
}

// --- FFI Trampoline & Drop Functions (Updated for Simple Event) ---

/// Trampoline function: Called by C++.
/// `user_data` is a raw pointer to `Box<Box<dyn FnMut(Event) + 'static>>`.
#[no_mangle]
pub unsafe extern "C" fn rust_event_handler_trampoline(
    user_data: *mut c_void,
    event_ptr_cvoid: *mut c_void,
) {
    if user_data.is_null() {
        /* ... error handling ... */
        return;
    }

    // UPDATED: Cast to Box<dyn FnMut(Event)>
    let closure_box = &mut *(user_data as *mut Box<dyn FnMut(Event) + 'static>);
    let event_ptr = event_ptr_cvoid as *mut ffi::wxd_Event_t;

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // UPDATED: Create simple Event
        let safe_event = Event::from_ptr(event_ptr);
        (*closure_box)(safe_event);
    }));

    if result.is_err() { /* ... error handling ... */ }
}

/// Function called by C++ to drop the Rust closure Box.
/// `ptr` is a raw pointer to `Box<Box<dyn FnMut(Event) + 'static>>`.
#[no_mangle]
pub unsafe extern "C" fn drop_rust_closure_box(ptr: *mut c_void) {
    if !ptr.is_null() {
        // UPDATED: Drop simple Box<Box<dyn FnMut(Event)>>
        let _: Box<Box<dyn FnMut(Event) + 'static>> = Box::from_raw(ptr as *mut _);
    } else { /* ... */
    }
}
