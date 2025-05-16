#include "../include/wxdragon.h"
#include "../src/wxd_utils.h"
#include <wx/dataview.h>
#include <wx/string.h> // For wxString methods
#include <wx/tokenzr.h> // For wxStringTokenizer
#include <wx/bitmap.h> // For wxBitmap
#include <wx/datetime.h> // For wxDateTime

extern "C" {

// Base DataViewCtrl functions
WXD_EXPORTED wxd_Window_t* wxd_DataViewCtrl_Create(wxd_Window_t* parent, int64_t id, 
                                            const wxd_Point* pos, const wxd_Size* size, 
                                            int64_t style) {
    if (!parent) return nullptr;
    
    wxWindow* p = reinterpret_cast<wxWindow*>(parent);
    wxPoint wxPos = pos ? wxPoint(pos->x, pos->y) : wxDefaultPosition;
    wxSize wxSizeObj = size ? wxSize(size->width, size->height) : wxDefaultSize;
    
    wxDataViewCtrl* ctrl = new wxDataViewCtrl(p, id, wxPos, wxSizeObj, style);
    return reinterpret_cast<wxd_Window_t*>(ctrl);
}

WXD_EXPORTED wxd_Window_t* wxd_DataViewListCtrl_Create(wxd_Window_t* parent, int64_t id, 
                                                const wxd_Point* pos, const wxd_Size* size, 
                                                int64_t style) {
    if (!parent) return nullptr;
    
    wxWindow* p = reinterpret_cast<wxWindow*>(parent);
    wxPoint wxPos = pos ? wxPoint(pos->x, pos->y) : wxDefaultPosition;
    wxSize wxSizeObj = size ? wxSize(size->width, size->height) : wxDefaultSize;
    
    wxDataViewListCtrl* ctrl = new wxDataViewListCtrl(p, id, wxPos, wxSizeObj, style);
    return reinterpret_cast<wxd_Window_t*>(ctrl);
}

WXD_EXPORTED wxd_Window_t* wxd_DataViewTreeCtrl_Create(wxd_Window_t* parent, int64_t id, 
                                                const wxd_Point* pos, const wxd_Size* size, 
                                                int64_t style) {
    if (!parent) return nullptr;
    
    wxWindow* p = reinterpret_cast<wxWindow*>(parent);
    wxPoint wxPos = pos ? wxPoint(pos->x, pos->y) : wxDefaultPosition;
    wxSize wxSizeObj = size ? wxSize(size->width, size->height) : wxDefaultSize;
    
    wxDataViewTreeCtrl* ctrl = new wxDataViewTreeCtrl(p, id, wxPos, wxSizeObj, style);
    return reinterpret_cast<wxd_Window_t*>(ctrl);
}

// Column management
WXD_EXPORTED wxd_DataViewColumn_t* wxd_DataViewColumn_Create(const char* title, 
                                                     wxd_DataViewRenderer_t* renderer, 
                                                     int64_t model_column, 
                                                     int width, 
                                                     int64_t align) {
    if (!renderer) return nullptr;
    
    wxString wxTitle = wxString::FromUTF8(title ? title : "");
    wxDataViewRenderer* r = reinterpret_cast<wxDataViewRenderer*>(renderer);
    
    wxDataViewColumn* column = new wxDataViewColumn(wxTitle, r, static_cast<unsigned int>(model_column), width, static_cast<wxAlignment>(align));
    return reinterpret_cast<wxd_DataViewColumn_t*>(column);
}

WXD_EXPORTED bool wxd_DataViewCtrl_AppendColumn(wxd_Window_t* self, wxd_DataViewColumn_t* column) {
    if (!self || !column) return false;
    
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    wxDataViewColumn* col = reinterpret_cast<wxDataViewColumn*>(column);
    
    return ctrl->AppendColumn(col);
}

WXD_EXPORTED bool wxd_DataViewCtrl_PrependColumn(wxd_Window_t* self, wxd_DataViewColumn_t* column) {
    if (!self || !column) return false;
    
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    wxDataViewColumn* col = reinterpret_cast<wxDataViewColumn*>(column);
    
    return ctrl->PrependColumn(col);
}

WXD_EXPORTED bool wxd_DataViewCtrl_InsertColumn(wxd_Window_t* self, int64_t pos, wxd_DataViewColumn_t* column) {
    if (!self || !column) return false;
    
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    wxDataViewColumn* col = reinterpret_cast<wxDataViewColumn*>(column);
    
    return ctrl->InsertColumn(static_cast<unsigned int>(pos), col);
}

// Renderer creation functions
WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewTextRenderer_Create(const char* varianttype, 
                                                             int64_t mode, 
                                                             int64_t align) {
    wxString wxVarType = wxString::FromUTF8(varianttype ? varianttype : "string");
    wxDataViewTextRenderer* renderer = new wxDataViewTextRenderer(wxVarType, 
                                                               static_cast<wxDataViewCellMode>(mode), 
                                                               static_cast<wxAlignment>(align));
    return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
}

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewIconTextRenderer_Create(const char* varianttype, 
                                                                    int64_t mode, 
                                                                    int64_t align) {
    wxString wxVarType = wxString::FromUTF8(varianttype ? varianttype : "wxDataViewIconText");
    wxDataViewIconTextRenderer* renderer = new wxDataViewIconTextRenderer(wxVarType, 
                                                                      static_cast<wxDataViewCellMode>(mode), 
                                                                      static_cast<wxAlignment>(align));
    return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
}

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewToggleRenderer_Create(const char* varianttype, 
                                                                 int64_t mode, 
                                                                 int64_t align) {
    wxString wxVarType = wxString::FromUTF8(varianttype ? varianttype : "bool");
    wxDataViewToggleRenderer* renderer = new wxDataViewToggleRenderer(wxVarType, 
                                                                  static_cast<wxDataViewCellMode>(mode), 
                                                                  static_cast<wxAlignment>(align));
    return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
}

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewProgressRenderer_Create(const char* varianttype, 
                                                                    int64_t mode, 
                                                                    int64_t align) {
    wxString wxVarType = wxString::FromUTF8(varianttype ? varianttype : "long");
    // The constructor signature is different from other renderers
    wxDataViewProgressRenderer* renderer = new wxDataViewProgressRenderer(
        wxEmptyString,  // label
        wxVarType,      // varianttype
        static_cast<wxDataViewCellMode>(mode)); // mode
        
    return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
}

// Additional renderer implementations
WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewBitmapRenderer_Create(const char* varianttype, 
                                                                 int64_t mode, 
                                                                 int64_t align) {
    wxString wxVarType = wxString::FromUTF8(varianttype ? varianttype : "wxBitmap");
    wxDataViewBitmapRenderer* renderer = new wxDataViewBitmapRenderer(
        wxVarType, 
        static_cast<wxDataViewCellMode>(mode),
        static_cast<wxAlignment>(align));
        
    return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
}

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewDateRenderer_Create(const char* varianttype, 
                                                              int64_t mode, 
                                                              int64_t align) {
    wxString wxVarType = wxString::FromUTF8(varianttype ? varianttype : "datetime");
    wxDataViewDateRenderer* renderer = new wxDataViewDateRenderer(
        wxVarType, 
        static_cast<wxDataViewCellMode>(mode),
        static_cast<wxAlignment>(align));
        
    return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
}

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewSpinRenderer_Create(const char* varianttype, 
                                                              int64_t mode, 
                                                              int64_t align,
                                                              int32_t min,
                                                              int32_t max,
                                                              int32_t inc) {
    // The constructor order is different: min and max come first, then mode and align
    wxDataViewSpinRenderer* renderer = new wxDataViewSpinRenderer(
        min,
        max,
        static_cast<wxDataViewCellMode>(mode),
        static_cast<int>(align));
        
    return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
}

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewChoiceRenderer_Create(const char* varianttype, 
                                                                const char* choices_str,
                                                                int64_t mode, 
                                                                int64_t align) {
    wxString wxChoices = wxString::FromUTF8(choices_str ? choices_str : "");
    
    // Parse choices and create wxArrayString
    wxArrayString choices;
    wxStringTokenizer tokenizer(wxChoices, ",");
    while (tokenizer.HasMoreTokens()) {
        choices.Add(tokenizer.GetNextToken().Trim());
    }
    
    wxDataViewChoiceRenderer* renderer = new wxDataViewChoiceRenderer(
        choices,
        static_cast<wxDataViewCellMode>(mode),
        static_cast<int>(align));
        
    return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
}

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewCheckIconTextRenderer_Create(const char* varianttype, 
                                                                       int64_t mode, 
                                                                       int64_t align) {
    // This renderer doesn't accept a varianttype parameter
    wxDataViewCheckIconTextRenderer* renderer = new wxDataViewCheckIconTextRenderer(
        static_cast<wxDataViewCellMode>(mode),
        static_cast<int>(align));
        
    return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
}

// Custom renderer implementation
class WxDCustomRenderer : public wxDataViewCustomRenderer {
private:
    wxd_DataViewRenderer_RenderCallback m_callback;
    void* m_user_data;
    
public:
    WxDCustomRenderer(const wxString& varianttype,
                     wxDataViewCellMode mode,
                     int align,
                     wxd_DataViewRenderer_RenderCallback callback,
                     void* user_data) 
        : wxDataViewCustomRenderer(varianttype, mode, align),
          m_callback(callback),
          m_user_data(user_data) {}
          
    virtual bool Render(wxRect cell, wxDC* dc, int state) override {
        wxd_DC_t* dc_ptr = reinterpret_cast<wxd_DC_t*>(dc);
        wxd_Rect rect = {cell.x, cell.y, cell.width, cell.height};
        return m_callback(m_user_data, dc_ptr, &rect, state, 0); // Need to add item/column info
    }
    
    virtual wxSize GetSize() const override {
        // Default implementation
        return wxSize(80, 20);
    }
};

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewCustomRenderer_Create(
    const char* varianttype, 
    int64_t mode,
    int64_t align,
    wxd_DataViewRenderer_RenderCallback render_callback,
    void* user_data) {
    
    if (!render_callback) return nullptr;
    
    wxString wxVarType = wxString::FromUTF8(varianttype ? varianttype : "string");
    WxDCustomRenderer* renderer = new WxDCustomRenderer(
        wxVarType, 
        static_cast<wxDataViewCellMode>(mode),
        static_cast<int>(align),
        render_callback,
        user_data);
        
    return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
}

// DataViewModel implementation
class WxDDataViewModel : public wxDataViewModel {
private:
    wxd_DataViewModel_GetColumnCountCallback m_get_column_count;
    wxd_DataViewModel_GetRowCountCallback m_get_row_count;
    wxd_DataViewModel_GetValueCallback m_get_value;
    wxd_DataViewModel_SetValueCallback m_set_value;
    void* m_user_data;
    
public:
    WxDDataViewModel(
        wxd_DataViewModel_GetColumnCountCallback get_column_count,
        wxd_DataViewModel_GetRowCountCallback get_row_count,
        wxd_DataViewModel_GetValueCallback get_value,
        wxd_DataViewModel_SetValueCallback set_value,
        void* user_data)
        : m_get_column_count(get_column_count),
          m_get_row_count(get_row_count),
          m_get_value(get_value),
          m_set_value(set_value),
          m_user_data(user_data) {}
          
    // wxDataViewModel interface implementation
    virtual unsigned int GetColumnCount() const override {
        if (!m_get_column_count) return 0;
        return static_cast<unsigned int>(m_get_column_count(m_user_data));
    }
    
    virtual wxString GetColumnType(unsigned int col) const override {
        // We'll need a way to get column types...
        return wxS("string");
    }
    
    virtual void GetValue(wxVariant& variant, 
                         const wxDataViewItem& item, 
                         unsigned int col) const override {
        if (!m_get_value) return;
        
        // Convert wxDataViewItem to row index
        unsigned int row = wxDataViewItem(item).GetID() ? static_cast<unsigned int>(reinterpret_cast<uintptr_t>(item.GetID())) - 1 : 0;
        
        // Create wxd_Variant_t for the callback
        wxd_Variant_t wxd_variant;
        memset(&wxd_variant, 0, sizeof(wxd_variant));
        
        // Call the callback
        m_get_value(m_user_data, row, col, &wxd_variant);
        
        // Convert wxd_Variant_t to wxVariant
        switch (wxd_variant.type) {
            case WXD_VARIANT_TYPE_BOOL:
                variant = wxd_variant.data.bool_val;
                break;
            case WXD_VARIANT_TYPE_INT32:
                variant = static_cast<long>(wxd_variant.data.int32_val);
                break;
            case WXD_VARIANT_TYPE_INT64:
                variant = static_cast<long>(wxd_variant.data.int64_val);
                break;
            case WXD_VARIANT_TYPE_DOUBLE:
                variant = wxd_variant.data.double_val;
                break;
            case WXD_VARIANT_TYPE_STRING:
                if (wxd_variant.data.string_val) {
                    variant = wxString::FromUTF8(wxd_variant.data.string_val);
                    // We should free the string here
                    // free(wxd_variant.data.string_val);
                } else {
                    variant = wxString();
                }
                break;
            // Handle other types as needed
            default:
                // Set an empty variant
                variant.Clear();
                break;
        }
    }
    
    virtual bool SetValue(const wxVariant& variant, 
                         const wxDataViewItem& item, 
                         unsigned int col) override {
        if (!m_set_value) return false;
        
        // Convert wxDataViewItem to row index
        unsigned int row = wxDataViewItem(item).GetID() ? static_cast<unsigned int>(reinterpret_cast<uintptr_t>(item.GetID())) - 1 : 0;
        
        // Create wxd_Variant_t for the callback
        wxd_Variant_t wxd_variant;
        memset(&wxd_variant, 0, sizeof(wxd_variant));
        
        // Convert wxVariant to wxd_Variant_t
        if (variant.GetType() == "bool") {
            wxd_variant.type = WXD_VARIANT_TYPE_BOOL;
            wxd_variant.data.bool_val = variant.GetBool();
        } else if (variant.GetType() == "long") {
            wxd_variant.type = WXD_VARIANT_TYPE_INT64;
            wxd_variant.data.int64_val = variant.GetLong();
        } else if (variant.GetType() == "double") {
            wxd_variant.type = WXD_VARIANT_TYPE_DOUBLE;
            wxd_variant.data.double_val = variant.GetDouble();
        } else if (variant.GetType() == "string") {
            wxd_variant.type = WXD_VARIANT_TYPE_STRING;
            wxd_variant.data.string_val = strdup(variant.GetString().ToUTF8().data());
        }
        // Handle other types as needed
        
        // Call the callback
        bool result = m_set_value(m_user_data, row, col, &wxd_variant);
        
        // Free any allocated memory
        if (wxd_variant.type == WXD_VARIANT_TYPE_STRING && wxd_variant.data.string_val) {
            free(wxd_variant.data.string_val);
        }
        
        return result;
    }
    
    virtual wxDataViewItem GetParent(const wxDataViewItem& item) const override {
        // For list models, items have no parent
        return wxDataViewItem(nullptr);
    }
    
    virtual bool IsContainer(const wxDataViewItem& item) const override {
        // For list models, only the invisible root is a container
        return !item.IsOk();
    }
    
    virtual unsigned int GetChildren(const wxDataViewItem& parent, 
                                    wxDataViewItemArray& children) const override {
        if (!m_get_row_count) return 0;
        
        // For a list model, only the invisible root item has children
        if (!parent.IsOk()) {
            int count = static_cast<int>(m_get_row_count(m_user_data));
            for (int i = 0; i < count; ++i) {
                // Use index as the item ID
                children.Add(wxDataViewItem(reinterpret_cast<void*>(static_cast<uintptr_t>(i + 1))));
            }
            return count;
        }
        return 0;
    }
};

// Model creation and attachment
WXD_EXPORTED wxd_DataViewModel_t* wxd_DataViewModel_Create(
    wxd_DataViewModel_GetColumnCountCallback get_column_count,
    wxd_DataViewModel_GetRowCountCallback get_row_count,
    wxd_DataViewModel_GetValueCallback get_value,
    wxd_DataViewModel_SetValueCallback set_value,
    void* user_data) {
    
    if (!get_column_count || !get_row_count || !get_value) return nullptr;
    
    WxDDataViewModel* model = new WxDDataViewModel(
        get_column_count,
        get_row_count,
        get_value,
        set_value,
        user_data);
        
    return reinterpret_cast<wxd_DataViewModel_t*>(model);
}

WXD_EXPORTED bool wxd_DataViewCtrl_AssociateModel(wxd_Window_t* self, wxd_DataViewModel_t* model) {
    if (!self || !model) return false;
    
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    wxDataViewModel* m = reinterpret_cast<wxDataViewModel*>(model);
    
    return ctrl->AssociateModel(m);
}

// Selection management
WXD_EXPORTED bool wxd_DataViewCtrl_SelectRow(wxd_Window_t* self, int64_t row) {
    if (!self) return false;
    
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    wxDataViewItem item(reinterpret_cast<void*>(static_cast<uintptr_t>(row + 1)));
    
    ctrl->Select(item);
    return true;
}

WXD_EXPORTED int64_t wxd_DataViewCtrl_GetSelectedRow(wxd_Window_t* self) {
    if (!self) return -1;
    
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    wxDataViewItem item = ctrl->GetSelection();
    
    if (!item.IsOk()) return -1;
    
    return reinterpret_cast<uintptr_t>(item.GetID()) - 1;
}

WXD_EXPORTED void wxd_DataViewCtrl_UnselectAll(wxd_Window_t* self) {
    if (!self) return;
    
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    ctrl->UnselectAll();
}

// Standard DataViewListModel implementation
class WxDDataViewListModel : public wxDataViewListStore {
private:
    // Keep track of our columns
    struct ColumnInfo {
        wxString name;
        wxString type;
    };
    wxVector<ColumnInfo> m_columns;

public:
    // Add a column to our model
    bool AppendColumnInfo(const wxString& name, const wxString& type = "string") {
        ColumnInfo info;
        info.name = name;
        info.type = type;
        m_columns.push_back(info);
        return true;
    }
    
    // Get number of columns we've defined
    unsigned int GetColumnCount() const {
        return static_cast<unsigned int>(m_columns.size());
    }
};

// Standard models
WXD_EXPORTED wxd_DataViewModel_t* wxd_DataViewListModel_Create() {
    WxDDataViewListModel* model = new WxDDataViewListModel();
    return reinterpret_cast<wxd_DataViewModel_t*>(model);
}

WXD_EXPORTED bool wxd_DataViewListModel_AppendColumn(wxd_DataViewModel_t* self, const char* name) {
    if (!self) return false;
    
    WxDDataViewListModel* model = reinterpret_cast<WxDDataViewListModel*>(self);
    wxString wxName = wxString::FromUTF8(name ? name : "");
    
    // Actually add the column to our model
    return model->AppendColumnInfo(wxName);
}

WXD_EXPORTED bool wxd_DataViewListModel_AppendRow(wxd_DataViewModel_t* self) {
    if (!self) return false;
    
    WxDDataViewListModel* model = reinterpret_cast<WxDDataViewListModel*>(self);
    
    // Get the number of columns from our model
    size_t colCount = model->GetColumnCount();
    if (colCount == 0) {
        // No columns defined yet, can't add rows
        return false;
    }
    
    // Create proper-sized vector of variants for the new row
    wxVector<wxVariant> values;
    values.resize(colCount);  // Initialize with empty values for each column
    
    model->AppendItem(values);
    return true;
}

WXD_EXPORTED bool wxd_DataViewListModel_SetValue(wxd_DataViewModel_t* self, 
                                       uint64_t row, 
                                       uint64_t col, 
                                       const wxd_Variant_t* variant) {
    if (!self || !variant) return false;
    
    WxDDataViewListModel* model = reinterpret_cast<WxDDataViewListModel*>(self);
    
    // Make sure we have enough columns defined
    if (col >= model->GetColumnCount()) {
        return false;  // Column index out of bounds
    }
    
    // Create a wxVariant from our wxd_Variant_t
    wxVariant wxVariantValue;
    
    // Convert wxd_Variant_t to wxVariant
    switch (variant->type) {
        case WXD_VARIANT_TYPE_BOOL:
            wxVariantValue = variant->data.bool_val;
            break;
        case WXD_VARIANT_TYPE_INT32:
            wxVariantValue = static_cast<long>(variant->data.int32_val);
            break;
        case WXD_VARIANT_TYPE_INT64:
            wxVariantValue = static_cast<long>(variant->data.int64_val);
            break;
        case WXD_VARIANT_TYPE_DOUBLE:
            wxVariantValue = variant->data.double_val;
            break;
        case WXD_VARIANT_TYPE_STRING:
            if (variant->data.string_val) {
                wxVariantValue = wxString::FromUTF8(variant->data.string_val);
            } else {
                wxVariantValue = wxString();
            }
            break;
        case WXD_VARIANT_TYPE_BITMAP:
            if (variant->data.bitmap_val) {
                wxBitmap* bitmap = reinterpret_cast<wxBitmap*>(variant->data.bitmap_val);
                // Create a wxVariant with a bitmap value using native wxWidgets support
                wxVariantValue << *bitmap;
            } else {
                wxVariantValue.Clear();
            }
            break;
        case WXD_VARIANT_TYPE_DATETIME:
            {
                // Convert wxd_DateTime_t to wxDateTime
                wxDateTime dt;
                dt.Set(variant->data.datetime_val.day,
                      static_cast<wxDateTime::Month>(variant->data.datetime_val.month - 1),
                      variant->data.datetime_val.year,
                      variant->data.datetime_val.hour,
                      variant->data.datetime_val.minute,
                      variant->data.datetime_val.second);
                wxVariantValue = dt;
            }
            break;
        default:
            // Set an empty variant
            wxVariantValue.Clear();
            break;
    }
    
    // Create a wxDataViewItem for the row
    wxDataViewItem item(reinterpret_cast<void*>(static_cast<uintptr_t>(row + 1)));
    
    // Set the value
    return model->SetValue(wxVariantValue, item, static_cast<unsigned int>(col));
}

} // extern "C" 