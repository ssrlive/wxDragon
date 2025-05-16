#ifndef WXD_STATICBITMAP_H
#define WXD_STATICBITMAP_H

#include "../wxd_types.h"

// --- StaticBitmap Functions ---
WXD_EXPORTED wxd_StaticBitmap_t* wxd_StaticBitmap_CreateWithBitmap(wxd_Window_t* parent, wxd_Id id, wxd_Bitmap_t* bitmap, wxd_Point pos, wxd_Size size, wxd_Style_t style, const char* name);
WXD_EXPORTED void wxd_StaticBitmap_SetBitmap(wxd_StaticBitmap_t* self, wxd_Bitmap_t* bitmap);
WXD_EXPORTED wxd_Bitmap_t* wxd_StaticBitmap_GetBitmap(wxd_StaticBitmap_t* self);

#endif // WXD_STATICBITMAP_H 