#include "wxdragon.h"
#include <wx/wx.h>
#include <wx/listctrl.h>
#include <wx/string.h> // For wxString::FromUTF8 / wxString::ToUTF8
#include "wxd_utils.h" // Added for copy_wxstring_to_buffer

// --- wxListCtrl ---

extern "C" {

WXD_EXPORTED wxd_ListCtrl_t* wxd_ListCtrl_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style) {
    return reinterpret_cast<wxd_ListCtrl_t*>(
        new wxListCtrl(
            reinterpret_cast<wxWindow*>(parent),
            id,
            wxPoint(pos.x, pos.y),
            wxSize(size.width, size.height),
            style
        )
    );
}

WXD_EXPORTED int32_t wxd_ListCtrl_InsertColumn(wxd_ListCtrl_t* self, long col, const char* heading, int format, int width) {
    if (!self) return -1;
    return static_cast<int32_t>(reinterpret_cast<wxListCtrl*>(self)->InsertColumn(col, wxString::FromUTF8(heading), format, width));
}

WXD_EXPORTED bool wxd_ListCtrl_SetColumnWidth(wxd_ListCtrl_t* self, long col, int width) {
    if (!self) return false;
    return reinterpret_cast<wxListCtrl*>(self)->SetColumnWidth(col, width);
}

WXD_EXPORTED int wxd_ListCtrl_GetColumnWidth(wxd_ListCtrl_t* self, long col) {
    if (!self) return -1; // wxLIST_AUTOSIZE_USEHEADER or actual width
    return reinterpret_cast<wxListCtrl*>(self)->GetColumnWidth(col);
}

WXD_EXPORTED int wxd_ListCtrl_GetColumnCount(wxd_ListCtrl_t* self) {
    if (!self) return 0;
    return reinterpret_cast<wxListCtrl*>(self)->GetColumnCount();
}

WXD_EXPORTED int32_t wxd_ListCtrl_InsertItem_Simple(wxd_ListCtrl_t* self, long index, const char* label) {
    if (!self) return -1;
    wxListItem item;
    item.SetId(index); // This sets the position where item is inserted
    item.SetText(wxString::FromUTF8(label));
    // For other views, you might set image, etc.
    // item.SetMask(wxLIST_MASK_TEXT | wxLIST_MASK_IMAGE | wxLIST_MASK_DATA); // if using data/image
    return static_cast<int32_t>(reinterpret_cast<wxListCtrl*>(self)->InsertItem(item));
}

WXD_EXPORTED void wxd_ListCtrl_SetItemText(wxd_ListCtrl_t* self, long index, const char* text) {
    if (!self) return;
    // Use the 2-argument SetItemText (likely for the main item label)
    reinterpret_cast<wxListCtrl*>(self)->SetItemText(index, wxString::FromUTF8(text ? text : ""));
}

WXD_EXPORTED int wxd_ListCtrl_GetItemText(wxd_ListCtrl_t* self, long index, int col, char* buffer, int buffer_len) {
    if (!self) return -1;
    wxString text = reinterpret_cast<wxListCtrl*>(self)->GetItemText(index, col);
    // Use the utility function
    size_t source_len_no_null = wxd_cpp_utils::copy_wxstring_to_buffer(text, buffer, static_cast<size_t>(buffer_len));
    return static_cast<int>(source_len_no_null); // Return original length, caller can check against buffer_len
}

WXD_EXPORTED int wxd_ListCtrl_GetItemCount(wxd_ListCtrl_t* self) {
    if (!self) return 0;
    return reinterpret_cast<wxListCtrl*>(self)->GetItemCount();
}

WXD_EXPORTED bool wxd_ListCtrl_SetItemState(wxd_ListCtrl_t* self, long item, long state, long stateMask) {
    if (!self) return false;
    return reinterpret_cast<wxListCtrl*>(self)->SetItemState(item, state, stateMask);
}

WXD_EXPORTED int32_t wxd_ListCtrl_GetItemState(wxd_ListCtrl_t* self, long item, long stateMask) {
    if (!self) return 0; // Or some error indicator
    return static_cast<int32_t>(reinterpret_cast<wxListCtrl*>(self)->GetItemState(item, stateMask));
}

WXD_EXPORTED int32_t wxd_ListCtrl_GetNextItem(wxd_ListCtrl_t* self, long item, int geometry, int state) {
    if (!self) return -1; // wxNOT_FOUND
    return static_cast<int32_t>(reinterpret_cast<wxListCtrl*>(self)->GetNextItem(item, geometry, state));
}

