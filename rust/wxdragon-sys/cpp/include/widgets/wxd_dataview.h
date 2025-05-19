#ifndef WXD_DATAVIEW_H
#define WXD_DATAVIEW_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// Forward declarations
typedef struct wxd_DataViewCtrl_tag wxd_DataViewCtrl_t;
// Don't redefine these types as they're already defined in wxd_types.h
// typedef struct wxd_DataViewModel_tag wxd_DataViewModel_t;
// typedef struct wxd_DataViewModel_tag wxd_DataViewModel_t;
typedef struct wxd_DataViewRenderer_tag wxd_DataViewRenderer_t;
// typedef struct wxd_DataViewColumn_tag wxd_DataViewColumn_t;

// Definition for DataViewItem type
typedef wxd_DataViewItemWithID_t wxd_DataViewItem_t;

// Define the alignment enum if not defined
typedef enum {
    WXD_ALIGN_LEFT = 0,
    WXD_ALIGN_RIGHT,
    WXD_ALIGN_CENTER,
} wxd_AlignmentCEnum;

// Define wxd_Id if needed
// typedef int64_t wxd_Id; -- Already defined in wxd_types.h

// Base DataViewCtrl functions
WXD_EXPORTED wxd_Window_t* wxd_DataViewCtrl_Create(wxd_Window_t* parent, int64_t id, 
                                            const wxd_Point* pos, const wxd_Size* size, 
                                            int64_t style);

WXD_EXPORTED wxd_Window_t* wxd_DataViewListCtrl_Create(wxd_Window_t* parent, int64_t id, 
                                                const wxd_Point* pos, const wxd_Size* size, 
                                                int64_t style);

WXD_EXPORTED wxd_Window_t* wxd_DataViewTreeCtrl_Create(wxd_Window_t* parent, int64_t id, 
                                                const wxd_Point* pos, const wxd_Size* size, 
                                                int64_t style);

// Column management
WXD_EXPORTED wxd_DataViewColumn_t* wxd_DataViewColumn_Create(const char* title, 
                                                    wxd_DataViewRenderer_t* renderer, 
                                                    int64_t model_column, 
                                                    int width, 
                                                    int64_t align);

WXD_EXPORTED bool wxd_DataViewCtrl_AppendColumn(wxd_Window_t* self, wxd_DataViewColumn_t* column);
WXD_EXPORTED bool wxd_DataViewCtrl_PrependColumn(wxd_Window_t* self, wxd_DataViewColumn_t* column);
WXD_EXPORTED bool wxd_DataViewCtrl_InsertColumn(wxd_Window_t* self, int64_t pos, wxd_DataViewColumn_t* column);

// Additional column management
WXD_EXPORTED int wxd_DataViewCtrl_GetColumnCount(wxd_Window_t* self);
WXD_EXPORTED wxd_DataViewColumn_t* wxd_DataViewCtrl_GetColumn(wxd_Window_t* self, uint32_t pos);
WXD_EXPORTED int wxd_DataViewCtrl_GetColumnPosition(wxd_Window_t* self, wxd_DataViewColumn_t* column);
WXD_EXPORTED bool wxd_DataViewCtrl_ClearColumns(wxd_Window_t* self);

// Item management
WXD_EXPORTED void wxd_DataViewCtrl_Select(wxd_Window_t* self, wxd_DataViewItem_t item);
WXD_EXPORTED void wxd_DataViewCtrl_Unselect(wxd_Window_t* self, wxd_DataViewItem_t item);
WXD_EXPORTED void wxd_DataViewCtrl_SelectAll(wxd_Window_t* self);
WXD_EXPORTED bool wxd_DataViewCtrl_IsSelected(wxd_Window_t* self, wxd_DataViewItem_t item);
WXD_EXPORTED uint32_t wxd_DataViewCtrl_GetSelectedItemsCount(wxd_Window_t* self);
WXD_EXPORTED void wxd_DataViewCtrl_GetSelections(wxd_Window_t* self, wxd_DataViewItem_t* items, uint32_t max_count);
WXD_EXPORTED void wxd_DataViewCtrl_SetSelections(wxd_Window_t* self, const wxd_DataViewItem_t* items, uint32_t count);

