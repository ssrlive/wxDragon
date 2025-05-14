#include "../include/wxdragon.h"
#include <wx/wx.h>
#include <wx/dcmemory.h>
#include <wx/dcscreen.h>

// Type aliases for easier reference
using wxd_DC_t = struct wxd_DC_t;
using wxd_WindowDC_t = struct wxd_WindowDC_t;
using wxd_ClientDC_t = struct wxd_ClientDC_t;
using wxd_PaintDC_t = struct wxd_PaintDC_t;
using wxd_MemoryDC_t = struct wxd_MemoryDC_t;
using wxd_ScreenDC_t = struct wxd_ScreenDC_t;

// The wxWidgets DC hierarchy provides some challenges for C FFI
// We need to use the opaque type pattern and typecasts:
// - wxDC (base class)
//   - wxWindowDC
//     - wxClientDC
//     - wxPaintDC
//   - wxMemoryDC
//   - wxScreenDC

// Since wxDC is abstract, we need to ensure we only expose classes that wxWidgets actually provides

// DC Creation/Destruction
wxd_WindowDC_t* wxd_WindowDC_Create(wxd_Window_t* window) {
    if (!window) return nullptr;
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    return reinterpret_cast<wxd_WindowDC_t*>(new wxWindowDC(wx_window));
}

void wxd_WindowDC_Destroy(wxd_WindowDC_t* dc) {
    if (dc) {
        delete reinterpret_cast<wxWindowDC*>(dc);
    }
}

wxd_ClientDC_t* wxd_ClientDC_Create(wxd_Window_t* window) {
    if (!window) return nullptr;
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    return reinterpret_cast<wxd_ClientDC_t*>(new wxClientDC(wx_window));
}

void wxd_ClientDC_Destroy(wxd_ClientDC_t* dc) {
    if (dc) {
        delete reinterpret_cast<wxClientDC*>(dc);
    }
}

wxd_PaintDC_t* wxd_PaintDC_Create(wxd_Window_t* window) {
    if (!window) return nullptr;
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    return reinterpret_cast<wxd_PaintDC_t*>(new wxPaintDC(wx_window));
}

void wxd_PaintDC_Destroy(wxd_PaintDC_t* dc) {
    if (dc) {
        delete reinterpret_cast<wxPaintDC*>(dc);
    }
}

wxd_MemoryDC_t* wxd_MemoryDC_Create(void) {
    return reinterpret_cast<wxd_MemoryDC_t*>(new wxMemoryDC());
}

void wxd_MemoryDC_Destroy(wxd_MemoryDC_t* dc) {
    if (dc) {
        delete reinterpret_cast<wxMemoryDC*>(dc);
    }
}

wxd_ScreenDC_t* wxd_ScreenDC_Create(void) {
    return reinterpret_cast<wxd_ScreenDC_t*>(new wxScreenDC());
}

void wxd_ScreenDC_Destroy(wxd_ScreenDC_t* dc) {
    if (dc) {
        delete reinterpret_cast<wxScreenDC*>(dc);
    }
}

// Type casting functions
wxd_DC_t* wxd_WindowDC_AsDC(wxd_WindowDC_t* dc) {
    return reinterpret_cast<wxd_DC_t*>(static_cast<wxDC*>(reinterpret_cast<wxWindowDC*>(dc)));
}

wxd_DC_t* wxd_ClientDC_AsDC(wxd_ClientDC_t* dc) {
    return reinterpret_cast<wxd_DC_t*>(static_cast<wxDC*>(reinterpret_cast<wxClientDC*>(dc)));
}

wxd_DC_t* wxd_PaintDC_AsDC(wxd_PaintDC_t* dc) {
    return reinterpret_cast<wxd_DC_t*>(static_cast<wxDC*>(reinterpret_cast<wxPaintDC*>(dc)));
}

wxd_DC_t* wxd_MemoryDC_AsDC(wxd_MemoryDC_t* dc) {
    return reinterpret_cast<wxd_DC_t*>(static_cast<wxDC*>(reinterpret_cast<wxMemoryDC*>(dc)));
}

wxd_DC_t* wxd_ScreenDC_AsDC(wxd_ScreenDC_t* dc) {
    return reinterpret_cast<wxd_DC_t*>(static_cast<wxDC*>(reinterpret_cast<wxScreenDC*>(dc)));
}

