#ifndef WXDRAGON_H
#define WXDRAGON_H

// Define WXDRAGON_API for export/import
#ifdef _WIN32
    #ifdef WXDRAGON_BUILDING_STATIC_LIB // Changed from WXDRAGON_BUILDING_DLL
        #define WXDRAGON_API // Empty for static lib
    #elif defined(WXDRAGON_USING_DLL) // New: if we ever make a DLL and use it
        #define WXDRAGON_API __declspec(dllimport)
    #else // Default to empty, assuming static lib if not specified
        #define WXDRAGON_API
    #endif
#else
    #define WXDRAGON_API // Define as empty for non-Windows (static linking)
#endif

// Use standard C types
#include <stdbool.h> 
#include <stdint.h> // For integer types if needed
// #include <wx/event.h> // REMOVED - No longer needed for type definitions
#include <stddef.h>

// --- Utility Macros for String Handling (Moved Up) ---
#ifdef __cplusplus
    #include <wx/string.h> // Include wxString header for the macros below

    // Helper macro to convert const char* to wxString, handling nulls and UTF-8
    // wxString::FromUTF8(input ? input : "")
    #define WXD_STR_TO_WX_STRING_UTF8_NULL_OK(input_text) wxString::FromUTF8(input_text ? input_text : "")

    // Helper macro/function template for getting wxString result into C buffer
    // Returns: length needed (excluding null), or -1 on error.
    // On success and if buffer is sufficient, copies string and null-terminates.
    // If buffer is null or bufLen is 0, returns length needed without copying.
    // This is a MACRO that would wrap a call to a utility function like wxd_cpp_utils::copy_wxstring_to_buffer
    #define GET_WX_STRING_RESULT(wx_str_expr, c_buffer, c_buf_len) wxd_cpp_utils::copy_wxstring_to_buffer(wx_str_expr, c_buffer, c_buf_len)
#endif
// --- End Utility Macros ---

// Define export macro (empty for static linking by default)
#ifndef WXD_EXPORTED
    #define WXD_EXPORTED WXDRAGON_API // WXD_EXPORTED will now use WXDRAGON_API logic
#endif

