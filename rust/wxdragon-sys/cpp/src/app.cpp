#include "../include/wxdragon.h"
#include <wx/wx.h>
#include <wx/app.h>
#include <wx/image.h>
#include <cstdlib>

// --- Globals --- 
// Store the C callback and user data provided to wxd_Main
static wxd_OnInitCallback g_OnInitCallback = nullptr;
static void* g_OnInitUserData = nullptr;

// Function to process Rust callbacks, implemented in Rust
extern "C" void process_rust_callbacks();

// --- Internal C++ App Class --- 

class WxdApp : public wxApp {
public:
    // Called by wxWidgets framework on application startup.
    virtual bool OnInit() override;

    // Idle event handler to process callbacks
    void OnIdle(wxIdleEvent& event);
    
    // Optional: Override OnExit for cleanup if needed
    // virtual int OnExit() override;
};

// Implementation of OnInit - this is where we call the C callback
bool WxdApp::OnInit() {
    // Call base class OnInit (important)
    if (!wxApp::OnInit()) {
        return false;
    }

#ifdef __WXMSW__
    // Use best available visual (important for proper rendering, especially with transparency)
    SetUseBestVisual(true);
#endif

    // Initialize all stock items (standard icons, etc.)
    wxInitializeStockLists();

    // Bind idle event to process callbacks
    Bind(wxEVT_IDLE, &WxdApp::OnIdle, this);

    // Call the stored C callback function
    if (g_OnInitCallback) {
        // The callback is responsible for creating the main window
        // and calling wxd_App_SetTopWindow.
        bool success = g_OnInitCallback(g_OnInitUserData);
        return success;
    } else {
        // Should not happen if wxd_Main is used correctly
        wxLogError("wxDragon: No OnInit callback provided to wxd_Main.");
        return false;
    }
}

// Process callbacks on idle
void WxdApp::OnIdle(wxIdleEvent& event) {
    // Process any pending Rust callbacks
    process_rust_callbacks();
    
    // Request more idle events if there are more callbacks
    // This ensures the event loop keeps processing our callbacks
    event.RequestMore();
}

// --- C API Implementation --- 

// This macro creates the necessary wxWidgets entry points (like main or WinMain)
// and instantiates our WxdApp class when wxEntry is called.
// It effectively hides the platform-specific entry point boilerplate.
// However, it means our C API user doesn't write main(), they write a function
// that calls wxd_Main(), and we need a way to trigger wxEntry.

// Let's use DECLARE/IMPLEMENT_APP_NO_MAIN. This requires us to provide
// the actual main() function or equivalent, allowing our wxd_Main to control
// the startup sequence.
wxDECLARE_APP(WxdApp);
wxIMPLEMENT_APP_NO_MAIN(WxdApp);

// Main entry point implementation
int wxd_Main(int argc, char** argv, wxd_OnInitCallback on_init_cb, void* userData) {
    if (!on_init_cb) {
        fprintf(stderr, "wxDragon Error: No OnInit callback provided to wxd_Main.\n");
        return 1;
    }

    g_OnInitCallback = on_init_cb;
    g_OnInitUserData = userData;

    if (!wxEntryStart(argc, argv)) {
        fprintf(stderr, "wxDragon Error: Failed to initialize wxWidgets (wxEntryStart failed).\n");
        return 1;
    }

    // Initialize all available image handlers (PNG, JPEG, etc.)
    // This must be done after wxEntryStart and before any image loading (e.g., in app OnInit).
    wxInitAllImageHandlers();

    // wxTheApp should now be a WxdApp instance.
    // CallOnInit will execute WxdApp::OnInit, which calls the Rust g_OnInitCallback.
    if (wxTheApp && wxTheApp->CallOnInit()) {
        // Rust initialization was successful (returned true)
        wxTheApp->OnRun(); // Start the main event loop
    } else {
        // wxApp initialization failed (CallOnInit returned false or wxTheApp was null)
        // Log this case, as WxdApp::OnInit (and thus Rust init) might have returned false.
        fprintf(stderr, "wxDragon Error: wxApp initialization failed or Rust OnInit callback returned false.\n");
        // wxEntryCleanup will be called below if wxEntryStart succeeded.
    }
    
    wxEntryCleanup();
    g_OnInitCallback = nullptr;
    g_OnInitUserData = nullptr;
    return 0; // Consider returning an error code if init failed
}

// Gets the handle to the global application instance.
wxd_App_t* wxd_GetApp() {
    // wxTheApp is the global pointer to the wxApp instance
    return reinterpret_cast<wxd_App_t*>(wxTheApp);
}

// Sets the top window (main frame) for the application.
void wxd_App_SetTopWindow(wxd_App_t* app, wxd_Window_t* window) {
    if (!app || !window) return;
    WxdApp* wx_app = reinterpret_cast<WxdApp*>(app);
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    wx_app->SetTopWindow(wx_window);
}

// Manual callback processing for cases where we need to trigger it
void wxd_App_ProcessCallbacks() {
    process_rust_callbacks();
}

// Implementation for wxd_free_string
void wxd_free_string(char* str) {
    if (str) {
        free(str); // free is from <cstdlib> or <stdlib.h>
    }
}

// Implementation for wxd_free_int_array
void wxd_free_int_array(int* ptr) {
    if (ptr) {
        free(ptr);
    }
}

#ifdef __cplusplus
extern "C" {
#endif

    // Dummy implementations to avoid linker errors in case Rust doesn't define them
    #pragma weak process_rust_callbacks
    void process_rust_callbacks() {
        // This is a weak symbol that will be replaced by the actual Rust implementation
        // If the Rust implementation is not available, this will be used instead
    }

#ifdef __cplusplus
}
#endif
