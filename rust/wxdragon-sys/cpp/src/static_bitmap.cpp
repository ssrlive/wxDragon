#include "wx/wxprec.h"

#ifndef WX_PRECOMP
    #include "wx/wx.h"
#endif

#include <wx/control.h>
#include "wx/statbmp.h"
#include "../include/wxdragon.h"

extern "C" {

WXDRAGON_API wxd_StaticBitmap* wxd_StaticBitmap_Create(wxd_Window_t* parent_wxd, int id, const char* bitmap_path, int x, int y, int width, int height, long style) {
    wxWindow* parent = (wxWindow*)parent_wxd;
    if (!parent) return nullptr;

    wxBitmap bitmap;
    if (bitmap_path && strlen(bitmap_path) > 0) {
        // wxBITMAP_TYPE_ANY allows loading various formats
        if (!bitmap.LoadFile(wxString::FromUTF8(bitmap_path), wxBITMAP_TYPE_ANY)) {
            wxLogError("Failed to load bitmap: %s", bitmap_path);
            // Decide on error handling: return nullptr or create with an invalid bitmap?
            // For now, let's return nullptr if bitmap loading fails, as a StaticBitmap without a bitmap is often not useful.
            return nullptr; 
        }
    } else {
        // If no path, create with an empty/invalid bitmap or return nullptr.
        // wxStaticBitmap can be created with wxNullBitmap.
        // Let's proceed with wxNullBitmap if no path is provided, or if loading fails and we decide to not return nullptr above.
        // For now, the above returns nullptr on load failure. If path is empty, we use wxNullBitmap.
        bitmap = wxNullBitmap;
    }
    
    wxStaticBitmap* ctrl = new wxStaticBitmap(parent, id, bitmap, wxPoint(x, y), wxSize(width, height), style);
    return (wxd_StaticBitmap*)ctrl;
}

WXDRAGON_API wxd_StaticBitmap* wxd_StaticBitmap_CreateWithBitmap(
    wxd_Window_t* parent_wxd, int id, 
    wxd_Bitmap_t* bitmap_handle,
    int x, int y, int width, int height, long style,
    int scale_mode
) {
    wxWindow* parent = (wxWindow*)parent_wxd;
    if (!parent) return nullptr;

    wxBitmap* bmp_ptr = (wxBitmap*)bitmap_handle;

    // wxStaticBitmap constructor handles null or invalid bitmaps gracefully.
    // It will use the provided bitmap (wxBitmap is ref-counted) or wxNullBitmap if bmp_ptr is null.
    wxStaticBitmap* ctrl = new wxStaticBitmap(parent, id, bmp_ptr ? *bmp_ptr : wxNullBitmap, wxPoint(x, y), wxSize(width, height), style);
    
    if (ctrl) {
        ctrl->SetScaleMode(static_cast<decltype(wxStaticBitmapBase::Scale_None)>(scale_mode));
    }

    return (wxd_StaticBitmap*)ctrl;
}

// Add wxd_StaticBitmap_SetBitmap if needed in future
// WXDRAGON_API void wxd_StaticBitmap_SetBitmap(wxd_StaticBitmap* self, const char* bitmap_path) {
//     if (!self) return;
//     wxStaticBitmap* ctrl = (wxStaticBitmap*)self;
//     wxBitmap bitmap;
//     if (bitmap_path && strlen(bitmap_path) > 0) {
//         if (bitmap.LoadFile(wxString::FromUTF8(bitmap_path), wxBITMAP_TYPE_ANY)) {
//             ctrl->SetBitmap(bitmap);
//         } else {
//             wxLogError("Failed to load bitmap for SetBitmap: %s", bitmap_path);
//             ctrl->SetBitmap(wxNullBitmap); // or some other error indication
//         }
//     } else {
//         ctrl->SetBitmap(wxNullBitmap);
//     }
// }

} // extern "C" 