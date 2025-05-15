#include "../include/wxdragon.h"
#include <wx/wx.h>
#include <unordered_map>
#include <memory> // For std::unique_ptr if we want safer memory management
#include <tuple>  // For std::pair used in map key
#include <wx/event.h>
#include <wx/app.h>
#include <wx/window.h> // For wxCloseEvent
#include <wx/tglbtn.h> // ADDED for wxEVT_TOGGLEBUTTON
#include <wx/treectrl.h> // ADDED: For wxEVT_TREE_* constants
#include <wx/slider.h> // ADDED: For wxEVT_SCROLL_CHANGED etc.
#include <wx/spinctrl.h> // ADDED: For wxEVT_SPINCTRL
#include <wx/spinbutt.h> // ADDED: For wxEVT_SPIN*
#include <wx/notebook.h> // ADDED: For wxEVT_NOTEBOOK_PAGE_CHANGED
#include <wx/splitter.h> // ADDED: For wxEVT_SPLITTER_*
#include <wx/listctrl.h> // ADDED: For wxEVT_LIST_*
#include <wx/clrpicker.h> // ADDED: For wxEVT_COLOURPICKER_CHANGED
#include <wx/dateevt.h> // ADDED: For wxEVT_DATE_CHANGED
#include <wx/treebook.h> // ADDED: For wxEVT_TREEBOOK_*
#include <wx/srchctrl.h> // ADDED: For wxEVT_SEARCHCTRL_SEARCH_BTN, wxEVT_SEARCHCTRL_CANCEL_BTN
#include <wx/hyperlink.h> // ADDED: For wxHyperlinkEvent
#include <wx/calctrl.h> // ADDED: For wxCalendarCtrl events
#include <wx/filepicker.h> // ADDED: For wxEVT_FILEPICKER_CHANGED and wxEVT_DIRPICKER_CHANGED
#include <wx/fontpicker.h> // ADDED: For wxEVT_FONTPICKER_CHANGED
#include <wx/notifmsg.h> // For wxNotificationMessage events
#include <wx/dnd.h> // ADDED: For drag and drop events (wxEVT_BEGIN_DRAG, wxEVT_DROP_TEXT, etc.)
#include <wx/timectrl.h> // ADDED: For wxTimePickerCtrl and wxEVT_TIME_CHANGED
#include <wx/mediactrl.h> // ADDED: For MediaCtrl events
#include "wxd_utils.h"

// --- Internal C++ Structures/Classes (Not exposed in C API) ---

// Define a hash function for std::pair<int, int>
struct PairHash {
    template <class T1, class T2>
    std::size_t operator() (const std::pair<T1, T2>& p) const {
        auto hash1 = std::hash<T1>{}(p.first);
        auto hash2 = std::hash<T2>{}(p.second);
        // Simple combination hash - consider better alternatives if collisions become an issue
        return hash1 ^ (hash2 << 1); 
    }
};

// Structure to hold the Rust closure information
struct RustClosureInfo {
    void* closure_ptr = nullptr;
    // We might need the trampoline pointer here too if it varies,
    // but for now, assume a single global trampoline `rust_event_handler_trampoline`.
    wxd_ClosureCallback rust_trampoline = nullptr; // Store the trampoline func ptr
};

// Forward declaration
class WxdEventHandler;

// ClientData class to hold our handler pointer and ensure deletion
class WxdHandlerClientData : public wxClientData {
public:
    WxdEventHandler* handler; // Pointer to the handler associated with the window

    WxdHandlerClientData(WxdEventHandler* h) : handler(h) {}
    virtual ~WxdHandlerClientData(); // Defined after WxdEventHandler
};

// Custom Event Handler class to connect wx events to Rust closures
class WxdEventHandler : public wxEvtHandler {
public:
    // Map (eventType, widgetId) pair to the Rust closure info
    std::unordered_map<std::pair<wxEventType, wxd_Id>, RustClosureInfo, PairHash> closureMap; 
    wxd_EvtHandler_t* c_handle = nullptr; // Changed type to wxd_EvtHandler_t*
    wxEvtHandler* ownerHandler = nullptr; // Store the actual wxEvtHandler*

    WxdEventHandler(wxd_EvtHandler_t* handle, wxEvtHandler* owner) : c_handle(handle), ownerHandler(owner) {}

    // Destructor - Now needs to notify Rust to drop closures via drop_rust_closure_box
    ~WxdEventHandler(); // Declaration moved, definition below

    // The actual event handler called by wxWidgets
    void OnAnyEvent(wxEvent& event); 
};

// Define WxdHandlerClientData destructor (no change needed here, it still just deletes the handler)
WxdHandlerClientData::~WxdHandlerClientData() {
    delete handler;
}

// --- Declare the Rust helper functions ---
// These functions will be implemented in the Rust `wxdragon` crate.
extern "C" {
    // The trampoline function implemented in Rust
    // Its signature MUST match wxd_ClosureCallback in wxdragon.h
    // void rust_event_handler_trampoline(wxd_Event_t* event_ptr, void* user_data);
    
    // Function implemented in Rust to drop the Box<dyn FnMut(Event)>.
    void drop_rust_closure_box(void* ptr);
}

// WxdEventHandler Destructor Implementation
WxdEventHandler::~WxdEventHandler() {
    // wxLogDebug("WxdEventHandler destroying for handler %p. Notifying Rust to drop closures.", ownerHandler);
    for (auto const& [key, info] : closureMap) {
        if (info.closure_ptr) {
            // Tell Rust to drop the Box corresponding to this pointer
            drop_rust_closure_box(info.closure_ptr);
        }
    }
    // Clear the map (optional, as the handler is being destroyed)
    closureMap.clear();
}

