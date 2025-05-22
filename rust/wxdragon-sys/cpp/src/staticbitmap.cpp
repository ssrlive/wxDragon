#include "../include/wxdragon.h"
#include <wx/wx.h>
#include <wx/statbmp.h>

extern "C" {

wxd_StaticBitmap_t* wxd_StaticBitmap_CreateWithBitmap(wxd_Window_t* parent, wxd_Id id, wxd_Bitmap_t* bitmap, wxd_Point pos, wxd_Size size, wxd_Style_t style, const char* name) {
    if (!parent) return nullptr;
    
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    wxBitmap* bmp = reinterpret_cast<wxBitmap*>(bitmap);
    wxString wxName = wxString::FromUTF8(name ? name : "");
    
    wxStaticBitmap* staticBitmap = new wxStaticBitmap(
        parentWin,
        id,
        bmp ? *bmp : wxBitmap(),
        wxPoint(pos.x, pos.y),
        wxSize(size.width, size.height),
        style
    );
    
    if (!wxName.empty()) {
        staticBitmap->SetName(wxName);
    }
    
    return reinterpret_cast<wxd_StaticBitmap_t*>(staticBitmap);
}

void wxd_StaticBitmap_SetBitmap(wxd_StaticBitmap_t* self, wxd_Bitmap_t* bitmap) {
    if (!self) return;
    
    wxStaticBitmap* staticBitmap = reinterpret_cast<wxStaticBitmap*>(self);
    wxBitmap* bmp = reinterpret_cast<wxBitmap*>(bitmap);
    
    staticBitmap->SetBitmap(bmp ? *bmp : wxBitmap());
}

wxd_Bitmap_t* wxd_StaticBitmap_GetBitmap(wxd_StaticBitmap_t* self) {
    if (!self) return nullptr;
    
    wxStaticBitmap* staticBitmap = reinterpret_cast<wxStaticBitmap*>(self);
    wxBitmap bmp = staticBitmap->GetBitmap();
    
    // Create a new bitmap for the caller to own
    wxBitmap* newBitmap = new wxBitmap(bmp);
    return reinterpret_cast<wxd_Bitmap_t*>(newBitmap);
}

// BitmapBundle support implementations
wxd_StaticBitmap_t* wxd_StaticBitmap_CreateWithBitmapBundle(wxd_Window_t* parent, wxd_Id id, wxd_BitmapBundle_t* bundle) {
    if (!parent) return nullptr;
    
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    wxBitmapBundle* bundlePtr = reinterpret_cast<wxBitmapBundle*>(bundle);
    
    wxStaticBitmap* staticBitmap = new wxStaticBitmap(
        parentWin,
        id,
        bundle ? *bundlePtr : wxBitmapBundle()
    );
    
    return reinterpret_cast<wxd_StaticBitmap_t*>(staticBitmap);
}

void wxd_StaticBitmap_SetBitmapBundle(wxd_StaticBitmap_t* staticBitmap, wxd_BitmapBundle_t* bundle) {
    if (!staticBitmap) return;
    
    wxStaticBitmap* sb = reinterpret_cast<wxStaticBitmap*>(staticBitmap);
    wxBitmapBundle* bundlePtr = reinterpret_cast<wxBitmapBundle*>(bundle);
    
    sb->SetBitmap(bundle ? *bundlePtr : wxBitmapBundle());
}

} // extern "C"