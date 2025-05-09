#ifndef WXD_CHOICES_H
#define WXD_CHOICES_H

#include "../wxd_types.h"

// --- ListBox ---
WXD_EXPORTED wxd_ListBox_t* wxd_ListBox_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void wxd_ListBox_Append(wxd_ListBox_t* self, const char* item);
WXD_EXPORTED void wxd_ListBox_Clear(wxd_ListBox_t* listbox);
WXD_EXPORTED int wxd_ListBox_GetSelection(wxd_ListBox_t* listbox);
WXD_EXPORTED int wxd_ListBox_GetStringSelection(wxd_ListBox_t* listbox, char* buffer, int buffer_len);
WXD_EXPORTED void wxd_ListBox_SetSelection(wxd_ListBox_t* listbox, int index, bool select);
WXD_EXPORTED int wxd_ListBox_GetString(wxd_ListBox_t* listbox, int index, char* buffer, int buffer_len);
WXD_EXPORTED unsigned int wxd_ListBox_GetCount(wxd_ListBox_t* listbox);

// --- Choice ---
WXD_EXPORTED wxd_Choice_t* wxd_Choice_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void wxd_Choice_Append(wxd_Choice_t* self, const char* item);
WXD_EXPORTED void wxd_Choice_Clear(wxd_Choice_t* choice);
WXD_EXPORTED int wxd_Choice_GetSelection(wxd_Choice_t* choice);
WXD_EXPORTED int wxd_Choice_GetStringSelection(wxd_Choice_t* choice, char* buffer, int buffer_len);
WXD_EXPORTED void wxd_Choice_SetSelection(wxd_Choice_t* choice, int index);
WXD_EXPORTED int wxd_Choice_GetString(wxd_Choice_t* choice, int index, char* buffer, int buffer_len);
WXD_EXPORTED unsigned int wxd_Choice_GetCount(wxd_Choice_t* choice);

// --- ComboBox ---
WXD_EXPORTED wxd_ComboBox_t* wxd_ComboBox_Create(wxd_Window_t* parent, wxd_Id id, const char* value, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void wxd_ComboBox_Append(wxd_ComboBox_t* combo, const char* item);
WXD_EXPORTED void wxd_ComboBox_Clear(wxd_ComboBox_t* combo);
WXD_EXPORTED int wxd_ComboBox_GetSelection(wxd_ComboBox_t* combo);
WXD_EXPORTED int wxd_ComboBox_GetStringSelection(wxd_ComboBox_t* combo, char* buffer, int buffer_len);
WXD_EXPORTED void wxd_ComboBox_SetSelection(wxd_ComboBox_t* combo, int index);
WXD_EXPORTED int wxd_ComboBox_GetString(wxd_ComboBox_t* combo, int index, char* buffer, int buffer_len);
WXD_EXPORTED unsigned int wxd_ComboBox_GetCount(wxd_ComboBox_t* combo);
WXD_EXPORTED void wxd_ComboBox_SetValue(wxd_ComboBox_t* combo, const char* value);
WXD_EXPORTED int wxd_ComboBox_GetValue(wxd_ComboBox_t* combo, char* buffer, int buffer_len);

// --- CheckListBox ---
WXD_EXPORTED wxd_CheckListBox_t* wxd_CheckListBox_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void wxd_CheckListBox_Append(wxd_CheckListBox_t* clbox, const char* item);
WXD_EXPORTED void wxd_CheckListBox_Clear(wxd_CheckListBox_t* clbox);
WXD_EXPORTED int wxd_CheckListBox_GetSelection(wxd_CheckListBox_t* clbox);
WXD_EXPORTED int wxd_CheckListBox_GetStringSelection(wxd_CheckListBox_t* clbox, char* buffer, int buffer_len);
WXD_EXPORTED void wxd_CheckListBox_SetSelection(wxd_CheckListBox_t* clbox, int index, bool select);
WXD_EXPORTED int wxd_CheckListBox_GetString(wxd_CheckListBox_t* clbox, int index, char* buffer, int buffer_len);
WXD_EXPORTED unsigned int wxd_CheckListBox_GetCount(wxd_CheckListBox_t* clbox);
WXD_EXPORTED bool wxd_CheckListBox_IsChecked(wxd_CheckListBox_t* clbox, unsigned int index);
WXD_EXPORTED void wxd_CheckListBox_Check(wxd_CheckListBox_t* clbox, unsigned int index, bool check);

// --- RadioBox ---
WXD_EXPORTED wxd_RadioBox_t* wxd_RadioBox_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos, wxd_Size size, int n, const char* const* choices, int majorDimension, wxd_Style_t style);
WXD_EXPORTED int wxd_RadioBox_GetSelection(wxd_RadioBox_t* self);
WXD_EXPORTED void wxd_RadioBox_SetSelection(wxd_RadioBox_t* self, int n);
WXD_EXPORTED int wxd_RadioBox_GetString(wxd_RadioBox_t* self, int n, char* buffer, int buffer_len);
WXD_EXPORTED unsigned int wxd_RadioBox_GetCount(wxd_RadioBox_t* self);
WXD_EXPORTED bool wxd_RadioBox_EnableItem(wxd_RadioBox_t* self, int n, bool enable);
WXD_EXPORTED bool wxd_RadioBox_IsItemEnabled(wxd_RadioBox_t* self, int n);
WXD_EXPORTED bool wxd_RadioBox_ShowItem(wxd_RadioBox_t* self, int n, bool show);
WXD_EXPORTED bool wxd_RadioBox_IsItemShown(wxd_RadioBox_t* self, int n);

