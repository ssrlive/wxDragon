#include "../include/wxdragon.h"
#include "wxd_utils.h" // For wxd_cpp_utils::to_wxPoint, to_wxSize if used, and string conversions.
#include <wx/wx.h>
#include <wx/fontdlg.h> // wxFontDialog

extern "C" {

// --- wxFontData Implementation ---

wxd_FontData_t* wxd_FontData_Create(void) {
    wxFontData* data = new wxFontData();
    return reinterpret_cast<wxd_FontData_t*>(data);
}

void wxd_FontData_Destroy(wxd_FontData_t* self) {
    if (!self) return;
    wxFontData* data = reinterpret_cast<wxFontData*>(self);
    delete data;
}

void wxd_FontData_EnableEffects(wxd_FontData_t* self, bool enable) {
    if (!self) return;
    wxFontData* data = reinterpret_cast<wxFontData*>(self);
    data->EnableEffects(enable);
}

bool wxd_FontData_GetEnableEffects(wxd_FontData_t* self) {
    if (!self) return false;
    wxFontData* data = reinterpret_cast<wxFontData*>(self);
    return data->GetEnableEffects();
}

void wxd_FontData_SetInitialFont(wxd_FontData_t* self, const wxd_Font_t* font) {
    if (!self || !font) return;
    wxFontData* data = reinterpret_cast<wxFontData*>(self);
    wxFont* wxfont = reinterpret_cast<wxFont*>(const_cast<wxd_Font_t*>(font));
    data->SetInitialFont(*wxfont);
}

// --- wxFont Implementation ---

wxd_Font_t* wxd_Font_Create(void) {
    wxFont* font = new wxFont(wxNullFont);
    return reinterpret_cast<wxd_Font_t*>(font);
}

void wxd_Font_Destroy(wxd_Font_t* self) {
    if (!self) return;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    delete font;
}

int wxd_Font_GetPointSize(wxd_Font_t* self) {
    if (!self) return 0;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    return font->GetPointSize();
}

int wxd_Font_GetFamily(wxd_Font_t* self) {
    if (!self) return 0;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    return font->GetFamily();
}

int wxd_Font_GetStyle(wxd_Font_t* self) {
    if (!self) return 0;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    return font->GetStyle();
}

int wxd_Font_GetWeight(wxd_Font_t* self) {
    if (!self) return 0;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    return font->GetWeight();
}

bool wxd_Font_GetUnderlined(wxd_Font_t* self) {
    if (!self) return false;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    return font->GetUnderlined();
}

int wxd_Font_GetFaceName(wxd_Font_t* self, char* buffer, int buffer_len) {
    if (!self) return 0;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    wxString faceName = font->GetFaceName();
    return wxd_cpp_utils::copy_wxstring_to_buffer(faceName, buffer, buffer_len);
}

bool wxd_Font_IsOk(wxd_Font_t* self) {
    if (!self) return false;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    return font->IsOk();
}

// --- wxFontDialog Implementation ---

wxd_FontDialog_t* wxd_FontDialog_Create(
    wxd_Window_t* parent,
    const char* title,
    wxd_FontData_t* data
) {
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    wxFontData* fontData = nullptr;
    
    if (data) {
        fontData = reinterpret_cast<wxFontData*>(data);
    } else {
        // If no data provided, create a default one
        fontData = new wxFontData();
    }

    wxFontDialog* dialog = new wxFontDialog(parentWin, *fontData);
    
    // Set title if provided
    if (title && *title) {
        dialog->SetTitle(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(title));
    }
    
    // If we created our own fontData, we need to delete it
    if (!data) {
        delete fontData;
    }
    
    return reinterpret_cast<wxd_FontDialog_t*>(dialog);
}

wxd_FontData_t* wxd_FontDialog_GetFontData(wxd_FontDialog_t* self) {
    if (!self) return nullptr;
    
    wxFontDialog* dialog = reinterpret_cast<wxFontDialog*>(self);
    // Note: This returns a reference to the internal wxFontData, not a new instance
    // The pointer will be valid as long as the dialog exists
    return reinterpret_cast<wxd_FontData_t*>(&dialog->GetFontData());
}

wxd_Font_t* wxd_FontDialog_GetFont(wxd_FontDialog_t* self) {
    if (!self) return nullptr;
    
    wxFontDialog* dialog = reinterpret_cast<wxFontDialog*>(self);
    wxFont font = dialog->GetFontData().GetChosenFont();
    
    if (!font.IsOk()) {
        return nullptr;
    }
    
    // Create a new wxFont object to return (will be owned by Rust)
    wxFont* newFont = new wxFont(font);
    return reinterpret_cast<wxd_Font_t*>(newFont);
}

} // extern "C" 