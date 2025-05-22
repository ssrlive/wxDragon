WXD_EXPORT wxd_DataViewCtrl_t *wxd_DataViewCtrl_Create(wxd_Window_t *parent, int id,
                                                        const wxd_Point_t *pos, const wxd_Size_t *size,
                                                        long style, wxd_Validator_t *validator);

WXD_EXPORT bool wxd_DataViewCtrl_AssociateModel(wxd_DataViewCtrl_t *self, wxd_DataViewModel_t *model);
WXD_EXPORT bool wxd_DataViewCtrl_AppendColumn(wxd_DataViewCtrl_t *self, wxd_DataViewColumn_t *col);
WXD_EXPORT bool wxd_DataViewCtrl_AppendTextColumn(wxd_DataViewCtrl_t *self, const char *label, unsigned int model_column, int width, int flags);
WXD_EXPORT bool wxd_DataViewCtrl_AppendToggleColumn(wxd_DataViewCtrl_t *self, const char *label, unsigned int model_column, int width, int flags);
WXD_EXPORT bool wxd_DataViewCtrl_AppendProgressColumn(wxd_DataViewCtrl_t *self, const char *label, unsigned int model_column, int width, int flags);
WXD_EXPORT bool wxd_DataViewCtrl_AppendIconTextColumn(wxd_DataViewCtrl_t *self, const char *label, unsigned int model_column, int width, int flags);
// Add other DataViewCtrl methods here as needed...

WXD_EXPORT unsigned int wxd_DataViewCtrl_GetColumnCount(wxd_DataViewCtrl_t *self);
WXD_EXPORT wxd_DataViewColumn_t *wxd_DataViewCtrl_GetColumn(wxd_DataViewCtrl_t *self, unsigned int pos);
WXD_EXPORT int wxd_DataViewCtrl_GetColumnPosition(wxd_DataViewCtrl_t *self, const wxd_DataViewColumn_t *column);
WXD_EXPORT void wxd_DataViewCtrl_DeleteColumn(wxd_DataViewCtrl_t *self, wxd_DataViewColumn_t *column);
WXD_EXPORT void wxd_DataViewCtrl_ClearColumns(wxd_DataViewCtrl_t *self);

WXD_EXPORT void wxd_DataViewCtrl_Expand(wxd_DataViewCtrl_t *self, const wxd_DataViewItem_t *item);
WXD_EXPORT void wxd_DataViewCtrl_Collapse(wxd_DataViewCtrl_t *self, const wxd_DataViewItem_t *item);
WXD_EXPORT bool wxd_DataViewCtrl_IsExpanded(wxd_DataViewCtrl_t *self, const wxd_DataViewItem_t *item);

WXD_EXPORT wxd_DataViewItem_t *wxd_DataViewCtrl_GetSelection(wxd_DataViewCtrl_t *self);
WXD_EXPORT int wxd_DataViewCtrl_GetSelections(wxd_DataViewCtrl_t *self, wxd_DataViewItemArray_t *sel);
WXD_EXPORT void wxd_DataViewCtrl_SetSelections(wxd_DataViewCtrl_t *self, const wxd_DataViewItemArray_t *sel);
WXD_EXPORT void wxd_DataViewCtrl_Unselect(wxd_DataViewCtrl_t *self, const wxd_DataViewItem_t *item);
WXD_EXPORT void wxd_DataViewCtrl_UnselectAll(wxd_DataViewCtrl_t *self);
WXD_EXPORT bool wxd_DataViewCtrl_IsSelected(wxd_DataViewCtrl_t *self, const wxd_DataViewItem_t *item);

WXD_EXPORT void wxd_DataViewCtrl_EnsureVisible(wxd_DataViewCtrl_t *self, const wxd_DataViewItem_t *item, const wxd_DataViewColumn_t *column);
WXD_EXPORT void wxd_DataViewCtrl_HitTest(wxd_DataViewCtrl_t *self, const wxd_Point_t *point, wxd_DataViewItem_t *item, wxd_DataViewColumn_t *col);

// --- wxDataViewEvent Accessors ---
WXD_EXPORT wxd_DataViewItem_t *wxd_DataViewEvent_GetItem(wxd_Event_t *event);
WXD_EXPORT int wxd_DataViewEvent_GetColumn(wxd_Event_t *event); // Returns model column index or -1
WXD_EXPORT wxd_DataViewColumn_t *wxd_DataViewEvent_GetDataViewColumn(wxd_Event_t *event); // Returns the DataViewColumn pointer
WXD_EXPORT wxd_Variant_t *wxd_DataViewEvent_GetValue(wxd_Event_t *event);
WXD_EXPORT bool wxd_DataViewEvent_IsEditCancelled(wxd_Event_t *event); // For ItemEditingDone
WXD_EXPORT int wxd_DataViewEvent_GetCacheFrom(wxd_Event_t *event); // For CacheHint, returns row index
WXD_EXPORT int wxd_DataViewEvent_GetCacheTo(wxd_Event_t *event);   // For CacheHint, returns row index
// TODO: Add accessors for drag/drop events like GetDataObject, GetDropEffect, SetDropEffect, etc.
// For now, these are the most common ones for item interaction and editing.

#ifdef __cplusplus
}
#endif

#endif // WXD_DATAVIEW_CTRL_H 