#ifdef __cplusplus
extern "C" {
#endif

// --- Basic Types --- 
typedef int wxd_Id;

typedef struct {
    int x, y;
} wxd_Point;

typedef struct {
    int width, height;
} wxd_Size;

// ADDED: C struct for wxRect
typedef struct wxd_Rect {
    int x;
    int y;
    int width;
    int height;
} wxd_Rect;

// NEW: Define wxd_Colour_t for RGBA colours
typedef struct {
    unsigned char r;
    unsigned char g;
    unsigned char b;
    unsigned char a; // Alpha component (wxColour usually uses 0-255, default 255=opaque)
} wxd_Colour_t;

// Define C types for constants/flags used in the API
// REMOVED: No longer passing wxEventType directly
// typedef int wxd_EventType_t;

// --- NEW: Stable C Enum for Event Types ---
typedef enum {
    WXD_EVENT_TYPE_NULL = 0, // Or some other sentinel value
    WXD_EVENT_TYPE_COMMAND_BUTTON_CLICKED = 1,
    WXD_EVENT_TYPE_CLOSE_WINDOW = 2,
    WXD_EVENT_TYPE_CHECKBOX = 3,
    WXD_EVENT_TYPE_TEXT = 4,
    WXD_EVENT_TYPE_TEXT_ENTER = 5,
    WXD_EVENT_TYPE_SIZE = 6,
    WXD_EVENT_TYPE_MENU = 7,
    WXD_EVENT_TYPE_LEFT_DOWN = 8,
    WXD_EVENT_TYPE_LEFT_UP = 9,
    WXD_EVENT_TYPE_MOTION = 10,
    WXD_EVENT_TYPE_MOUSEWHEEL = 11,
    WXD_EVENT_TYPE_KEY_DOWN = 12,
    WXD_EVENT_TYPE_KEY_UP = 13,
    WXD_EVENT_TYPE_CHAR = 14,
    WXD_EVENT_TYPE_COMMAND_RADIOBUTTON_SELECTED = 15,
    WXD_EVENT_TYPE_COMMAND_RADIOBOX_SELECTED = 16, // NEW: wxEVT_RADIOBOX
    WXD_EVENT_TYPE_COMMAND_LISTBOX_SELECTED = 17, // Renumbered
    WXD_EVENT_TYPE_COMMAND_CHOICE_SELECTED = 18,   // Renumbered
    WXD_EVENT_TYPE_COMMAND_COMBOBOX_SELECTED = 19,  // Renumbered
    WXD_EVENT_TYPE_COMMAND_CHECKLISTBOX_SELECTED = 20, // Renumbered
    WXD_EVENT_TYPE_COMMAND_TOGGLEBUTTON_CLICKED = 21, // Renumbered
    // TreeCtrl Events
    WXD_EVENT_TYPE_TREE_BEGIN_LABEL_EDIT = 22,      // Renumbered
    WXD_EVENT_TYPE_TREE_END_LABEL_EDIT = 23,        // Renumbered
    WXD_EVENT_TYPE_TREE_SEL_CHANGED = 24,         // Renumbered
    WXD_EVENT_TYPE_TREE_ITEM_ACTIVATED = 25,    // Renumbered
    // ADDED: Slider Event
    WXD_EVENT_TYPE_SLIDER = 26,                 // Renumbered
    // ADDED: SpinCtrl Event
    WXD_EVENT_TYPE_SPINCTRL = 27,               // Renumbered
    // ADDED: SpinButton Events
    WXD_EVENT_TYPE_SPIN_UP = 28,                // Renumbered
    WXD_EVENT_TYPE_SPIN_DOWN = 29,              // Renumbered
    WXD_EVENT_TYPE_SPIN = 30,                   // Renumbered
    // ADDED: Notebook Event
    WXD_EVENT_TYPE_NOTEBOOK_PAGE_CHANGED = 31, // Renumbered
    // ADDED: Splitter Events
    WXD_EVENT_TYPE_SPLITTER_SASH_POS_CHANGED = 32,    // Renumbered
    WXD_EVENT_TYPE_SPLITTER_SASH_POS_CHANGING = 33,   // Renumbered
    WXD_EVENT_TYPE_SPLITTER_DOUBLECLICKED = 34,       // Renumbered
    WXD_EVENT_TYPE_SPLITTER_UNSPLIT = 35,             // Renumbered
    // Add ListCtrl Event Types
    WXD_EVENT_TYPE_LIST_ITEM_SELECTED = 36,         // Renumbered
    WXD_EVENT_TYPE_LIST_ITEM_ACTIVATED = 37,        // Renumbered
    WXD_EVENT_TYPE_LIST_COL_CLICK = 38,             // Renumbered
    WXD_EVENT_TYPE_LIST_BEGIN_LABEL_EDIT = 39,      // Renumbered
    WXD_EVENT_TYPE_LIST_END_LABEL_EDIT = 40,        // Renumbered
    // ADDED: ColourPickerCtrl Event
    WXD_EVENT_TYPE_COLOURPICKER_CHANGED = 41,       // New event type
    // ADDED: DatePicker Event
    WXD_EVENT_TYPE_DATE_CHANGED = 42,
    // ADDED: Treebook Events
    WXD_EVENT_TYPE_TREEBOOK_PAGE_CHANGED = 43,
    WXD_EVENT_TYPE_TREEBOOK_PAGE_CHANGING = 44, // Note: May require event.Allow()/Veto()
    WXD_EVENT_TYPE_TREEBOOK_NODE_EXPANDED = 45,
    WXD_EVENT_TYPE_TREEBOOK_NODE_COLLAPSED = 46,
    // ADDED: SearchCtrl Events
    WXD_EVENT_TYPE_COMMAND_SEARCHCTRL_SEARCH_BTN = 47,
    WXD_EVENT_TYPE_COMMAND_SEARCHCTRL_CANCEL_BTN = 48,
    // ADDED: Hyperlink Event
    WXD_EVENT_TYPE_COMMAND_HYPERLINK = 49,
    // ADDED: SpinCtrlDouble Event
    WXD_EVENT_TYPE_SPINCTRLDOUBLE = 50,
    // ADDED: CalendarCtrl Event
    WXD_EVENT_TYPE_CALENDAR_SEL_CHANGED = 51, // wxEVT_CALENDAR_SEL_CHANGED
    // ADDED: ScrollBar Events (wxScrollWinEvent)
    WXD_EVENT_TYPE_SCROLL_TOP = 52,            // wxEVT_SCROLL_TOP
    WXD_EVENT_TYPE_SCROLL_BOTTOM = 53,         // wxEVT_SCROLL_BOTTOM
    WXD_EVENT_TYPE_SCROLL_LINEUP = 54,         // wxEVT_SCROLL_LINEUP
    WXD_EVENT_TYPE_SCROLL_LINEDOWN = 55,       // wxEVT_SCROLL_LINEDOWN
    WXD_EVENT_TYPE_SCROLL_PAGEUP = 56,         // wxEVT_SCROLL_PAGEUP
    WXD_EVENT_TYPE_SCROLL_PAGEDOWN = 57,       // wxEVT_SCROLL_PAGEDOWN
    WXD_EVENT_TYPE_SCROLL_THUMBTRACK = 58,     // wxEVT_SCROLL_THUMBTRACK
    WXD_EVENT_TYPE_SCROLL_THUMBRELEASE = 59,   // wxEVT_SCROLL_THUMBRELEASE
    WXD_EVENT_TYPE_SCROLL_CHANGED = 60,        // wxEVT_SCROLL_CHANGED
    // Add new event types above this line
    WXD_EVENT_TYPE_MAX // Sentinel value, not a real event type
} WXDEventTypeCEnum;

typedef long wxd_Style_t;      // Reverted back to long for style flags
typedef int wxd_Direction_t;   // For flags like wxHORIZONTAL, wxVERTICAL, wxBOTH (typically int)
typedef int wxd_Orientation_t; // For flags like wxHORIZONTAL, wxVERTICAL (typically int)
typedef int wxd_SizerFlags_t;  // For sizer alignment/border flags (typically int)

// NOTE: All constants (IDs, flags, styles, event types) are passed as 
// these C integer types. The constant VALUES are provided by the
// constant extraction mechanism and used by the safe Rust wrapper.

// --- Opaque Struct Typedefs --- 
typedef struct wxd_App_t wxd_App_t;
typedef struct wxd_Window_t wxd_Window_t;
typedef struct wxd_Event_t wxd_Event_t;
typedef struct wxd_EvtHandler_t wxd_EvtHandler_t;
typedef struct wxd_Control_t wxd_Control_t; // MOVED to be with other opaque types, before first use
typedef struct wxd_Frame_t wxd_Frame_t;
typedef struct wxd_Button_t wxd_Button_t;
typedef struct wxd_Sizer_t wxd_Sizer_t;
typedef struct wxd_StaticText_t wxd_StaticText_t;
typedef struct wxd_TextCtrl_t wxd_TextCtrl_t;
typedef struct wxd_CheckBox_t wxd_CheckBox_t;
typedef struct wxd_Panel_t wxd_Panel_t;
typedef struct wxd_RadioButton_t wxd_RadioButton_t;
typedef struct wxd_ListBox_t wxd_ListBox_t; // ADDED: Opaque type for ListBox
typedef struct wxd_Choice_t wxd_Choice_t;     // ADDED: Opaque type for Choice
typedef struct wxd_ComboBox_t wxd_ComboBox_t; // ADDED: Opaque type for ComboBox
typedef struct wxd_CheckListBox_t wxd_CheckListBox_t; // Add opaque type using correct naming convention
typedef struct wxd_ToggleButton_t wxd_ToggleButton_t; // Add opaque type
typedef struct wxd_RadioBox_t wxd_RadioBox_t; // ADDED
typedef struct wxd_BitmapComboBox_t wxd_BitmapComboBox_t; // ADDED

// ADDED: TreeCtrl Opaque Types
typedef struct wxd_TreeCtrl_t wxd_TreeCtrl_t;

// wxTreeItemId is not a pointer type, it's a struct/class. We need an opaque
// pointer to *our* representation of it (which will wrap the wxTreeItemId).
// Note: A wxTreeItemId can be invalid (e.g., before adding a root).
// C++ side will need to handle creation/copying/destruction/validity checks.
typedef struct WXD_TreeItemId_t WXD_TreeItemId_t; 

// ADDED: StaticBox Opaque Type
typedef struct wxd_StaticBox_t wxd_StaticBox_t;

// ADDED: Gauge Opaque Type
typedef struct wxd_Gauge_t wxd_Gauge_t;

// ADDED: Slider Opaque Type
typedef struct wxd_Slider_t wxd_Slider_t;

// ADDED: SpinCtrl Opaque Type
typedef struct wxd_SpinCtrl_t wxd_SpinCtrl_t;

// ADDED: SpinButton Opaque Type
typedef struct wxd_SpinButton_t wxd_SpinButton_t;

// ADDED: Notebook Opaque Type
typedef struct wxd_Notebook_t wxd_Notebook_t;

// ADDED: SplitterWindow Opaque Type
typedef struct wxd_SplitterWindow_t wxd_SplitterWindow_t;

// ADDED: Bitmap Opaque Type
typedef struct wxd_Bitmap_t wxd_Bitmap_t;

// ADDED: BitmapButton Opaque Type
typedef struct wxd_BitmapButton_t wxd_BitmapButton_t;

// ADDED: ScrolledWindow Opaque Type
typedef struct wxd_ScrolledWindow_t wxd_ScrolledWindow_t;

// ADDED: StatusBar Opaque Type
typedef struct wxd_StatusBar_t wxd_StatusBar_t;

// ADDED: ToolBar Opaque Type
typedef struct wxd_ToolBar_t wxd_ToolBar_t;

// --- ADDED: Menu Opaque Types ---
typedef struct wxd_MenuBar_t wxd_MenuBar_t;
typedef struct wxd_Menu_t wxd_Menu_t;
typedef struct wxd_MenuItem_t wxd_MenuItem_t;

// ADDED: ListCtrl Opaque Type
typedef struct wxd_ListCtrl_t wxd_ListCtrl_t;

// ADDED: ColourPickerCtrl Opaque Type
typedef struct wxd_ColourPickerCtrl_t wxd_ColourPickerCtrl_t;

// ADDED: DatePickerCtrl Opaque Type
typedef struct wxd_DatePickerCtrl_t wxd_DatePickerCtrl_t;

// C struct for wxDateTime
typedef struct {
    short day;        // 1-31
    unsigned short month; // 0-11 for wxDateTime (Jan=0), adjust in C++ layer
    int year;         // e.g., 2023
    short hour;       // 0-23
    short minute;     // 0-59
    short second;     // 0-59
} wxd_DateTime_t;

// --- Function Pointer Typedefs --- 
// Passed directly to wxd_Main
typedef bool (*wxd_OnInitCallback)(void* userData);

// NEW: Callback type for Rust closures (trampoline function pointer)
// Matches the signature of rust_event_handler_trampoline in Rust
typedef void (*wxd_ClosureCallback)(wxd_Event_t* event, void* closure_ptr);

// Define wxEventType explicitly if not pulled in robustly by #include <wx/event.h>
// Bindgen might struggle otherwise. Rely on wxWidgets for the actual definition.
// typedef int wxEventType; // REMOVED - Replaced by wxd_EventType_t definition above

// --- Function Declarations --- 

WXD_EXPORTED wxd_App_t* wxd_GetApp();
WXD_EXPORTED int wxd_Main(int argc, char** argv, wxd_OnInitCallback on_init, void* userData);
WXD_EXPORTED void wxd_App_SetTopWindow(wxd_App_t* app, wxd_Window_t* window);
// void wxd_App_ExitMainLoop(wxd_App_t* app); // Define if needed

// *** Use the new stable C enum for event type ***
WXD_EXPORTED void wxd_EvtHandler_Bind(
    wxd_EvtHandler_t* handler, 
    WXDEventTypeCEnum eventTypeC,       // Use the C enum type
    void* rust_trampoline_fn,          
    void* rust_closure_ptr             
);

// REMOVED: Unbind is handled automatically by CxxClosureVoid destructor
// WXD_EXPORTED void wxd_EvtHandler_Unbind(wxd_EvtHandler_t* handler, wxd_Id id, wxd_EventType_t eventTypeC);

// Keep existing wxd_Window_SetSizer*, etc.
WXD_EXPORTED void wxd_Window_SetSizer(wxd_Window_t* window, wxd_Sizer_t* sizer, bool deleteOldSizer);
WXD_EXPORTED void wxd_Window_SetSizerAndFit(wxd_Window_t* window, wxd_Sizer_t* sizer, bool deleteOldSizer);
WXD_EXPORTED int wxd_Window_GetId(wxd_Window_t* window);
// ADDED: Common window methods
WXD_EXPORTED void wxd_Window_Fit(wxd_Window_t* window);
WXD_EXPORTED wxd_Size wxd_Window_GetBestSize(wxd_Window_t* window);
// void wxd_Window_Show(wxd_Window_t* window, bool show); // Define if needed
WXD_EXPORTED void wxd_Window_Destroy(wxd_Window_t* window);

// ADDED: Set background color
WXD_EXPORTED void wxd_Window_SetBackgroundColor(wxd_Window_t* window, wxd_Colour_t color);

// ADDED: Set minimum size
WXD_EXPORTED void wxd_Window_SetMinSize(wxd_Window_t* window, wxd_Size size);

// ADDED: Refresh window
WXD_EXPORTED void wxd_Window_Refresh(wxd_Window_t* window, int eraseBackground, const wxd_Rect* rect);

// ADDED: Set ToolTip for a window/widget
WXD_EXPORTED void wxd_Window_SetToolTip(wxd_Window_t* window, const char* tipString);

WXD_EXPORTED wxd_Id wxd_Event_GetId(wxd_Event_t* event);
WXD_EXPORTED wxd_Window_t* wxd_Event_GetEventObject(wxd_Event_t* event);
// Ensure Skip is present, correct signature (bool skip)
WXD_EXPORTED void wxd_Event_Skip(wxd_Event_t* event, bool skip); // Add bool argument

// --- Event Data Accessors ---
WXD_EXPORTED int wxd_CommandEvent_GetString(wxd_Event_t* event, char* buffer, int buffer_len); // Returns length needed, like GetLabel
WXD_EXPORTED bool wxd_CommandEvent_IsChecked(wxd_Event_t* event); // Returns check state, false if not applicable
WXD_EXPORTED wxd_Point wxd_MouseEvent_GetPosition(wxd_Event_t* event); // Returns Point, { -1, -1 } if not applicable?
WXD_EXPORTED int wxd_KeyEvent_GetKeyCode(wxd_Event_t* event); // Returns key code, maybe 0 if not applicable?
WXD_EXPORTED int wxd_CommandEvent_GetInt(wxd_Event_t* event); // Returns int value, or -1 if not applicable/error

// ADDED: Scroll Event Data Accessors
WXD_EXPORTED int wxd_ScrollEvent_GetPosition(wxd_Event_t* event);
WXD_EXPORTED int wxd_ScrollEvent_GetOrientation(wxd_Event_t* event);

// ADDED: Notebook Event Data Accessor
WXD_EXPORTED int wxd_NotebookEvent_GetSelection(wxd_Event_t* event);
WXD_EXPORTED int wxd_NotebookEvent_GetOldSelection(wxd_Event_t* event);

// ADDED: Splitter Event Data Accessor
WXD_EXPORTED int wxd_SplitterEvent_GetSashPosition(wxd_Event_t* event);
// WXD_EXPORTED int wxd_SplitterEvent_GetX(wxd_Event_t* event); // Add if needed for dbl click
// WXD_EXPORTED int wxd_SplitterEvent_GetY(wxd_Event_t* event); // Add if needed for dbl click

// ADDED: ColourPicker Event Data Accessor
WXD_EXPORTED wxd_Colour_t wxd_ColourPickerEvent_GetColour(wxd_Event_t* event);

WXD_EXPORTED wxd_Frame_t* wxd_Frame_Create(wxd_Window_t* parent, wxd_Id id, const char* title, wxd_Point pos, wxd_Size size, wxd_Style_t style); // Use wxd_Style_t
WXD_EXPORTED void wxd_Frame_Destroy(wxd_Frame_t* frame);
WXD_EXPORTED void wxd_Frame_Show(wxd_Frame_t* frame, bool show);
WXD_EXPORTED void wxd_Frame_SetTitle(wxd_Frame_t* frame, const char* title);
WXD_EXPORTED void wxd_Frame_Centre(wxd_Frame_t* frame, wxd_Direction_t direction); // Use wxd_Direction_t
WXD_EXPORTED void wxd_Frame_Close(wxd_Frame_t* frame, bool force); // ADDED
WXD_EXPORTED void wxd_Frame_SetMenuBar(wxd_Frame_t* frame, wxd_MenuBar_t* menubar);
WXD_EXPORTED void wxd_Frame_SetStatusBar(wxd_Frame_t* frame, wxd_StatusBar_t* statusBar); // Now the type is known
WXD_EXPORTED void wxd_Frame_SetToolBar(wxd_Frame_t* frame, wxd_ToolBar_t* toolBar); // ADDED
WXD_EXPORTED wxd_ToolBar_t* wxd_Frame_CreateToolBar(wxd_Frame_t* frame, wxd_Style_t style, wxd_Id id);

WXD_EXPORTED wxd_Button_t* wxd_Button_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos, wxd_Size size, wxd_Style_t style); // Use wxd_Style_t
WXD_EXPORTED void wxd_Button_Destroy(wxd_Button_t* button);
WXD_EXPORTED void wxd_Button_SetLabel(wxd_Button_t* button, const char* label);
WXD_EXPORTED int wxd_Button_GetLabel(wxd_Button_t* button, char* buffer, int buffer_len);