// --- BitmapComboBox ---
WXD_EXPORTED wxd_BitmapComboBox_t* wxd_BitmapComboBox_Create(wxd_Window_t* parent, wxd_Id id, const char* value, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void wxd_BitmapComboBox_Append(wxd_BitmapComboBox_t* self, const char* item, wxd_Bitmap_t* bitmap);
WXD_EXPORTED void wxd_BitmapComboBox_Clear(wxd_BitmapComboBox_t* self);
WXD_EXPORTED int wxd_BitmapComboBox_GetSelection(wxd_BitmapComboBox_t* self);
WXD_EXPORTED void wxd_BitmapComboBox_SetSelection(wxd_BitmapComboBox_t* self, int index);
WXD_EXPORTED int wxd_BitmapComboBox_GetString(wxd_BitmapComboBox_t* self, int index, char* buffer, int buffer_len);
WXD_EXPORTED unsigned int wxd_BitmapComboBox_GetCount(wxd_BitmapComboBox_t* self);
WXD_EXPORTED void wxd_BitmapComboBox_SetValue(wxd_BitmapComboBox_t* self, const char* value);
WXD_EXPORTED int wxd_BitmapComboBox_GetValue(wxd_BitmapComboBox_t* self, char* buffer, int buffer_len);
WXD_EXPORTED wxd_Bitmap_t* wxd_BitmapComboBox_GetItemBitmap(wxd_BitmapComboBox_t* self, unsigned int n);
WXD_EXPORTED void wxd_BitmapComboBox_SetItemBitmap(wxd_BitmapComboBox_t* self, unsigned int n, wxd_Bitmap_t* bitmap);

// --- TreeCtrl ---
WXD_EXPORTED wxd_TreeCtrl_t* wxd_TreeCtrl_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED WXD_TreeItemId_t* wxd_TreeCtrl_AddRoot(wxd_TreeCtrl_t* self, const char* text, int image, int selImage, void* data);
WXD_EXPORTED WXD_TreeItemId_t* wxd_TreeCtrl_AppendItem(wxd_TreeCtrl_t* self, WXD_TreeItemId_t* parent_id, const char* text, int image, int selImage, void* data);
WXD_EXPORTED void wxd_TreeCtrl_Delete(wxd_TreeCtrl_t* self, WXD_TreeItemId_t* item_id);
WXD_EXPORTED WXD_TreeItemId_t* wxd_TreeCtrl_GetSelection(wxd_TreeCtrl_t* self);
WXD_EXPORTED void wxd_TreeCtrl_SelectItem(wxd_TreeCtrl_t* self, WXD_TreeItemId_t* item_id);
WXD_EXPORTED void wxd_TreeItemId_Free(WXD_TreeItemId_t* item_id);
WXD_EXPORTED bool wxd_TreeItemId_IsOk(WXD_TreeItemId_t* item_id);

// --- ListCtrl ---
WXD_EXPORTED wxd_ListCtrl_t* wxd_ListCtrl_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED int32_t wxd_ListCtrl_InsertColumn(wxd_ListCtrl_t* self, long col, const char* heading, int format, int width);
WXD_EXPORTED bool wxd_ListCtrl_SetColumnWidth(wxd_ListCtrl_t* self, long col, int width);
WXD_EXPORTED int wxd_ListCtrl_GetColumnWidth(wxd_ListCtrl_t* self, long col);
WXD_EXPORTED int wxd_ListCtrl_GetColumnCount(wxd_ListCtrl_t* self);
WXD_EXPORTED int32_t wxd_ListCtrl_InsertItem_Simple(wxd_ListCtrl_t* self, long index, const char* label);
WXD_EXPORTED void wxd_ListCtrl_SetItemText(wxd_ListCtrl_t* self, long index, const char* text); // Note: wxListCtrl uses SetItemText(itemId, col, text), this simplified version assumes col 0
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
WXD_EXPORTED void wxd_ListCtrl_EditLabel(wxd_ListCtrl_t* self, long item);

#endif // WXD_CHOICES_H 