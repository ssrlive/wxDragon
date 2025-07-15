#define wxUSE_GUI 1
#define wxUSE_TASKBARICON 1

#include "wx/wxprec.h"

#ifndef WX_PRECOMP
#include "wx/wx.h"
#endif

#include <wx/artprov.h>
#include <wx/taskbar.h>

class MyTaskBarIcon : public wxTaskBarIcon {
private:
    bool check = true;

public:
#if defined(__WXOSX__) && wxOSX_USE_COCOA
    MyTaskBarIcon(wxTaskBarIconType iconType = wxTBI_DEFAULT_TYPE) : wxTaskBarIcon(iconType) {}
#else
    MyTaskBarIcon() {}
#endif

    void OnLeftButtonDClick(wxTaskBarIconEvent&);
    void OnMenuRestore(wxCommandEvent&);
    void OnMenuExit(wxCommandEvent&);
    void OnMenuSetNewIcon(wxCommandEvent&);
    void OnMenuCheckmark(wxCommandEvent&);
    void OnMenuUICheckmark(wxUpdateUIEvent&);
    void OnMenuSub(wxCommandEvent&);
    virtual wxMenu* CreatePopupMenu() override;

    wxDECLARE_EVENT_TABLE();
};

class MyApp : public wxApp {
public:
    virtual bool OnInit() override;
};

class MyFrame : public wxFrame {
public:
    MyFrame(const wxString& title);
    virtual ~MyFrame();

protected:
    void OnBtnAbout(wxCommandEvent& event);
    void OnBtnHide(wxCommandEvent& event);
    void OnBtnExit(wxCommandEvent& event);
    void OnEvtClose(wxCloseEvent& event);

    MyTaskBarIcon* m_taskBarIcon;
#if defined(__WXOSX__) && wxOSX_USE_COCOA
    MyTaskBarIcon* m_dockIcon;
#endif

    wxDECLARE_EVENT_TABLE();
};

// ============================================================================
// implementation
// ============================================================================

static MyFrame* gs_frame = nullptr;

wxIMPLEMENT_APP(MyApp);

bool MyApp::OnInit() {
    if (!wxApp::OnInit()) {
        return false;
    }

    if (!wxTaskBarIcon::IsAvailable()) {
        wxMessageBox("There appears to be no system tray support in your current environment."
            "This sample may not behave as expected.",
            "Warning", wxOK | wxICON_EXCLAMATION);
    }

    // Create the main window
    gs_frame = new MyFrame("wxTaskBarIcon Test Dialog");

    gs_frame->Show(true);

    return true;
}

wxBEGIN_EVENT_TABLE(MyFrame, wxFrame)
EVT_BUTTON(wxID_ABOUT, MyFrame::OnBtnAbout)
EVT_BUTTON(wxID_OK, MyFrame::OnBtnHide)
EVT_BUTTON(wxID_EXIT, MyFrame::OnBtnExit)
EVT_CLOSE(MyFrame::OnEvtClose)
wxEND_EVENT_TABLE();

MyFrame::MyFrame(const wxString& title) : wxFrame(nullptr, wxID_ANY, title) {
    wxSizer* const sizerTop = new wxBoxSizer(wxVERTICAL);

    wxSizerFlags flags;
    flags.DoubleBorder(wxALL);

    const char* const info1 = "Press 'Hide me' to hide this window, Exit to quit.";
    sizerTop->Add(new wxStaticText(this, wxID_ANY, info1), flags);

    const char* const info2 = "Double-click on the taskbar icon to show me again.";
    sizerTop->Add(new wxStaticText(this, wxID_ANY, info2), flags);

    sizerTop->AddStretchSpacer()->SetMinSize(200, 50);

    wxSizer* const sizerBtns = new wxBoxSizer(wxHORIZONTAL);
    sizerBtns->Add(new wxButton(this, wxID_ABOUT, "&About"), flags);
    sizerBtns->Add(new wxButton(this, wxID_OK, "&Hide"), flags);
    sizerBtns->Add(new wxButton(this, wxID_EXIT, "E&xit"), flags);

    sizerTop->Add(sizerBtns, flags.Align(wxALIGN_CENTER_HORIZONTAL));
    SetSizerAndFit(sizerTop);
    Centre();

    m_taskBarIcon = new MyTaskBarIcon();

    // we should be able to show up to 128 characters on Windows
    const wxString tooltip =
        "wxTaskBarIcon Sample\n"
        "With a very, very, very, very\n"
        "long tooltip whose length is\n"
        "greater than 64 characters.";
    auto icon = wxArtProvider::GetBitmapBundle(wxART_WX_LOGO, wxART_OTHER, wxSize(32, 32));
    if (!m_taskBarIcon->SetIcon(icon, tooltip)) {
        wxLogError("Could not set icon.");
    }

#if defined(__WXOSX__) && wxOSX_USE_COCOA
    m_dockIcon = new MyTaskBarIcon(wxTBI_DOCK);
    if (!m_dockIcon->SetIcon(wxArtProvider::GetBitmapBundle(
        wxART_WX_LOGO, wxART_OTHER, wxSize(32, 32)))) {
        wxLogError("Could not set icon.");
    }
#endif
}

