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
#include <wx/notifmsg.h>   // Include for wxNotificationMessage

// IDs for the controls and menu items
enum
{
    ID_Quit = wxID_EXIT,
    ID_About,
    ID_ShowNotification // New ID for our button
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
    void OnShowNotification(wxCommandEvent& event); // Declare new handler
    wxDECLARE_EVENT_TABLE();
};

// Event table for MyFrame
wxBEGIN_EVENT_TABLE(MyFrame, wxFrame)
    EVT_MENU(ID_Quit, MyFrame::OnQuit)
    EVT_MENU(ID_About, MyFrame::OnAbout)
    EVT_BUTTON(ID_ShowNotification, MyFrame::OnShowNotification) // Add button event
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

    // --- Main Sizer for Frame (to hold the button and notebook) ---
    wxBoxSizer* frameSizer = new wxBoxSizer(wxVERTICAL);

    // --- Button to Show Notification ---
    wxButton* notificationButton = new wxButton(this, ID_ShowNotification, "Show Notification");
    frameSizer->Add(notificationButton, 0, wxALL | wxALIGN_CENTER_HORIZONTAL, 10);

    CreateStatusBar(1);
    SetStatusText("C++ Notification Test Ready");
}

void MyFrame::OnQuit(wxCommandEvent& WXUNUSED(event))
{
    Close(true);
}

void MyFrame::OnAbout(wxCommandEvent& WXUNUSED(event))
{
    wxMessageBox("This is a wxWidgets Notification Test in C++",
                 "About C++ Notification Test", wxOK | wxICON_INFORMATION, this);
}

void MyFrame::OnShowNotification(wxCommandEvent& WXUNUSED(event))
{
    wxNotificationMessage notif;
    notif.SetTitle("C++ Test Notification");
    notif.SetMessage("This notification is from the C++ example.");
    notif.SetFlags(wxICON_INFORMATION);
    // Optionally set parent to this frame
    notif.SetParent(this);

    // Add actions (optional)
    if (notif.AddAction(101, "Action One C++")) {
        // Action added
    }
    if (notif.AddAction(102, "Action Two C++")) {
        // Action added
    }

    // Show the notification
    if (!notif.Show(wxNotificationMessage::Timeout_Never)) // Use Timeout_Never
    {
        wxLogError("Failed to show notification message from C++ example.");
    }
} 