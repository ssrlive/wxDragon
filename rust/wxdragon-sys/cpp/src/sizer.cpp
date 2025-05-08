#include "../include/wxdragon.h"
#include <wx/wx.h>
#include <wx/sizer.h>
#include <wx/statbox.h>

extern "C" {

// --- Sizer Functions ---

WXD_EXPORTED wxd_Sizer_t* wxd_BoxSizer_Create(wxd_Orientation_t orient) {
    // Cast integer orientation to wxOrientation
    wxOrientation wx_orient = static_cast<wxOrientation>(orient);
    wxBoxSizer* sizer = new wxBoxSizer(wx_orient);
    return reinterpret_cast<wxd_Sizer_t*>(sizer);
}

WXD_EXPORTED void wxd_Sizer_AddWindow(
    wxd_Sizer_t* sizer, 
    wxd_Window_t* window, 
    int proportion, 
    wxd_SizerFlags_t flag,
    int border) 
{
    if (!sizer || !window) return;
    wxSizer* wx_sizer = reinterpret_cast<wxSizer*>(sizer);
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    
    // Ensure flag is treated as an int for wxWidgets flags (safe due to typedef)
    wx_sizer->Add(wx_window, proportion, flag, border);
}

WXD_EXPORTED void wxd_Sizer_AddSpacer(wxd_Sizer_t* sizer, int size) {
    if (!sizer) return;
    wxSizer* wx_sizer = reinterpret_cast<wxSizer*>(sizer);
    wx_sizer->AddSpacer(size);
}

WXD_EXPORTED void wxd_Sizer_AddSizer(
    wxd_Sizer_t* sizer,
    wxd_Sizer_t* childSizer,
    int proportion,
    wxd_SizerFlags_t flag,
    int border)
{
    if (!sizer || !childSizer) return;
    wxSizer* wx_sizer = reinterpret_cast<wxSizer*>(sizer);
    wxSizer* wx_child_sizer = reinterpret_cast<wxSizer*>(childSizer);

    // The parent sizer takes ownership of the child sizer pointer
    // when added. The Rust side should `std::mem::forget` the wrapper
    // for the child sizer to prevent a double free.
    wx_sizer->Add(wx_child_sizer, proportion, flag, border);
}

WXD_EXPORTED void wxd_Sizer_AddStretchSpacer(wxd_Sizer_t* sizer, int prop)
{
    if (!sizer) return;
    wxSizer* wx_sizer = reinterpret_cast<wxSizer*>(sizer);
    wx_sizer->AddStretchSpacer(prop);
}

// --- ADDED: wxStaticBoxSizer Implementation ---

WXD_EXPORTED wxd_StaticBoxSizer_t* wxd_StaticBoxSizer_Create_WithBox(
    wxd_StaticBox_t* box, 
    wxd_Orientation_t orient
) {
    if (!box) return nullptr;
    wxStaticBox* wx_box = reinterpret_cast<wxStaticBox*>(box);
    wxOrientation wx_orient = static_cast<wxOrientation>(orient);
    wxStaticBoxSizer* sizer = new wxStaticBoxSizer(wx_box, wx_orient);
    return reinterpret_cast<wxd_StaticBoxSizer_t*>(sizer);
}

WXD_EXPORTED wxd_StaticBoxSizer_t* wxd_StaticBoxSizer_Create_WithLabel(
    wxd_Orientation_t orient, 
    wxd_Window_t* parent,
    const char* label
) {
    if (!parent) return nullptr;
    wxWindow* wx_parent = reinterpret_cast<wxWindow*>(parent);
    wxOrientation wx_orient = static_cast<wxOrientation>(orient);
    wxStaticBoxSizer* sizer = new wxStaticBoxSizer(
        wx_orient,
        wx_parent, 
        wxString::FromUTF8(label ? label : "")
    );
    return reinterpret_cast<wxd_StaticBoxSizer_t*>(sizer);
}

wxd_StaticBox_t* wxd_StaticBoxSizer_GetStaticBox(wxd_StaticBoxSizer_t *self) {
    if (!self) return nullptr;
    wxStaticBoxSizer* wx_sizer = reinterpret_cast<wxStaticBoxSizer*>(self);
    return reinterpret_cast<wxd_StaticBox_t*>(wx_sizer->GetStaticBox());
}

// --- wxFlexGridSizer ---
wxd_FlexGridSizer_t *wxd_FlexGridSizer_Create(int rows, int cols, int vgap, int hgap) {
    wxFlexGridSizer* sizer = new wxFlexGridSizer(rows, cols, vgap, hgap);
    return reinterpret_cast<wxd_FlexGridSizer_t*>(sizer);
}

wxd_FlexGridSizer_t *wxd_FlexGridSizer_CreateWithGap(int rows, int cols, int gap_width, int gap_height) {
    wxFlexGridSizer* sizer = new wxFlexGridSizer(rows, cols, wxSize(gap_width, gap_height));
    return reinterpret_cast<wxd_FlexGridSizer_t*>(sizer);
}

void wxd_FlexGridSizer_AddGrowableCol(wxd_FlexGridSizer_t *self, size_t idx, int proportion) {
    if (self) {
        wxFlexGridSizer* wx_sizer = reinterpret_cast<wxFlexGridSizer*>(self);
        wx_sizer->AddGrowableCol(idx, proportion);
    }
}

void wxd_FlexGridSizer_AddGrowableRow(wxd_FlexGridSizer_t *self, size_t idx, int proportion) {
    if (self) {
        wxFlexGridSizer* wx_sizer = reinterpret_cast<wxFlexGridSizer*>(self);
        wx_sizer->AddGrowableRow(idx, proportion);
    }
}

void wxd_FlexGridSizer_SetFlexibleDirection(wxd_FlexGridSizer_t *self, int direction) {
    if (self) {
        wxFlexGridSizer* wx_sizer = reinterpret_cast<wxFlexGridSizer*>(self);
        wx_sizer->SetFlexibleDirection(direction);
    }
}

void wxd_FlexGridSizer_SetNonFlexibleGrowMode(wxd_FlexGridSizer_t *self, int mode) {
    if (self) {
        wxFlexGridSizer* wx_sizer = reinterpret_cast<wxFlexGridSizer*>(self);
        wx_sizer->SetNonFlexibleGrowMode(static_cast<wxFlexSizerGrowMode>(mode));
    }
}

// --- Treebook ---

} // extern "C" 