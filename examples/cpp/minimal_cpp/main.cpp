// XRC Test Application - Testing border styles on different platforms
#include <wx/wx.h>
#include <wx/xrc/xmlres.h>
#include <wx/panel.h>
#include <wx/button.h>
#include <wx/stattext.h>
#include <wx/sizer.h>

// Define a new application type
class XrcTestApp : public wxApp
{
public:
    virtual bool OnInit();
};

// Define a new frame type
class XrcTestFrame : public wxFrame
{
public:
    XrcTestFrame();

private:
    void OnTestSimple(wxCommandEvent& event);
    void OnTestTheme(wxCommandEvent& event);
    void OnExit(wxCommandEvent& event);
    
    wxPanel* m_currentPanel;
    
    wxDECLARE_EVENT_TABLE();
};

// Event IDs
enum
{
    ID_TEST_SIMPLE = 1000,
    ID_TEST_THEME = 1001
};

// Event table for XrcTestFrame
wxBEGIN_EVENT_TABLE(XrcTestFrame, wxFrame)
    EVT_MENU(ID_TEST_SIMPLE, XrcTestFrame::OnTestSimple)
    EVT_MENU(ID_TEST_THEME, XrcTestFrame::OnTestTheme)
    EVT_MENU(wxID_EXIT, XrcTestFrame::OnExit)
wxEND_EVENT_TABLE()

// Main program
wxIMPLEMENT_APP(XrcTestApp);

// XrcTestApp implementation
bool XrcTestApp::OnInit()
{
    if (!wxApp::OnInit())
        return false;

    // Initialize XRC system
    wxXmlResource::Get()->InitAllHandlers();
    
    // Load our XRC file
    if (!wxXmlResource::Get()->Load("test_panel.xrc"))
    {
        wxLogError("Failed to load XRC resource file 'test_panel.xrc'");
        return false;
    }

    XrcTestFrame *frame = new XrcTestFrame();
    frame->Show(true);
    return true;
}

// XrcTestFrame implementation
XrcTestFrame::XrcTestFrame()
    : wxFrame(NULL, wxID_ANY, "XRC Border Style Test", wxDefaultPosition, wxSize(600, 400)),
      m_currentPanel(nullptr)
{
    // Create menu
    wxMenu *testMenu = new wxMenu;
    testMenu->Append(ID_TEST_SIMPLE, "Test &Simple Border\tCtrl-S", "Test panel with wxBORDER_SIMPLE");
    testMenu->Append(ID_TEST_THEME, "Test &Theme Border\tCtrl-T", "Test panel with wxBORDER_THEME");
    testMenu->AppendSeparator();
    testMenu->Append(wxID_EXIT, "E&xit\tAlt-X", "Quit this program");
    
    wxMenuBar *menuBar = new wxMenuBar;
    menuBar->Append(testMenu, "&Test");
    SetMenuBar(menuBar);
    
    // Create a main sizer
    wxBoxSizer* mainSizer = new wxBoxSizer(wxVERTICAL);
    
    // Add instructions
    wxStaticText* instructions = new wxStaticText(this, wxID_ANY, 
        "Use the Test menu to load panels with different border styles.\n"
        "This will help us identify if the issue is platform-specific.");
    mainSizer->Add(instructions, 0, wxEXPAND | wxALL, 10);
    
    SetSizer(mainSizer);
    
    CreateStatusBar();
    SetStatusText("Ready - Use Test menu to load XRC panels");
}

void XrcTestFrame::OnTestSimple(wxCommandEvent& WXUNUSED(event))
{
    // Remove current panel if any
    if (m_currentPanel)
    {
        m_currentPanel->Destroy();
        m_currentPanel = nullptr;
    }
    
    // Try to load panel with wxBORDER_SIMPLE
    m_currentPanel = wxXmlResource::Get()->LoadPanel(this, "test_panel");
    
    if (m_currentPanel)
{
        // Add to sizer
        wxSizer* sizer = GetSizer();
        if (sizer)
        {
            sizer->Add(m_currentPanel, 1, wxEXPAND | wxALL, 5);
            Layout();
        }
        SetStatusText("SUCCESS: Loaded panel with wxBORDER_SIMPLE");
        
        // Log the actual style
        long style = m_currentPanel->GetWindowStyleFlag();
        wxLogMessage("Panel style flags: 0x%08lX", style);
    }
    else
    {
        SetStatusText("FAILED: Could not load panel with wxBORDER_SIMPLE");
        wxLogError("Failed to load test_panel from XRC");
    }
}

void XrcTestFrame::OnTestTheme(wxCommandEvent& WXUNUSED(event))
{
    // Remove current panel if any
    if (m_currentPanel)
    {
        m_currentPanel->Destroy();
        m_currentPanel = nullptr;
    }
    
    // Try to load panel with wxBORDER_THEME
    m_currentPanel = wxXmlResource::Get()->LoadPanel(this, "theme_panel");
    
    if (m_currentPanel)
    {
        // Add to sizer
        wxSizer* sizer = GetSizer();
        if (sizer)
        {
            sizer->Add(m_currentPanel, 1, wxEXPAND | wxALL, 5);
            Layout();
        }
        SetStatusText("SUCCESS: Loaded panel with wxBORDER_THEME");
        
        // Log the actual style
        long style = m_currentPanel->GetWindowStyleFlag();
        wxLogMessage("Panel style flags: 0x%08lX", style);
    }
    else
    {
        SetStatusText("FAILED: Could not load panel with wxBORDER_THEME");
        wxLogError("Failed to load theme_panel from XRC");
    }
}

void XrcTestFrame::OnExit(wxCommandEvent& WXUNUSED(event))
{
    Close(true);
} 