MyFrame::~MyFrame() { delete m_taskBarIcon; }

void MyFrame::OnBtnAbout(wxCommandEvent& WXUNUSED(event)) {
    static const char* const title = "About wxWidgets Taskbar Sample";
    static const char* const message = "wxWidgets sample showing wxTaskBarIcon class";

#if defined(__WXMSW__) && wxUSE_TASKBARICON_BALLOONS
    auto icon = wxBitmapBundle::FromSVGFile("info.svg", wxSize(64, 64));
    m_taskBarIcon->ShowBalloon(title, message, 15000, wxICON_INFORMATION, icon);
#else  // !__WXMSW__
    wxMessageBox(message, title, wxICON_INFORMATION | wxOK, this);
#endif // __WXMSW__/!__WXMSW__
}

void MyFrame::OnBtnHide(wxCommandEvent& WXUNUSED(event)) { Show(false); }
void MyFrame::OnBtnExit(wxCommandEvent& WXUNUSED(event)) { Close(); }
void MyFrame::OnEvtClose(wxCloseEvent& event) {
    if (event.CanVeto()) {
        int answer = wxMessageBox(
            _("Are you sure you want to close the window?"),
            _("Confirm Exit"),
            wxYES_NO | wxCANCEL | wxICON_QUESTION, this);
        if (answer == wxNO || answer == wxCANCEL) {
            event.Veto();
            return;
        }
    }
    event.Skip(); // Destroy();
}

enum {
    PU_RESTORE = 10001,
    PU_NEW_ICON,
    PU_EXIT,
    PU_CHECKMARK,
    PU_SUB1,
    PU_SUB2,
    PU_SUBMAIN
};

wxBEGIN_EVENT_TABLE(MyTaskBarIcon, wxTaskBarIcon)
EVT_MENU(PU_RESTORE, MyTaskBarIcon::OnMenuRestore)
EVT_MENU(PU_EXIT, MyTaskBarIcon::OnMenuExit)
EVT_MENU(PU_NEW_ICON, MyTaskBarIcon::OnMenuSetNewIcon)
EVT_MENU(PU_CHECKMARK, MyTaskBarIcon::OnMenuCheckmark)
EVT_UPDATE_UI(PU_CHECKMARK, MyTaskBarIcon::OnMenuUICheckmark)
EVT_TASKBAR_LEFT_DCLICK(MyTaskBarIcon::OnLeftButtonDClick)
EVT_MENU(PU_SUB1, MyTaskBarIcon::OnMenuSub)
EVT_MENU(PU_SUB2, MyTaskBarIcon::OnMenuSub)
wxEND_EVENT_TABLE();

void MyTaskBarIcon::OnMenuRestore(wxCommandEvent&) { gs_frame->Show(true); }

void MyTaskBarIcon::OnMenuExit(wxCommandEvent&) { gs_frame->Close(); }

void MyTaskBarIcon::OnMenuCheckmark(wxCommandEvent&) { check = !check; }

void MyTaskBarIcon::OnMenuUICheckmark(wxUpdateUIEvent& event) { event.Check(check); }

void MyTaskBarIcon::OnMenuSetNewIcon(wxCommandEvent&) {
    wxBitmap icon = wxArtProvider::GetBitmap(wxART_WARNING, wxART_MENU, wxSize(16, 16));
    if (!SetIcon(icon)) {
        wxMessageBox("Could not set new icon.");
    }
}

void MyTaskBarIcon::OnMenuSub(wxCommandEvent&) {
    wxMessageBox("You clicked on a submenu!");
}

// Overridables
wxMenu* MyTaskBarIcon::CreatePopupMenu() {
    wxMenu* menu = new wxMenu;
    menu->Append(PU_RESTORE, "&Restore main window");
    menu->AppendSeparator();
    menu->Append(PU_NEW_ICON, "&Set New Icon");
    menu->AppendSeparator();
    menu->AppendCheckItem(PU_CHECKMARK, "Test &check mark");
    menu->AppendSeparator();
    wxMenu* submenu = new wxMenu;
    submenu->Append(PU_SUB1, "One submenu");
    submenu->AppendSeparator();
    submenu->Append(PU_SUB2, "Another submenu");
    menu->Append(PU_SUBMAIN, "Submenu", submenu);
    /* OSX has built-in quit menu for the dock menu, but not for the status item */
#ifdef __WXOSX__
    if (OSXIsStatusItem())
#endif
    {
        menu->AppendSeparator();
        menu->Append(PU_EXIT, "E&xit");
    }
    return menu;
}

void MyTaskBarIcon::OnLeftButtonDClick(wxTaskBarIconEvent&) {
    gs_frame->Show(true);
}
