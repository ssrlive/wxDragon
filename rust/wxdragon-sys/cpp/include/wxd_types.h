#ifndef WXD_TYPES_H
#define WXD_TYPES_H

// Use standard C types
#include <stdbool.h> 
#include <stdint.h> // For integer types if needed
#include <stddef.h> // For size_t

// --- Basic Types --- 
typedef int wxd_Id;

typedef struct {
    int x, y;
} wxd_Point;

typedef struct {
    int width, height;
} wxd_Size;

typedef struct wxd_Rect {
    int x;
    int y;
    int width;
    int height;
} wxd_Rect;

typedef struct {
    unsigned char r;
    unsigned char g;
    unsigned char b;
    unsigned char a;
} wxd_Colour_t;

typedef enum {
    WXD_EVENT_TYPE_NULL = 0,
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
    WXD_EVENT_TYPE_COMMAND_RADIOBOX_SELECTED = 16,
    WXD_EVENT_TYPE_COMMAND_LISTBOX_SELECTED = 17,
    WXD_EVENT_TYPE_COMMAND_CHOICE_SELECTED = 18,
    WXD_EVENT_TYPE_COMMAND_COMBOBOX_SELECTED = 19,
    WXD_EVENT_TYPE_COMMAND_CHECKLISTBOX_SELECTED = 20,
    WXD_EVENT_TYPE_COMMAND_TOGGLEBUTTON_CLICKED = 21,
    WXD_EVENT_TYPE_TREE_BEGIN_LABEL_EDIT = 22,
    WXD_EVENT_TYPE_TREE_END_LABEL_EDIT = 23,
    WXD_EVENT_TYPE_TREE_SEL_CHANGED = 24,
    WXD_EVENT_TYPE_TREE_ITEM_ACTIVATED = 25,
    WXD_EVENT_TYPE_SLIDER = 26,
    WXD_EVENT_TYPE_SPINCTRL = 27,
    WXD_EVENT_TYPE_SPIN_UP = 28,
    WXD_EVENT_TYPE_SPIN_DOWN = 29,
    WXD_EVENT_TYPE_SPIN = 30,
    WXD_EVENT_TYPE_NOTEBOOK_PAGE_CHANGED = 31,
    WXD_EVENT_TYPE_SPLITTER_SASH_POS_CHANGED = 32,
    WXD_EVENT_TYPE_SPLITTER_SASH_POS_CHANGING = 33,
    WXD_EVENT_TYPE_SPLITTER_DOUBLECLICKED = 34,
    WXD_EVENT_TYPE_SPLITTER_UNSPLIT = 35,
    WXD_EVENT_TYPE_LIST_ITEM_SELECTED = 36,
    WXD_EVENT_TYPE_LIST_ITEM_ACTIVATED = 37,
    WXD_EVENT_TYPE_LIST_COL_CLICK = 38,
    WXD_EVENT_TYPE_LIST_BEGIN_LABEL_EDIT = 39,
    WXD_EVENT_TYPE_LIST_END_LABEL_EDIT = 40,
    WXD_EVENT_TYPE_COLOURPICKER_CHANGED = 41,
    WXD_EVENT_TYPE_DATE_CHANGED = 42,
    WXD_EVENT_TYPE_TREEBOOK_PAGE_CHANGED = 43,
    WXD_EVENT_TYPE_TREEBOOK_PAGE_CHANGING = 44,
    WXD_EVENT_TYPE_TREEBOOK_NODE_EXPANDED = 45,
    WXD_EVENT_TYPE_TREEBOOK_NODE_COLLAPSED = 46,
    WXD_EVENT_TYPE_COMMAND_SEARCHCTRL_SEARCH_BTN = 47,
    WXD_EVENT_TYPE_COMMAND_SEARCHCTRL_CANCEL_BTN = 48,
    WXD_EVENT_TYPE_COMMAND_HYPERLINK = 49,
    WXD_EVENT_TYPE_SPINCTRLDOUBLE = 50,
    WXD_EVENT_TYPE_CALENDAR_SEL_CHANGED = 51,
    WXD_EVENT_TYPE_SCROLL_TOP = 52,
    WXD_EVENT_TYPE_SCROLL_BOTTOM = 53,
    WXD_EVENT_TYPE_SCROLL_LINEUP = 54,
    WXD_EVENT_TYPE_SCROLL_LINEDOWN = 55,
    WXD_EVENT_TYPE_SCROLL_PAGEUP = 56,
    WXD_EVENT_TYPE_SCROLL_PAGEDOWN = 57,
    WXD_EVENT_TYPE_SCROLL_THUMBTRACK = 58,
    WXD_EVENT_TYPE_SCROLL_THUMBRELEASE = 59,
    WXD_EVENT_TYPE_SCROLL_CHANGED = 60,
    WXD_EVENT_TYPE_FILEPICKER_CHANGED = 61,
    WXD_EVENT_TYPE_MAX
} WXDEventTypeCEnum;

typedef long wxd_Style_t;
typedef int wxd_Direction_t;
typedef int wxd_Orientation_t;
typedef int wxd_SizerFlags_t;