WXD_EXPORTED wxd_StaticText_t* wxd_StaticText_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos, wxd_Size size, wxd_Style_t style); // Use wxd_Style_t
WXD_EXPORTED void wxd_StaticText_Destroy(wxd_StaticText_t* stext);
WXD_EXPORTED void wxd_StaticText_SetLabel(wxd_StaticText_t* stext, const char* label);
WXD_EXPORTED int wxd_StaticText_GetLabel(wxd_StaticText_t* stext, char* buffer, int buffer_len);

WXD_EXPORTED wxd_TextCtrl_t* wxd_TextCtrl_Create(wxd_Window_t* parent, wxd_Id id, const char* value, wxd_Point pos, wxd_Size size, wxd_Style_t style); // Use wxd_Style_t
WXD_EXPORTED void wxd_TextCtrl_SetValue(wxd_TextCtrl_t* textCtrl, const char* value);
WXD_EXPORTED int wxd_TextCtrl_GetValue(wxd_TextCtrl_t* textCtrl, char* buffer, int buffer_len);

WXD_EXPORTED wxd_CheckBox_t* wxd_CheckBox_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos, wxd_Size size, wxd_Style_t style); // Use wxd_Style_t
WXD_EXPORTED bool wxd_CheckBox_IsChecked(wxd_CheckBox_t* checkBox);
WXD_EXPORTED void wxd_CheckBox_SetValue(wxd_CheckBox_t* checkBox, bool value);

WXD_EXPORTED wxd_Sizer_t* wxd_BoxSizer_Create(wxd_Orientation_t orient); // Use wxd_Orientation_t
WXD_EXPORTED void wxd_Sizer_AddWindow(
    wxd_Sizer_t* sizer, 
    wxd_Window_t* window, 
    int proportion, 
    wxd_SizerFlags_t flag, // Use wxd_SizerFlags_t
    int border
);

WXD_EXPORTED void wxd_Sizer_AddSpacer(wxd_Sizer_t* sizer, int size);
WXD_EXPORTED void wxd_Sizer_AddSizer(
    wxd_Sizer_t* sizer, 
    wxd_Sizer_t* childSizer, 
    int proportion, 
    wxd_SizerFlags_t flag, // Use wxd_SizerFlags_t
    int border
);
WXD_EXPORTED void wxd_Sizer_AddStretchSpacer(wxd_Sizer_t* sizer, int prop);

// --- Event Type Constants (Wrapper Defines) ---
// REMOVED temporary defines. Constants will be defined in the safe Rust wrapper.

// Declare the Rust function that C++ will call to drop the closure Box
// NOTE: This function is IMPLEMENTED IN RUST
WXD_EXPORTED void drop_rust_closure_box(void* ptr);

// --- Window Client Data (Cleanup Notifier) ---
// REMOVED: WXDRAGON_API void wxd_Window_SetRustClientData(wxd_Window_t* win, void* ptr);
// REMOVED: WXDRAGON_API void* wxd_Window_GetRustClientData(wxd_Window_t* win);
// REMOVED: WXDRAGON_API void* wxd_Window_TakeRustClientData(wxd_Window_t* win);

// Attaches a C++ object to the window whose destructor will notify Rust.
WXD_EXPORTED void wxd_Window_AttachCleanupNotifier(wxd_Window_t* win_ptr);

// Detaches the C++ notifier object *without* triggering the notification.
// Used when Rust takes back ownership of the data.
WXD_EXPORTED void wxd_Window_DetachCleanupNotifier(wxd_Window_t* win_ptr);

// Declare the Rust function C++ calls when the notifier is destroyed.
// It receives the original window pointer to identify which data to clean up.
// NOTE: This function is IMPLEMENTED IN RUST.
WXD_EXPORTED void notify_rust_of_cleanup(wxd_Window_t* win_ptr);

// ADDED Function declaration for wxPanel
WXD_EXPORTED wxd_Panel_t* wxd_Panel_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);

// ADDED Function declarations for wxRadioButton
WXD_EXPORTED wxd_RadioButton_t* wxd_RadioButton_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED bool wxd_RadioButton_GetValue(wxd_RadioButton_t* radio);
WXD_EXPORTED void wxd_RadioButton_SetValue(wxd_RadioButton_t* radio, bool value);

// --- ADDED: wxListBox Functions ---
WXD_EXPORTED wxd_ListBox_t* wxd_ListBox_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style
);
WXD_EXPORTED void wxd_ListBox_Append(wxd_ListBox_t* self, const char* item);
WXD_EXPORTED void wxd_ListBox_Clear(wxd_ListBox_t* listbox);
WXD_EXPORTED int wxd_ListBox_GetSelection(wxd_ListBox_t* listbox); // Returns index or -1 (wxNOT_FOUND)
WXD_EXPORTED int wxd_ListBox_GetStringSelection(wxd_ListBox_t* listbox, char* buffer, int buffer_len); // Returns length needed, copies string
WXD_EXPORTED void wxd_ListBox_SetSelection(wxd_ListBox_t* listbox, int index, bool select);
 // Note: SetSelection(index, true) only works for single/extended selection modes
WXD_EXPORTED int wxd_ListBox_GetString(wxd_ListBox_t* listbox, int index, char* buffer, int buffer_len); // Returns length needed, copies string
WXD_EXPORTED unsigned int wxd_ListBox_GetCount(wxd_ListBox_t* listbox);

