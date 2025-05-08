#include "wx/wxprec.h"

#ifdef __BORLANDC__
    #pragma hdrstop
#endif

#ifndef WX_PRECOMP
    #include "wx/wx.h"
#endif

#include "wx/treectrl.h"
#include "../include/wxdragon.h"
#include "wxd_utils.h"

// Wrapper struct for wxTreeItemId
// wxTreeItemId itself cannot be directly passed across FFI as it might contain complex C++ members.
// We pass an opaque pointer to this struct instead.
struct WXD_TreeItemId_t {
    wxTreeItemId id;
};

extern "C" {

// --- wxTreeCtrl Functions ---

WXD_EXPORTED wxd_TreeCtrl_t* wxd_TreeCtrl_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style) {
    wxWindow* parentWin = (wxWindow*)parent;
    wxTreeCtrl* ctrl = new wxTreeCtrl(parentWin, id, wxPoint(pos.x, pos.y), wxSize(size.width, size.height), style);
    // Automatically attach the cleanup notifier
    wxd_Window_AttachCleanupNotifier((wxd_Window_t*)ctrl); 
    return (wxd_TreeCtrl_t*)ctrl;
}

// Note: Simplified AddRoot/AppendItem - ignores image and data for now.
// We return a *newly allocated* WXD_TreeItemId_t wrapping the result. Rust side is responsible for freeing it via wxd_TreeItemId_Free.
WXD_EXPORTED WXD_TreeItemId_t* wxd_TreeCtrl_AddRoot(wxd_TreeCtrl_t* self, const char* text, int image, int selImage, void* data) {
    wxTreeCtrl* ctrl = (wxTreeCtrl*)self;
    if (!ctrl) return nullptr;
    // Ignore image, selImage for now.
    // Pass nullptr for data, as wxTreeCtrl expects wxTreeItemData*.
    // TODO: Implement proper wxTreeItemData wrapping if needed later.
    wxTreeItemId rootId = ctrl->AddRoot(wxString::FromUTF8(text), image, selImage, nullptr /*(wxClientData*)data*/);
    if (!rootId.IsOk()) return nullptr;

    WXD_TreeItemId_t* wxd_id = new WXD_TreeItemId_t{rootId};
    return wxd_id;
}

WXD_EXPORTED WXD_TreeItemId_t* wxd_TreeCtrl_AppendItem(wxd_TreeCtrl_t* self, WXD_TreeItemId_t* parent_id_wxd, const char* text, int image, int selImage, void* data) {
    wxTreeCtrl* ctrl = (wxTreeCtrl*)self;
    if (!ctrl || !parent_id_wxd) return nullptr;
    wxTreeItemId parentId = parent_id_wxd->id;
    if (!parentId.IsOk()) return nullptr; // Don't append to invalid parent

    // Ignore image, selImage for now.
    // Pass nullptr for data.
    // TODO: Implement proper wxTreeItemData wrapping if needed later.
    wxTreeItemId newItemId = ctrl->AppendItem(parentId, wxString::FromUTF8(text), image, selImage, nullptr /*(wxClientData*)data*/);
    if (!newItemId.IsOk()) return nullptr;

    WXD_TreeItemId_t* wxd_id = new WXD_TreeItemId_t{newItemId};
    return wxd_id;
}

WXD_EXPORTED void wxd_TreeCtrl_Delete(wxd_TreeCtrl_t* self, WXD_TreeItemId_t* item_id_wxd) {
    wxTreeCtrl* ctrl = (wxTreeCtrl*)self;
    if (!ctrl || !item_id_wxd) return;
    wxTreeItemId itemId = item_id_wxd->id;
    if (itemId.IsOk()) { // Check validity before deleting
        ctrl->Delete(itemId);
    }
    // Note: We don't delete the item_id_wxd C++ struct here, 
    // Rust should still call wxd_TreeItemId_Free on its pointer after this call returns.
}

WXD_EXPORTED WXD_TreeItemId_t* wxd_TreeCtrl_GetSelection(wxd_TreeCtrl_t* self) {
    wxTreeCtrl* ctrl = (wxTreeCtrl*)self;
    if (!ctrl) return nullptr;
    wxTreeItemId selectedId = ctrl->GetSelection();
    if (!selectedId.IsOk()) return nullptr;

    WXD_TreeItemId_t* wxd_id = new WXD_TreeItemId_t{selectedId};
    return wxd_id;
}

WXD_EXPORTED void wxd_TreeCtrl_SelectItem(wxd_TreeCtrl_t* self, WXD_TreeItemId_t* item_id_wxd) {
    wxTreeCtrl* ctrl = (wxTreeCtrl*)self;
    if (!ctrl || !item_id_wxd) return;
    wxTreeItemId itemId = item_id_wxd->id;
    // wxWidgets SelectItem handles invalid IDs gracefully (does nothing)
    ctrl->SelectItem(itemId);
}

// --- wxTreeItemId Functions ---

WXD_EXPORTED void wxd_TreeItemId_Free(WXD_TreeItemId_t* item_id_wxd) {
    delete item_id_wxd; // Free the C++ wrapper struct allocated in AddRoot/AppendItem/GetSelection etc.
}

WXD_EXPORTED bool wxd_TreeItemId_IsOk(WXD_TreeItemId_t* item_id_wxd) {
    if (!item_id_wxd) return false;
    return item_id_wxd->id.IsOk();
}

// --- wxTreeEvent Functions ---

// Helper to get the wxTreeEvent from the generic wxEvent
wxTreeEvent* GetTreeEvent(wxd_Event_t* event) {
    if (!event) return nullptr;
    // Use dynamic_cast for type safety, though static_cast is often used in wx examples
    wxTreeEvent* treeEvent = dynamic_cast<wxTreeEvent*>((wxEvent*)event);
    return treeEvent;
}

WXD_EXPORTED WXD_TreeItemId_t* wxd_TreeEvent_GetItem(wxd_Event_t* event) {
    wxTreeEvent* treeEvent = GetTreeEvent(event);
    if (!treeEvent) return nullptr;
    
    wxTreeItemId itemId = treeEvent->GetItem();
    if (!itemId.IsOk()) return nullptr;

    WXD_TreeItemId_t* wxd_id = new WXD_TreeItemId_t{itemId};
    return wxd_id;
}

WXD_EXPORTED int wxd_TreeEvent_GetLabel(wxd_Event_t* event, char* buffer, int buffer_len) {
    wxTreeEvent* treeEvent = GetTreeEvent(event);
    if (!treeEvent) {
        if (buffer && buffer_len > 0) buffer[0] = '\0';
        return -1; // Indicate error or nothing to copy
    }

    wxString label = treeEvent->GetLabel();
    size_t source_len_no_null = wxd_cpp_utils::copy_wxstring_to_buffer(label, buffer, static_cast<size_t>(buffer_len));

    // Adapt return value to the existing FFI contract:
    // If buffer_len was sufficient (i.e. buffer_len > source_len_no_null for the string + null term),
    // the old code returned source_len_no_null (bytes copied excluding null).
    // If buffer_len was insufficient, the old code returned source_len_no_null + 1 (total needed for string + null).
    if (buffer && buffer_len > 0 && static_cast<size_t>(buffer_len) > source_len_no_null) {
        return static_cast<int>(source_len_no_null); // Bytes copied (excluding null)
    } else {
        // Buffer was NULL, zero length, or too small.
        // Return total needed size for string + null terminator.
        return static_cast<int>(source_len_no_null + 1);
    }
}

} // extern "C" 