WXD_EXPORTED bool wxd_ListCtrl_DeleteItem(wxd_ListCtrl_t* self, long item) {
    if (!self) return false;
    return reinterpret_cast<wxListCtrl*>(self)->DeleteItem(item);
}

WXD_EXPORTED bool wxd_ListCtrl_DeleteAllItems(wxd_ListCtrl_t* self) {
    if (!self) return false;
    return reinterpret_cast<wxListCtrl*>(self)->DeleteAllItems();
}

WXD_EXPORTED bool wxd_ListCtrl_ClearAll(wxd_ListCtrl_t* self) {
    if (!self) return false;
    reinterpret_cast<wxListCtrl*>(self)->ClearAll(); // wxListCtrl::ClearAll is void, but DeleteAllColumns is bool
    return reinterpret_cast<wxListCtrl*>(self)->DeleteAllColumns();
}

WXD_EXPORTED int wxd_ListCtrl_GetSelectedItemCount(wxd_ListCtrl_t* self) {
    if (!self) return 0;
    return reinterpret_cast<wxListCtrl*>(self)->GetSelectedItemCount();
}

WXD_EXPORTED bool wxd_ListCtrl_EnsureVisible(wxd_ListCtrl_t* self, long item) {
    if (!self) return false;
    return reinterpret_cast<wxListCtrl*>(self)->EnsureVisible(item);
}

WXD_EXPORTED int32_t wxd_ListCtrl_HitTest(wxd_ListCtrl_t* self, wxd_Point point, int* flags_ptr, long* subitem_ptr) {
    if (!self || !flags_ptr || !subitem_ptr) return -1; // wxNOT_FOUND
    wxPoint pt(point.x, point.y);
    int flags;
    long subitem; // wxListCtrl uses long for subitem index in HitTest
    long item = reinterpret_cast<wxListCtrl*>(self)->HitTest(pt, flags, &subitem);
    *flags_ptr = flags;
    *subitem_ptr = subitem;
    return static_cast<int32_t>(item);
}

WXD_EXPORTED void wxd_ListCtrl_EditLabel(wxd_ListCtrl_t* self, long item) {
    if (!self) return;
    reinterpret_cast<wxListCtrl*>(self)->EditLabel(item);
}

// --- ListCtrl Event Data Accessors ---
WXD_EXPORTED int32_t wxd_ListEvent_GetItemIndex(wxd_Event_t* event) {
    if (!event) return -1;
    wxListEvent* evt = static_cast<wxListEvent*>(reinterpret_cast<wxEvent*>(event));
    return static_cast<int32_t>(evt->GetIndex());
}

WXD_EXPORTED int wxd_ListEvent_GetColumn(wxd_Event_t* event) {
    if (!event) return -1; // Or some other error value
    wxListEvent* evt = static_cast<wxListEvent*>(reinterpret_cast<wxEvent*>(event));
    return evt->GetColumn(); // For column click events
}

WXD_EXPORTED int wxd_ListEvent_GetLabel(wxd_Event_t* event, char* buffer, int buffer_len) {
    if (!event) { // Simplified check
        if (buffer && buffer_len > 0) buffer[0] = '\0';
        return -1; // Or 0?
    }
    wxListEvent* evt = static_cast<wxListEvent*>(reinterpret_cast<wxEvent*>(event));
    wxString label = evt->GetLabel(); // or evt->GetText()
    
    size_t source_len_no_null = wxd_cpp_utils::copy_wxstring_to_buffer(label, buffer, static_cast<size_t>(buffer_len));
    return static_cast<int>(source_len_no_null);
}

WXD_EXPORTED bool wxd_ListEvent_IsEditCancelled(wxd_Event_t* event) {
    if (!event) return true; // Default to cancelled if event is null
    wxListEvent* evt = static_cast<wxListEvent*>(reinterpret_cast<wxEvent*>(event));
    return evt->IsEditCancelled();
}

// --- wxListBox ---
// WXD_EXPORTED wxd_ListBox_t* wxd_ListBox_Create( // REMOVE THIS FUNCTION DEFINITION
//     wxd_Window_t* parent,
//     wxd_Id id,
//     wxd_Point pos,
//     wxd_Size size,
//     wxd_Style_t style
// ) {
//     if (!parent) return NULL;
//     wxListBox* listbox = new wxListBox(
//         reinterpret_cast<wxWindow*>(parent),
//         id,
//         wxPoint(pos.x, pos.y),
//         wxSize(size.width, size.height),
//         0, // n, initially empty
//         NULL, // choices, initially empty
//         style
//     );
//     return reinterpret_cast<wxd_ListBox_t*>(listbox);
// }

} // extern "C" 