// --- ADDED: wxChoice Functions ---
WXD_EXPORTED wxd_Choice_t* wxd_Choice_Create(
    wxd_Window_t* parent, 
    wxd_Id id, 
    wxd_Point pos, 
    wxd_Size size, 
    wxd_Style_t style
);
WXD_EXPORTED void wxd_Choice_Append(wxd_Choice_t* self, const char* item);
WXD_EXPORTED void wxd_Choice_Clear(wxd_Choice_t* choice);
WXD_EXPORTED int wxd_Choice_GetSelection(wxd_Choice_t* choice); // Returns index or -1 (wxNOT_FOUND)
WXD_EXPORTED int wxd_Choice_GetStringSelection(wxd_Choice_t* choice, char* buffer, int buffer_len); // Returns length needed, copies string
WXD_EXPORTED void wxd_Choice_SetSelection(wxd_Choice_t* choice, int index);
WXD_EXPORTED int wxd_Choice_GetString(wxd_Choice_t* choice, int index, char* buffer, int buffer_len); // Returns length needed, copies string
WXD_EXPORTED unsigned int wxd_Choice_GetCount(wxd_Choice_t* choice);

// --- ADDED: wxComboBox Functions ---
WXD_EXPORTED wxd_ComboBox_t* wxd_ComboBox_Create(wxd_Window_t* parent, wxd_Id id, const char* value, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void wxd_ComboBox_Append(wxd_ComboBox_t* combo, const char* item);
WXD_EXPORTED void wxd_ComboBox_Clear(wxd_ComboBox_t* combo);
WXD_EXPORTED int wxd_ComboBox_GetSelection(wxd_ComboBox_t* combo); // Returns index or -1 (wxNOT_FOUND)
WXD_EXPORTED int wxd_ComboBox_GetStringSelection(wxd_ComboBox_t* combo, char* buffer, int buffer_len); // Returns length needed, copies string
WXD_EXPORTED void wxd_ComboBox_SetSelection(wxd_ComboBox_t* combo, int index);
WXD_EXPORTED int wxd_ComboBox_GetString(wxd_ComboBox_t* combo, int index, char* buffer, int buffer_len); // Returns length needed, copies string
WXD_EXPORTED unsigned int wxd_ComboBox_GetCount(wxd_ComboBox_t* combo);
WXD_EXPORTED void wxd_ComboBox_SetValue(wxd_ComboBox_t* combo, const char* value); // Like TextCtrl
WXD_EXPORTED int wxd_ComboBox_GetValue(wxd_ComboBox_t* combo, char* buffer, int buffer_len); // Like TextCtrl

// --- ADDED: wxBitmapComboBox Functions ---
WXD_EXPORTED wxd_BitmapComboBox_t* wxd_BitmapComboBox_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    const char* value, 
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style
);
WXD_EXPORTED void wxd_BitmapComboBox_Append(
    wxd_BitmapComboBox_t* self, 
    const char* item, 
    wxd_Bitmap_t* bitmap // Can be NULL
);
WXD_EXPORTED void wxd_BitmapComboBox_Clear(wxd_BitmapComboBox_t* self);
WXD_EXPORTED int wxd_BitmapComboBox_GetSelection(wxd_BitmapComboBox_t* self);
WXD_EXPORTED void wxd_BitmapComboBox_SetSelection(wxd_BitmapComboBox_t* self, int index);
WXD_EXPORTED int wxd_BitmapComboBox_GetString(wxd_BitmapComboBox_t* self, int index, char* buffer, int buffer_len);
WXD_EXPORTED unsigned int wxd_BitmapComboBox_GetCount(wxd_BitmapComboBox_t* self);
WXD_EXPORTED void wxd_BitmapComboBox_SetValue(wxd_BitmapComboBox_t* self, const char* value);
WXD_EXPORTED int wxd_BitmapComboBox_GetValue(wxd_BitmapComboBox_t* self, char* buffer, int buffer_len);
WXD_EXPORTED wxd_Bitmap_t* wxd_BitmapComboBox_GetItemBitmap(wxd_BitmapComboBox_t* self, unsigned int n);
WXD_EXPORTED void wxd_BitmapComboBox_SetItemBitmap(wxd_BitmapComboBox_t* self, unsigned int n, wxd_Bitmap_t* bitmap);
// Note: GetItemBitmap returns a *new* wxd_Bitmap_t handle that Rust must manage (create Bitmap from ptr and drop).

// --- ADDED: wxCheckListBox Functions ---
WXD_EXPORTED wxd_CheckListBox_t* wxd_CheckListBox_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void wxd_CheckListBox_Append(wxd_CheckListBox_t* clbox, const char* item);
WXD_EXPORTED void wxd_CheckListBox_Clear(wxd_CheckListBox_t* clbox);
WXD_EXPORTED int wxd_CheckListBox_GetSelection(wxd_CheckListBox_t* clbox); // Returns index or -1 (wxNOT_FOUND)
WXD_EXPORTED int wxd_CheckListBox_GetStringSelection(wxd_CheckListBox_t* clbox, char* buffer, int buffer_len); // Returns length needed, copies string
WXD_EXPORTED void wxd_CheckListBox_SetSelection(wxd_CheckListBox_t* clbox, int index, bool select);
WXD_EXPORTED int wxd_CheckListBox_GetString(wxd_CheckListBox_t* clbox, int index, char* buffer, int buffer_len); // Returns length needed, copies string
WXD_EXPORTED unsigned int wxd_CheckListBox_GetCount(wxd_CheckListBox_t* clbox);
WXD_EXPORTED bool wxd_CheckListBox_IsChecked(wxd_CheckListBox_t* clbox, unsigned int index);
WXD_EXPORTED void wxd_CheckListBox_Check(wxd_CheckListBox_t* clbox, unsigned int index, bool check);

// --- ADDED: wxToggleButton Functions ---
WXD_EXPORTED wxd_ToggleButton_t* wxd_ToggleButton_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED bool wxd_ToggleButton_GetValue(wxd_ToggleButton_t* tglbtn);
WXD_EXPORTED void wxd_ToggleButton_SetValue(wxd_ToggleButton_t* tglbtn, bool state);
WXD_EXPORTED void wxd_ToggleButton_SetLabel(wxd_ToggleButton_t* tglbtn, const char* label);
WXD_EXPORTED int wxd_ToggleButton_GetLabel(wxd_ToggleButton_t* tglbtn, char* buffer, int buffer_len);

// --- ADDED: wxTreeCtrl Functions ---
WXD_EXPORTED wxd_TreeCtrl_t* wxd_TreeCtrl_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED WXD_TreeItemId_t* wxd_TreeCtrl_AddRoot(wxd_TreeCtrl_t* self, const char* text, int image, int selImage, void* data);
WXD_EXPORTED WXD_TreeItemId_t* wxd_TreeCtrl_AppendItem(wxd_TreeCtrl_t* self, WXD_TreeItemId_t* parent_id, const char* text, int image, int selImage, void* data);
WXD_EXPORTED void wxd_TreeCtrl_Delete(wxd_TreeCtrl_t* self, WXD_TreeItemId_t* item_id);
WXD_EXPORTED WXD_TreeItemId_t* wxd_TreeCtrl_GetSelection(wxd_TreeCtrl_t* self); // Returns a NEW item ID ptr, needs freeing
WXD_EXPORTED void wxd_TreeCtrl_SelectItem(wxd_TreeCtrl_t* self, WXD_TreeItemId_t* item_id);
// TODO: Consider functions like: DeleteChildren, GetItemText, SetItemText, GetRootItem, GetItemParent, GetChildrenCount, Collapse, Expand, etc.

// --- ADDED: wxTreeItemId Functions ---
// Need functions to manage the opaque WXD_TreeItemId_t
WXD_EXPORTED void wxd_TreeItemId_Free(WXD_TreeItemId_t* item_id); // Free the memory allocated for the C struct
WXD_EXPORTED bool wxd_TreeItemId_IsOk(WXD_TreeItemId_t* item_id); // Check if the underlying wxTreeItemId is valid

// --- ADDED: wxTreeEvent Functions ---
WXD_EXPORTED WXD_TreeItemId_t* wxd_TreeEvent_GetItem(wxd_Event_t* event); // Returns a NEW item ID ptr, needs freeing
WXD_EXPORTED int wxd_TreeEvent_GetLabel(wxd_Event_t* event, char* buffer, int buffer_len); // For label editing

// --- ADDED: wxStaticBox Functions ---
WXD_EXPORTED wxd_StaticBox_t* wxd_StaticBox_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos, wxd_Size size, wxd_Style_t style);

// --- StaticBoxSizer ---
typedef struct wxd_StaticBoxSizer_t wxd_StaticBoxSizer_t;
// This one already existed, it's for creating from an existing StaticBox.
// WXD_EXPORTED wxd_StaticBoxSizer_t *wxd_StaticBoxSizer_Create(wxd_StaticBox_t *box, wxd_Orientation_t orient);
// The C++ implementation for the above is actually wxd_StaticBoxSizer_Create_WithBox based on sizer.cpp
// Let's make the C API match the C++ implementation names for clarity.

WXD_EXPORTED wxd_StaticBoxSizer_t *wxd_StaticBoxSizer_Create_WithBox(wxd_StaticBox_t* box, wxd_Orientation_t orient);
WXD_EXPORTED wxd_StaticBoxSizer_t *wxd_StaticBoxSizer_Create_WithLabel(wxd_Orientation_t orient, wxd_Window_t* parent, const char* label);
WXD_EXPORTED wxd_StaticBox_t *wxd_StaticBoxSizer_GetStaticBox(wxd_StaticBoxSizer_t *self);

// --- ADDED: Menu Functions ---
WXD_EXPORTED wxd_MenuBar_t* wxd_MenuBar_Create(wxd_Style_t style); // Style might be needed (e.g., wxMB_DOCKABLE)
WXD_EXPORTED void wxd_MenuBar_Append(wxd_MenuBar_t* menubar, wxd_Menu_t* menu, const char* title);

