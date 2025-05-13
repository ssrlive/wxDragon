// Clean wxWidgets C++ example demonstrating wxAUI functionality
#include <wx/wx.h>
#include <wx/aui/aui.h>
#include <wx/textctrl.h>
#include <wx/button.h>
#include <wx/sizer.h>

// Define a new application type
class MyApp : public wxApp
{
public:
    virtual bool OnInit();
};

// Define a new frame type
class MyFrame : public wxFrame
{
public:
    MyFrame(const wxString& title);
    virtual ~MyFrame();

private:
    wxAuiManager m_mgr;
    
    void OnExit(wxCommandEvent& event);
    void OnSavePerspective(wxCommandEvent& event);
    void OnLoadPerspective(wxCommandEvent& event);
    
    wxString m_savedPerspective;
    
    wxDECLARE_EVENT_TABLE();
};

// Event table for MyFrame
wxBEGIN_EVENT_TABLE(MyFrame, wxFrame)
    EVT_MENU(wxID_EXIT, MyFrame::OnExit)
wxEND_EVENT_TABLE()

// Main program
wxIMPLEMENT_APP(MyApp);

// MyApp implementation
bool MyApp::OnInit()
{
    if (!wxApp::OnInit())
        return false;

    MyFrame *frame = new MyFrame("AUI Manager Demo");
    frame->Show(true);
    return true;
}

// MyFrame implementation
MyFrame::MyFrame(const wxString& title)
    : wxFrame(NULL, wxID_ANY, title, wxDefaultPosition, wxSize(800, 600))
{
    // Initialize the AUI manager
    m_mgr.SetManagedWindow(this);
    
    // Create menu
    wxMenu *fileMenu = new wxMenu;
    fileMenu->Append(wxID_EXIT, "E&xit\tAlt-X", "Quit this program");
    
    wxMenuBar *menuBar = new wxMenuBar;
    menuBar->Append(fileMenu, "&File");
    SetMenuBar(menuBar);
    
    // Create a panel with buttons for saving/loading perspective
    wxPanel* toolPanel = new wxPanel(this, wxID_ANY);
    wxBoxSizer* toolSizer = new wxBoxSizer(wxHORIZONTAL);
    
    wxButton* saveButton = new wxButton(toolPanel, wxID_ANY, "Save Perspective");
    wxButton* loadButton = new wxButton(toolPanel, wxID_ANY, "Load Perspective");
    
    toolSizer->Add(saveButton, 1, wxEXPAND | wxALL, 5);
    toolSizer->Add(loadButton, 1, wxEXPAND | wxALL, 5);
    
    toolPanel->SetSizer(toolSizer);
    
    // Connect event handlers for buttons
    saveButton->Bind(wxEVT_BUTTON, &MyFrame::OnSavePerspective, this);
    loadButton->Bind(wxEVT_BUTTON, &MyFrame::OnLoadPerspective, this);
    
    // Create some text controls for the panes
    wxTextCtrl* text1 = new wxTextCtrl(this, wxID_ANY, "Text Control 1", 
                                      wxDefaultPosition, wxSize(200, 150),
                                      wxTE_MULTILINE);
                                      
    wxTextCtrl* text2 = new wxTextCtrl(this, wxID_ANY, "Text Control 2", 
                                      wxDefaultPosition, wxSize(200, 150),
                                      wxTE_MULTILINE);
                                      
    wxTextCtrl* text3 = new wxTextCtrl(this, wxID_ANY, "Text Control 3", 
                                      wxDefaultPosition, wxSize(200, 150),
                                      wxTE_MULTILINE);
    
    // Add the panes to the manager with different directions
    m_mgr.AddPane(toolPanel, wxAuiPaneInfo()
                 .Name("toolbar")
                 .Caption("Toolbar")
                 .CaptionVisible(true)
                 .Top()
                 .ToolbarPane());
    
    m_mgr.AddPane(text1, wxAuiPaneInfo()
                 .Name("text1")
                 .Caption("Left Pane")
                 .CaptionVisible(true)
                 .Left()
                 .MinSize(wxSize(200, 200))
                 .BestSize(wxSize(300, 300))
                 .Floatable(true)
                 .Movable(true)
                 .CloseButton(true)
                 .MaximizeButton(true));
    
    m_mgr.AddPane(text2, wxAuiPaneInfo()
                 .Name("text2")
                 .Caption("Bottom Pane")
                 .CaptionVisible(true)
                 .Bottom()
                 .MinSize(wxSize(200, 200))
                 .BestSize(wxSize(300, 300))
                 .Floatable(true)
                 .Movable(true)
                 .CloseButton(true)
                 .MaximizeButton(true));
    
    m_mgr.AddPane(text3, wxAuiPaneInfo()
                 .Name("text3")
                 .Caption("Center Pane")
                 .CaptionVisible(true)
                 .CenterPane()
                 .MinSize(wxSize(200, 200))
                 .Floatable(true)
                 .Movable(true)
                 .CloseButton(true)
                 .MaximizeButton(true));
    
    // Commit all changes
    m_mgr.Update();
    
    CreateStatusBar();
    SetStatusText("Drag the caption bars to move panes around");
}

MyFrame::~MyFrame()
{
    // Deinitialize the AUI manager
    m_mgr.UnInit();
}

void MyFrame::OnExit(wxCommandEvent& WXUNUSED(event))
{
    Close(true);
}

void MyFrame::OnSavePerspective(wxCommandEvent& WXUNUSED(event))
{
    m_savedPerspective = m_mgr.SavePerspective();
    SetStatusText("Perspective saved");
}

void MyFrame::OnLoadPerspective(wxCommandEvent& WXUNUSED(event))
{
    if (!m_savedPerspective.IsEmpty())
    {
        m_mgr.LoadPerspective(m_savedPerspective);
        SetStatusText("Perspective loaded");
    }
    else
    {
        SetStatusText("No perspective to load");
    }
} 