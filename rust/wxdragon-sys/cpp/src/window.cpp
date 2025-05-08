#include "wxdragon.h" // Needed for WXD_EXPORTED, wxd_* types, WXD_ID_ANY
#include <wx/window.h> // Needed for wxWindow
#include <wx/sizer.h>  // Needed for wxSizer
#include "wx/gdicmn.h" // For wxSize, wxPoint

// Implementation for wxd_Window_SetSizer
WXD_EXPORTED void wxd_Window_SetSizer(wxd_Window_t* window, wxd_Sizer_t* sizer, bool deleteOldSizer) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    wxSizer* wx_sizer = reinterpret_cast<wxSizer*>(sizer);
    if (wx_window && wx_sizer) {
        wx_window->SetSizer(wx_sizer, deleteOldSizer);
    }
}

// Implementation for wxd_Window_SetSizerAndFit
WXD_EXPORTED void wxd_Window_SetSizerAndFit(wxd_Window_t* window, wxd_Sizer_t* sizer, bool deleteOldSizer) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    wxSizer* wx_sizer = reinterpret_cast<wxSizer*>(sizer);
    if (wx_window && wx_sizer) {
        wx_window->SetSizerAndFit(wx_sizer, deleteOldSizer);
    }
}

// Implementation for wxd_Window_GetId
WXD_EXPORTED int wxd_Window_GetId(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (!wx_window) {
        return wxID_ANY; // Use wxID_ANY instead of WXD_ID_ANY
    }
    return wx_window->GetId();
}

// Placeholder for wxd_Window_Destroy if needed later
WXD_EXPORTED void wxd_Window_Destroy(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        // For non-top-level windows, Destroy() schedules deletion.
        // For top-level windows (wxFrame, wxDialog), it tries to close them first.
        // If the window is a child of another, its parent will typically manage its actual deletion
        // from memory after it's Destroy()'d (removed from window hierarchy and pending deletion).
        // If it's a dynamically allocated top-level window not managed by wxApp, direct deletion
        // might be considered but Destroy() is safer for proper cleanup.
        wx_window->Destroy();
    }
}

WXD_EXPORTED void wxd_Window_SetBackgroundColor(wxd_Window_t* window, wxd_Colour_t color) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wx_window->SetBackgroundColour(wxColour(color.r, color.g, color.b, color.a));
    }
}

WXD_EXPORTED void wxd_Window_SetMinSize(wxd_Window_t* window, wxd_Size size) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wx_window->SetMinSize(wxSize(size.width, size.height));
    }
}

WXD_EXPORTED void wxd_Window_Refresh(wxd_Window_t* window, int eraseBackground, const wxd_Rect* rect) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        if (rect) {
            wx_window->RefreshRect(wxRect(rect->x, rect->y, rect->width, rect->height), eraseBackground);
        } else {
            wx_window->Refresh(eraseBackground);
        }
    }
}

extern "C" {

// --- General Window Functions ---

WXD_EXPORTED void wxd_Window_Fit(wxd_Window_t* window) {
    wxWindow* win = (wxWindow*)window;
    if (win) {
        win->Fit();
    }
}

WXD_EXPORTED wxd_Size wxd_Window_GetBestSize(wxd_Window_t* window) {
    wxWindow* win = (wxWindow*)window;
    wxd_Size result = { -1, -1 }; // Default invalid size
    if (win) {
        wxSize bestSize = win->GetBestSize();
        result.width = bestSize.GetWidth();
        result.height = bestSize.GetHeight();
    }
    return result;
}

// ADDED: Implementation for wxd_Window_SetToolTip
WXD_EXPORTED void wxd_Window_SetToolTip(wxd_Window_t* window, const char* tipString) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        // wxString::FromUTF8 handles NULL tipString gracefully (creates empty string)
        wx_window->SetToolTip(wxString::FromUTF8(tipString)); 
    }
}

// Implement other wxd_Window_* functions from the header here (e.g., Show, Destroy) if needed.

// Note: SetSizer and SetSizerAndFit are often implemented in sizer.cpp or 
// directly where wxSizer is involved, but could be here too if preferred.
// Checking existing code... It seems SetSizer and SetSizerAndFit are in sizer.cpp.

// Attach/Detach/Notify cleanup functions are already implemented (likely in app.cpp or event.cpp).

} // extern "C"