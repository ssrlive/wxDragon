// Minimal TaskBarIcon Test - Just the essential code to create a visible icon
#include <wx/wx.h>
#include <wx/taskbar.h>
#include <wx/artprov.h>

class MinimalApp : public wxApp
{
public:
    virtual bool OnInit();
};

class MinimalTaskBarIcon : public wxTaskBarIcon
{
public:
    MinimalTaskBarIcon() : wxTaskBarIcon(wxTBI_CUSTOM_STATUSITEM) {}
    
    virtual wxMenu* CreatePopupMenu() override
    {
        wxMenu* menu = new wxMenu;
        menu->Append(wxID_EXIT, "Exit", "Exit the application");
        return menu;
    }
};

wxIMPLEMENT_APP(MinimalApp);

bool MinimalApp::OnInit()
{
    if (!wxApp::OnInit())
        return false;

    // Create TaskBarIcon
    wxLogMessage("Creating TaskBarIcon with CustomStatusItem...");
    MinimalTaskBarIcon* taskbar = new MinimalTaskBarIcon();
    
    // Get system icon
    wxLogMessage("Getting system warning icon...");
    wxBitmap icon = wxArtProvider::GetBitmap(wxART_WARNING, wxART_MENU, wxSize(16, 16));
    
    if (!icon.IsOk()) {
        wxLogError("Failed to create system icon!");
        return false;
    }

    wxLogMessage("Icon bitmap: %dx%d, valid=%s", icon.GetWidth(), icon.GetHeight(), icon.IsOk() ? "true" : "false");
    
    // Set icon
    wxLogMessage("Setting icon...");
    bool result = taskbar->SetIcon(icon, "Minimal TaskBarIcon Test");
    wxLogMessage("SetIcon result: %s", result ? "true" : "false");
    wxLogMessage("IsIconInstalled: %s", taskbar->IsIconInstalled() ? "true" : "false");
    
    if (result && taskbar->IsIconInstalled()) {
        wxLogMessage("SUCCESS: TaskBarIcon should be visible in menu bar!");
    } else {
        wxLogError("FAILED: TaskBarIcon not working");
    }
    
    // Keep the app running (no main window needed)
    return true;
} 