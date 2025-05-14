#ifndef WXD_LISTCTRL_H
#define WXD_LISTCTRL_H

#include "../wxd_types.h"

// --- ListCtrl Functions ---
WXD_EXPORTED wxd_ListCtrl_t* wxd_ListCtrl_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED int32_t wxd_ListCtrl_InsertColumn(wxd_ListCtrl_t* self, long col, const char* heading, int format, int width);
WXD_EXPORTED bool wxd_ListCtrl_SetColumnWidth(wxd_ListCtrl_t* self, long col, int width);
WXD_EXPORTED int wxd_ListCtrl_GetColumnWidth(wxd_ListCtrl_t* self, long col);
WXD_EXPORTED int wxd_ListCtrl_GetColumnCount(wxd_ListCtrl_t* self);
WXD_EXPORTED int32_t wxd_ListCtrl_InsertItem_Simple(wxd_ListCtrl_t* self, long index, const char* label);
WXD_EXPORTED void wxd_ListCtrl_SetItemText(wxd_ListCtrl_t* self, long index, const char* text);
WXD_EXPORTED bool wxd_ListCtrl_SetItem(wxd_ListCtrl_t* self, long item, int col, const char* text, int image, int format, long state, long stateMask, long data, long mask);
WXD_EXPORTED int wxd_ListCtrl_GetItemText(wxd_ListCtrl_t* self, long index, int col, char* buffer, int buffer_len);
WXD_EXPORTED int wxd_ListCtrl_GetItemCount(wxd_ListCtrl_t* self);
WXD_EXPORTED bool wxd_ListCtrl_SetItemState(wxd_ListCtrl_t* self, long item, long state, long stateMask);
WXD_EXPORTED int32_t wxd_ListCtrl_GetItemState(wxd_ListCtrl_t* self, long item, long stateMask);
WXD_EXPORTED int32_t wxd_ListCtrl_GetNextItem(wxd_ListCtrl_t* self, long item, int geometry, int state);
WXD_EXPORTED bool wxd_ListCtrl_DeleteItem(wxd_ListCtrl_t* self, long item);
WXD_EXPORTED bool wxd_ListCtrl_DeleteAllItems(wxd_ListCtrl_t* self);
WXD_EXPORTED bool wxd_ListCtrl_ClearAll(wxd_ListCtrl_t* self);
WXD_EXPORTED int wxd_ListCtrl_GetSelectedItemCount(wxd_ListCtrl_t* self);
WXD_EXPORTED bool wxd_ListCtrl_EnsureVisible(wxd_ListCtrl_t* self, long item);
WXD_EXPORTED int32_t wxd_ListCtrl_HitTest(wxd_ListCtrl_t* self, wxd_Point point, int* flags_ptr, long* subitem_ptr);
WXD_EXPORTED wxd_TextCtrl_t* wxd_ListCtrl_EditLabel(wxd_ListCtrl_t* self, long item);

// --- Advanced ListCtrl Capabilities ---
// Item Data Functions
WXD_EXPORTED bool wxd_ListCtrl_SetItemData(wxd_ListCtrl_t* self, long item, long data);
WXD_EXPORTED bool wxd_ListCtrl_SetItemPtrData(wxd_ListCtrl_t* self, long item, void* data);
WXD_EXPORTED long wxd_ListCtrl_GetItemData(wxd_ListCtrl_t* self, long item);
WXD_EXPORTED void* wxd_ListCtrl_GetItemPtrData(wxd_ListCtrl_t* self, long item);

// Item Appearance
WXD_EXPORTED void wxd_ListCtrl_SetItemBackgroundColour(wxd_ListCtrl_t* self, long item, wxd_Colour_t colour);
WXD_EXPORTED void wxd_ListCtrl_SetItemTextColour(wxd_ListCtrl_t* self, long item, wxd_Colour_t colour);
WXD_EXPORTED wxd_Colour_t wxd_ListCtrl_GetItemBackgroundColour(wxd_ListCtrl_t* self, long item);
WXD_EXPORTED wxd_Colour_t wxd_ListCtrl_GetItemTextColour(wxd_ListCtrl_t* self, long item);

// Column Management
WXD_EXPORTED bool wxd_ListCtrl_SetColumnsOrder(wxd_ListCtrl_t* self, int count, int* orders);
WXD_EXPORTED int* wxd_ListCtrl_GetColumnsOrder(wxd_ListCtrl_t* self, int* count);
WXD_EXPORTED int wxd_ListCtrl_GetColumnOrder(wxd_ListCtrl_t* self, int col);
WXD_EXPORTED int wxd_ListCtrl_GetColumnIndexFromOrder(wxd_ListCtrl_t* self, int pos);

// Virtual List Support
WXD_EXPORTED void wxd_ListCtrl_SetItemCount(wxd_ListCtrl_t* self, long count);
WXD_EXPORTED void wxd_ListCtrl_RefreshItem(wxd_ListCtrl_t* self, long item);
WXD_EXPORTED void wxd_ListCtrl_RefreshItems(wxd_ListCtrl_t* self, long itemFrom, long itemTo);

// Sorting
WXD_EXPORTED bool wxd_ListCtrl_SortItems(wxd_ListCtrl_t* self, int (*cmpFunc)(void*, void*, void*), void* data);
WXD_EXPORTED void wxd_ListCtrl_ShowSortIndicator(wxd_ListCtrl_t* self, int col, bool ascending);

// Image List Support
WXD_EXPORTED void wxd_ListCtrl_SetImageList(wxd_ListCtrl_t* self, void* imageList, int which);
WXD_EXPORTED void wxd_ListCtrl_AssignImageList(wxd_ListCtrl_t* self, void* imageList, int which);
WXD_EXPORTED void* wxd_ListCtrl_GetImageList(wxd_ListCtrl_t* self, int which);

#endif // WXD_LISTCTRL_H 