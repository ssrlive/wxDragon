#ifndef WXD_EVENT_API_H
#define WXD_EVENT_API_H

#include "../wxd_types.h"

// --- Event Handling & Data Access --- 
WXD_EXPORTED void wxd_EvtHandler_Bind(
    wxd_EvtHandler_t* handler, 
    WXDEventTypeCEnum eventTypeC,
    void* rust_trampoline_fn,          
    void* rust_closure_ptr             
);

WXD_EXPORTED wxd_Id wxd_Event_GetId(wxd_Event_t* event);
WXD_EXPORTED wxd_Window_t* wxd_Event_GetEventObject(wxd_Event_t* event);
WXD_EXPORTED void wxd_Event_Skip(wxd_Event_t* event, bool skip);

WXD_EXPORTED int wxd_CommandEvent_GetString(wxd_Event_t* event, char* buffer, int buffer_len);
WXD_EXPORTED bool wxd_CommandEvent_IsChecked(wxd_Event_t* event);
WXD_EXPORTED wxd_Point wxd_MouseEvent_GetPosition(wxd_Event_t* event);
WXD_EXPORTED int wxd_KeyEvent_GetKeyCode(wxd_Event_t* event);
WXD_EXPORTED int wxd_CommandEvent_GetInt(wxd_Event_t* event);

WXD_EXPORTED int wxd_ScrollEvent_GetPosition(wxd_Event_t* event);
WXD_EXPORTED int wxd_ScrollEvent_GetOrientation(wxd_Event_t* event);

WXD_EXPORTED int wxd_NotebookEvent_GetSelection(wxd_Event_t* event);
WXD_EXPORTED int wxd_NotebookEvent_GetOldSelection(wxd_Event_t* event);

WXD_EXPORTED int wxd_SplitterEvent_GetSashPosition(wxd_Event_t* event);

WXD_EXPORTED wxd_Colour_t wxd_ColourPickerEvent_GetColour(wxd_Event_t* event);

WXD_EXPORTED wxd_TreeItemId_t* wxd_TreeEvent_GetItem(wxd_Event_t* event);
WXD_EXPORTED int wxd_TreeEvent_GetLabel(wxd_Event_t* event, char* buffer, int buffer_len);

WXD_EXPORTED int32_t wxd_ListEvent_GetItemIndex(wxd_Event_t* event);
WXD_EXPORTED int wxd_ListEvent_GetColumn(wxd_Event_t* event);
WXD_EXPORTED int wxd_ListEvent_GetLabel(wxd_Event_t* event, char* buffer, int buffer_len);
WXD_EXPORTED bool wxd_ListEvent_IsEditCancelled(wxd_Event_t* event);

// DataView event accessors
WXD_EXPORTED bool wxd_DataViewEvent_GetColumn(wxd_Event_t* event, int32_t* column);
WXD_EXPORTED bool wxd_DataViewEvent_GetRow(wxd_Event_t* event, int64_t* row);
WXD_EXPORTED bool wxd_DataViewEvent_GetValue(wxd_Event_t* event, wxd_Variant_t* value);
WXD_EXPORTED bool wxd_DataViewEvent_SetValue(wxd_Event_t* event, const wxd_Variant_t* value);
WXD_EXPORTED bool wxd_DataViewEvent_IsEditCancelled(wxd_Event_t* event);

// Rust callback for dropping closure box
WXD_EXPORTED void drop_rust_closure_box(void* ptr);

// Rust callback for cleanup notifier
WXD_EXPORTED void notify_rust_of_cleanup(wxd_Window_t* win_ptr);


#endif // WXD_EVENT_API_H 