// --- Opaque Struct Typedefs --- 
typedef struct wxd_App_t wxd_App_t;
typedef struct wxd_Window_t wxd_Window_t;
typedef struct wxd_Event_t wxd_Event_t;
typedef struct wxd_EvtHandler_t wxd_EvtHandler_t;
typedef struct wxd_Control_t wxd_Control_t;
typedef struct wxd_Frame_t wxd_Frame_t;
typedef struct wxd_Button_t wxd_Button_t;
typedef struct wxd_Sizer_t wxd_Sizer_t;
typedef struct wxd_StaticText_t wxd_StaticText_t;
typedef struct wxd_TextCtrl_t wxd_TextCtrl_t;
typedef struct wxd_CheckBox_t wxd_CheckBox_t;
typedef struct wxd_Panel_t wxd_Panel_t;
typedef struct wxd_RadioButton_t wxd_RadioButton_t;
typedef struct wxd_ListBox_t wxd_ListBox_t;
typedef struct wxd_Choice_t wxd_Choice_t;
typedef struct wxd_ComboBox_t wxd_ComboBox_t;
typedef struct wxd_CheckListBox_t wxd_CheckListBox_t;
typedef struct wxd_ToggleButton_t wxd_ToggleButton_t;
typedef struct wxd_RadioBox_t wxd_RadioBox_t;
typedef struct wxd_BitmapComboBox_t wxd_BitmapComboBox_t;
typedef struct wxd_TreeCtrl_t wxd_TreeCtrl_t;
typedef struct WXD_TreeItemId_t WXD_TreeItemId_t; 
typedef struct wxd_StaticBox_t wxd_StaticBox_t;
typedef struct wxd_Gauge_t wxd_Gauge_t;
typedef struct wxd_Slider_t wxd_Slider_t;
typedef struct wxd_SpinCtrl_t wxd_SpinCtrl_t;
typedef struct wxd_SpinButton_t wxd_SpinButton_t;
typedef struct wxd_Notebook_t wxd_Notebook_t;
typedef struct wxd_SplitterWindow_t wxd_SplitterWindow_t;
typedef struct wxd_Bitmap_t wxd_Bitmap_t;
typedef struct wxd_BitmapButton_t wxd_BitmapButton_t;
typedef struct wxd_ScrolledWindow_t wxd_ScrolledWindow_t;
typedef struct wxd_StatusBar_t wxd_StatusBar_t;
typedef struct wxd_ToolBar_t wxd_ToolBar_t;
typedef struct wxd_MenuBar_t wxd_MenuBar_t;
typedef struct wxd_Menu_t wxd_Menu_t;
typedef struct wxd_MenuItem_t wxd_MenuItem_t;
typedef struct wxd_ListCtrl_t wxd_ListCtrl_t;
typedef struct wxd_ColourPickerCtrl_t wxd_ColourPickerCtrl_t;
typedef struct wxd_DatePickerCtrl_t wxd_DatePickerCtrl_t;
typedef struct wxd_Treebook wxd_Treebook_t;
typedef struct wxd_SearchCtrl wxd_SearchCtrl_t;
typedef struct wxd_HyperlinkCtrl wxd_HyperlinkCtrl_t;
typedef struct wxd_ActivityIndicator wxd_ActivityIndicator_t;
typedef struct wxd_SpinCtrlDouble wxd_SpinCtrlDouble_t;
typedef struct wxd_CalendarCtrl_t wxd_CalendarCtrl_t;
typedef struct wxd_FlexGridSizer_t wxd_FlexGridSizer_t;
typedef struct wxd_StaticBoxSizer_t wxd_StaticBoxSizer_t;
typedef struct wxd_StaticBitmap_t wxd_StaticBitmap_t;
typedef struct wxd_StaticLine_t wxd_StaticLine_t;
typedef struct wxd_ScrollBar_t wxd_ScrollBar_t;
typedef struct wxd_CommandLinkButton_t wxd_CommandLinkButton_t;
typedef struct wxd_Dialog wxd_Dialog_t;
typedef struct wxd_MessageDialog wxd_MessageDialog_t;
typedef struct wxd_ArrayString { void* internal_data; } wxd_ArrayString_t;
typedef struct wxd_FileDialog wxd_FileDialog_t;
typedef struct wxd_ColourData wxd_ColourData_t;
typedef struct wxd_ColourDialog wxd_ColourDialog_t;
typedef struct wxd_FontData wxd_FontData_t;
typedef struct wxd_Font wxd_Font_t;
typedef struct wxd_FontDialog wxd_FontDialog_t;
typedef struct wxd_TextEntryDialog wxd_TextEntryDialog_t;
typedef struct wxd_ProgressDialog wxd_ProgressDialog_t;
typedef struct wxd_AnimationCtrl wxd_AnimationCtrl_t;
typedef struct wxd_FilePickerCtrl_t wxd_FilePickerCtrl_t;

typedef struct {
    short day;
    unsigned short month;
    int year;
    short hour;
    short minute;
    short second;
} wxd_DateTime_t;

// --- Function Pointer Typedefs --- 
typedef bool (*wxd_OnInitCallback)(void* userData);
typedef void (*wxd_ClosureCallback)(wxd_Event_t* event, void* closure_ptr);

#endif // WXD_TYPES_H 