// Modify OnAnyEvent to call the Rust trampoline
void WxdEventHandler::OnAnyEvent(wxEvent& event) {
    wxEventType eventType = event.GetEventType();
    wxd_Id id = event.GetId(); // Get the widget ID from the event

    // Create the key pair
    std::pair<wxEventType, wxd_Id> key = {eventType, id};

    auto it = closureMap.find(key); // Look up using the combined key

    if (it != closureMap.end()) {
        RustClosureInfo& info = it->second;
        if (info.closure_ptr && info.rust_trampoline) {
            // wxLogDebug("[DEBUG C++] Found closure for type=%d, id=%d. Calling trampoline.", eventType, id);
            // Call the specific Rust trampoline function stored in info
            info.rust_trampoline(reinterpret_cast<wxd_Event_t*>(&event), info.closure_ptr);

            // We assume the Rust closure handles event.Skip() if needed via wxd_Event_Skip.
            return; // Event handled (or skipped) by Rust closure
        } else {
            // Should not happen if Bind succeeded, but log if it does
            // wxLogDebug("[DEBUG C++] OnAnyEvent: Found key (%d, %d) but closure_ptr or rust_trampoline is null!", eventType, id);
        }
    } else {
    // If no Rust closure was found for this specific event type,
    // allow default processing.
    event.Skip();
    }
}

// --- C API Implementation --- 

// Gets the handler associated with the wxEvtHandler via client data,
// creating it if it doesn't exist.
WxdEventHandler* GetOrCreateEventHandler(wxEvtHandler* handler, wxd_EvtHandler_t* c_handle) {
    if (!handler) return nullptr;

    WxdHandlerClientData* clientData = static_cast<WxdHandlerClientData*>(handler->GetClientData());
    WxdEventHandler* customHandler = nullptr;

    if (!clientData) {
        // Create the handler
        customHandler = new WxdEventHandler(c_handle, handler);
        // Create the client data wrapper to manage the handler's lifetime
        clientData = new WxdHandlerClientData(customHandler);
        // Associate client data with the wxEvtHandler
        handler->SetClientData(clientData);
        // wxLogDebug("Created WxdEventHandler %p and WxdHandlerClientData %p for wxEvtHandler %p (c_handle %p)", customHandler, clientData, handler, c_handle);
    } else {
        customHandler = clientData->handler;
        // Ensure c_handle is up-to-date (shouldn't change, but good practice)
        if (customHandler) {
            customHandler->c_handle = c_handle; // Update C handle if needed
            customHandler->ownerHandler = handler; // Update owner pointer
        }
    }

    return customHandler;
}

// --- C++ Closure Wrapper (Functor) ---

// A simple functor class to wrap the Rust callback and data pointer.
// Its lifetime is managed by wxWidgets when bound using `wxEvtHandler::Bind`.
// Based on wxRust2 CxxClosureVoid.
class CxxClosureVoid {
public:
    // Type alias for the Rust trampoline function signature
    // It takes the Rust closure data (as void*) and the event pointer (as void*)
    // Note: Argument order might differ from previous attempts, match Rust definition.
    typedef void (*RustTrampolineFn)(void* closure_data, void* event_ptr);

    RustTrampolineFn fn_ptr;    // Pointer to the Rust trampoline function
    void*            param_ptr; // Pointer to the Rust closure Box
    bool             owned_by_wx; // NEW: Flag to track ownership transfer

    // Constructor: Store the Rust pointers, initially not owned by wx
    CxxClosureVoid(void* fn, void* param) : 
        fn_ptr(reinterpret_cast<RustTrampolineFn>(fn)), 
        param_ptr(param), 
        owned_by_wx(false) // Initialize flag
    {
         // wxLogDebug("CxxClosureVoid %p created fn=%p, param=%p, owned=%d", this, fn, param, owned_by_wx);
    }
    
    // Copy Constructor: Also copies the ownership flag state.
    CxxClosureVoid(const CxxClosureVoid& other) : 
        fn_ptr(other.fn_ptr), 
        param_ptr(other.param_ptr), 
        owned_by_wx(other.owned_by_wx) // CORRECT: Copy should inherit ownership state (initially false)
    {
        // wxLogDebug("CxxClosureVoid %p copy constructed from %p (owned=%d)", this, &other, owned_by_wx);
    }
    
    // Destructor: Only the wxWidgets-managed copy should drop the Rust Box.
    ~CxxClosureVoid() {
        // wxLogDebug("CxxClosureVoid %p destroyed. Checking ownership (owned=%d) for param=%p", this, owned_by_wx, param_ptr);
        
        // If owned_by_wx is TRUE, this is the *original* stack-allocated functor
        // whose ownership was transferred to the wxWidgets copy. DO NOT DROP here.
        if (owned_by_wx) {
            // wxLogDebug("CxxClosureVoid %p: Original functor (%p) destroyed, NOT dropping param=%p as owned by wx", this, param_ptr);
            return; // Don't drop if ownership was transferred
        }

        // If owned_by_wx is FALSE, this is either:
        // 1. The wxWidgets-managed copy being destroyed (param_ptr should be valid).
        // 2. The original functor being destroyed *because binding failed* (param_ptr should be valid).
        // In both cases where owned_by_wx is false, we *should* drop the box if the pointer is valid.
        if (param_ptr) {
            // wxLogDebug("CxxClosureVoid %p: Dropping Rust box param=%p as NOT owned by wx", this, param_ptr);
            drop_rust_closure_box(param_ptr);
            param_ptr = nullptr; // Avoid potential double drop if destructor called again somehow
        }
        // else: Warning if param_ptr is null when not owned? Might indicate logic error elsewhere.
    }

