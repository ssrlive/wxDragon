#ifndef WXD_DC_H
#define WXD_DC_H

#include "../wxd_types.h"
#include "wxd_dc_constants.h"

#ifdef __cplusplus
extern "C" {
#endif

// Opaque DC type declarations
typedef struct wxd_DC_t wxd_DC_t;
typedef struct wxd_WindowDC_t wxd_WindowDC_t;
typedef struct wxd_ClientDC_t wxd_ClientDC_t;
typedef struct wxd_PaintDC_t wxd_PaintDC_t;
typedef struct wxd_MemoryDC_t wxd_MemoryDC_t;
typedef struct wxd_ScreenDC_t wxd_ScreenDC_t;

// DC Creation/Destruction
WXD_EXPORTED wxd_WindowDC_t* wxd_WindowDC_Create(wxd_Window_t* window);
WXD_EXPORTED void wxd_WindowDC_Destroy(wxd_WindowDC_t* dc);

WXD_EXPORTED wxd_ClientDC_t* wxd_ClientDC_Create(wxd_Window_t* window);
WXD_EXPORTED void wxd_ClientDC_Destroy(wxd_ClientDC_t* dc);

WXD_EXPORTED wxd_PaintDC_t* wxd_PaintDC_Create(wxd_Window_t* window);
WXD_EXPORTED void wxd_PaintDC_Destroy(wxd_PaintDC_t* dc);

WXD_EXPORTED wxd_MemoryDC_t* wxd_MemoryDC_Create(void);
WXD_EXPORTED void wxd_MemoryDC_Destroy(wxd_MemoryDC_t* dc);

WXD_EXPORTED wxd_ScreenDC_t* wxd_ScreenDC_Create(void);
WXD_EXPORTED void wxd_ScreenDC_Destroy(wxd_ScreenDC_t* dc);

// Common DC operations (operates on the base type)
WXD_EXPORTED void wxd_DC_Clear(wxd_DC_t* dc);
WXD_EXPORTED void wxd_DC_SetBackground(wxd_DC_t* dc, wxd_Colour_t background);
WXD_EXPORTED void wxd_DC_SetBackgroundMode(wxd_DC_t* dc, int mode); // WXD_TRANSPARENT or WXD_SOLID
WXD_EXPORTED void wxd_DC_SetTextBackground(wxd_DC_t* dc, wxd_Colour_t colour);
WXD_EXPORTED void wxd_DC_SetTextForeground(wxd_DC_t* dc, wxd_Colour_t colour);
WXD_EXPORTED void wxd_DC_SetFont(wxd_DC_t* dc, const wxd_Font_t* font);
WXD_EXPORTED void wxd_DC_SetPen(wxd_DC_t* dc, wxd_Colour_t colour, int width, int style);
WXD_EXPORTED void wxd_DC_SetBrush(wxd_DC_t* dc, wxd_Colour_t colour, int style);

// Drawing operations
WXD_EXPORTED void wxd_DC_DrawPoint(wxd_DC_t* dc, int x, int y);
WXD_EXPORTED void wxd_DC_DrawLine(wxd_DC_t* dc, int x1, int y1, int x2, int y2);
WXD_EXPORTED void wxd_DC_DrawRectangle(wxd_DC_t* dc, int x, int y, int width, int height);
WXD_EXPORTED void wxd_DC_DrawCircle(wxd_DC_t* dc, int x, int y, int radius);
WXD_EXPORTED void wxd_DC_DrawEllipse(wxd_DC_t* dc, int x, int y, int width, int height);
WXD_EXPORTED void wxd_DC_DrawText(wxd_DC_t* dc, const char* text, int x, int y);
WXD_EXPORTED void wxd_DC_DrawBitmap(wxd_DC_t* dc, const wxd_Bitmap_t* bitmap, int x, int y, bool transparent);

// Device capabilities
WXD_EXPORTED wxd_Size wxd_DC_GetSize(wxd_DC_t* dc);
WXD_EXPORTED void wxd_DC_GetTextExtent(wxd_DC_t* dc, const char* string, int* w, int* h);

// MemoryDC specific operations
WXD_EXPORTED void wxd_MemoryDC_SelectObject(wxd_MemoryDC_t* dc, wxd_Bitmap_t* bitmap);
WXD_EXPORTED void wxd_MemoryDC_SelectObjectAsSource(wxd_MemoryDC_t* dc, wxd_Bitmap_t* bitmap);

// Type casting functions (for safely using base DC functions with derived types)
WXD_EXPORTED wxd_DC_t* wxd_WindowDC_AsDC(wxd_WindowDC_t* dc);
WXD_EXPORTED wxd_DC_t* wxd_ClientDC_AsDC(wxd_ClientDC_t* dc);
WXD_EXPORTED wxd_DC_t* wxd_PaintDC_AsDC(wxd_PaintDC_t* dc);
WXD_EXPORTED wxd_DC_t* wxd_MemoryDC_AsDC(wxd_MemoryDC_t* dc);
WXD_EXPORTED wxd_DC_t* wxd_ScreenDC_AsDC(wxd_ScreenDC_t* dc);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WXD_DC_H 