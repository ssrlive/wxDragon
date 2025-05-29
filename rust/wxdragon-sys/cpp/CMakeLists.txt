cmake_minimum_required(VERSION 3.15)
project(wxDragon LANGUAGES CXX C)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# --- Output Directories (relative to CMAKE_BINARY_DIR which build.rs sets to $OUT_DIR/build) ---
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# --- Option for wxWidgets Source Directory (passed from build.rs) ---
option(WXWIDGETS_SOURCE_DIR "Path to the wxWidgets source tree" "")
if(NOT WXWIDGETS_SOURCE_DIR OR NOT EXISTS "${WXWIDGETS_SOURCE_DIR}/CMakeLists.txt")
    message(FATAL_ERROR "WXWIDGETS_SOURCE_DIR ('${WXWIDGETS_SOURCE_DIR}') is not set or is not a valid wxWidgets source directory. It should be passed from the Rust build script via -D.")
endif()
message(STATUS "Using wxWidgets source from: ${WXWIDGETS_SOURCE_DIR}")

# --- Configure wxWidgets Build Options (applied to the downloaded source) ---
message(STATUS "Configuring wxWidgets build options...")
set(wxBUILD_SHARED OFF)
set(wxBUILD_SAMPLES OFF)
set(wxBUILD_TESTS OFF)
set(wxBUILD_DEMOS OFF)
set(wxBUILD_BENCHMARKS OFF)
set(wxUSE_EXCEPTIONS ON)
set(wxUSE_GUI ON)
set(wxUSE_STC ON)
set(wxUSE_OPENGL ON)
set(wxUSE_HTML ON)
option(wxdUSE_WEBVIEW "Use the Webview widget" OFF)
if (wxdUSE_WEBVIEW)
    set(wxUSE_WEBVIEW ON)
endif()
option(wxdUSE_MEDIACTRL "Use Media Ctrl widgets" OFF)
if (wxdUSE_MEDIACTRL)
    set(wxUSE_MEDIACTRL ON) 
endif()

# Platform-specific options
if(CMAKE_SYSTEM_NAME MATCHES "Darwin")
    message(STATUS "Setting macOS specific wxWidgets options")
    set(CMAKE_OSX_DEPLOYMENT_TARGET "10.13" CACHE STRING "Minimum macOS version")
    set(wxBUILD_TOOLKIT "osx_cocoa" CACHE STRING "wxWidgets Toolkit")
elseif(CMAKE_SYSTEM_NAME MATCHES "Windows")
    message(STATUS "Setting Windows specific wxWidgets options")
    set(wxBUILD_TOOLKIT "msw" CACHE STRING "wxWidgets Toolkit")
    # For Windows, you might need to set compiler/linker flags for static runtime if wxWidgets needs it
    # set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} /MT")
    # set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} /MTd")
elseif(CMAKE_SYSTEM_NAME MATCHES "Linux")
    message(STATUS "Setting Linux specific wxWidgets options")
    set(wxBUILD_TOOLKIT "gtk3" CACHE STRING "wxWidgets Toolkit")
else()
    message(WARNING "Unsupported OS for platform-specific wxWidgets options, using defaults.")
endif()

message(STATUS "Adding wxWidgets subdirectory from ${WXWIDGETS_SOURCE_DIR}")
# Add the downloaded wxWidgets source directory. This will configure and build wxWidgets.
# The second argument defines the build directory for wxWidgets *within* CMAKE_BINARY_DIR.
add_subdirectory(${WXWIDGETS_SOURCE_DIR} ${CMAKE_CURRENT_BINARY_DIR}/wxwidgets_build EXCLUDE_FROM_ALL)

