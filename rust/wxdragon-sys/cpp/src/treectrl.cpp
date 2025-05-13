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
#include <unordered_map>
#include <mutex>

// Wrapper struct for wxTreeItemId
// wxTreeItemId itself cannot be directly passed across FFI as it might contain complex C++ members.
// We pass an opaque pointer to this struct instead.
struct WXD_TreeItemId_t {
    wxTreeItemId id;
};

// Custom wxTreeItemData subclass that carries our client data pointer
class TreeItemDataImpl : public wxTreeItemData {
public:
    TreeItemDataImpl(void* client_data) : client_data_(client_data) {}
    void* GetClientData() const { return client_data_; }
    void SetClientData(void* data) { client_data_ = data; }

private:
    void* client_data_;
};

// Wrapper struct for wxTreeItemData
// This provides a way to associate arbitrary data with tree items.
// The FFI will store a client_data pointer to allow Rust to associate its own data.
struct WXD_TreeItemData_t {
    TreeItemDataImpl* impl;
    
    WXD_TreeItemData_t(void* client_data_ptr) {
        impl = new TreeItemDataImpl(client_data_ptr);
    }
    
    ~WXD_TreeItemData_t() {
        // impl will be deleted by wxTreeCtrl when the item is deleted
        // so we don't delete it here to avoid double-deletion
    }
};

// Data registry to keep track of all tree item data
// This allows us to retrieve the actual data from Rust
class TreeItemDataRegistry {
public:
    // Register a data pointer
    void RegisterData(const void* impl_ptr, void* client_data) {
        std::lock_guard<std::mutex> lock(mutex_);
        data_registry_[impl_ptr] = client_data;
    }
    
    // Lookup a data pointer
    void* LookupData(const void* impl_ptr) {
        std::lock_guard<std::mutex> lock(mutex_);
        auto it = data_registry_.find(impl_ptr);
        if (it != data_registry_.end()) {
            return it->second;
        }
        return nullptr;
    }
    
    // Remove a data pointer
    void RemoveData(const void* impl_ptr) {
        std::lock_guard<std::mutex> lock(mutex_);
        data_registry_.erase(impl_ptr);
    }
    
