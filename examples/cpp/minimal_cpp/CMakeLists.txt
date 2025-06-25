cmake_minimum_required(VERSION 3.16)
project(xrc_test)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Auto-detect the prebuilt wxWidgets directory
file(GLOB WXWIDGETS_LIBS_PATH
    "${CMAKE_CURRENT_SOURCE_DIR}/../../../target/debug/build/wxdragon-sys-*/out/wxwidgets-*-macos-arm64-debug"
)

if(NOT WXWIDGETS_LIBS_PATH)
    message(FATAL_ERROR "Could not find prebuilt wxWidgets directory. Please run 'cargo build' first to download prebuilt libraries.")
endif()

# Use the first match (there should only be one)
list(GET WXWIDGETS_LIBS_PATH 0 WXWIDGETS_LIB_DIR)
message(STATUS "Found wxWidgets at: ${WXWIDGETS_LIB_DIR}")

# Platform detection and configuration (following const_extractor pattern)
if(CMAKE_SYSTEM_NAME MATCHES "Darwin")
    message(STATUS "Building for macOS")
    set(PLATFORM_NAME "macos")
    # Define platform-specific preprocessor macros
    add_compile_definitions(__WXOSX_COCOA__ __WXMAC__ __WXOSX__ _FILE_OFFSET_BITS=64 wxDEBUG_LEVEL=1 wxUSE_UNICODE=1)
elseif(CMAKE_SYSTEM_NAME MATCHES "Windows") 
    message(STATUS "Building for Windows")
    set(PLATFORM_NAME "windows")
    add_compile_definitions(WXD_TARGET_WINDOWS __WXMSW__ _FILE_OFFSET_BITS=64 wxUSE_UNICODE=1 UNICODE _UNICODE)
elseif(CMAKE_SYSTEM_NAME MATCHES "Linux")
    message(STATUS "Building for Linux") 
    set(PLATFORM_NAME "linux")
    add_compile_definitions(__WXGTK__ _FILE_OFFSET_BITS=64)
else()
    message(FATAL_ERROR "Unsupported platform: ${CMAKE_SYSTEM_NAME}")
endif()

# Set up wxWidgets include directories (following const_extractor pattern)
set(WX_INCLUDE_PATHS)

# First, add main wxWidgets source headers
set(WX_MAIN_INCLUDE_DIR "${WXWIDGETS_LIB_DIR}/include")
if(EXISTS "${WX_MAIN_INCLUDE_DIR}")
    list(APPEND WX_INCLUDE_PATHS "${WX_MAIN_INCLUDE_DIR}")
    message(STATUS "Found main wxWidgets headers: ${WX_MAIN_INCLUDE_DIR}")
else()
    message(FATAL_ERROR "Cannot find main wxWidgets include directory at ${WX_MAIN_INCLUDE_DIR}")
endif()

# Second, add platform-specific generated headers (for setup.h, etc.)
if(PLATFORM_NAME STREQUAL "macos")
    # For macOS, check multiple possible locations for setup.h
    set(WX_PLATFORM_DIRS_TO_CHECK
        "${WXWIDGETS_LIB_DIR}/osx_cocoa-unicode-static-3.3"
        "${WXWIDGETS_LIB_DIR}/wx/include/osx_cocoa-unicode-static-3.3"
    )
    foreach(dir IN LISTS WX_PLATFORM_DIRS_TO_CHECK)
        if(EXISTS "${dir}")
            list(APPEND WX_INCLUDE_PATHS "${dir}")
            message(STATUS "Found macOS-specific headers: ${dir}")
            break()
        endif()
    endforeach()
elseif(PLATFORM_NAME STREQUAL "windows")
    # For Windows builds, ensure setup.h is available at the expected location
    set(WX_MSW_SETUP "${WXWIDGETS_LIB_DIR}/include/wx/msw/setup.h")
    set(WX_TARGET_SETUP "${WXWIDGETS_LIB_DIR}/include/wx/setup.h")
    
    if(EXISTS "${WX_MSW_SETUP}" AND NOT EXISTS "${WX_TARGET_SETUP}")
        file(COPY "${WX_MSW_SETUP}" DESTINATION "${WXWIDGETS_LIB_DIR}/include/wx/")
        message(STATUS "Copied Windows setup.h from msw to wx directory")
    endif()
elseif(PLATFORM_NAME STREQUAL "linux")
    # For Linux, check multiple possible locations for setup.h
    set(WX_PLATFORM_DIRS_TO_CHECK
        "${WXWIDGETS_LIB_DIR}/gtk3-unicode-static-3.3"
        "${WXWIDGETS_LIB_DIR}/wx/include/gtk3-unicode-static-3.3"
    )
    foreach(dir IN LISTS WX_PLATFORM_DIRS_TO_CHECK)
        if(EXISTS "${dir}")
            list(APPEND WX_INCLUDE_PATHS "${dir}")
            message(STATUS "Found Linux-specific headers: ${dir}")
            break()
        endif()
    endforeach()