# --- Define wxDragon Library Target ---
message(STATUS "Defining wxdragon static library target")
# Source files are relative to this CMakeLists.txt (rust/wxdragon-sys/cpp)
set(WXDRAGON_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/src/core/timer.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/activity_indicator.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/animation_ctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/app.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/array_string.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/art_provider.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/aui_manager.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/aui_mdi_child_frame.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/aui_mdi_parent_frame.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/aui_notebook.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/aui_toolbar.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/bitmap.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/bitmap_button.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/bitmapbundle.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/bitmapbutton.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/bitmapcombobox.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/button.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/calendar_ctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/checkbox.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/checklistbox.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/choice.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/clipboard.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/colourdialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/colourpickerctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/combobox.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/commandlinkbutton.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dataobject.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dataview.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dataviewtreectrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dataviewvirtuallistmodel.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dataviewvirtuallistmodel_custom.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/datepickerctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/datetime.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dc.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dir_dialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dirpickerctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dnd.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/droptarget.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/editablelistbox.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/event.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/file_ctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/file_dialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/filepickerctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/font_dialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/fontpickerctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/frame.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/gauge.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/hyperlink_ctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/imagelist.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/item.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/list_ctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/listbox.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/menu.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/message_dialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/multi_choice_dialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/notebook.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/notificationmessage.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/panel.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/progressdialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/radio_button.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/radiobox.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/rearrangelist.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/scrollbar.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/scrolled_window.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/search_ctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/single_choice_dialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/sizer.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/slider.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/spinbutton.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/spinctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/spinctrldouble.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/splitterwindow.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/static_bitmap.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/static_line.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/static_text.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/staticbox.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/statusbar.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/text_entry_dialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/textctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/timepickerctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/togglebutton.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/toolbar.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/treebook.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/treectrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/window.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/wxd_utils.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/wxd_utils.h
    ${CMAKE_CURRENT_SOURCE_DIR}/src/xrc.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/collapsiblepane.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/styledtextctrl.cpp
)

if (wxdUSE_MEDIACTRL)
    list(APPEND ${WXDRAGON_SOURCES} ${CMAKE_CURRENT_SOURCE_DIR}/src/media_ctrl.cpp)
endif()
if (wxdUSE_WEBVIEW)
    # similarly append webview wrapper
endif()

if(NOT WXDRAGON_SOURCES)
    message(FATAL_ERROR "No source files found in ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")
endif()
message(STATUS "wxDragon Sources (found via GLOB_RECURSE): ${WXDRAGON_SOURCES}")

# Define the library using the discovered sources
add_library(wxdragon STATIC ${WXDRAGON_SOURCES})

# Set required include directories for wxdragon
# - Its own headers (rust/wxdragon-sys/cpp/include)
# - wxWidgets headers (will be added by target_link_libraries with wxWidgets targets)
target_include_directories(wxdragon PRIVATE 
    ${CMAKE_CURRENT_SOURCE_DIR}/include # For wxdragon.h
)

# Link wxdragon against the wxWidgets targets built by add_subdirectory above.
# These targets (e.g., wx::core, wx::base) should provide necessary include paths and link libraries.
# The exact target names might vary slightly based on wxWidgets version/config,
# but wxcore, wxbase, etc. are common for wxWidgets 3.1+ CMake builds.
# For wxWidgets 3.2.x, the targets are typically not namespaced with wx:: but are direct names.
target_link_libraries(wxdragon PRIVATE 
    wxcore 
    wxbase
    wxadv     # For controls like wxDatePickerCtrl, wxCalendarCtrl
    wxaui     # If you use wxAuiNotebook, wxAuiManager
    wxmedia   # If you use wxMediaCtrl
    wxpropgrid # If you use wxPropertyGrid
    wxgl
    wxstc
    wxhtml
    wxwebview
    wxxml
    wxxrc
    wxexpat   # For XML parsing functionality
)

# If on MSVC, and wxWidgets was built with a different CRT, you might need to adjust
# For example, if wxWidgets is /MD and you want wxdragon to be /MT, that can cause issues.
# Best to ensure wxWidgets and wxdragon use compatible runtimes.

message(STATUS "wxdragon target defined and configured.")

message(STATUS "CMake configuration for libwxdragon finished.")
 