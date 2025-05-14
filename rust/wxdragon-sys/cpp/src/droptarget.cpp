#include "../include/wxdragon.h"
#include <wx/wx.h>
#include <wx/dnd.h>
#include <wx/tokenzr.h>

// Full-featured text drop target implementation
class WxdTextDropTargetFull : public wxTextDropTarget {
public:
    WxdTextDropTargetFull(
        wxd_OnEnter_Callback onEnter,
        wxd_OnDragOver_Callback onDragOver,
        wxd_OnLeave_Callback onLeave,
        wxd_OnDrop_Callback onDrop,
        wxd_OnData_Callback onData,
        wxd_OnDropText_Callback onDropText,
        void* userData)
        : m_onEnter(onEnter),
          m_onDragOver(onDragOver),
          m_onLeave(onLeave),
          m_onDrop(onDrop),
          m_onData(onData),
          m_onDropText(onDropText),
          m_userData(userData) {}

    virtual wxDragResult OnEnter(wxCoord x, wxCoord y, wxDragResult defResult) override {
        if (m_onEnter) {
            wxd_DragResult result = m_onEnter(x, y, static_cast<wxd_DragResult>(defResult), m_userData);
            return static_cast<wxDragResult>(result);
        }
        return wxTextDropTarget::OnEnter(x, y, defResult);
    }

    virtual wxDragResult OnDragOver(wxCoord x, wxCoord y, wxDragResult defResult) override {
        if (m_onDragOver) {
            wxd_DragResult result = m_onDragOver(x, y, static_cast<wxd_DragResult>(defResult), m_userData);
            return static_cast<wxDragResult>(result);
        }
        return wxTextDropTarget::OnDragOver(x, y, defResult);
    }

    virtual void OnLeave() override {
        if (m_onLeave) {
            m_onLeave(m_userData);
        } else {
            wxTextDropTarget::OnLeave();
        }
    }

    virtual bool OnDrop(wxCoord x, wxCoord y) override {
        if (m_onDrop) {
            return m_onDrop(x, y, m_userData);
        }
        return wxTextDropTarget::OnDrop(x, y);
    }

    virtual wxDragResult OnData(wxCoord x, wxCoord y, wxDragResult defResult) override {
        // Call the base implementation to get the data
        wxDragResult baseResult = wxTextDropTarget::OnData(x, y, defResult);
        
        // If the base implementation succeeded and we have custom handler
        if (baseResult != wxDragNone && m_onData) {
            wxd_DragResult result = m_onData(x, y, static_cast<wxd_DragResult>(defResult), m_userData);
            return static_cast<wxDragResult>(result);
        }
        
        return baseResult;
    }

    virtual bool OnDropText(wxCoord x, wxCoord y, const wxString& text) override {
        if (m_onDropText) {
            wxScopedCharBuffer utf8 = text.utf8_str();
            return m_onDropText(utf8.data(), x, y, m_userData);
        }
        return false;
    }

private:
    wxd_OnEnter_Callback m_onEnter;
    wxd_OnDragOver_Callback m_onDragOver;
    wxd_OnLeave_Callback m_onLeave;
    wxd_OnDrop_Callback m_onDrop;
    wxd_OnData_Callback m_onData;
    wxd_OnDropText_Callback m_onDropText;
    void* m_userData;
};

// Simple text drop target (legacy API support)
class WxdTextDropTarget : public wxTextDropTarget {
public:
    WxdTextDropTarget(void* callback, void* userData)
        : m_callback(reinterpret_cast<wxd_OnDropText_Callback>(callback)),
          m_userData(userData) {}

    virtual bool OnDropText(wxCoord x, wxCoord y, const wxString& text) override {
        if (m_callback) {
            wxScopedCharBuffer utf8 = text.utf8_str();
            return m_callback(utf8.data(), x, y, m_userData);
        }
        return false;
    }

private:
    wxd_OnDropText_Callback m_callback;
    void* m_userData;
};

