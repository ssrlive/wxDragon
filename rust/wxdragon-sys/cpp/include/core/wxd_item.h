// Purpose: Defines C-compatible item types for wxDragon FFI.
#ifndef WXD_ITEM_H
#define WXD_ITEM_H

#include "../wxd_types.h" // For WXD_EXPORTED and other basic types if needed

#ifdef __cplusplus
extern "C" {
#endif

// wxd_MenuItem_t is defined in wxd_types.h as: typedef struct wxd_MenuItem_t wxd_MenuItem_t;
// Opaque pointer for wxMenuItem
// typedef void wxd_MenuItem_t; // REMOVED - Defined in wxd_types.h

// Opaque pointer for wxDataViewItem, which is internally a void*
// For FFI, we treat it as an opaque struct containing the void*.
// The `id` will point to a heap-allocated wxDataViewItem in C++
// that Rust takes ownership of via FromWxDVI and releases via wxd_DataViewItem_Release.
typedef struct {
    void* id; 
} wxd_DataViewItem_t;

// Releases the heap-allocated wxDataViewItem pointed to by item.id
WXD_EXPORTED void wxd_DataViewItem_Release(wxd_DataViewItem_t item);

#ifdef __cplusplus
}
#endif

#endif // WXD_ITEM_H 