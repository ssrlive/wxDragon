#include "wxdragon.h"
#include <wx/wx.h>
#include <wx/notebook.h>

// Implementation for wxd_Notebook_Create
WXD_EXPORTED wxd_Notebook_t* wxd_Notebook_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style
) {
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    wxNotebook* notebook = new wxNotebook(
        parentWin,
        id,
        wxPoint(pos.x, pos.y),
        wxSize(size.width, size.height),
        style
    );
    // Attach cleanup notifier like other widgets if needed
    // wxd_Window_AttachCleanupNotifier(reinterpret_cast<wxd_Window_t*>(notebook));
    return reinterpret_cast<wxd_Notebook_t*>(notebook);
}

// Implementation for wxd_Notebook_AddPage
WXD_EXPORTED bool wxd_Notebook_AddPage(
    wxd_Notebook_t* self,
    wxd_Window_t* page,
    const char* text,
    bool select
) {
    wxNotebook* notebook = reinterpret_cast<wxNotebook*>(self);
    wxWindow* pageWin = reinterpret_cast<wxWindow*>(page);
    if (!notebook || !pageWin) return false;
    
    return notebook->AddPage(pageWin, wxString::FromUTF8(text ? text : ""), select);
}

// Implementation for wxd_Notebook_GetSelection
WXD_EXPORTED int wxd_Notebook_GetSelection(wxd_Notebook_t* self) {
    wxNotebook* notebook = reinterpret_cast<wxNotebook*>(self);
    if (!notebook) return wxNOT_FOUND; // wxNOT_FOUND is typically -1
    return notebook->GetSelection();
}

// Implementation for wxd_Notebook_SetSelection
WXD_EXPORTED int wxd_Notebook_SetSelection(wxd_Notebook_t* self, int page) {
    wxNotebook* notebook = reinterpret_cast<wxNotebook*>(self);
    if (!notebook) return wxNOT_FOUND; // Or return previous selection? Follow wxWidgets: returns old selection.
    return notebook->SetSelection(static_cast<size_t>(page)); // wx uses size_t
}

// Implementation for wxd_NotebookEvent_GetSelection
WXD_EXPORTED int wxd_NotebookEvent_GetSelection(wxd_Event_t* event) {
    if (!event) return wxNOT_FOUND;
    wxEvent* baseEvent = reinterpret_cast<wxEvent*>(event);
    wxBookCtrlEvent* notebookEvent = dynamic_cast<wxBookCtrlEvent*>(baseEvent); // wxNotebookEvent derives from wxBookCtrlEvent
    if (!notebookEvent) return wxNOT_FOUND; 

    return notebookEvent->GetSelection();
}

// Implementation for wxd_NotebookEvent_GetOldSelection
WXD_EXPORTED int wxd_NotebookEvent_GetOldSelection(wxd_Event_t* event) {
    if (!event) return wxNOT_FOUND;
    wxEvent* baseEvent = reinterpret_cast<wxEvent*>(event);
    wxBookCtrlEvent* notebookEvent = dynamic_cast<wxBookCtrlEvent*>(baseEvent);
    if (!notebookEvent) return wxNOT_FOUND;

    return notebookEvent->GetOldSelection();
}

// Destroy function is implicitly handled by wxWidgets hierarchy when parent is destroyed,
// or via manual Destroy() call if needed (e.g., wxd_Window_Destroy).
// Explicit wxd_Notebook_Destroy is likely not needed if it's always owned by a parent. 