endif()

message(STATUS "Using wxWidgets include directories: ${WX_INCLUDE_PATHS}")

# Find required libraries
find_library(WX_BASE_LIB wx_baseu-3.3 PATHS ${WXWIDGETS_LIB_DIR} NO_DEFAULT_PATH)
find_library(WX_CORE_LIB wx_osx_cocoau_core-3.3 PATHS ${WXWIDGETS_LIB_DIR} NO_DEFAULT_PATH)
find_library(WX_XRC_LIB wx_osx_cocoau_xrc-3.3 PATHS ${WXWIDGETS_LIB_DIR} NO_DEFAULT_PATH)
find_library(WX_XML_LIB wx_baseu_xml-3.3 PATHS ${WXWIDGETS_LIB_DIR} NO_DEFAULT_PATH)
find_library(WX_HTML_LIB wx_osx_cocoau_html-3.3 PATHS ${WXWIDGETS_LIB_DIR} NO_DEFAULT_PATH)

# Check if libraries were found
if(NOT WX_BASE_LIB OR NOT WX_CORE_LIB OR NOT WX_XRC_LIB OR NOT WX_XML_LIB OR NOT WX_HTML_LIB)
    message(FATAL_ERROR "Could not find required wxWidgets libraries")
endif()

message(STATUS "Found wxWidgets libraries:")
message(STATUS "  Base: ${WX_BASE_LIB}")
message(STATUS "  Core: ${WX_CORE_LIB}")
message(STATUS "  XRC: ${WX_XRC_LIB}")
message(STATUS "  XML: ${WX_XML_LIB}")
message(STATUS "  HTML: ${WX_HTML_LIB}")

# Create the executable
add_executable(xrc_test main.cpp)

# Set include directories using the same pattern as const_extractor
target_include_directories(xrc_test PRIVATE ${WX_INCLUDE_PATHS})

# Find additional wxWidgets libraries (following build.rs pattern)
find_library(WX_JPEG_LIB wxjpeg-3.3 PATHS ${WXWIDGETS_LIB_DIR} NO_DEFAULT_PATH)
find_library(WX_PNG_LIB wxpng-3.3 PATHS ${WXWIDGETS_LIB_DIR} NO_DEFAULT_PATH)
find_library(WX_TIFF_LIB wxtiff-3.3 PATHS ${WXWIDGETS_LIB_DIR} NO_DEFAULT_PATH)
find_library(WX_REGEX_LIB wxregexu-3.3 PATHS ${WXWIDGETS_LIB_DIR} NO_DEFAULT_PATH)

# Link libraries (following the same order as const_extractor, but adding XRC and XML)
target_link_libraries(xrc_test
    ${WX_XRC_LIB}
    ${WX_HTML_LIB}
    ${WX_CORE_LIB}
    ${WX_BASE_LIB}
    ${WX_XML_LIB}
    ${WX_JPEG_LIB}
    ${WX_PNG_LIB}
    ${WX_TIFF_LIB}
    ${WX_REGEX_LIB}
    # System libraries
    expat
    z
    iconv
    c++
    # macOS frameworks
    "-framework AudioToolbox"
    "-framework CoreFoundation"
    "-framework Security"
    "-framework Carbon"
    "-framework Cocoa"
    "-framework IOKit"
    "-framework QuartzCore"
    "-framework AppKit"
    "-framework CoreGraphics"
    "-framework Foundation"
    "-framework SystemConfiguration"
    "-framework OpenGL"
    "-framework System"
)

# Copy XRC file to build directory
configure_file(
    "${CMAKE_CURRENT_SOURCE_DIR}/test_panel.xrc"
    "${CMAKE_CURRENT_BINARY_DIR}/test_panel.xrc"
    COPYONLY
)

# On macOS, create an app bundle
if(APPLE)
    set_target_properties(xrc_test PROPERTIES
        MACOSX_BUNDLE TRUE
        MACOSX_BUNDLE_INFO_PLIST "${CMAKE_CURRENT_SOURCE_DIR}/Info.plist"
    )
    
    # Copy XRC file to bundle resources
    set_source_files_properties("${CMAKE_CURRENT_SOURCE_DIR}/test_panel.xrc" 
        PROPERTIES MACOSX_PACKAGE_LOCATION Resources)
    target_sources(xrc_test PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}/test_panel.xrc")
endif()

message(STATUS "xrc_test configured successfully") 