WXD_EXPORTED wxd_DataViewItem_t wxd_DataViewCtrl_GetCurrentItem(wxd_Window_t* self);
WXD_EXPORTED void wxd_DataViewCtrl_SetCurrentItem(wxd_Window_t* self, wxd_DataViewItem_t item);

// Visual appearance
WXD_EXPORTED int wxd_DataViewCtrl_GetIndent(wxd_Window_t* self);
WXD_EXPORTED void wxd_DataViewCtrl_SetIndent(wxd_Window_t* self, int indent);
WXD_EXPORTED wxd_DataViewColumn_t* wxd_DataViewCtrl_GetExpanderColumn(wxd_Window_t* self);
WXD_EXPORTED void wxd_DataViewCtrl_SetExpanderColumn(wxd_Window_t* self, wxd_DataViewColumn_t* column);
WXD_EXPORTED bool wxd_DataViewCtrl_SetRowHeight(wxd_Window_t* self, int height);
WXD_EXPORTED bool wxd_DataViewCtrl_SetAlternateRowColour(wxd_Window_t* self, const wxd_Colour_t* colour);

// Renderer creation functions
WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewTextRenderer_Create(const char* varianttype, 
                                                            int64_t mode, 
                                                            int64_t align);

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewIconTextRenderer_Create(const char* varianttype, 
                                                                  int64_t mode, 
                                                                  int64_t align);

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewToggleRenderer_Create(const char* varianttype, 
                                                               int64_t mode, 
                                                               int64_t align);

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewProgressRenderer_Create(const char* varianttype, 
                                                                  int64_t mode, 
                                                                  int64_t align);

// Additional renderer creation functions
WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewBitmapRenderer_Create(const char* varianttype, 
                                                                int64_t mode, 
                                                                int64_t align);

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewDateRenderer_Create(const char* varianttype, 
                                                              int64_t mode, 
                                                              int64_t align);

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewSpinRenderer_Create(const char* varianttype, 
                                                              int64_t mode, 
                                                              int64_t align,
                                                              int32_t min,
                                                              int32_t max,
                                                              int32_t inc);

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewChoiceRenderer_Create(const char* varianttype, 
                                                                const char* choices,
                                                                int64_t mode, 
                                                                int64_t align);

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewCheckIconTextRenderer_Create(const char* varianttype, 
                                                                       int64_t mode, 
                                                                       int64_t align);

// Custom renderer callback type
typedef bool (*wxd_DataViewRenderer_RenderCallback)(void* user_data, 
                                                  wxd_DC_t* dc, 
                                                  wxd_Rect* cell, 
                                                  int64_t item, 
                                                  int64_t column);

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewCustomRenderer_Create(
    const char* varianttype, 
    int64_t mode,
    int64_t align,
    wxd_DataViewRenderer_RenderCallback render_callback,
    void* user_data);

// Model callback types
typedef uint64_t (*wxd_DataViewModel_GetColumnCountCallback)(void* user_data);
typedef uint64_t (*wxd_DataViewModel_GetRowCountCallback)(void* user_data);
typedef void (*wxd_DataViewModel_GetValueCallback)(void* user_data, 
                                                uint64_t row, 
                                                uint64_t col, 
                                                wxd_Variant_t* variant);
typedef bool (*wxd_DataViewModel_SetValueCallback)(void* user_data, 
                                                uint64_t row, 
                                                uint64_t col, 
                                                const wxd_Variant_t* variant);

// Model creation and attachment
WXD_EXPORTED wxd_DataViewModel_t* wxd_DataViewModel_Create(
    wxd_DataViewModel_GetColumnCountCallback get_column_count,
    wxd_DataViewModel_GetRowCountCallback get_row_count,
    wxd_DataViewModel_GetValueCallback get_value,
    wxd_DataViewModel_SetValueCallback set_value,
    void* user_data);

WXD_EXPORTED bool wxd_DataViewCtrl_AssociateModel(wxd_Window_t* self, wxd_DataViewModel_t* model);

// Standard models
WXD_EXPORTED wxd_DataViewModel_t* wxd_DataViewListModel_Create();
WXD_EXPORTED bool wxd_DataViewListModel_AppendColumn(wxd_DataViewModel_t* self, const char* name);
WXD_EXPORTED bool wxd_DataViewListModel_AppendRow(wxd_DataViewModel_t* self);
WXD_EXPORTED bool wxd_DataViewListModel_SetValue(wxd_DataViewModel_t* self, 
                                         uint64_t row, 
                                         uint64_t col, 
                                         const wxd_Variant_t* variant);

