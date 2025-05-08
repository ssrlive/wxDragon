#include "../include/wxdragon.h"
#include <wx/wx.h>
#include <wx/artprov.h>
#include <wx/bitmap.h>

WXD_EXPORTED wxd_Bitmap_t* wxd_ArtProvider_GetBitmap(const char* id, const char* client, wxd_Size size_req) {
    wxString wxId = wxString::FromUTF8(id);
    wxString wxClient = wxString::FromUTF8(client);
    if (wxClient.IsEmpty()) {
        // wxWidgets uses wxART_OTHER if client is empty string, but being explicit is fine.
        // However, wxArtProvider::GetBitmap takes wxArtClient which is also a wxString.
        // An empty wxString is the correct way to represent wxART_OTHER if no specific client is given.
    }

    wxSize wxSizeReq(size_req.width, size_req.height);

    // wxArtProvider::GetBitmap directly returns wxBitmap
    // For more modern approach, wxArtProvider::GetBitmapBundle then bundle.GetBitmap(size) is used.
    // Let's use the direct GetBitmap for simplicity matching the older API style if GetBitmapBundle isn't straightforward here.
    // wxArtProvider::GetBitmap is static.
    wxBitmap bitmap = wxArtProvider::GetBitmap(wxId, wxClient, wxSizeReq);

    if (!bitmap.IsOk()) {
        return nullptr;
    }

    // The returned bitmap from wxArtProvider is a new copy or a ref-counted shared instance.
    // We need to return a heap-allocated wxBitmap for consistency with other Create functions.
    wxBitmap* new_bitmap = new wxBitmap(bitmap); 
    return reinterpret_cast<wxd_Bitmap_t*>(new_bitmap);
} 