// Common DC operations
void wxd_DC_Clear(wxd_DC_t* dc) {
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->Clear();
    }
}

void wxd_DC_SetBackground(wxd_DC_t* dc, wxd_Colour_t background) {
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxColour wx_color(background.r, background.g, background.b, background.a);
        wx_dc->SetBackground(wxBrush(wx_color));
    }
}

void wxd_DC_SetBackgroundMode(wxd_DC_t* dc, int mode) {
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        int wx_mode = mode == WXD_TRANSPARENT ? wxTRANSPARENT : wxSOLID;
        wx_dc->SetBackgroundMode(wx_mode);
    }
}

void wxd_DC_SetTextBackground(wxd_DC_t* dc, wxd_Colour_t colour) {
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxColour wx_color(colour.r, colour.g, colour.b, colour.a);
        wx_dc->SetTextBackground(wx_color);
    }
}

void wxd_DC_SetTextForeground(wxd_DC_t* dc, wxd_Colour_t colour) {
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxColour wx_color(colour.r, colour.g, colour.b, colour.a);
        wx_dc->SetTextForeground(wx_color);
    }
}

void wxd_DC_SetFont(wxd_DC_t* dc, const wxd_Font_t* font) {
    if (dc && font) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxFont* wx_font = reinterpret_cast<wxFont*>(const_cast<wxd_Font_t*>(font));
        wx_dc->SetFont(*wx_font);
    }
}

void wxd_DC_SetPen(wxd_DC_t* dc, wxd_Colour_t colour, int width, int style) {
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxColour wx_color(colour.r, colour.g, colour.b, colour.a);
        
        wxPenStyle wx_style;
        switch (style) {
            case WXD_PENSTYLE_SOLID: wx_style = wxPENSTYLE_SOLID; break;
            case WXD_PENSTYLE_DOT: wx_style = wxPENSTYLE_DOT; break;
            case WXD_PENSTYLE_LONG_DASH: wx_style = wxPENSTYLE_LONG_DASH; break;
            case WXD_PENSTYLE_SHORT_DASH: wx_style = wxPENSTYLE_SHORT_DASH; break;
            case WXD_PENSTYLE_DOT_DASH: wx_style = wxPENSTYLE_DOT_DASH; break;
            case WXD_PENSTYLE_TRANSPARENT: wx_style = wxPENSTYLE_TRANSPARENT; break;
            case WXD_PENSTYLE_STIPPLE: wx_style = wxPENSTYLE_STIPPLE; break;
            case WXD_PENSTYLE_USER_DASH: wx_style = wxPENSTYLE_USER_DASH; break;
            case WXD_PENSTYLE_BDIAGONAL_HATCH: wx_style = wxPENSTYLE_BDIAGONAL_HATCH; break;
            case WXD_PENSTYLE_CROSSDIAG_HATCH: wx_style = wxPENSTYLE_CROSSDIAG_HATCH; break;
            case WXD_PENSTYLE_FDIAGONAL_HATCH: wx_style = wxPENSTYLE_FDIAGONAL_HATCH; break;
            case WXD_PENSTYLE_CROSS_HATCH: wx_style = wxPENSTYLE_CROSS_HATCH; break;
            case WXD_PENSTYLE_HORIZONTAL_HATCH: wx_style = wxPENSTYLE_HORIZONTAL_HATCH; break;
            case WXD_PENSTYLE_VERTICAL_HATCH: wx_style = wxPENSTYLE_VERTICAL_HATCH; break;
            default: wx_style = wxPENSTYLE_SOLID; break;
        }
        
        wx_dc->SetPen(wxPen(wx_color, width, wx_style));
    }
}