    // Get the singleton instance
    static TreeItemDataRegistry& GetInstance() {
        static TreeItemDataRegistry instance;
        return instance;
    }
    
private:
    TreeItemDataRegistry() = default;
    std::unordered_map<const void*, void*> data_registry_;
    std::mutex mutex_;
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

// Create a new TreeItemData object to associate with tree items
WXD_EXPORTED WXD_TreeItemData_t* wxd_TreeItemData_Create(void* client_data) {
    auto data = new WXD_TreeItemData_t(client_data);
    // Register this data in the registry
    TreeItemDataRegistry::GetInstance().RegisterData(data->impl, client_data);
    return data;
}

// Free a TreeItemData object
WXD_EXPORTED void wxd_TreeItemData_Free(WXD_TreeItemData_t* data) {
    if (!data) return;
    
    // Remove from registry if we have an impl pointer
    if (data->impl) {
        TreeItemDataRegistry::GetInstance().RemoveData(data->impl);
    }
    
    delete data;
}

// Get the client data from a TreeItemData object
WXD_EXPORTED void* wxd_TreeItemData_GetClientData(WXD_TreeItemData_t* data) {
    if (!data || !data->impl) return nullptr;
    
    // First try to directly get from the impl
    void* client_data = data->impl->GetClientData();
    
    // If that doesn't work, try the registry
    if (!client_data) {
        client_data = TreeItemDataRegistry::GetInstance().LookupData(data->impl);
    }
    
    return client_data;
}

// Set the client data for a TreeItemData object
WXD_EXPORTED void wxd_TreeItemData_SetClientData(WXD_TreeItemData_t* data, void* client_data) {
    if (!data || !data->impl) return;
    data->impl->SetClientData(client_data);
    
    // Update the registry
    TreeItemDataRegistry::GetInstance().RegisterData(data->impl, client_data);
}

// Updated to properly handle TreeItemData
WXD_EXPORTED WXD_TreeItemId_t* wxd_TreeCtrl_AddRoot(wxd_TreeCtrl_t* self, const char* text, int image, int selImage, void* data) {
    wxTreeCtrl* ctrl = (wxTreeCtrl*)self;
    if (!ctrl) return nullptr;
    
    wxTreeItemId rootId;
    
    if (data) {
        WXD_TreeItemData_t* wxd_data = static_cast<WXD_TreeItemData_t*>(data);
        rootId = ctrl->AddRoot(wxString::FromUTF8(text), image, selImage, wxd_data->impl);
    } else {
        rootId = ctrl->AddRoot(wxString::FromUTF8(text), image, selImage, nullptr);
    }
    
    if (!rootId.IsOk()) return nullptr;

    WXD_TreeItemId_t* wxd_id = new WXD_TreeItemId_t{rootId};
    return wxd_id;
}

// Updated to properly handle TreeItemData
WXD_EXPORTED WXD_TreeItemId_t* wxd_TreeCtrl_AppendItem(wxd_TreeCtrl_t* self, WXD_TreeItemId_t* parent_id_wxd, const char* text, int image, int selImage, void* data) {
    wxTreeCtrl* ctrl = (wxTreeCtrl*)self;
    if (!ctrl || !parent_id_wxd) return nullptr;
    wxTreeItemId parentId = parent_id_wxd->id;
    if (!parentId.IsOk()) return nullptr; // Don't append to invalid parent

    wxTreeItemId newItemId;
    
    if (data) {
        WXD_TreeItemData_t* wxd_data = static_cast<WXD_TreeItemData_t*>(data);
        newItemId = ctrl->AppendItem(parentId, wxString::FromUTF8(text), image, selImage, wxd_data->impl);
    } else {
        newItemId = ctrl->AppendItem(parentId, wxString::FromUTF8(text), image, selImage, nullptr);
    }
    
    if (!newItemId.IsOk()) return nullptr;

    WXD_TreeItemId_t* wxd_id = new WXD_TreeItemId_t{newItemId};
    return wxd_id;
}

// Get TreeItemData from an item
WXD_EXPORTED WXD_TreeItemData_t* wxd_TreeCtrl_GetItemData(wxd_TreeCtrl_t* self, WXD_TreeItemId_t* item_id_wxd) {
    wxTreeCtrl* ctrl = (wxTreeCtrl*)self;
    if (!ctrl || !item_id_wxd) return nullptr;
    wxTreeItemId itemId = item_id_wxd->id;
    if (!itemId.IsOk()) return nullptr;
    
    // Get the tree item data
    TreeItemDataImpl* impl = static_cast<TreeItemDataImpl*>(ctrl->GetItemData(itemId));
    if (!impl) return nullptr;
    
    // Create a new WXD_TreeItemData_t that wraps the existing impl
    WXD_TreeItemData_t* data = new WXD_TreeItemData_t(nullptr);
    // Replace the new impl with the existing one from the tree
    delete data->impl;
    data->impl = impl;
    
    return data;
}

// Set TreeItemData for an item
WXD_EXPORTED bool wxd_TreeCtrl_SetItemData(wxd_TreeCtrl_t* self, WXD_TreeItemId_t* item_id_wxd, WXD_TreeItemData_t* data) {
    wxTreeCtrl* ctrl = (wxTreeCtrl*)self;
    if (!ctrl || !item_id_wxd || !data) return false;
    wxTreeItemId itemId = item_id_wxd->id;
    if (!itemId.IsOk()) return false;
    
    // Set the TreeItemDataImpl as the item data
    // wxTreeCtrl takes ownership of the impl pointer
    TreeItemDataImpl* impl = data->impl;
    data->impl = nullptr; // Prevent the wrapper from deleting the impl
    
    ctrl->SetItemData(itemId, impl);
    return true;
}

WXD_EXPORTED void wxd_TreeCtrl_Delete(wxd_TreeCtrl_t* self, WXD_TreeItemId_t* item_id_wxd) {
    wxTreeCtrl* ctrl = (wxTreeCtrl*)self;
    if (!ctrl || !item_id_wxd) return;
    wxTreeItemId itemId = item_id_wxd->id;
    if (itemId.IsOk()) { // Check validity before deleting
        // Get the item data and remove it from the registry
        TreeItemDataImpl* impl = static_cast<TreeItemDataImpl*>(ctrl->GetItemData(itemId));
        if (impl) {
            TreeItemDataRegistry::GetInstance().RemoveData(impl);
        }
        
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