    // operator(): This is called by wxWidgets when the event fires.
    // It must accept the specific wxEvent subclass corresponding to the event type
    // it was bound with (e.g., wxCommandEvent&, wxCloseEvent&).
    // We define multiple operator() overloads or use templates if needed,
    // but the trampoline simplifies this: we just need one that takes wxEvent&.
    void operator()(wxEvent& event) {
        if (fn_ptr && param_ptr) {
            fn_ptr(param_ptr, reinterpret_cast<void*>(&event)); 
        } else {
             wxLogWarning("CxxClosureVoid operator() called but fn_ptr or param_ptr is null!");
             event.Skip();
        }
    }
};

// --- C API Implementation ---

extern "C" void wxd_EvtHandler_Bind(
    wxd_EvtHandler_t* handler,
    WXDEventTypeCEnum eventTypeC, 
    void* rust_trampoline_fn, 
    void* rust_closure_ptr   
) {
    wxEvtHandler* wx_handler = reinterpret_cast<wxEvtHandler*>(handler);
    if (!wx_handler) {
         wxLogWarning("wxd_EvtHandler_Bind called with null handler."); 
         if (rust_closure_ptr) { drop_rust_closure_box(rust_closure_ptr); }
         return;
    }

    if (!rust_trampoline_fn || !rust_closure_ptr) {
        wxLogWarning("wxd_EvtHandler_Bind called with null trampoline (%p) or closure (%p).", rust_trampoline_fn, rust_closure_ptr);
        if (rust_closure_ptr) { drop_rust_closure_box(rust_closure_ptr); } // Drop if trampoline is null but closure isn't
        return;
    }

    // Create the functor that wraps the Rust data
    CxxClosureVoid functor(rust_trampoline_fn, rust_closure_ptr);
    bool bound = false; // Flag to track if binding succeeded

    // Switch on the stable C enum value and map to the wxWidgets event tag
    switch (eventTypeC) {
        // --- Command Events ---
        case WXD_EVENT_TYPE_COMMAND_BUTTON_CLICKED:
             wx_handler->Bind(wxEVT_BUTTON, functor); 
             bound = true;
             break;
        case WXD_EVENT_TYPE_CHECKBOX:
            wx_handler->Bind(wxEVT_CHECKBOX, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_COMMAND_RADIOBUTTON_SELECTED:
             wx_handler->Bind(wxEVT_RADIOBUTTON, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_COMMAND_RADIOBOX_SELECTED:
             wx_handler->Bind(wxEVT_RADIOBOX, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_COMMAND_LISTBOX_SELECTED:
             wx_handler->Bind(wxEVT_LISTBOX, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_COMMAND_CHOICE_SELECTED:
             wx_handler->Bind(wxEVT_CHOICE, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_COMMAND_COMBOBOX_SELECTED:
             wx_handler->Bind(wxEVT_COMBOBOX, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_COMMAND_CHECKLISTBOX_SELECTED:
             wx_handler->Bind(wxEVT_CHECKLISTBOX, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_COMMAND_TOGGLEBUTTON_CLICKED:
             wx_handler->Bind(wxEVT_TOGGLEBUTTON, functor); 
             bound = true;
             break;
        case WXD_EVENT_TYPE_MENU:
             wx_handler->Bind(wxEVT_MENU, functor);
             bound = true;
             break;

        // --- Text Events ---
        case WXD_EVENT_TYPE_TEXT:
             wx_handler->Bind(wxEVT_TEXT, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_TEXT_ENTER:
             wx_handler->Bind(wxEVT_TEXT_ENTER, functor);
             bound = true;
             break;

        // --- Tree Events ---
        case WXD_EVENT_TYPE_TREE_BEGIN_LABEL_EDIT:
             wx_handler->Bind(wxEVT_TREE_BEGIN_LABEL_EDIT, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_TREE_END_LABEL_EDIT:
             wx_handler->Bind(wxEVT_TREE_END_LABEL_EDIT, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_TREE_SEL_CHANGED:
             wx_handler->Bind(wxEVT_TREE_SEL_CHANGED, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_TREE_ITEM_ACTIVATED:
             wx_handler->Bind(wxEVT_TREE_ITEM_ACTIVATED, functor);
             bound = true;
             break;

        // --- Slider/Spin Events ---
        case WXD_EVENT_TYPE_SLIDER:
             wx_handler->Bind(wxEVT_SLIDER, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_SPINCTRL:
             wx_handler->Bind(wxEVT_SPINCTRL, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_SPIN_UP:
             wx_handler->Bind(wxEVT_SPIN_UP, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_SPIN_DOWN:
             wx_handler->Bind(wxEVT_SPIN_DOWN, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_SPIN:
             wx_handler->Bind(wxEVT_SPIN, functor);
             bound = true;
             break;

        // --- Notebook Event ---
        case WXD_EVENT_TYPE_NOTEBOOK_PAGE_CHANGED:
             wx_handler->Bind(wxEVT_NOTEBOOK_PAGE_CHANGED, functor);
             bound = true;
             break;

        // --- Splitter Events ---
        case WXD_EVENT_TYPE_SPLITTER_SASH_POS_CHANGED:
             wx_handler->Bind(wxEVT_SPLITTER_SASH_POS_CHANGED, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_SPLITTER_SASH_POS_CHANGING:
             wx_handler->Bind(wxEVT_SPLITTER_SASH_POS_CHANGING, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_SPLITTER_DOUBLECLICKED:
             wx_handler->Bind(wxEVT_SPLITTER_DOUBLECLICKED, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_SPLITTER_UNSPLIT:
             wx_handler->Bind(wxEVT_SPLITTER_UNSPLIT, functor);
             bound = true;
             break;

        // --- ListCtrl Events ---
        case WXD_EVENT_TYPE_LIST_ITEM_SELECTED:
             wx_handler->Bind(wxEVT_LIST_ITEM_SELECTED, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_LIST_ITEM_ACTIVATED:
             wx_handler->Bind(wxEVT_LIST_ITEM_ACTIVATED, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_LIST_COL_CLICK:
             wx_handler->Bind(wxEVT_LIST_COL_CLICK, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_LIST_BEGIN_LABEL_EDIT:
             wx_handler->Bind(wxEVT_LIST_BEGIN_LABEL_EDIT, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_LIST_END_LABEL_EDIT:
             wx_handler->Bind(wxEVT_LIST_END_LABEL_EDIT, functor);
             bound = true;
             break;
        // ADDED: Additional ListCtrl events
        case WXD_EVENT_TYPE_LIST_BEGIN_DRAG:
             wx_handler->Bind(wxEVT_LIST_BEGIN_DRAG, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_LIST_BEGIN_RDRAG:
             wx_handler->Bind(wxEVT_LIST_BEGIN_RDRAG, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_LIST_DELETE_ITEM:
             wx_handler->Bind(wxEVT_LIST_DELETE_ITEM, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_LIST_DELETE_ALL_ITEMS:
             wx_handler->Bind(wxEVT_LIST_DELETE_ALL_ITEMS, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_LIST_ITEM_DESELECTED:
             wx_handler->Bind(wxEVT_LIST_ITEM_DESELECTED, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_LIST_ITEM_FOCUSED:
             wx_handler->Bind(wxEVT_LIST_ITEM_FOCUSED, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_LIST_ITEM_MIDDLE_CLICK:
             wx_handler->Bind(wxEVT_LIST_ITEM_MIDDLE_CLICK, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_LIST_ITEM_RIGHT_CLICK:
             wx_handler->Bind(wxEVT_LIST_ITEM_RIGHT_CLICK, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_LIST_KEY_DOWN:
             wx_handler->Bind(wxEVT_LIST_KEY_DOWN, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_LIST_INSERT_ITEM:
             wx_handler->Bind(wxEVT_LIST_INSERT_ITEM, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_LIST_COL_RIGHT_CLICK:
             wx_handler->Bind(wxEVT_LIST_COL_RIGHT_CLICK, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_LIST_COL_BEGIN_DRAG:
             wx_handler->Bind(wxEVT_LIST_COL_BEGIN_DRAG, functor);
             bound = true;
             break;

        // --- ColourPicker Event ---
        case WXD_EVENT_TYPE_COLOURPICKER_CHANGED: {
            int id = wxID_ANY;
            wxObject* cxx_data_copy = nullptr;
            wx_handler->Bind(wxEVT_COLOURPICKER_CHANGED, functor, id, wxID_ANY, cxx_data_copy);
            bound = true;
            break;
        }
        case WXD_EVENT_TYPE_DATE_CHANGED: {
            int id = wxID_ANY;
            wxObject* cxx_data_copy = nullptr;
            wx_handler->Bind(wxEVT_DATE_CHANGED, functor, id, wxID_ANY, cxx_data_copy);
            bound = true;
            break;
        }

        // --- Window Events ---
        case WXD_EVENT_TYPE_CLOSE_WINDOW:
            wx_handler->Bind(wxEVT_CLOSE_WINDOW, functor); 
            bound = true;
            break;
        case WXD_EVENT_TYPE_SIZE:
             wx_handler->Bind(wxEVT_SIZE, functor);
             bound = true;
             break;

        // --- Mouse Events ---
        case WXD_EVENT_TYPE_LEFT_DOWN:
             wx_handler->Bind(wxEVT_LEFT_DOWN, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_LEFT_UP:
             wx_handler->Bind(wxEVT_LEFT_UP, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_MOTION:
             wx_handler->Bind(wxEVT_MOTION, functor); 
             bound = true;
             break;
        case WXD_EVENT_TYPE_MOUSEWHEEL:
             wx_handler->Bind(wxEVT_MOUSEWHEEL, functor);
             bound = true;
             break;

        // --- Keyboard Events ---
        case WXD_EVENT_TYPE_KEY_DOWN:
             wx_handler->Bind(wxEVT_KEY_DOWN, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_KEY_UP:
             wx_handler->Bind(wxEVT_KEY_UP, functor);
             bound = true;
             break;
        case WXD_EVENT_TYPE_CHAR:
             wx_handler->Bind(wxEVT_CHAR, functor);
             bound = true;
             break;
             
        // ADDED: Treebook Events
        case WXD_EVENT_TYPE_TREEBOOK_PAGE_CHANGED:
            wx_handler->Bind(wxEVT_TREEBOOK_PAGE_CHANGED, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_TREEBOOK_PAGE_CHANGING:
            wx_handler->Bind(wxEVT_TREEBOOK_PAGE_CHANGING, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_TREEBOOK_NODE_EXPANDED:
            wx_handler->Bind(wxEVT_TREEBOOK_NODE_EXPANDED, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_TREEBOOK_NODE_COLLAPSED:
            wx_handler->Bind(wxEVT_TREEBOOK_NODE_COLLAPSED, functor);
            bound = true;
            break;
        // ADDED: SearchCtrl Events
        case WXD_EVENT_TYPE_COMMAND_SEARCHCTRL_SEARCH_BTN:
            wx_handler->Bind(wxEVT_SEARCHCTRL_SEARCH_BTN, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_COMMAND_SEARCHCTRL_CANCEL_BTN:
            wx_handler->Bind(wxEVT_SEARCHCTRL_CANCEL_BTN, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_COMMAND_HYPERLINK:
            wx_handler->Bind(wxEVT_HYPERLINK, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_SPINCTRLDOUBLE:
            wx_handler->Bind(wxEVT_SPINCTRLDOUBLE, functor);
            bound = true;
            break;

        // ADDED: Calendar Control Event
        case WXD_EVENT_TYPE_CALENDAR_SEL_CHANGED:
            wx_handler->Bind(wxEVT_CALENDAR_SEL_CHANGED, functor);
            bound = true;
            break;

        // ADDED: ScrollBar Events
        case WXD_EVENT_TYPE_SCROLL_TOP:
            wx_handler->Bind(wxEVT_SCROLL_TOP, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_SCROLL_BOTTOM:
            wx_handler->Bind(wxEVT_SCROLL_BOTTOM, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_SCROLL_LINEUP:
            wx_handler->Bind(wxEVT_SCROLL_LINEUP, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_SCROLL_LINEDOWN:
            wx_handler->Bind(wxEVT_SCROLL_LINEDOWN, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_SCROLL_PAGEUP:
            wx_handler->Bind(wxEVT_SCROLL_PAGEUP, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_SCROLL_PAGEDOWN:
            wx_handler->Bind(wxEVT_SCROLL_PAGEDOWN, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_SCROLL_THUMBTRACK:
            wx_handler->Bind(wxEVT_SCROLL_THUMBTRACK, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_SCROLL_THUMBRELEASE:
            wx_handler->Bind(wxEVT_SCROLL_THUMBRELEASE, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_SCROLL_CHANGED:
            wx_handler->Bind(wxEVT_SCROLL_CHANGED, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_FILEPICKER_CHANGED:
            wx_handler->Bind(wxEVT_FILEPICKER_CHANGED, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_DIRPICKER_CHANGED:
            wx_handler->Bind(wxEVT_DIRPICKER_CHANGED, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_FONTPICKER_CHANGED:
            wx_handler->Bind(wxEVT_FONTPICKER_CHANGED, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_NOTIFICATION_MESSAGE_CLICK:
            wx_handler->Bind(wxEVT_NOTIFICATION_MESSAGE_CLICK, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_NOTIFICATION_MESSAGE_DISMISSED:
            wx_handler->Bind(wxEVT_NOTIFICATION_MESSAGE_DISMISSED, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_NOTIFICATION_MESSAGE_ACTION:
            wx_handler->Bind(wxEVT_NOTIFICATION_MESSAGE_ACTION, functor);
            bound = true;
            break;
        // MediaCtrl events
        #if wxUSE_MEDIACTRL
        case WXD_EVENT_TYPE_MEDIA_LOADED:
            wx_handler->Bind(wxEVT_MEDIA_LOADED, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_MEDIA_STOP:
            wx_handler->Bind(wxEVT_MEDIA_STOP, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_MEDIA_FINISHED:
            wx_handler->Bind(wxEVT_MEDIA_FINISHED, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_MEDIA_STATECHANGED:
            wx_handler->Bind(wxEVT_MEDIA_STATECHANGED, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_MEDIA_PLAY:
            wx_handler->Bind(wxEVT_MEDIA_PLAY, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_MEDIA_PAUSE:
            wx_handler->Bind(wxEVT_MEDIA_PAUSE, functor);
            bound = true;
            break;
        #endif // wxUSE_MEDIACTRL
        case WXD_EVENT_TYPE_IDLE:
            wx_handler->Bind(wxEVT_IDLE, functor);
            bound = true;
            break;
        // Drag and drop events
        case WXD_EVENT_TYPE_DROP_FILES:
            wx_handler->Bind(wxEVT_DROP_FILES, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_PAINT:
            wx_handler->Bind(wxEVT_PAINT, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_TIME_CHANGED:
            wx_handler->Bind(wxEVT_TIME_CHANGED, functor);
            bound = true;
            break;
        case WXD_EVENT_TYPE_DESTROY:
            wx_handler->Bind(wxEVT_DESTROY, functor);
            bound = true;
            break;

        // Default case for unhandled/unknown event types
        default:
            wxLogWarning("wxd_EvtHandler_Bind: Unsupported WXDEventTypeCEnum value %d for handler %p.", (int)eventTypeC, wx_handler);
            bound = false;
            break;
    }
    
    // Handle ownership transfer based on whether binding occurred
    if (bound) {
        // wxWidgets has taken ownership of the functor (a copy of it).
        // Mark the original functor on the stack so its destructor doesn't drop the Rust Box.
        functor.owned_by_wx = true; 
        // wxLogDebug("wxd_EvtHandler_Bind: Bound event type %d. Functor %p marked as owned by wx.", (int)eventTypeC, &functor);
    } else {
        // Binding failed (unknown event type?), functor going out of scope.
        // Its destructor will drop the Rust Box unless already marked owned (which it isn't here).
        wxLogDebug("wxd_EvtHandler_Bind: Did not bind event type %d. Functor %p destructor will drop Rust box.", (int)eventTypeC, &functor);
        // No need to explicitly call drop_rust_closure_box here, the functor destructor handles it.
    }
    // Original functor goes out of scope here.
}

// --- Event Accessors (Unchanged) ---

// Implementation for wxd_Event_GetId
wxd_Id wxd_Event_GetId(wxd_Event_t* event) {
    if (!event) return wxID_ANY;
    return ((wxEvent*)event)->GetId();
}

// Implementation for wxd_Event_GetEventObject
wxd_Window_t* wxd_Event_GetEventObject(wxd_Event_t* event) {
    if (!event) return nullptr;
    // GetEventObject returns wxObject*. We need to check if it's a window.
    wxObject* obj = ((wxEvent*)event)->GetEventObject();
    wxWindow* win = wxDynamicCast(obj, wxWindow);
    return reinterpret_cast<wxd_Window_t*>(win);
}

// ADDED: Correct signature for Skip
extern "C" void wxd_Event_Skip(wxd_Event_t* event, bool skip) {
    if (!event) return;
    ((wxEvent*)event)->Skip(skip);
}

// --- NEW: Event Data Accessors Implementation ---

// Accessors for specific event types
WXD_EXPORTED int wxd_CommandEvent_GetString(wxd_Event_t* event, char* buffer, int buffer_len) {
    wxCommandEvent* cmdEvent = wxDynamicCast((wxEvent*)event, wxCommandEvent);
    if (!cmdEvent) {
        if (buffer && buffer_len > 0) buffer[0] = '\0';
        return 0; // Return 0 if not a command event or event is null
    }
    wxString str = cmdEvent->GetString();
    size_t needed_len_no_null = wxd_cpp_utils::copy_wxstring_to_buffer(str, buffer, (size_t)buffer_len);
    return (int)(needed_len_no_null + 1); // Return required size including null terminator
}

WXD_EXPORTED bool wxd_CommandEvent_IsChecked(wxd_Event_t* event) {
    if (!event) return false;
    wxEvent* baseEvent = static_cast<wxEvent*>(static_cast<void*>(event)); // Cast via void*
    wxCommandEvent* cmdEvent = dynamic_cast<wxCommandEvent*>(baseEvent);
    if (!cmdEvent) return false; // Not a command event or derived

    return cmdEvent->IsChecked();
}

WXD_EXPORTED wxd_Point wxd_MouseEvent_GetPosition(wxd_Event_t* event) {
    wxd_Point defaultPos = { -1, -1 };
    if (!event) return defaultPos;
    wxEvent* baseEvent = static_cast<wxEvent*>(static_cast<void*>(event)); // Cast via void*
    wxMouseEvent* mouseEvent = dynamic_cast<wxMouseEvent*>(baseEvent);
    if (!mouseEvent) return defaultPos; // Not a mouse event or derived
    
    wxPoint wxPos = mouseEvent->GetPosition();
    wxd_Point pos = { wxPos.x, wxPos.y };
    return pos;
}

WXD_EXPORTED int wxd_KeyEvent_GetKeyCode(wxd_Event_t* event) {
    if (!event) return 0; 
    wxEvent* baseEvent = static_cast<wxEvent*>(static_cast<void*>(event)); // Cast via void*
    wxKeyEvent* keyEvent = dynamic_cast<wxKeyEvent*>(baseEvent);
    if (!keyEvent) return 0; // Not a key event or derived

    return keyEvent->GetKeyCode();
}

// ADDED: Implementation for wxd_CommandEvent_GetInt
WXD_EXPORTED int wxd_CommandEvent_GetInt(wxd_Event_t* event) {
    if (!event) return -1;
    wxEvent* baseEvent = static_cast<wxEvent*>(static_cast<void*>(event)); // Cast via void*
    wxCommandEvent* cmdEvent = dynamic_cast<wxCommandEvent*>(baseEvent);
    if (!cmdEvent) return -1; // Not a command event or derived

    return cmdEvent->GetInt();
}

// ADDED: Scroll Event Data Accessors
WXD_EXPORTED int wxd_ScrollEvent_GetPosition(wxd_Event_t* event) {
    if (!event) return -1; 
    wxScrollEvent* scrollEvent = wxDynamicCast((wxEvent*)event, wxScrollEvent);
    if (!scrollEvent) {
        wxScrollWinEvent* scrollWinEvent = wxDynamicCast((wxEvent*)event, wxScrollWinEvent);
        if (!scrollWinEvent) return -1;
        return scrollWinEvent->GetPosition();
    }
    return scrollEvent->GetPosition();
}

WXD_EXPORTED int wxd_ScrollEvent_GetOrientation(wxd_Event_t* event) {
    if (!event) return -1; 
    wxScrollEvent* scrollEvent = wxDynamicCast((wxEvent*)event, wxScrollEvent);
    if (!scrollEvent) {
        wxScrollWinEvent* scrollWinEvent = wxDynamicCast((wxEvent*)event, wxScrollWinEvent);
        if (!scrollWinEvent) return -1;
        return scrollWinEvent->GetOrientation();
    }
    return scrollEvent->GetOrientation();
}

// Maps our stable C enum value to wxWidgets' dynamic event types
static wxEventType get_wx_event_type_for_c_enum(WXDEventTypeCEnum c_enum_val) {
    switch (c_enum_val) {
        case WXD_EVENT_TYPE_COMMAND_BUTTON_CLICKED:
            return wxEVT_BUTTON;
        case WXD_EVENT_TYPE_CLOSE_WINDOW:
            return wxEVT_CLOSE_WINDOW;
        case WXD_EVENT_TYPE_CHECKBOX:
            return wxEVT_CHECKBOX;
        case WXD_EVENT_TYPE_TEXT:
            return wxEVT_TEXT;
        case WXD_EVENT_TYPE_TEXT_ENTER:
            return wxEVT_TEXT_ENTER;
        case WXD_EVENT_TYPE_SIZE:
            return wxEVT_SIZE;
        case WXD_EVENT_TYPE_MENU:
            return wxEVT_MENU;
        case WXD_EVENT_TYPE_LEFT_DOWN:
            return wxEVT_LEFT_DOWN;
        case WXD_EVENT_TYPE_LEFT_UP:
            return wxEVT_LEFT_UP;
        case WXD_EVENT_TYPE_MOTION:
            return wxEVT_MOTION;
        case WXD_EVENT_TYPE_MOUSEWHEEL:
            return wxEVT_MOUSEWHEEL;
        case WXD_EVENT_TYPE_KEY_DOWN:
            return wxEVT_KEY_DOWN;
        case WXD_EVENT_TYPE_KEY_UP:
            return wxEVT_KEY_UP;
        case WXD_EVENT_TYPE_CHAR:
            return wxEVT_CHAR;
        case WXD_EVENT_TYPE_COMMAND_RADIOBUTTON_SELECTED:
            return wxEVT_RADIOBUTTON;
        case WXD_EVENT_TYPE_COMMAND_RADIOBOX_SELECTED:
            return wxEVT_RADIOBOX;
        case WXD_EVENT_TYPE_COMMAND_LISTBOX_SELECTED:
            return wxEVT_LISTBOX;
        case WXD_EVENT_TYPE_COMMAND_CHOICE_SELECTED:
            return wxEVT_CHOICE;
        case WXD_EVENT_TYPE_COMMAND_COMBOBOX_SELECTED:
            return wxEVT_COMBOBOX;
        case WXD_EVENT_TYPE_COMMAND_CHECKLISTBOX_SELECTED:
            return wxEVT_CHECKLISTBOX;
        case WXD_EVENT_TYPE_COMMAND_TOGGLEBUTTON_CLICKED:
            return wxEVT_TOGGLEBUTTON;
        case WXD_EVENT_TYPE_TREE_BEGIN_LABEL_EDIT:
            return wxEVT_TREE_BEGIN_LABEL_EDIT;
        case WXD_EVENT_TYPE_TREE_END_LABEL_EDIT:
            return wxEVT_TREE_END_LABEL_EDIT;
        case WXD_EVENT_TYPE_TREE_SEL_CHANGED:
            return wxEVT_TREE_SEL_CHANGED;
        case WXD_EVENT_TYPE_TREE_ITEM_ACTIVATED:
            return wxEVT_TREE_ITEM_ACTIVATED;
        case WXD_EVENT_TYPE_SLIDER:
            return wxEVT_SLIDER;
        case WXD_EVENT_TYPE_SPINCTRL:
            return wxEVT_SPINCTRL;
        case WXD_EVENT_TYPE_SPIN_UP:
            return wxEVT_SPIN_UP;
        case WXD_EVENT_TYPE_SPIN_DOWN:
            return wxEVT_SPIN_DOWN;
        case WXD_EVENT_TYPE_SPIN:
            return wxEVT_SPIN;
        case WXD_EVENT_TYPE_NOTEBOOK_PAGE_CHANGED:
            return wxEVT_NOTEBOOK_PAGE_CHANGED;
        case WXD_EVENT_TYPE_SPLITTER_SASH_POS_CHANGED:
            return wxEVT_SPLITTER_SASH_POS_CHANGED;
        case WXD_EVENT_TYPE_SPLITTER_SASH_POS_CHANGING:
            return wxEVT_SPLITTER_SASH_POS_CHANGING;
        case WXD_EVENT_TYPE_SPLITTER_DOUBLECLICKED:
            return wxEVT_SPLITTER_DOUBLECLICKED;
        case WXD_EVENT_TYPE_SPLITTER_UNSPLIT:
            return wxEVT_SPLITTER_UNSPLIT;
        case WXD_EVENT_TYPE_LIST_ITEM_SELECTED:
            return wxEVT_LIST_ITEM_SELECTED;
        case WXD_EVENT_TYPE_LIST_ITEM_ACTIVATED:
            return wxEVT_LIST_ITEM_ACTIVATED;
        case WXD_EVENT_TYPE_LIST_COL_CLICK:
            return wxEVT_LIST_COL_CLICK;
        case WXD_EVENT_TYPE_LIST_BEGIN_LABEL_EDIT:
            return wxEVT_LIST_BEGIN_LABEL_EDIT;
        case WXD_EVENT_TYPE_LIST_END_LABEL_EDIT:
            return wxEVT_LIST_END_LABEL_EDIT;
        case WXD_EVENT_TYPE_LIST_BEGIN_DRAG:
            return wxEVT_LIST_BEGIN_DRAG;
        case WXD_EVENT_TYPE_LIST_BEGIN_RDRAG:
            return wxEVT_LIST_BEGIN_RDRAG;
        case WXD_EVENT_TYPE_LIST_DELETE_ITEM:
            return wxEVT_LIST_DELETE_ITEM;
        case WXD_EVENT_TYPE_LIST_DELETE_ALL_ITEMS:
            return wxEVT_LIST_DELETE_ALL_ITEMS;
        case WXD_EVENT_TYPE_LIST_ITEM_DESELECTED:
            return wxEVT_LIST_ITEM_DESELECTED;
        case WXD_EVENT_TYPE_LIST_ITEM_FOCUSED:
            return wxEVT_LIST_ITEM_FOCUSED;
        case WXD_EVENT_TYPE_LIST_ITEM_MIDDLE_CLICK:
            return wxEVT_LIST_ITEM_MIDDLE_CLICK;
        case WXD_EVENT_TYPE_LIST_ITEM_RIGHT_CLICK:
            return wxEVT_LIST_ITEM_RIGHT_CLICK;
        case WXD_EVENT_TYPE_LIST_KEY_DOWN:
            return wxEVT_LIST_KEY_DOWN;
        case WXD_EVENT_TYPE_LIST_INSERT_ITEM:
            return wxEVT_LIST_INSERT_ITEM;
        case WXD_EVENT_TYPE_LIST_COL_RIGHT_CLICK:
            return wxEVT_LIST_COL_RIGHT_CLICK;
        case WXD_EVENT_TYPE_LIST_COL_BEGIN_DRAG:
            return wxEVT_LIST_COL_BEGIN_DRAG;
        case WXD_EVENT_TYPE_COLOURPICKER_CHANGED:
            return wxEVT_COLOURPICKER_CHANGED;
        case WXD_EVENT_TYPE_DATE_CHANGED:
            return wxEVT_DATE_CHANGED;
        case WXD_EVENT_TYPE_TREEBOOK_PAGE_CHANGED:
            return wxEVT_TREEBOOK_PAGE_CHANGED;
        case WXD_EVENT_TYPE_TREEBOOK_PAGE_CHANGING:
            return wxEVT_TREEBOOK_PAGE_CHANGING;
        case WXD_EVENT_TYPE_TREEBOOK_NODE_EXPANDED:
            return wxEVT_TREEBOOK_NODE_COLLAPSED;
        case WXD_EVENT_TYPE_TREEBOOK_NODE_COLLAPSED:
            return wxEVT_TREEBOOK_NODE_COLLAPSED;
        case WXD_EVENT_TYPE_COMMAND_SEARCHCTRL_SEARCH_BTN:
            return wxEVT_SEARCHCTRL_SEARCH_BTN;
        case WXD_EVENT_TYPE_COMMAND_SEARCHCTRL_CANCEL_BTN:
            return wxEVT_SEARCHCTRL_CANCEL_BTN;
        case WXD_EVENT_TYPE_COMMAND_HYPERLINK:
            return wxEVT_HYPERLINK;
        case WXD_EVENT_TYPE_SPINCTRLDOUBLE:
            return wxEVT_SPINCTRLDOUBLE;
        case WXD_EVENT_TYPE_CALENDAR_SEL_CHANGED:
            return wxEVT_CALENDAR_SEL_CHANGED;
        case WXD_EVENT_TYPE_SCROLL_TOP:
            return wxEVT_SCROLL_TOP;
        case WXD_EVENT_TYPE_SCROLL_BOTTOM:
            return wxEVT_SCROLL_BOTTOM;
        case WXD_EVENT_TYPE_SCROLL_LINEUP:
            return wxEVT_SCROLL_LINEUP;
        case WXD_EVENT_TYPE_SCROLL_LINEDOWN:
            return wxEVT_SCROLL_LINEDOWN;
        case WXD_EVENT_TYPE_SCROLL_PAGEUP:
            return wxEVT_SCROLL_PAGEUP;
        case WXD_EVENT_TYPE_SCROLL_PAGEDOWN:
            return wxEVT_SCROLL_PAGEDOWN;
        case WXD_EVENT_TYPE_SCROLL_THUMBTRACK:
            return wxEVT_SCROLL_THUMBTRACK;
        case WXD_EVENT_TYPE_SCROLL_THUMBRELEASE:
            return wxEVT_SCROLL_THUMBRELEASE;
        case WXD_EVENT_TYPE_SCROLL_CHANGED:
            return wxEVT_SCROLL_CHANGED;
        case WXD_EVENT_TYPE_FILEPICKER_CHANGED:
            return wxEVT_FILEPICKER_CHANGED;
        case WXD_EVENT_TYPE_DIRPICKER_CHANGED:
            return wxEVT_DIRPICKER_CHANGED;
        case WXD_EVENT_TYPE_FONTPICKER_CHANGED:
            return wxEVT_FONTPICKER_CHANGED;
        case WXD_EVENT_TYPE_NOTIFICATION_MESSAGE_CLICK:
            return wxEVT_NOTIFICATION_MESSAGE_CLICK;
        case WXD_EVENT_TYPE_NOTIFICATION_MESSAGE_DISMISSED:
            return wxEVT_NOTIFICATION_MESSAGE_DISMISSED;
        case WXD_EVENT_TYPE_NOTIFICATION_MESSAGE_ACTION:
            return wxEVT_NOTIFICATION_MESSAGE_ACTION;
        case WXD_EVENT_TYPE_MEDIA_LOADED:
            return wxEVT_MEDIA_LOADED;
        case WXD_EVENT_TYPE_MEDIA_STOP:
            return wxEVT_MEDIA_STOP;
        case WXD_EVENT_TYPE_MEDIA_FINISHED:
            return wxEVT_MEDIA_FINISHED;
        case WXD_EVENT_TYPE_MEDIA_STATECHANGED:
            return wxEVT_MEDIA_STATECHANGED;
        case WXD_EVENT_TYPE_MEDIA_PLAY:
            return wxEVT_MEDIA_PLAY;
        case WXD_EVENT_TYPE_MEDIA_PAUSE:
            return wxEVT_MEDIA_PAUSE;
        case WXD_EVENT_TYPE_IDLE:
            return wxEVT_IDLE;
        case WXD_EVENT_TYPE_DROP_FILES:
            return wxEVT_DROP_FILES;
        case WXD_EVENT_TYPE_PAINT:
            return wxEVT_PAINT;
        case WXD_EVENT_TYPE_TIME_CHANGED:
            return wxEVT_TIME_CHANGED;
        case WXD_EVENT_TYPE_DESTROY:
            return wxEVT_DESTROY;
        default:
            // Unknown event type - should we handle this differently?
            // For now let's use a null/placeholder value
            return wxEVT_NULL;
    }
}

// Function to convert a generic wxEvent* to a wxd_Event_t* (which is also wxEvent*),
// but potentially casting to a more specific wx derived event type if known and safe.
// This is primarily for potential future use or debugging; for now, it's mostly identity.
wxd_Event_t* convert_wx_event_to_wxd_event(wxEvent* event) {
    if (!event) return nullptr;

    wxEventType type = event->GetEventType();

    // Current guideline: The C-API `wxd_Event_t` is an opaque `wxEvent*`.
    // Specific event data access (e.g., `wxd_CommandEvent_GetString`) will perform
    // the necessary cast from `wxd_Event_t*` (which is a wxEvent*) to the correct `wx...Event*`.
    // So, this function can generally just return the event pointer as is.
    // The main purpose of checking types here is for potential sanity checks or future needs
    // if we wanted this function to do more aggressive casting, but it's not strictly needed
    // for the current accessor pattern.

    if (type == wxEVT_NULL) return nullptr; // Should not happen for valid events

    // Most events, including wxCommandEvent and its many derivatives (wxButton, wxTextCtrl, wxChoice,
    // wxDateEvent, wxCalendarEvent, wxTreeEvent, wxNotebookEvent, wxSpinEvent, wxColourPickerEvent etc.)
    // will be covered by IsCommandEvent() or are other common event types.
    // We can simply return the event as wxd_Event_t* (which is an opaque wxEvent*).
    // The specific accessor functions (e.g., wxd_CommandEvent_GetString, wxd_MouseEvent_GetPosition)
    // are responsible for casting to the correct wx...Event type from wxd_Event_t*.

    // No complex casting logic needed here for now. Return the opaque pointer.
    return reinterpret_cast<wxd_Event_t*>(event);
}
