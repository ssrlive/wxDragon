#include "../include/wxdragon.h"
#include <wx/wx.h>

// Only compile when XRC is enabled
#if WXD_USE_XRC

#include <wx/xrc/xmlres.h>
#include <wx/xrc/xh_stbmp.h>  // Include for XRC_MAKE_INSTANCE macro
#include <wx/statbmp.h>
#ifdef __WXMSW__
#include <wx/generic/statbmpg.h>
#endif

/**
 * Custom XRC handler that creates platform-appropriate StaticBitmap widgets:
 * - On Windows: creates wxGenericStaticBitmap for proper scaling support
 * - On other platforms: creates native wxStaticBitmap
 */
class WxdPlatformAwareStaticBitmapHandler : public wxXmlResourceHandler
{
public:
    WxdPlatformAwareStaticBitmapHandler() {}

    virtual wxObject *DoCreateResource()
    {
        // Get the bitmap - can be from 'bitmap' attribute or child <bitmap> node
        wxBitmap bitmap = GetBitmap(wxT("bitmap"), wxART_OTHER);

        // Create the appropriate control based on platform
        wxStaticBitmap* control = nullptr;

#ifdef __WXMSW__
        // On Windows, use wxGenericStaticBitmap for proper scaling
        control = new wxGenericStaticBitmap(
            m_parentAsWindow,
            GetID(),
            bitmap,
            GetPosition(),
            GetSize(),
            GetStyle(),
            GetName()
        );
#else
        // On other platforms, use native wxStaticBitmap
        control = new wxStaticBitmap(
            m_parentAsWindow,
            GetID(),
            bitmap,
            GetPosition(),
            GetSize(),
            GetStyle(),
            GetName()
        );
#endif

        // Handle scale mode if specified (works on both implementations)
        wxString scaleMode = GetParamValue(wxT("scalemode"));
        if (!scaleMode.IsEmpty()) {
            if (scaleMode == wxT("None") || scaleMode == wxT("0")) {
                control->SetScaleMode(wxStaticBitmap::Scale_None);
            } else if (scaleMode == wxT("Fill") || scaleMode == wxT("1")) {
                control->SetScaleMode(wxStaticBitmap::Scale_Fill);
            } else if (scaleMode == wxT("AspectFit") || scaleMode == wxT("2")) {
                control->SetScaleMode(wxStaticBitmap::Scale_AspectFit);
            } else if (scaleMode == wxT("AspectFill") || scaleMode == wxT("3")) {
                control->SetScaleMode(wxStaticBitmap::Scale_AspectFill);
            }
        }

        SetupWindow(control);

        return control;
    }

    virtual bool CanHandle(wxXmlNode *node)
    {
        return IsOfClass(node, wxT("wxStaticBitmap"));
    }
};

/**
 * @brief Registers the platform-aware StaticBitmap XRC handler
 * 
 * This function should be called after creating the XmlResource but before loading XRC files.
 * It replaces the default wxStaticBitmap handler with our platform-aware version.
 */
WXD_EXPORTED void wxd_XmlResource_InitPlatformAwareStaticBitmapHandler(wxd_XmlResource_t* resource) {
    wxXmlResource* res = reinterpret_cast<wxXmlResource*>(resource);
    if (!res) return;

    // Add our custom handler - it will take precedence over the default one
    res->AddHandler(new WxdPlatformAwareStaticBitmapHandler());
}

#else // WXD_USE_XRC

// Stub implementation when XRC is not enabled
WXD_EXPORTED void wxd_XmlResource_InitPlatformAwareStaticBitmapHandler(wxd_XmlResource_t* resource) {
    // Do nothing when XRC is not enabled
    (void)resource; // Suppress unused parameter warning
}

#endif // WXD_USE_XRC 