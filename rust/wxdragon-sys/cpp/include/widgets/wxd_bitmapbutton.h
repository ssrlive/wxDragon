#ifndef WXD_BITMAPBUTTON_H
#define WXD_BITMAPBUTTON_H

#include "../wxd_types.h"

// --- BitmapButton Functions ---
WXD_EXPORTED wxd_BitmapButton_t* wxd_BitmapButton_Create(
    wxd_Window_t* parent, 
    wxd_Id id, 
    wxd_Bitmap_t* bitmap,         // Main bitmap (normal state)
    wxd_Point pos, 
    wxd_Size size, 
    wxd_Style_t style, 
    const char* name,
    wxd_Bitmap_t* bitmap_disabled, // Disabled state bitmap (can be NULL)
    wxd_Bitmap_t* bitmap_focus,    // Focus state bitmap (can be NULL)
    wxd_Bitmap_t* bitmap_hover     // Hover state bitmap (can be NULL)
);

#endif // WXD_BITMAPBUTTON_H 