// Full-featured file drop target implementation
class WxdFileDropTargetFull : public wxFileDropTarget {
public:
    WxdFileDropTargetFull(
        wxd_OnEnter_Callback onEnter,
        wxd_OnDragOver_Callback onDragOver,
        wxd_OnLeave_Callback onLeave,
        wxd_OnDrop_Callback onDrop,
        wxd_OnData_Callback onData,
        wxd_OnDropFiles_Callback onDropFiles,
        void* userData)
        : m_onEnter(onEnter),
          m_onDragOver(onDragOver),
          m_onLeave(onLeave),
          m_onDrop(onDrop),
          m_onData(onData),
          m_onDropFiles(onDropFiles),
          m_userData(userData) {}

    virtual wxDragResult OnEnter(wxCoord x, wxCoord y, wxDragResult defResult) override {
        if (m_onEnter) {
            wxd_DragResult result = m_onEnter(x, y, static_cast<wxd_DragResult>(defResult), m_userData);
            return static_cast<wxDragResult>(result);
        }
        return wxFileDropTarget::OnEnter(x, y, defResult);
    }

    virtual wxDragResult OnDragOver(wxCoord x, wxCoord y, wxDragResult defResult) override {
        if (m_onDragOver) {
            wxd_DragResult result = m_onDragOver(x, y, static_cast<wxd_DragResult>(defResult), m_userData);
            return static_cast<wxDragResult>(result);
        }
        return wxFileDropTarget::OnDragOver(x, y, defResult);
    }

    virtual void OnLeave() override {
        if (m_onLeave) {
            m_onLeave(m_userData);
        } else {
            wxFileDropTarget::OnLeave();
        }
    }

    virtual bool OnDrop(wxCoord x, wxCoord y) override {
        if (m_onDrop) {
            return m_onDrop(x, y, m_userData);
        }
        return wxFileDropTarget::OnDrop(x, y);
    }

    virtual wxDragResult OnData(wxCoord x, wxCoord y, wxDragResult defResult) override {
        // Call the base implementation to get the data
        wxDragResult baseResult = wxFileDropTarget::OnData(x, y, defResult);
        
        // If the base implementation succeeded and we have custom handler
        if (baseResult != wxDragNone && m_onData) {
            wxd_DragResult result = m_onData(x, y, static_cast<wxd_DragResult>(defResult), m_userData);
            return static_cast<wxDragResult>(result);
        }
        
        return baseResult;
    }

    virtual bool OnDropFiles(wxCoord x, wxCoord y, const wxArrayString& filenames) override {
        if (m_onDropFiles) {
            // Create a wxd_ArrayString_t to pass to the callback
            wxd_ArrayString_t* wxdArray = new wxd_ArrayString_t();
            wxdArray->internal_data = new wxArrayString(filenames);
            
            bool result = m_onDropFiles(wxdArray, x, y, m_userData);
            
            // Clean up
            delete static_cast<wxArrayString*>(wxdArray->internal_data);
            delete wxdArray;
            
            return result;
        }
        return false;
    }

private:
    wxd_OnEnter_Callback m_onEnter;
    wxd_OnDragOver_Callback m_onDragOver;
    wxd_OnLeave_Callback m_onLeave;
    wxd_OnDrop_Callback m_onDrop;
    wxd_OnData_Callback m_onData;
    wxd_OnDropFiles_Callback m_onDropFiles;
    void* m_userData;
};

// Simple file drop target (legacy API support)
class WxdFileDropTarget : public wxFileDropTarget {
public:
    WxdFileDropTarget(void* callback, void* userData)
        : m_callback(reinterpret_cast<wxd_OnDropFiles_Callback>(callback)),
          m_userData(userData) {}

    virtual bool OnDropFiles(wxCoord x, wxCoord y, const wxArrayString& filenames) override {
        if (m_callback) {
            // Create a wxd_ArrayString_t to pass to the callback
            wxd_ArrayString_t* wxdArray = new wxd_ArrayString_t();
            wxdArray->internal_data = new wxArrayString(filenames);
            
            bool result = m_callback(wxdArray, x, y, m_userData);
            
            // Clean up
            delete static_cast<wxArrayString*>(wxdArray->internal_data);
            delete wxdArray;
            
            return result;
        }
        return false;
    }

private:
    wxd_OnDropFiles_Callback m_callback;
    void* m_userData;
};

