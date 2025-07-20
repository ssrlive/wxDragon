#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/dataview.h> // For wxDataViewItem

// Releases the heap-allocated wxDataViewItem pointed to by item_wrapper.id
WXD_EXPORTED void wxd_DataViewItem_Release(wxd_DataViewItem_t item_wrapper) {
    if (item_wrapper.id != nullptr) {
        delete reinterpret_cast<wxDataViewItem*>(item_wrapper.id);
    }
} 