// Minimal wxWidgets C++ example to test wxToolBar visibility and wxBitmapButton events
#include <wx/wx.h>
#include <wx/toolbar.h>
#include <wx/artprov.h>
#include <wx/bmpbuttn.h> // Include header for wxBitmapButton
#include <wx/sizer.h>    // Include header for wxBoxSizer
#include <wx/notebook.h>   // For wxNotebook
#include <wx/treebook.h>   // For wxTreebook
#include <wx/panel.h>      // For pages
#include <wx/stattext.h>   // For content in pages
#include <wx/button.h>     // For content in pages

// IDs for the controls and menu items
enum
{
    ID_Quit = wxID_EXIT,
    ID_About,
    // Removed toolbar/bitmap button specific IDs for this demo
};

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

private:
    void OnQuit(wxCommandEvent& event);
    void OnAbout(wxCommandEvent& event);
    // Removed OnBitmapButtonClick as the button is removed for this demo
    wxDECLARE_EVENT_TABLE();
};

// Event table for MyFrame
wxBEGIN_EVENT_TABLE(MyFrame, wxFrame)
    EVT_MENU(ID_Quit, MyFrame::OnQuit)
    EVT_MENU(ID_About, MyFrame::OnAbout)
    // Removed EVT_BUTTON for the bitmap button
wxEND_EVENT_TABLE()

// `Main program` equivalent
wxIMPLEMENT_APP(MyApp);

// MyApp implementation
bool MyApp::OnInit()
{
    if (!wxApp::OnInit())
        return false;

    MyFrame *frame = new MyFrame("C++ Notebook + Treebook Test");
    frame->Show(true);
    return true;
}

// MyFrame implementation
MyFrame::MyFrame(const wxString& title)
    : wxFrame(NULL, wxID_ANY, title, wxDefaultPosition, wxSize(600, 550)) // Adjusted size
{
    // --- Menu ---
    wxMenu *fileMenu = new wxMenu;
    fileMenu->Append(ID_Quit, "E&xit\tAlt-X", "Quit this program");
    wxMenu *helpMenu = new wxMenu;
    helpMenu->Append(ID_About, "&About\tF1", "Show about dialog");
    wxMenuBar *menuBar = new wxMenuBar;
    menuBar->Append(fileMenu, "&File");
    menuBar->Append(helpMenu, "&Help");
    SetMenuBar(menuBar);

    // --- Main Sizer for Frame (to hold the notebook) ---
    wxBoxSizer* frameSizer = new wxBoxSizer(wxVERTICAL);

    // --- Notebook ---
    wxNotebook* notebook = new wxNotebook(this, wxID_ANY);

    // --- Panel for the Treebook (this will be a page in the Notebook) ---
    wxPanel* treebookContainerPanel = new wxPanel(notebook, wxID_ANY);
    // Set a distinct background color for diagnostic purposes
    treebookContainerPanel->SetBackgroundColour(wxColour(0, 255, 255)); // Cyan

    // Sizer for the treebookContainerPanel (to hold the Treebook)
    wxBoxSizer* treebookPageSizer = new wxBoxSizer(wxVERTICAL);

    // --- Treebook ---
    wxTreebook* treebook = new wxTreebook(treebookContainerPanel, wxID_ANY, 
                                         wxDefaultPosition, wxDefaultSize, // Let sizer manage initial size
                                         wxBK_DEFAULT);
    // treebook->SetMinSize(wxSize(250, 200)); // Optionally set a minimum size
    // treebook->SetInitialSize(wxSize(300,250)); // Or an initial size

    // Page 1 for Treebook
    wxPanel* tbPage1 = new wxPanel(treebook, wxID_ANY);
    wxStaticText* tbPage1Text = new wxStaticText(tbPage1, wxID_ANY, "This is Treebook Page 1.");
    wxButton* tbPage1Button = new wxButton(tbPage1, wxID_ANY, "Button on TB1");
    wxBoxSizer* tbPage1Sizer = new wxBoxSizer(wxVERTICAL);
    tbPage1Sizer->Add(tbPage1Text, 0, wxALL | wxEXPAND, 10);
    tbPage1Sizer->Add(tbPage1Button, 0, wxALL | wxALIGN_CENTER_HORIZONTAL, 5);
    tbPage1->SetSizerAndFit(tbPage1Sizer);
    treebook->AddPage(tbPage1, "Treebook Page 1", true);

    // Page 2 for Treebook
    wxPanel* tbPage2 = new wxPanel(treebook, wxID_ANY);
    wxStaticText* tbPage2Text = new wxStaticText(tbPage2, wxID_ANY, "Content for Treebook Page 2.");
    wxBoxSizer* tbPage2Sizer = new wxBoxSizer(wxVERTICAL);
    tbPage2Sizer->Add(tbPage2Text, 0, wxALL | wxEXPAND, 10);
    tbPage2->SetSizerAndFit(tbPage2Sizer);
    treebook->AddPage(tbPage2, "Treebook Page 2");

    // Sub-page for Page 2
    wxPanel* tbSubPage2 = new wxPanel(treebook, wxID_ANY);
    wxStaticText* tbSubPage2Text = new wxStaticText(tbSubPage2, wxID_ANY, "This is a Sub-Page of Page 2.");
    wxBoxSizer* tbSubPage2Sizer = new wxBoxSizer(wxVERTICAL);
    tbSubPage2Sizer->Add(tbSubPage2Text, 0, wxALL | wxEXPAND, 10);
    tbSubPage2->SetSizerAndFit(tbSubPage2Sizer);
    treebook->AddSubPage(tbSubPage2, "Sub-Page 2.1");
    
    // Add Treebook to its container panel's sizer
    treebookPageSizer->Add(treebook, 1, wxEXPAND | wxALL, 5); // Treebook expands to fill its container
    treebookContainerPanel->SetSizerAndFit(treebookPageSizer);

    // Add the treebook container panel to the notebook
    notebook->AddPage(treebookContainerPanel, "Treebook Demo", true);

    // --- Another simple page for the Notebook (for comparison) ---
    wxPanel* simplePage = new wxPanel(notebook, wxID_ANY);
    new wxStaticText(simplePage, wxID_ANY, "This is another notebook tab.", wxPoint(10,10));
    notebook->AddPage(simplePage, "Other Tab");

    // Add Notebook to the frame sizer
    frameSizer->Add(notebook, 1, wxEXPAND | wxALL, 0); // Notebook expands to fill frame
    SetSizerAndFit(frameSizer);

    CreateStatusBar(1);
    SetStatusText("C++ Treebook Test Ready");
}

void MyFrame::OnQuit(wxCommandEvent& WXUNUSED(event))
{
    Close(true);
}

void MyFrame::OnAbout(wxCommandEvent& WXUNUSED(event))
{
    wxMessageBox("This is a wxWidgets Treebook Test in C++ (minimal_cpp example)",
                 "About C++ Treebook Test", wxOK | wxICON_INFORMATION, this);
} 