extern "C" {
    
// Create text drop target with full callback set
WXD_EXPORTED wxd_TextDropTarget_t* wxd_TextDropTarget_CreateFull(
    wxd_Window_t* window, 
    wxd_OnEnter_Callback onEnter,
    wxd_OnDragOver_Callback onDragOver,
    wxd_OnLeave_Callback onLeave,
    wxd_OnDrop_Callback onDrop,
    wxd_OnData_Callback onData,
    wxd_OnDropText_Callback onDropText,
    void* userData) {
    
    if (!window) return nullptr;

    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    
    WxdTextDropTargetFull* drop_target = new WxdTextDropTargetFull(
        onEnter, onDragOver, onLeave, onDrop, onData, onDropText, userData);
    
    wx_window->SetDropTarget(drop_target);
    
    return reinterpret_cast<wxd_TextDropTarget_t*>(drop_target);
}

// Create file drop target with full callback set
WXD_EXPORTED wxd_FileDropTarget_t* wxd_FileDropTarget_CreateFull(
    wxd_Window_t* window, 
    wxd_OnEnter_Callback onEnter,
    wxd_OnDragOver_Callback onDragOver,
    wxd_OnLeave_Callback onLeave,
    wxd_OnDrop_Callback onDrop,
    wxd_OnData_Callback onData,
    wxd_OnDropFiles_Callback onDropFiles,
    void* userData) {
    
    if (!window) return nullptr;

    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    
    WxdFileDropTargetFull* drop_target = new WxdFileDropTargetFull(
        onEnter, onDragOver, onLeave, onDrop, onData, onDropFiles, userData);
    
    wx_window->SetDropTarget(drop_target);
    
    return reinterpret_cast<wxd_FileDropTarget_t*>(drop_target);
}

// Create text drop target (simplified version)
WXD_EXPORTED wxd_TextDropTarget_t* wxd_TextDropTarget_Create(
    wxd_Window_t* window,
    void* onDropTextCallback,
    void* userData) {
    
    if (!window) return nullptr;

    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    
    WxdTextDropTarget* drop_target = new WxdTextDropTarget(onDropTextCallback, userData);
    
    wx_window->SetDropTarget(drop_target);
    
    return reinterpret_cast<wxd_TextDropTarget_t*>(drop_target);
}

// Create file drop target (simplified version)
WXD_EXPORTED wxd_FileDropTarget_t* wxd_FileDropTarget_Create(
    wxd_Window_t* window,
    void* onDropFilesCallback,
    void* userData) {
    
    if (!window) return nullptr;

    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    
    WxdFileDropTarget* drop_target = new WxdFileDropTarget(onDropFilesCallback, userData);
    
    wx_window->SetDropTarget(drop_target);
    
    return reinterpret_cast<wxd_FileDropTarget_t*>(drop_target);
}

// Destroy drop targets
WXD_EXPORTED void wxd_TextDropTarget_Destroy(wxd_TextDropTarget_t* dropTarget) {
    // We don't actually delete the drop target here, because wxWindow takes ownership.
    // wxWidgets will delete it when the window is destroyed.
    // This function is kept for API completeness.
}

WXD_EXPORTED void wxd_FileDropTarget_Destroy(wxd_FileDropTarget_t* dropTarget) {
    // We don't actually delete the drop target here, because wxWindow takes ownership.
    // wxWidgets will delete it when the window is destroyed.
    // This function is kept for API completeness.
}

// ArrayString helper functions
WXD_EXPORTED int wxd_ArrayString_GetCount(wxd_ArrayString_t* array) {
    if (!array || !array->internal_data) return 0;
    wxArrayString* wx_array = static_cast<wxArrayString*>(array->internal_data);
    return wx_array->GetCount();
}

WXD_EXPORTED int wxd_ArrayString_GetString(wxd_ArrayString_t* array, int index, char* buffer, int bufferLen) {
    if (!array || !array->internal_data || !buffer || bufferLen <= 0) return -1;
    
    wxArrayString* wx_array = static_cast<wxArrayString*>(array->internal_data);
    if (index < 0 || index >= static_cast<int>(wx_array->GetCount())) return -1;
    
    wxString str = wx_array->Item(index);
    wxScopedCharBuffer utf8 = str.utf8_str();
    
    size_t len = strlen(utf8.data());
    if (len >= static_cast<size_t>(bufferLen)) {
        // Buffer too small, truncate
        len = bufferLen - 1;
    }
    
    memcpy(buffer, utf8.data(), len);
    buffer[len] = '\0';
    
    return len;
}

} // extern "C" 