WXD_EXPORTED wxd_Menu_t* wxd_Menu_Create(const char* title, wxd_Style_t style);
WXD_EXPORTED void wxd_Menu_Destroy(wxd_Menu_t* menu);
WXD_EXPORTED wxd_MenuItem_t* wxd_Menu_Append(wxd_Menu_t* menu, wxd_Id id, const char* item, const char* helpString, int kind /* wxItemKind, e.g., wxITEM_NORMAL */);
WXD_EXPORTED void wxd_Menu_AppendSeparator(wxd_Menu_t* menu);

// Note: MenuItem creation is combined with Append for simplicity now. If separate needed, add wxd_MenuItem_Create.
// WXD_EXPORTED wxd_MenuItem_t* wxd_MenuItem_Create(wxd_Menu_t* parentMenu, wxd_Id id, const char* text, const char* helpString, int kind);
WXD_EXPORTED void wxd_MenuItem_Destroy(wxd_MenuItem_t* item); // If manual destruction needed (unlikely?)

// -- wxGauge --
// Use the defined opaque types
WXD_EXPORTED wxd_Gauge_t *wxd_Gauge_Create(wxd_Window_t *parent, wxd_Id id, int range, int x, int y, int w, int h, wxd_Style_t style);
WXD_EXPORTED void wxd_Gauge_SetRange(wxd_Gauge_t *self, int range);
WXD_EXPORTED void wxd_Gauge_SetValue(wxd_Gauge_t *self, int value);
WXD_EXPORTED int wxd_Gauge_GetValue(const wxd_Gauge_t *self);

// --- ADDED: wxSlider Functions ---
WXD_EXPORTED wxd_Slider_t* wxd_Slider_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    int value,
    int minValue,
    int maxValue,
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style
);
WXD_EXPORTED int wxd_Slider_GetValue(wxd_Slider_t* self);
WXD_EXPORTED void wxd_Slider_SetValue(wxd_Slider_t* self, int value);
WXD_EXPORTED void wxd_Slider_SetRange(wxd_Slider_t* self, int minValue, int maxValue);
WXD_EXPORTED int wxd_Slider_GetMin(wxd_Slider_t* self);
WXD_EXPORTED int wxd_Slider_GetMax(wxd_Slider_t* self);

// --- ADDED: wxSpinCtrl Functions ---
WXD_EXPORTED wxd_SpinCtrl_t* wxd_SpinCtrl_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    const char* value, // Initial value as string
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style,
    int min_val, // Explicit min/max required for spinctrl
    int max_val,
    int initial_val
);
WXD_EXPORTED int wxd_SpinCtrl_GetValue(wxd_SpinCtrl_t* self);
WXD_EXPORTED void wxd_SpinCtrl_SetValue(wxd_SpinCtrl_t* self, int value);
// Can also set value via string if needed: wxd_SpinCtrl_SetValueString
WXD_EXPORTED void wxd_SpinCtrl_SetRange(wxd_SpinCtrl_t* self, int minVal, int maxVal);
WXD_EXPORTED int wxd_SpinCtrl_GetMin(wxd_SpinCtrl_t* self);
WXD_EXPORTED int wxd_SpinCtrl_GetMax(wxd_SpinCtrl_t* self);

// --- ADDED: wxSpinButton Functions ---
WXD_EXPORTED wxd_SpinButton_t* wxd_SpinButton_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style
);
WXD_EXPORTED int wxd_SpinButton_GetValue(wxd_SpinButton_t* self);
WXD_EXPORTED void wxd_SpinButton_SetValue(wxd_SpinButton_t* self, int value);
WXD_EXPORTED void wxd_SpinButton_SetRange(wxd_SpinButton_t* self, int minVal, int maxVal);
WXD_EXPORTED int wxd_SpinButton_GetMin(wxd_SpinButton_t* self);
WXD_EXPORTED int wxd_SpinButton_GetMax(wxd_SpinButton_t* self);

// --- ADDED: wxNotebook Functions ---
WXD_EXPORTED wxd_Notebook_t* wxd_Notebook_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED bool wxd_Notebook_AddPage(wxd_Notebook_t* self, wxd_Window_t* page, const char* text, bool select);
WXD_EXPORTED int wxd_Notebook_GetSelection(wxd_Notebook_t* self);
WXD_EXPORTED int wxd_Notebook_SetSelection(wxd_Notebook_t* self, int page);

// --- ADDED: wxSplitterWindow Functions ---
WXD_EXPORTED wxd_SplitterWindow_t* wxd_SplitterWindow_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED bool wxd_SplitterWindow_SplitVertically(wxd_SplitterWindow_t* self, wxd_Window_t* window1, wxd_Window_t* window2, int sashPosition);
WXD_EXPORTED bool wxd_SplitterWindow_SplitHorizontally(wxd_SplitterWindow_t* self, wxd_Window_t* window1, wxd_Window_t* window2, int sashPosition);
WXD_EXPORTED bool wxd_SplitterWindow_Unsplit(wxd_SplitterWindow_t* self, wxd_Window_t* toRemove);
WXD_EXPORTED void wxd_SplitterWindow_Initialize(wxd_SplitterWindow_t* self, wxd_Window_t* window);
WXD_EXPORTED void wxd_SplitterWindow_SetSashPosition(wxd_SplitterWindow_t* self, int position, bool redraw);
WXD_EXPORTED int wxd_SplitterWindow_GetSashPosition(wxd_SplitterWindow_t* self);
WXD_EXPORTED void wxd_SplitterWindow_SetMinimumPaneSize(wxd_SplitterWindow_t* self, int paneSize);

// --- Bitmap ---
WXD_EXPORTED wxd_Bitmap_t* wxd_Bitmap_CreateFromRGBA(const unsigned char* data, int width, int height);
WXD_EXPORTED void wxd_Bitmap_Destroy(wxd_Bitmap_t* bitmap);
// ADDED: Get bitmap dimensions and validity
WXD_EXPORTED int wxd_Bitmap_GetWidth(wxd_Bitmap_t* bitmap);
WXD_EXPORTED int wxd_Bitmap_GetHeight(wxd_Bitmap_t* bitmap);
WXD_EXPORTED bool wxd_Bitmap_IsOk(wxd_Bitmap_t* bitmap);

// --- BitmapButton ---
WXD_EXPORTED wxd_BitmapButton_t* wxd_BitmapButton_Create(
    wxd_Window_t* parent, 
    wxd_Id id, 
    wxd_Bitmap_t* bitmap, 
    wxd_Point pos, 
    wxd_Size size, 
    wxd_Style_t style
);
// TODO: Add SetBitmapLabel, SetBitmapHover etc. later if needed

// --- ArtProvider ---
// Returns a new wxd_Bitmap_t* that the caller is responsible for destroying
// via wxd_Bitmap_Destroy, or nullptr if not found or error.
// size: pass {-1, -1} for default size. Client can be an empty string for wxART_OTHER.
WXD_EXPORTED wxd_Bitmap_t* wxd_ArtProvider_GetBitmap(const char* id, const char* client, wxd_Size size);

// --- StatusBar Functions ---
WXD_EXPORTED wxd_StatusBar_t* wxd_StatusBar_Create(wxd_Window_t* parent /* Should be Frame */, wxd_Id id, wxd_Style_t style);
WXD_EXPORTED void wxd_StatusBar_SetFieldsCount(wxd_StatusBar_t* self, int count);
WXD_EXPORTED void wxd_StatusBar_SetStatusText(wxd_StatusBar_t* self, const char* text, int fieldIndex);
WXD_EXPORTED void wxd_StatusBar_SetStatusWidths(wxd_StatusBar_t* self, int count, const int* widths);
WXD_EXPORTED void wxd_StatusBar_PushStatusText(wxd_StatusBar_t* self, const char* text, int fieldIndex);
WXD_EXPORTED void wxd_StatusBar_PopStatusText(wxd_StatusBar_t* self, int fieldIndex);

// --- ToolBar Functions ---
WXD_EXPORTED wxd_ToolBar_t* wxd_ToolBar_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);
// Note: `kind` uses wxItemKind values (e.g., wxITEM_NORMAL, wxITEM_CHECK, wxITEM_RADIO)
// Use 0, 1, 2? No, use the constants extracted from wxITEM_*. Client code must use WXD_ITEM_*.
// Returns the tool control if created, otherwise NULL (wxToolBarToolBase is opaque for now)
// Consider returning void or a specific wxd_Tool_t later if needed.
WXD_EXPORTED void* /* wxd_ToolBarToolBase_t* */ wxd_ToolBar_AddTool(
    wxd_ToolBar_t* self,
    wxd_Id toolId,
    const char* label,
    wxd_Bitmap_t* bitmap,       // Normal bitmap
    wxd_Bitmap_t* bitmapDisabled, // Disabled bitmap (can be NULL)
    int kind,                   // wxItemKind (WXD_ITEM_*)
    const char* shortHelp,
    const char* longHelp
    // ClientData might be added later if needed for tools
); 
WXD_EXPORTED void wxd_ToolBar_AddSeparator(wxd_ToolBar_t* self);
WXD_EXPORTED void wxd_ToolBar_AddControl(wxd_ToolBar_t* self, wxd_Window_t* control);
WXD_EXPORTED bool wxd_ToolBar_Realize(wxd_ToolBar_t* self);
WXD_EXPORTED void wxd_ToolBar_EnableTool(wxd_ToolBar_t* self, wxd_Id toolId, bool enable);
WXD_EXPORTED void wxd_ToolBar_ToggleTool(wxd_ToolBar_t* self, wxd_Id toolId, bool toggle);
WXD_EXPORTED bool wxd_ToolBar_IsToolEnabled(wxd_ToolBar_t* self, wxd_Id toolId);
WXD_EXPORTED bool wxd_ToolBar_GetToolState(wxd_ToolBar_t* self, wxd_Id toolId);
WXD_EXPORTED void wxd_ToolBar_SetToolShortHelp(wxd_ToolBar_t* self, wxd_Id toolId, const char* helpString);
// WXD_EXPORTED void wxd_ToolBar_SetToolLongHelp(wxd_ToolBar_t* self, wxd_Id toolId, const char* helpString); // Add if needed