// Selection management
WXD_EXPORTED bool wxd_DataViewCtrl_SelectRow(wxd_Window_t* self, int64_t row);
WXD_EXPORTED int64_t wxd_DataViewCtrl_GetSelectedRow(wxd_Window_t* self);
WXD_EXPORTED void wxd_DataViewCtrl_UnselectAll(wxd_Window_t* self);

// Free a wxd_Variant_t and its contents, including any dynamically allocated string memory
WXD_EXPORTED void wxd_Variant_Free(wxd_Variant_t* variant);

// DataViewVirtualListModel functions
WXD_EXPORTED wxd_DataViewModel_t* wxd_DataViewVirtualListModel_Create(uint64_t initial_size);
WXD_EXPORTED void wxd_DataViewVirtualListModel_RowPrepended(wxd_DataViewModel_t* model);
WXD_EXPORTED void wxd_DataViewVirtualListModel_RowInserted(wxd_DataViewModel_t* model, uint64_t before);
WXD_EXPORTED void wxd_DataViewVirtualListModel_RowAppended(wxd_DataViewModel_t* model);
WXD_EXPORTED void wxd_DataViewVirtualListModel_RowDeleted(wxd_DataViewModel_t* model, uint64_t row);
WXD_EXPORTED void wxd_DataViewVirtualListModel_RowsDeleted(wxd_DataViewModel_t* model, int32_t* rows, int32_t count);
WXD_EXPORTED void wxd_DataViewVirtualListModel_RowChanged(wxd_DataViewModel_t* model, uint64_t row);
WXD_EXPORTED void wxd_DataViewVirtualListModel_RowValueChanged(wxd_DataViewModel_t* model, uint64_t row, uint64_t col);
WXD_EXPORTED void wxd_DataViewVirtualListModel_Reset(wxd_DataViewModel_t* model, uint64_t new_size);
WXD_EXPORTED void* wxd_DataViewVirtualListModel_GetItem(wxd_DataViewModel_t* model, uint64_t row);
WXD_EXPORTED uint64_t wxd_DataViewVirtualListModel_GetRow(wxd_DataViewModel_t* model, void* item);

// Custom virtual list model with callbacks
typedef struct {
    bool has_text_colour;
    unsigned char text_colour_red;
    unsigned char text_colour_green;
    unsigned char text_colour_blue;
    unsigned char text_colour_alpha;
    
    bool has_bg_colour;
    unsigned char bg_colour_red;
    unsigned char bg_colour_green;
    unsigned char bg_colour_blue;
    unsigned char bg_colour_alpha;
    
    bool bold;
    bool italic;
} wxd_DataViewItemAttr_t;

WXD_EXPORTED wxd_DataViewModel_t* wxd_DataViewVirtualListModel_CreateWithCallbacks(
    uint64_t initial_size,
    void* userdata,
    void (*get_value_callback)(void* userdata, uint64_t row, uint64_t col, wxd_Variant_t* variant),
    bool (*set_value_callback)(void* userdata, const wxd_Variant_t* variant, uint64_t row, uint64_t col),
    bool (*get_attr_callback)(void* userdata, uint64_t row, uint64_t col, wxd_DataViewItemAttr_t* attr),
    bool (*is_enabled_callback)(void* userdata, uint64_t row, uint64_t col)
);

WXD_EXPORTED void wxd_DataViewVirtualListModel_ReleaseCallbacks(wxd_DataViewModel_t* model);

// Free function for custom model callbacks (used by Rust)
WXD_EXPORTED void drop_rust_virtual_list_model_callbacks(void* ptr);

// DataViewCtrl functions
WXD_EXPORTED wxd_DataViewColumn_t* wxd_DataViewCtrl_CreateTextColumn(wxd_Window_t* ctrl, const char* label, 
                                                     uint32_t model_column, wxd_DataViewCellModeCEnum mode, 
                                                     int width, wxd_AlignmentCEnum align, int flags);

#ifdef __cplusplus
}
#endif

#endif /* WXD_DATAVIEW_H */ 