void wxd_DC_SetBrush(wxd_DC_t* dc, wxd_Colour_t colour, int style) {
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxColour wx_color(colour.r, colour.g, colour.b, colour.a);
        
        wxBrushStyle wx_style;
        switch (style) {
            case WXD_BRUSHSTYLE_SOLID: wx_style = wxBRUSHSTYLE_SOLID; break;
            case WXD_BRUSHSTYLE_TRANSPARENT: wx_style = wxBRUSHSTYLE_TRANSPARENT; break;
            case WXD_BRUSHSTYLE_BDIAGONAL_HATCH: wx_style = wxBRUSHSTYLE_BDIAGONAL_HATCH; break;
            case WXD_BRUSHSTYLE_CROSSDIAG_HATCH: wx_style = wxBRUSHSTYLE_CROSSDIAG_HATCH; break;
            case WXD_BRUSHSTYLE_FDIAGONAL_HATCH: wx_style = wxBRUSHSTYLE_FDIAGONAL_HATCH; break;
            case WXD_BRUSHSTYLE_CROSS_HATCH: wx_style = wxBRUSHSTYLE_CROSS_HATCH; break;
            case WXD_BRUSHSTYLE_HORIZONTAL_HATCH: wx_style = wxBRUSHSTYLE_HORIZONTAL_HATCH; break;
            case WXD_BRUSHSTYLE_VERTICAL_HATCH: wx_style = wxBRUSHSTYLE_VERTICAL_HATCH; break;
            case WXD_BRUSHSTYLE_STIPPLE: wx_style = wxBRUSHSTYLE_STIPPLE; break;
            case WXD_BRUSHSTYLE_STIPPLE_MASK_OPAQUE: wx_style = wxBRUSHSTYLE_STIPPLE_MASK_OPAQUE; break;
            case WXD_BRUSHSTYLE_STIPPLE_MASK: wx_style = wxBRUSHSTYLE_STIPPLE_MASK; break;
            default: wx_style = wxBRUSHSTYLE_SOLID; break;
        }
        
        wx_dc->SetBrush(wxBrush(wx_color, wx_style));
    }
}

// Drawing operations
void wxd_DC_DrawPoint(wxd_DC_t* dc, int x, int y) {
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->DrawPoint(x, y);
    }
}

void wxd_DC_DrawLine(wxd_DC_t* dc, int x1, int y1, int x2, int y2) {
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->DrawLine(x1, y1, x2, y2);
    }
}

void wxd_DC_DrawRectangle(wxd_DC_t* dc, int x, int y, int width, int height) {
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->DrawRectangle(x, y, width, height);
    }
}

void wxd_DC_DrawCircle(wxd_DC_t* dc, int x, int y, int radius) {
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->DrawCircle(x, y, radius);
    }
}

void wxd_DC_DrawEllipse(wxd_DC_t* dc, int x, int y, int width, int height) {
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->DrawEllipse(x, y, width, height);
    }
}

void wxd_DC_DrawText(wxd_DC_t* dc, const char* text, int x, int y) {
    if (dc && text) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxString wx_text = wxString::FromUTF8(text ? text : "");
        wx_dc->DrawText(wx_text, x, y);
    }
}

void wxd_DC_DrawBitmap(wxd_DC_t* dc, const wxd_Bitmap_t* bitmap, int x, int y, bool transparent) {
    if (dc && bitmap) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxBitmap* wx_bitmap = reinterpret_cast<wxBitmap*>(const_cast<wxd_Bitmap_t*>(bitmap));
        wx_dc->DrawBitmap(*wx_bitmap, x, y, transparent);
    }
}

// Device capabilities
wxd_Size wxd_DC_GetSize(wxd_DC_t* dc) {
    wxd_Size size = {0, 0};
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxSize wx_size = wx_dc->GetSize();
        size.width = wx_size.x;
        size.height = wx_size.y;
    }
    return size;
}

void wxd_DC_GetTextExtent(wxd_DC_t* dc, const char* string, int* w, int* h) {
    if (dc && string && w && h) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxString wx_text = wxString::FromUTF8(string);
        int width, height;
        wx_dc->GetTextExtent(wx_text, &width, &height);
        *w = width;
        *h = height;
    }
}

// MemoryDC specific operations
void wxd_MemoryDC_SelectObject(wxd_MemoryDC_t* dc, wxd_Bitmap_t* bitmap) {
    if (dc && bitmap) {
        wxMemoryDC* wx_dc = reinterpret_cast<wxMemoryDC*>(dc);
        wxBitmap* wx_bitmap = reinterpret_cast<wxBitmap*>(bitmap);
        wx_dc->SelectObject(*wx_bitmap);
    }
}

void wxd_MemoryDC_SelectObjectAsSource(wxd_MemoryDC_t* dc, wxd_Bitmap_t* bitmap) {
    if (dc && bitmap) {
        wxMemoryDC* wx_dc = reinterpret_cast<wxMemoryDC*>(dc);
        wxBitmap* wx_bitmap = reinterpret_cast<wxBitmap*>(bitmap);
        wx_dc->SelectObjectAsSource(*wx_bitmap);
    }
} 