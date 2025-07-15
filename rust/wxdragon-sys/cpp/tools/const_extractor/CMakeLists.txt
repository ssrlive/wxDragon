cmake_minimum_required(VERSION 3.16)
project(const_extractor)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Auto-detect the prebuilt wxWidgets directory
file(GLOB WXWIDGETS_LIBS_PATH
    "${CMAKE_CURRENT_SOURCE_DIR}/../../../../../target/debug/build/wxdragon-sys-*/out/wxwidgets-*-debug"
)

if(NOT WXWIDGETS_LIBS_PATH)
    message(FATAL_ERROR "Could not find prebuilt wxWidgets directory. Please run 'cargo build' first to download prebuilt libraries.")
endif()

# Use the first match (there should only be one)
list(GET WXWIDGETS_LIBS_PATH 0 WXWIDGETS_LIB_DIR)
message(STATUS "Found wxWidgets at: ${WXWIDGETS_LIB_DIR}")

# Platform detection and configuration (following main CMakeLists.txt pattern)
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

# Set up wxWidgets include directories (following main CMakeLists.txt pattern)
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
    set(WX_BASE_NAME "wx_baseu-3.3")
    set(WX_CORE_NAME "wx_osx_cocoau_core-3.3")
    set(WX_ADV_NAME "wx_osx_cocoau_adv-3.3")
    set(WX_JPEG_NAME "wxjpeg-3.3")
    set(WX_PNG_NAME "wxpng-3.3")
    set(WX_TIFF_NAME "wxtiff-3.3")
    set(WX_REGEX_NAME "wxregexu-3.3")
    set(WX_EXPAT_NAME "expat")
elseif(PLATFORM_NAME STREQUAL "windows")
    # For Windows builds, ensure setup.h is available at the expected location
    set(WX_MSW_SETUP "${WXWIDGETS_LIB_DIR}/include/wx/msw/setup.h")
    set(WX_TARGET_SETUP "${WXWIDGETS_LIB_DIR}/include/wx/setup.h")
    
    if(EXISTS "${WX_MSW_SETUP}" AND NOT EXISTS "${WX_TARGET_SETUP}")
        file(COPY "${WX_MSW_SETUP}" DESTINATION "${WXWIDGETS_LIB_DIR}/include/wx/")
        message(STATUS "Copied Windows setup.h from msw to wx directory")
    endif()
    set(WX_BASE_NAME "wxbase33ud")
    set(WX_CORE_NAME "wxmsw33ud_core")
    set(WX_ADV_NAME "wxmsw33ud_adv")
    set(WX_JPEG_NAME "wxjpegd")
    set(WX_PNG_NAME "wxpngd")
    set(WX_TIFF_NAME "wxtiffd")
    set(WX_REGEX_NAME "wxregexud")
    set(WX_EXPAT_NAME "wxexpatd")
    
    set(WXWIDGETS_LIB_DIR "${WXWIDGETS_LIB_DIR}/vc_x64_lib")
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
    set(WX_BASE_NAME "wx_baseu-3.3")
    set(WX_CORE_NAME "wx_gtk3u_core-3.3")
    set(WX_ADV_NAME "wx_gtk3u_adv-3.3")
    set(WX_JPEG_NAME "wxjpeg-3.3")
    set(WX_PNG_NAME "wxpng-3.3")
    set(WX_TIFF_NAME "wxtiff-3.3")
    set(WX_REGEX_NAME "wxregexu-3.3")
    set(WX_EXPAT_NAME "expat")
endif()

message(STATUS "Using wxWidgets include directories: ${WX_INCLUDE_PATHS}")

# Find required libraries
find_library(WX_BASE_LIB ${WX_BASE_NAME} PATHS ${WXWIDGETS_LIB_DIR} NO_DEFAULT_PATH)
find_library(WX_CORE_LIB ${WX_CORE_NAME} PATHS ${WXWIDGETS_LIB_DIR} NO_DEFAULT_PATH)
find_library(WX_ADV_LIB ${WX_ADV_NAME} PATHS ${WXWIDGETS_LIB_DIR} NO_DEFAULT_PATH)

# Check if libraries were found
if(NOT WX_BASE_LIB OR NOT WX_CORE_LIB OR NOT WX_ADV_LIB)
    message(FATAL_ERROR "Could not find required wxWidgets libraries")
endif()

message(STATUS "Found wxWidgets libraries:")
message(STATUS "  Base: ${WX_BASE_LIB}")
message(STATUS "  Core: ${WX_CORE_LIB}")
message(STATUS "  Adv: ${WX_ADV_LIB}")

# Create the executable
add_executable(const_extractor main.cpp)

# Set include directories using the same pattern as main CMakeLists.txt
target_include_directories(const_extractor PRIVATE ${WX_INCLUDE_PATHS})

# Find additional wxWidgets libraries (following build.rs pattern)
find_library(WX_JPEG_LIB ${WX_JPEG_NAME} PATHS ${WXWIDGETS_LIB_DIR} NO_DEFAULT_PATH)
find_library(WX_PNG_LIB ${WX_PNG_NAME} PATHS ${WXWIDGETS_LIB_DIR} NO_DEFAULT_PATH)
find_library(WX_TIFF_LIB ${WX_TIFF_NAME} PATHS ${WXWIDGETS_LIB_DIR} NO_DEFAULT_PATH)
find_library(WX_REGEX_LIB ${WX_REGEX_NAME} PATHS ${WXWIDGETS_LIB_DIR} NO_DEFAULT_PATH)
find_library(WX_EXPAT_LIB ${WX_EXPAT_NAME} PATHS ${WXWIDGETS_LIB_DIR})

if(APPLE)
   set(MACOS_LINK_FLAGS 
    "-framework AudioToolbox "
    "-framework CoreFoundation "
    "-framework Security "
    "-framework Carbon "
    "-framework Cocoa "
    "-framework IOKit "
    "-framework QuartzCore "
    "-framework AppKit "
    "-framework CoreGraphics "
    "-framework Foundation "
    "-framework SystemConfiguration "
    "-framework OpenGL "
    "-framework System"
   )
else()
   set(MACOS_LINK_FLAGS "")
endif()

# Link libraries (following the same order as build.rs)
target_link_libraries(const_extractor
    ${WX_CORE_LIB}
    ${WX_BASE_LIB}
    ${WX_ADV_LIB}
    ${WX_JPEG_LIB}
    ${WX_PNG_LIB}
    ${WX_TIFF_LIB}
    ${WX_REGEX_LIB}
    # System libraries
    ${WX_EXPAT_LIB}
    z
    iconv
    c++
    ${MACOS_LINK_FLAGS}
)

message(STATUS "const_extractor configured successfully")