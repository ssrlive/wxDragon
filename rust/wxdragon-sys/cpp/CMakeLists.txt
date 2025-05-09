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
set(wxBUILD_SHARED OFF CACHE BOOL "Build static libraries")
set(wxBUILD_SAMPLES OFF CACHE BOOL "")
set(wxBUILD_TESTS OFF CACHE BOOL "")
set(wxBUILD_DEMOS OFF CACHE BOOL "")
set(wxBUILD_BENCHMARKS OFF CACHE BOOL "")
set(wxUSE_EXCEPTIONS ON CACHE BOOL "")
set(wxUSE_GUI ON CACHE BOOL "")
set(wxUSE_STC ON CACHE BOOL "")
set(wxUSE_WEBVIEW ON CACHE BOOL "")
set(wxUSE_OPENGL ON CACHE BOOL "")
set(wxUSE_HTML ON CACHE BOOL "")
set(wxUSE_MEDIACTRL ON CACHE BOOL "") 

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
# Use GLOB to find all .cpp files in the src directory automatically
file(GLOB WXDRAGON_SOURCES "src/*.cpp")
if(NOT WXDRAGON_SOURCES)
    message(FATAL_ERROR "No source files found in ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")
endif()
message(STATUS "wxDragon Sources (found via GLOB): ${WXDRAGON_SOURCES}")

# Add vendored wxWidgets as a subdirectory
set(WXDRAGON_SOURCES ${WXDRAGON_SOURCES}
    src/filepickerctrl.cpp
    src/dirpickerctrl.cpp
    src/fontpickerctrl.cpp
)

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
)

# If on MSVC, and wxWidgets was built with a different CRT, you might need to adjust
# For example, if wxWidgets is /MD and you want wxdragon to be /MT, that can cause issues.
# Best to ensure wxWidgets and wxdragon use compatible runtimes.

message(STATUS "wxdragon target defined and configured.")

message(STATUS "CMake configuration for libwxdragon finished.")
 