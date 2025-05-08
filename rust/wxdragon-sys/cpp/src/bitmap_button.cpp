#include "../include/wxdragon.h"
#include <wx/wx.h>
#include <wx/bmpbuttn.h>
#include <wx/bitmap.h>

// Helper function (already defined elsewhere, ensure linkage or redefine locally if needed)
// For simplicity here, assume they are accessible or redefine:
inline wxPoint wxd_to_wx_point(const wxd_Point& p) {
    if (p.x == -1 && p.y == -1) return wxDefaultPosition;
    return wxPoint(p.x, p.y);
}
inline wxSize wxd_to_wx_size(const wxd_Size& s) {
    if (s.width == -1 && s.height == -1) return wxDefaultSize;
    return wxSize(s.width, s.height);
}

// Implementation for wxd_BitmapButton_Create
WXD_EXPORTED wxd_BitmapButton_t* wxd_BitmapButton_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    wxd_Bitmap_t* bitmap, // Assuming this is the normal state bitmap
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style
) {
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    if (!parentWin) return nullptr;

    wxBitmap* bmp = reinterpret_cast<wxBitmap*>(bitmap);
    if (!bmp || !bmp->IsOk()) {
        // Cannot create without a valid bitmap
        wxLogError("wxd_BitmapButton_Create: Invalid bitmap provided.");
        return nullptr;
    }

    // Create the wxBitmapButton
    // Note: wxBitmapButton constructor takes bitmap by value (makes copy)
    wxBitmapButton* btn = new wxBitmapButton(
        parentWin,
        id,
        *bmp, // Pass bitmap by value/copy
        wxd_to_wx_point(pos),
        wxd_to_wx_size(size),
        style
        // TODO: Add validators later if needed
    );

    // wxBitmapButton is a wxWindow, cleanup handled by parent.
    return reinterpret_cast<wxd_BitmapButton_t*>(btn);
} 