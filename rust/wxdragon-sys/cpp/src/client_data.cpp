#include <wx/clntdata.h>
#include <wx/window.h>
#include "../include/wxdragon.h" // Include the FFI header

// Forward declaration of the Rust drop function defined in Rust
// It takes ownership of the pointer and drops the underlying Box<RefCell<dyn Any>>.
extern "C" void drop_rust_refcell_box(void* ptr);

// Forward declaration of the Rust cleanup function defined in Rust.
// It takes the original window pointer to identify which data to clean up.
extern "C" void notify_rust_of_cleanup(wxd_Window_t* win_ptr);

// Custom client data class to hold the raw Rust pointer.
class WxdRustClientData : public wxClientData {
public:
    void* m_rust_data_ptr;

    // Constructor stores the pointer.
    WxdRustClientData(void* rust_ptr) : m_rust_data_ptr(rust_ptr) {}

    // Destructor is called automatically by wxWidgets when the window is destroyed.
    virtual ~WxdRustClientData() {
        // If the pointer is valid, call the Rust function to drop the data.
        if (m_rust_data_ptr) {
            drop_rust_refcell_box(m_rust_data_ptr);
            m_rust_data_ptr = nullptr; // Prevent potential double-free if destructor were called again somehow.
        }
    }
};

// Custom client data class acting solely as a cleanup notifier.
class WxdCleanupNotifier : public wxClientData {
private:
    // Store the original window pointer to pass back to Rust.
    wxd_Window_t* m_window_ptr;

public:
    // Constructor stores the window pointer.
    WxdCleanupNotifier(wxd_Window_t* win_ptr) : m_window_ptr(win_ptr) {}

    // Destructor is called automatically by wxWidgets when the window is destroyed.
    virtual ~WxdCleanupNotifier() {
        // If the pointer is valid, call the Rust function to notify it.
        if (m_window_ptr) {
            notify_rust_of_cleanup(m_window_ptr);
        }
        // No Rust data pointer here, nothing else to clean up in C++.
    }

    // Allow detaching the notification (prevent callback)
    void Detach() {
        m_window_ptr = nullptr;
    }
};

// --- FFI Function Implementations ---

// Associates the Rust data pointer with the window.
// Takes ownership of the Rust data (pointer becomes owned by WxdRustClientData).
// Deletes any previously associated client data/object.
extern "C" WXDRAGON_API void wxd_Window_SetRustClientData(wxd_Window_t* win_ptr, void* ptr) {
    wxWindow* win = reinterpret_cast<wxWindow*>(win_ptr); // Cast to wxWindow*
    if (!win) return;
    // SetClientObject takes ownership of the new WxdRustClientData instance.
    // wxWidgets will delete it later, triggering the destructor.
    win->SetClientObject(new WxdRustClientData(ptr));
}

// Retrieves the associated Rust data pointer without taking ownership.
// Returns nullptr if no data is set or if it's not WxdRustClientData.
extern "C" WXDRAGON_API void* wxd_Window_GetRustClientData(wxd_Window_t* win_ptr) {
    wxWindow* win = reinterpret_cast<wxWindow*>(win_ptr); // Cast to wxWindow*
    if (!win) return nullptr;
    // Get the client object associated with the window.
    wxClientData* client_data = win->GetClientObject();
    // Check if it's our specific type using dynamic_cast.
    WxdRustClientData* rust_data = dynamic_cast<WxdRustClientData*>(client_data);
    // Return the stored pointer if the cast was successful, otherwise nullptr.
    return rust_data ? rust_data->m_rust_data_ptr : nullptr;
}

// Takes ownership of the associated Rust data pointer from the window.
// Removes the association from the window.
// Returns the pointer to Rust, which is now responsible for dropping it.
// Returns nullptr if no data is set or if it's not WxdRustClientData.
extern "C" WXDRAGON_API void* wxd_Window_TakeRustClientData(wxd_Window_t* win_ptr) {
     wxWindow* win = reinterpret_cast<wxWindow*>(win_ptr); // Cast to wxWindow*
     if (!win) return nullptr;
    // Get the client object.
    wxClientData* client_data = win->GetClientObject();
    // Check if it's our type.
    WxdRustClientData* rust_data = dynamic_cast<WxdRustClientData*>(client_data);
    if (!rust_data) return nullptr;

    // Retrieve the pointer *before* detaching/deleting the client data object.
    void* rust_ptr = rust_data->m_rust_data_ptr;
    // Set the internal pointer to null to prevent the Rust drop function
    // from being called when WxdRustClientData is deleted by SetClientObject(nullptr).
    rust_data->m_rust_data_ptr = nullptr;

    // Detach the client object from the window.
    // SetClientObject(nullptr) deletes the *current* client object (our rust_data instance).
    win->SetClientObject(nullptr);

    // Return the original pointer to Rust.
    return rust_ptr;
}

// Attaches the cleanup notifier to the window.
// Deletes any previously associated client data/object.
extern "C" WXDRAGON_API void wxd_Window_AttachCleanupNotifier(wxd_Window_t* win_ptr) {
    wxWindow* win = reinterpret_cast<wxWindow*>(win_ptr);
    if (!win) return;
    // SetClientObject takes ownership of the new WxdCleanupNotifier instance.
    // If a notifier already exists, it will be deleted first (triggering its destructor,
    // potentially leading to a redundant cleanup call if not handled carefully in Rust map logic).
    win->SetClientObject(new WxdCleanupNotifier(win_ptr));
}

// Detaches the cleanup notifier from the window *without* triggering the notification.
extern "C" WXDRAGON_API void wxd_Window_DetachCleanupNotifier(wxd_Window_t* win_ptr) {
    wxWindow* win = reinterpret_cast<wxWindow*>(win_ptr);
    if (!win) return;

    // Get the current client object.
    wxClientData* client_data = win->GetClientObject();
    // Check if it's our notifier type.
    WxdCleanupNotifier* notifier = dynamic_cast<WxdCleanupNotifier*>(client_data);

    if (notifier) {
        // Prevent the destructor from calling the Rust notification function.
        notifier->Detach();
        // Detach the client object. SetClientObject(nullptr) deletes the current object.
        win->SetClientObject(nullptr);
    } else {
        // If it's not our notifier (or null), just ensure nothing is attached.
        // This handles cases where SetClientObject(nullptr) might already have been called,
        // or if some other client data was attached.
        if (client_data) {
           win->SetClientObject(nullptr);
        }
    }
} 