// --- TreeCtrl Event Data ---
// TreeItemId might be needed for some events (e.g., GetItem)
// WXD_EXPORTED WXD_TreeItemId_t* wxd_TreeEvent_GetItem(wxd_Event_t* event); // Needs careful implementation

// --- Notebook Event Data ---
WXD_EXPORTED int wxd_NotebookEvent_GetSelection(wxd_Event_t* event);

// --- Splitter Event Data ---
WXD_EXPORTED int wxd_SplitterEvent_GetSashPosition(wxd_Event_t* event);

// --- Base Classes ---
// ... existing code ...
// ... existing code ...

// --- Panel ---
WXD_EXPORTED wxd_Panel_t* wxd_Panel_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);

// --- ScrolledWindow (inherits from Panel/Window) ---
WXD_EXPORTED wxd_ScrolledWindow_t* wxd_ScrolledWindow_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void wxd_ScrolledWindow_SetScrollRate(wxd_ScrolledWindow_t* self, int xstep, int ystep);
WXD_EXPORTED void wxd_ScrolledWindow_SetScrollbars(wxd_ScrolledWindow_t* self, int pixelsPerUnitX, int pixelsPerUnitY, int noUnitsX, int noUnitsY, int xPos, int yPos, bool noRefresh);
WXD_EXPORTED void wxd_ScrolledWindow_EnableScrolling(wxd_ScrolledWindow_t* self, bool xScrolling, bool yScrolling);
WXD_EXPORTED void wxd_ScrolledWindow_Scroll_Coord(wxd_ScrolledWindow_t* self, int x, int y); // wxWidgets has Scroll(x, y) and Scroll(pt)
WXD_EXPORTED void wxd_ScrolledWindow_Scroll_Point(wxd_ScrolledWindow_t* self, wxd_Point pt); // wxWidgets has Scroll(x, y) and Scroll(pt)
WXD_EXPORTED void wxd_ScrolledWindow_GetVirtualSize(wxd_ScrolledWindow_t* self, int* w, int* h);
WXD_EXPORTED void wxd_ScrolledWindow_GetScrollPixelsPerUnit(wxd_ScrolledWindow_t* self, int* xUnit, int* yUnit);

// --- RadioButton ---
WXD_EXPORTED wxd_RadioButton_t* wxd_RadioButton_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos, wxd_Size size, wxd_Style_t style);
// ... existing code ...

// --- ArtProvider ---
// Returns a new wxd_Bitmap_t* that the caller is responsible for destroying
// via wxd_Bitmap_Destroy, or nullptr if not found or error.
// size: pass {-1, -1} for default size. Client can be an empty string for wxART_OTHER.
WXD_EXPORTED wxd_Bitmap_t* wxd_ArtProvider_GetBitmap(const char* id, const char* client, wxd_Size size);

// --- ListCtrl Event Data Accessors ---
WXD_EXPORTED int32_t wxd_ListEvent_GetItemIndex(wxd_Event_t* event); // wxListEvent::GetItem - Changed return type to int32_t
WXD_EXPORTED int wxd_ListEvent_GetColumn(wxd_Event_t* event); // wxListEvent::GetColumn
WXD_EXPORTED int wxd_ListEvent_GetLabel(wxd_Event_t* event, char* buffer, int buffer_len); // wxListEvent::GetLabel or GetText for edit events
WXD_EXPORTED bool wxd_ListEvent_IsEditCancelled(wxd_Event_t* event); // wxListEvent::IsEditCancelled (for end edit event)

// --- wxListCtrl ---
WXD_EXPORTED wxd_ListCtrl_t* wxd_ListCtrl_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED int32_t wxd_ListCtrl_InsertColumn(wxd_ListCtrl_t* self, long col, const char* heading, int format, int width); // Changed return
WXD_EXPORTED bool wxd_ListCtrl_SetColumnWidth(wxd_ListCtrl_t* self, long col, int width);
WXD_EXPORTED int wxd_ListCtrl_GetColumnWidth(wxd_ListCtrl_t* self, long col);
WXD_EXPORTED int wxd_ListCtrl_GetColumnCount(wxd_ListCtrl_t* self);
WXD_EXPORTED int32_t wxd_ListCtrl_InsertItem_Simple(wxd_ListCtrl_t* self, long index, const char* label); // Basic version, returns item ID - Changed return
WXD_EXPORTED void wxd_ListCtrl_SetItemText(wxd_ListCtrl_t* self, long index, const char* text);
WXD_EXPORTED int wxd_ListCtrl_GetItemText(wxd_ListCtrl_t* self, long index, int col, char* buffer, int buffer_len);
WXD_EXPORTED int wxd_ListCtrl_GetItemCount(wxd_ListCtrl_t* self);
WXD_EXPORTED bool wxd_ListCtrl_SetItemState(wxd_ListCtrl_t* self, long item, long state, long stateMask);
WXD_EXPORTED int32_t wxd_ListCtrl_GetItemState(wxd_ListCtrl_t* self, long item, long stateMask); // Changed return
WXD_EXPORTED int32_t wxd_ListCtrl_GetNextItem(wxd_ListCtrl_t* self, long item, int geometry, int state); // geometry: wxLIST_NEXT_ALL etc., state: wxLIST_STATE_SELECTED etc. - Changed return
WXD_EXPORTED bool wxd_ListCtrl_DeleteItem(wxd_ListCtrl_t* self, long item);
WXD_EXPORTED bool wxd_ListCtrl_DeleteAllItems(wxd_ListCtrl_t* self);
WXD_EXPORTED bool wxd_ListCtrl_ClearAll(wxd_ListCtrl_t* self); // Deletes all items and columns
// WXD_EXPORTED void wxd_ListCtrl_SetImageList(wxd_ListCtrl_t* self, wxd_ImageList_t* imageList, int which); // Needs wxd_ImageList_t
// WXD_EXPORTED bool wxd_ListCtrl_SetItemImage(wxd_ListCtrl_t* self, long item, int image, int selImage);
WXD_EXPORTED int wxd_ListCtrl_GetSelectedItemCount(wxd_ListCtrl_t* self);
WXD_EXPORTED bool wxd_ListCtrl_EnsureVisible(wxd_ListCtrl_t* self, long item);
WXD_EXPORTED int32_t wxd_ListCtrl_HitTest(wxd_ListCtrl_t* self, wxd_Point point, int* flags_ptr, long* subitem_ptr); // flags and subitem are out-params - Changed return
WXD_EXPORTED void wxd_ListCtrl_EditLabel(wxd_ListCtrl_t* self, long item);

// wxFrame
typedef struct WXDFrame WXDFrame;
WXD_EXPORTED wxd_StatusBar_t* wxd_Frame_CreateStatusBar(wxd_Frame_t* frame, int number, long style, int id, const char* name); // Added based on README previously
WXD_EXPORTED void wxd_Frame_SetStatusBar(wxd_Frame_t* frame, wxd_StatusBar_t* statbar);

// New Frame methods
WXD_EXPORTED void wxd_Frame_CenterOnScreen(wxd_Frame_t* frame);
WXD_EXPORTED void wxd_Frame_SetStatusText(wxd_Frame_t* frame, const char* text, int number);
WXD_EXPORTED char* wxd_Frame_GetTitle(wxd_Frame_t* frame); // Remember: Returned string must be freed by Rust using wxd_rust_string_free or CString::from_raw
WXD_EXPORTED void wxd_Frame_Iconize(wxd_Frame_t* frame, bool iconize);
WXD_EXPORTED bool wxd_Frame_IsIconized(wxd_Frame_t* frame);
WXD_EXPORTED void wxd_Frame_Maximize(wxd_Frame_t* frame, bool maximize);
WXD_EXPORTED bool wxd_Frame_IsMaximized(wxd_Frame_t* frame);

// wxMenuBar
// ... existing code ...
// General utilities (if not already present)
// Consider adding this to a more general section if not frame-specific,
// or ensure it's declared if it's used by wxd_Frame_GetTitle's Rust counterpart.
// For now, assuming CString::from_raw will handle freeing memory allocated by strdup.
// If a dedicated free function is strictly needed by the guidelines for FFI-returned strings:
// void wxd_rust_string_free(char* str);

