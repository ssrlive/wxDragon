#ifndef WXD_CONTAINERS_H
#define WXD_CONTAINERS_H

#include "../wxd_types.h"

// --- Panel ---
WXD_EXPORTED wxd_Panel_t* wxd_Panel_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);

// --- StaticBox ---
WXD_EXPORTED wxd_StaticBox_t* wxd_StaticBox_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos, wxd_Size size, wxd_Style_t style);

// --- Notebook ---

// --- SplitterWindow ---
WXD_EXPORTED wxd_SplitterWindow_t* wxd_SplitterWindow_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED bool wxd_SplitterWindow_SplitVertically(wxd_SplitterWindow_t* self, wxd_Window_t* window1, wxd_Window_t* window2, int sashPosition);
WXD_EXPORTED bool wxd_SplitterWindow_SplitHorizontally(wxd_SplitterWindow_t* self, wxd_Window_t* window1, wxd_Window_t* window2, int sashPosition);
WXD_EXPORTED bool wxd_SplitterWindow_Unsplit(wxd_SplitterWindow_t* self, wxd_Window_t* toRemove);
WXD_EXPORTED void wxd_SplitterWindow_Initialize(wxd_SplitterWindow_t* self, wxd_Window_t* window);
WXD_EXPORTED void wxd_SplitterWindow_SetSashPosition(wxd_SplitterWindow_t* self, int position, bool redraw);
WXD_EXPORTED int wxd_SplitterWindow_GetSashPosition(wxd_SplitterWindow_t* self);
WXD_EXPORTED void wxd_SplitterWindow_SetMinimumPaneSize(wxd_SplitterWindow_t* self, int paneSize);

// --- ScrolledWindow ---
WXD_EXPORTED wxd_ScrolledWindow_t* wxd_ScrolledWindow_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void wxd_ScrolledWindow_SetScrollRate(wxd_ScrolledWindow_t* self, int xstep, int ystep);
WXD_EXPORTED void wxd_ScrolledWindow_SetScrollbars(wxd_ScrolledWindow_t* self, int pixelsPerUnitX, int pixelsPerUnitY, int noUnitsX, int noUnitsY, int xPos, int yPos, bool noRefresh);
WXD_EXPORTED void wxd_ScrolledWindow_EnableScrolling(wxd_ScrolledWindow_t* self, bool xScrolling, bool yScrolling);
WXD_EXPORTED void wxd_ScrolledWindow_Scroll_Coord(wxd_ScrolledWindow_t* self, int x, int y);
WXD_EXPORTED void wxd_ScrolledWindow_Scroll_Point(wxd_ScrolledWindow_t* self, wxd_Point pt);
WXD_EXPORTED void wxd_ScrolledWindow_GetVirtualSize(wxd_ScrolledWindow_t* self, int* w, int* h);
WXD_EXPORTED void wxd_ScrolledWindow_GetScrollPixelsPerUnit(wxd_ScrolledWindow_t* self, int* xUnit, int* yUnit);

// --- Treebook ---
WXD_EXPORTED wxd_Treebook_t* wxd_Treebook_new(wxd_Window_t* parent, int id, int x, int y, int width, int height, wxd_Style_t style);
WXD_EXPORTED int wxd_Treebook_AddPage(wxd_Treebook_t* self, wxd_Window_t* page, const char* text, int select, int imageId);
WXD_EXPORTED int wxd_Treebook_AddSubPage(wxd_Treebook_t* self, wxd_Window_t* page, const char* text, int select, int imageId);
WXD_EXPORTED int wxd_Treebook_GetPageCount(wxd_Treebook_t* self);
WXD_EXPORTED int wxd_Treebook_GetSelection(wxd_Treebook_t* self);
WXD_EXPORTED int wxd_Treebook_SetSelection(wxd_Treebook_t* self, size_t n);
WXD_EXPORTED void wxd_Treebook_SetPageText(wxd_Treebook_t* self, size_t n, const char* strText);
WXD_EXPORTED int wxd_Treebook_GetPageText(wxd_Treebook_t* self, size_t n, char* buffer, int bufLen);

#endif // WXD_CONTAINERS_H 