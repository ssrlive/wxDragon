#include "../include/wxdragon.h" // Needed for WXD_EXPORTED, wxd_* types, WXD_ID_ANY
#include <wx/window.h> // Needed for wxWindow
#include <wx/sizer.h>  // Needed for wxSizer
#include "wx/gdicmn.h" // For wxSize, wxPoint
#include <cstring> // For strdup
#include <cstdlib> // For free (though free typically comes from stdlib.h, cstdlib is C++ way)
#include <wx/font.h> // For wxFont in SetFont
#include <wx/settings.h> // For wxSystemSettings and wxSYS_DEFAULT_GUI_FONT

extern "C" {

// --- General Window Functions ---

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

// --- Window Manipulation --- 

WXD_EXPORTED void wxd_Window_Show(wxd_Window_t* self, bool show) {
    if (self) {
        reinterpret_cast<wxWindow*>(self)->Show(show);
    }
}

WXD_EXPORTED bool wxd_Window_Close(wxd_Window_t* self, bool force) {
    if (self) {
        return reinterpret_cast<wxWindow*>(self)->Close(force);
    }
    return false;
}

WXD_EXPORTED void wxd_Window_SetId(wxd_Window_t* self, int id) {
    if (self) {
        reinterpret_cast<wxWindow*>(self)->SetId(id);
    }
}

// Note: GetLabel for a generic wxWindow might not always be what's expected,
// as not all windows have a visible label in the same way as controls.
// However, wxWindow itself does have GetLabel/SetLabel.
WXD_EXPORTED void wxd_Window_SetLabel(wxd_Window_t* self, const char* label) {
    if (self) {
        reinterpret_cast<wxWindow*>(self)->SetLabel(wxString::FromUTF8(label ? label : ""));
    }
}

WXD_EXPORTED char* wxd_Window_GetLabel(wxd_Window_t* self) {
    if (self) {
        wxString label = reinterpret_cast<wxWindow*>(self)->GetLabel();
        const wxScopedCharBuffer utf8_buf = label.ToUTF8();
        if (utf8_buf.data()) { // Check if data is not null
            return strdup(utf8_buf.data()); // Allocate and copy string
        }
    }
    // Return a duplicated empty string if self is null or label is empty to avoid returning NULL
    // which Rust CString::from_raw would panic on.
    // Callers should check if the string is empty if that has meaning.
    return strdup(""); 
}

WXD_EXPORTED void wxd_Window_SetFont(wxd_Window_t* self, const wxd_Font_t* font) {
    if (!self) return;
    // If font is NULL, wxWidgets SetFont will likely use a default or do nothing.
    // If it requires a valid font, we might need to pass wxNullFont or wxSystemSettings::GetFont(wxSYS_DEFAULT_GUI_FONT).
    // For now, assume passing NULL (if wxd_Font_t* is NULL) is handled gracefully by wxFont constructor or SetFont.
    // wxFont takes a wxFont*, so if font is NULL, it will be wxFont(*NULL) which might be an issue.
    // Better to check for NULL font and pass wxNullFont explicitly.
    if (font) {
        ((wxWindow*)self)->SetFont(*((wxFont*)font));
    } else {
        // Attempt to set a default font or do nothing if wxNullFont isn't appropriate.
        // For now, let's try setting the system default GUI font if null is passed.
        // This behavior might need adjustment based on desired semantics for a null font.
        ((wxWindow*)self)->SetFont(wxSystemSettings::GetFont(wxSYS_DEFAULT_GUI_FONT));
    }
}

WXD_EXPORTED wxd_Point wxd_Window_GetPosition(wxd_Window_t* self) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(self);
    if (!wx_window) {
        return { -1, -1 }; // Default invalid position
    }
    wxPoint wx_position = wx_window->GetPosition();
    return { wx_position.x, wx_position.y };
}

} // extern "C"