// --- ADDED: wxRadioBox Functions ---
WXD_EXPORTED wxd_RadioBox_t* wxd_RadioBox_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    const char* label, 
    wxd_Point pos,
    wxd_Size size,
    int n, 
    const char* const* choices,
    int majorDimension,
    wxd_Style_t style
);
WXD_EXPORTED int wxd_RadioBox_GetSelection(wxd_RadioBox_t* self);
WXD_EXPORTED void wxd_RadioBox_SetSelection(wxd_RadioBox_t* self, int n);
WXD_EXPORTED int wxd_RadioBox_GetString(wxd_RadioBox_t* self, int n, char* buffer, int buffer_len); // Returns needed buffer len + 1, or 0 on success
WXD_EXPORTED unsigned int wxd_RadioBox_GetCount(wxd_RadioBox_t* self);
WXD_EXPORTED bool wxd_RadioBox_EnableItem(wxd_RadioBox_t* self, int n, bool enable);
WXD_EXPORTED bool wxd_RadioBox_IsItemEnabled(wxd_RadioBox_t* self, int n);
WXD_EXPORTED bool wxd_RadioBox_ShowItem(wxd_RadioBox_t* self, int n, bool show);
WXD_EXPORTED bool wxd_RadioBox_IsItemShown(wxd_RadioBox_t* self, int n);

// --- ADDED: wxColourPickerCtrl Functions ---
WXD_EXPORTED wxd_ColourPickerCtrl_t* wxd_ColourPickerCtrl_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    wxd_Colour_t initial_colour,
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style
);
WXD_EXPORTED wxd_Colour_t wxd_ColourPickerCtrl_GetColour(wxd_ColourPickerCtrl_t* self);
WXD_EXPORTED void wxd_ColourPickerCtrl_SetColour(wxd_ColourPickerCtrl_t* self, wxd_Colour_t colour);

// --- ADDED: wxDateTime Helper Functions ---
WXD_EXPORTED wxd_DateTime_t wxd_DateTime_Default();
WXD_EXPORTED wxd_DateTime_t wxd_DateTime_Now();
WXD_EXPORTED wxd_DateTime_t wxd_DateTime_FromComponents(int year, unsigned short month, short day, short hour, short minute, short second);
WXD_EXPORTED bool wxd_DateTime_IsValid(const wxd_DateTime_t* dt);

// --- ADDED: wxDatePickerCtrl Functions ---
WXD_EXPORTED wxd_DatePickerCtrl_t* wxd_DatePickerCtrl_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    const wxd_DateTime_t* dt, // Can be NULL for wxDefaultDateTime
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style
);
WXD_EXPORTED wxd_DateTime_t wxd_DatePickerCtrl_GetValue(wxd_DatePickerCtrl_t* self);
WXD_EXPORTED void wxd_DatePickerCtrl_SetValue(wxd_DatePickerCtrl_t* self, const wxd_DateTime_t* dt);
WXD_EXPORTED bool wxd_DatePickerCtrl_GetRange(wxd_DatePickerCtrl_t* self, wxd_DateTime_t* dt1, wxd_DateTime_t* dt2); // dt1 and dt2 are out-parameters
WXD_EXPORTED void wxd_DatePickerCtrl_SetRange(wxd_DatePickerCtrl_t* self, const wxd_DateTime_t* dt1, const wxd_DateTime_t* dt2);

// wxTreebook
typedef struct wxd_Treebook wxd_Treebook_t;
WXD_EXPORTED wxd_Treebook_t* wxd_Treebook_new(wxd_Window_t* parent, int id, int x, int y, int width, int height, wxd_Style_t style);
WXD_EXPORTED int wxd_Treebook_AddPage(wxd_Treebook_t* self, wxd_Window_t* page, const char* text, int select, int imageId);
WXD_EXPORTED int wxd_Treebook_AddSubPage(wxd_Treebook_t* self, wxd_Window_t* page, const char* text, int select, int imageId);
WXD_EXPORTED int wxd_Treebook_GetPageCount(wxd_Treebook_t* self);
WXD_EXPORTED int wxd_Treebook_GetSelection(wxd_Treebook_t* self);
WXD_EXPORTED int wxd_Treebook_SetSelection(wxd_Treebook_t* self, size_t n);
WXD_EXPORTED void wxd_Treebook_SetPageText(wxd_Treebook_t* self, size_t n, const char* strText);
WXD_EXPORTED int wxd_Treebook_GetPageText(wxd_Treebook_t* self, size_t n, char* buffer, int bufLen);

// wxSearchCtrl
typedef struct wxd_SearchCtrl wxd_SearchCtrl_t;
WXD_EXPORTED wxd_SearchCtrl_t *wxd_SearchCtrl_Create(wxd_Window_t *parent, int id, const char *value, int x, int y, int w, int h, long style);
WXD_EXPORTED void wxd_SearchCtrl_ShowSearchButton(wxd_SearchCtrl_t *self, bool show);
WXD_EXPORTED bool wxd_SearchCtrl_IsSearchButtonVisible(wxd_SearchCtrl_t *self);
WXD_EXPORTED void wxd_SearchCtrl_ShowCancelButton(wxd_SearchCtrl_t *self, bool show);
WXD_EXPORTED bool wxd_SearchCtrl_IsCancelButtonVisible(wxd_SearchCtrl_t *self);
WXD_EXPORTED wxd_Control_t* wxd_SearchCtrl_GetCancelButton(wxd_SearchCtrl_t* self); // Corrected return type
WXD_EXPORTED void wxd_SearchCtrl_SetMenu(wxd_SearchCtrl_t* self, wxd_Menu_t* menu); // Corrected parameter type to wxd_Menu_t*
WXD_EXPORTED wxd_Menu_t* wxd_SearchCtrl_GetMenu(wxd_SearchCtrl_t* self); // Corrected return type to wxd_Menu_t*

// wxHyperlinkCtrl
typedef struct wxd_HyperlinkCtrl wxd_HyperlinkCtrl_t;
WXD_EXPORTED wxd_HyperlinkCtrl_t *wxd_HyperlinkCtrl_Create(wxd_Window_t *parent, int id, const char *label, const char *url, int x, int y, int w, int h, long style);
WXD_EXPORTED const char *wxd_HyperlinkCtrl_GetURL(wxd_HyperlinkCtrl_t *self);
WXD_EXPORTED void wxd_HyperlinkCtrl_SetURL(wxd_HyperlinkCtrl_t *self, const char *url);
WXD_EXPORTED bool wxd_HyperlinkCtrl_GetVisited(wxd_HyperlinkCtrl_t *self);
WXD_EXPORTED void wxd_HyperlinkCtrl_SetVisited(wxd_HyperlinkCtrl_t *self, bool visited);
WXD_EXPORTED unsigned long wxd_HyperlinkCtrl_GetHoverColour(wxd_HyperlinkCtrl_t *self);
WXD_EXPORTED void wxd_HyperlinkCtrl_SetHoverColour(wxd_HyperlinkCtrl_t *self, unsigned long colour);
WXD_EXPORTED unsigned long wxd_HyperlinkCtrl_GetNormalColour(wxd_HyperlinkCtrl_t *self);
WXD_EXPORTED void wxd_HyperlinkCtrl_SetNormalColour(wxd_HyperlinkCtrl_t *self, unsigned long colour);
WXD_EXPORTED unsigned long wxd_HyperlinkCtrl_GetVisitedColour(wxd_HyperlinkCtrl_t *self);
WXD_EXPORTED void wxd_HyperlinkCtrl_SetVisitedColour(wxd_HyperlinkCtrl_t *self, unsigned long colour);

// wxActivityIndicator
typedef struct wxd_ActivityIndicator wxd_ActivityIndicator_t;
WXD_EXPORTED wxd_ActivityIndicator_t *wxd_ActivityIndicator_Create(wxd_Window_t *parent, int id, int x, int y, int w, int h, long style);
WXD_EXPORTED void wxd_ActivityIndicator_Start(wxd_ActivityIndicator_t *self);
WXD_EXPORTED void wxd_ActivityIndicator_Stop(wxd_ActivityIndicator_t *self);
WXD_EXPORTED bool wxd_ActivityIndicator_IsRunning(wxd_ActivityIndicator_t *self);

// wxSpinCtrlDouble
typedef struct wxd_SpinCtrlDouble wxd_SpinCtrlDouble_t;
WXD_EXPORTED wxd_SpinCtrlDouble_t *wxd_SpinCtrlDouble_Create(wxd_Window_t *parent, int id, const char *value, int x, int y, int w, int h, long style, double min_val, double max_val, double initial_val, double inc);
WXD_EXPORTED double wxd_SpinCtrlDouble_GetValue(wxd_SpinCtrlDouble_t *self);
WXD_EXPORTED void wxd_SpinCtrlDouble_SetValue(wxd_SpinCtrlDouble_t *self, double value);
WXD_EXPORTED void wxd_SpinCtrlDouble_SetRange(wxd_SpinCtrlDouble_t *self, double min_val, double max_val);
WXD_EXPORTED double wxd_SpinCtrlDouble_GetMin(wxd_SpinCtrlDouble_t *self);
WXD_EXPORTED double wxd_SpinCtrlDouble_GetMax(wxd_SpinCtrlDouble_t *self);
WXD_EXPORTED void wxd_SpinCtrlDouble_SetIncrements(wxd_SpinCtrlDouble_t *self, double inc); // Changed from SetIncrement to SetIncrements
WXD_EXPORTED double wxd_SpinCtrlDouble_GetIncrement(wxd_SpinCtrlDouble_t *self);
WXD_EXPORTED void wxd_SpinCtrlDouble_SetDigits(wxd_SpinCtrlDouble_t *self, unsigned int digits);
WXD_EXPORTED unsigned int wxd_SpinCtrlDouble_GetDigits(wxd_SpinCtrlDouble_t *self);

// ADDED: CalendarCtrl Opaque Type
typedef struct wxd_CalendarCtrl_t wxd_CalendarCtrl_t;

// --- CalendarCtrl ---
WXD_EXPORTED wxd_CalendarCtrl_t* wxd_CalendarCtrl_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    const wxd_DateTime_t* date, // Initial date, can be NULL for default (current date)
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style
);

WXD_EXPORTED bool wxd_CalendarCtrl_SetDate(wxd_CalendarCtrl_t* self, const wxd_DateTime_t* date);
WXD_EXPORTED wxd_DateTime_t wxd_CalendarCtrl_GetDate(wxd_CalendarCtrl_t* self);

// Event data accessor
// REMOVE: WXD_EXPORTED wxd_DateTime_t wxd_CalendarEvent_GetDate(wxd_Event_t* event);

// --- wxFlexGridSizer ---
typedef struct wxd_FlexGridSizer_t wxd_FlexGridSizer_t;
WXD_EXPORTED wxd_FlexGridSizer_t* wxd_FlexGridSizer_Create(int rows, int cols, int vgap, int hgap);
WXD_EXPORTED wxd_FlexGridSizer_t* wxd_FlexGridSizer_CreateWithGap(int rows, int cols, int gap_width, int gap_height);
WXD_EXPORTED void wxd_FlexGridSizer_AddGrowableCol(wxd_FlexGridSizer_t *self, size_t idx, int proportion);
WXD_EXPORTED void wxd_FlexGridSizer_AddGrowableRow(wxd_FlexGridSizer_t *self, size_t idx, int proportion);
WXD_EXPORTED void wxd_FlexGridSizer_SetFlexibleDirection(wxd_FlexGridSizer_t *self, int direction);
WXD_EXPORTED void wxd_FlexGridSizer_SetNonFlexibleGrowMode(wxd_FlexGridSizer_t *self, int mode);

// wxStaticBitmap
typedef struct wxd_StaticBitmap wxd_StaticBitmap;
WXD_EXPORTED wxd_StaticBitmap* wxd_StaticBitmap_Create(wxd_Window_t* parent, int id, const char* bitmap_path, int x, int y, int width, int height, long style);
WXD_EXPORTED wxd_StaticBitmap* wxd_StaticBitmap_CreateWithBitmap(
    wxd_Window_t* parent,
    int id,
    wxd_Bitmap_t* bitmap, 
    int x, int y, int width, int height,
    long style,
    int scale_mode 
);

// wxStaticLine
typedef struct wxd_StaticLine_t wxd_StaticLine_t;

// wxScrollBar
typedef struct wxd_ScrollBar_t wxd_ScrollBar_t; // Opaque pointer
// wxStaticLine
WXD_EXPORTED wxd_StaticLine_t* wxd_StaticLine_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style,
    const char* name
);

// wxScrollBar
WXD_EXPORTED wxd_ScrollBar_t* wxd_ScrollBar_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style,
    const char* name
);
WXD_EXPORTED void wxd_ScrollBar_SetScrollbar(
    wxd_ScrollBar_t* self,
    int position,
    int thumbSize,
    int range,
    int pageSize,
    bool refresh
);
WXD_EXPORTED int wxd_ScrollBar_GetThumbPosition(wxd_ScrollBar_t* self);

// ADDED: CommandLinkButton Opaque Type
typedef struct wxd_CommandLinkButton_t wxd_CommandLinkButton_t;

// --- wxCommandLinkButton ---
WXD_EXPORTED wxd_CommandLinkButton_t* wxd_CommandLinkButton_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    const char* mainLabel,
    const char* note,
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style
);
// Note: Main label can be set/get via wxd_Button_SetLabel/GetLabel
// Note: Standard button functions like Destroy are inherited via casting to wxd_Button_t* or wxd_Window_t*

WXD_EXPORTED void wxd_CommandLinkButton_SetNote(wxd_CommandLinkButton_t* self, const char* note);
// No GetNote is explicitly exposed here, as it's less common. If needed, it can be added later.

// Opaque struct for wxDialog
typedef struct wxd_Dialog wxd_Dialog_t;

// wxDialog
WXD_EXPORTED int wxd_Dialog_ShowModal(wxd_Dialog_t* self);
// Note: Creation and Destruction will be handled by derived dialog classes or generic wxd_Window_Destroy.

// wxMessageDialog
typedef struct wxd_MessageDialog wxd_MessageDialog_t; // Forward declaration

// wxMessageDialog
WXD_EXPORTED wxd_MessageDialog_t* wxd_MessageDialog_Create(wxd_Window_t* parent, const char* message, const char* caption, wxd_Style_t style);
// Note: ShowModal is via wxd_Dialog_ShowModal((wxd_Dialog*)dialog)
// Note: Destruction handled by wxd_Window_Destroy((wxd_Window_t*)dialog)

// --- ArrayString for GetPaths/GetFilenames ---
typedef struct wxd_ArrayString {
    void* internal_data;
} wxd_ArrayString_t;

WXD_EXPORTED wxd_ArrayString_t* wxd_ArrayString_Create();
WXD_EXPORTED void wxd_ArrayString_Free(wxd_ArrayString_t* self);
WXD_EXPORTED int wxd_ArrayString_GetCount(wxd_ArrayString_t* self);
WXD_EXPORTED int wxd_ArrayString_GetString(wxd_ArrayString_t* self, int index, char* buffer, int bufLen);

// --- wxFileDialog ---
typedef struct wxd_FileDialog wxd_FileDialog_t;

WXD_EXPORTED wxd_FileDialog_t* wxd_FileDialog_Create(
    wxd_Window_t* parent,
    const char* message,
    const char* defaultDir,
    const char* defaultFile,
    const char* wildcard,
    wxd_Style_t style,
    int x, int y,
    int width, int height);

WXD_EXPORTED int wxd_FileDialog_GetPath(wxd_FileDialog_t* self, char* buffer, int bufLen);
WXD_EXPORTED void wxd_FileDialog_GetPaths(wxd_FileDialog_t* self, wxd_ArrayString_t* paths);
WXD_EXPORTED int wxd_FileDialog_GetFilename(wxd_FileDialog_t* self, char* buffer, int bufLen);
WXD_EXPORTED void wxd_FileDialog_GetFilenames(wxd_FileDialog_t* self, wxd_ArrayString_t* filenames);
WXD_EXPORTED int wxd_FileDialog_GetDirectory(wxd_FileDialog_t* self, char* buffer, int bufLen);
WXD_EXPORTED int wxd_FileDialog_GetFilterIndex(wxd_FileDialog_t* self);

// --- wxColourData ---
typedef struct wxd_ColourData wxd_ColourData_t;

WXD_EXPORTED wxd_ColourData_t* wxd_ColourData_Create(void);
WXD_EXPORTED void wxd_ColourData_SetColour(wxd_ColourData_t* self, wxd_Colour_t colour);
WXD_EXPORTED wxd_Colour_t wxd_ColourData_GetColour(wxd_ColourData_t* self);
WXD_EXPORTED void wxd_ColourData_Destroy(wxd_ColourData_t* self);

// --- wxColourDialog ---
typedef struct wxd_ColourDialog wxd_ColourDialog_t;

WXD_EXPORTED wxd_ColourDialog_t* wxd_ColourDialog_Create(
    wxd_Window_t* parent,
    const char* title,
    wxd_ColourData_t* data);

WXD_EXPORTED wxd_ColourData_t* wxd_ColourDialog_GetColourData(wxd_ColourDialog_t* self);

// Setters (optional for now, can be added later if needed by Rust wrapper)
// WXD_EXPORTED void wxd_FileDialog_SetMessage(wxd_FileDialog_t* self, const char* message);
// WXD_EXPORTED void wxd_FileDialog_SetPath(wxd_FileDialog_t* self, const char* path);

// --- wxTextEntryDialog ---
typedef struct wxd_TextEntryDialog wxd_TextEntryDialog_t;

WXD_EXPORTED wxd_TextEntryDialog_t* wxd_TextEntryDialog_Create(
    wxd_Window_t* parent,
    const char* message,
    const char* caption,
    const char* defaultValue,
    wxd_Style_t style,
    int x, int y,
    int width, int height);

WXD_EXPORTED int wxd_TextEntryDialog_GetValue(wxd_TextEntryDialog_t* self, char* buffer, int bufLen);

// Setters (optional)
// WXD_EXPORTED void wxd_TextEntryDialog_SetValue(wxd_TextEntryDialog_t* self, const char* value);

#ifdef __cplusplus
}

// C++ specific utility functions or class declarations can go here, outside extern "C"
namespace wxd_cpp_utils {
    // Declaration for the utility function used by GET_WX_STRING_RESULT macro
    // Its definition is in wxd_utils.cpp (or will be if not already)
    size_t copy_wxstring_to_buffer(const wxString& str, char* buffer, size_t buffer_len);
}

#endif // __cplusplus

#endif // WXDRAGON_H
