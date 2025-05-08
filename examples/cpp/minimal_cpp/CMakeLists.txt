cmake_minimum_required(VERSION 3.10)
project(MinimalWxToolBarTest LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# --- Finding wxWidgets ---
# This example assumes that the main wxDragon project has already built wxWidgets.
# The wxWidgets build will generate a wxWidgetsConfig.cmake file.
# You need to tell CMake where to find it.
# Option 1: Set wxWidgets_DIR as an environment variable or CMake variable:
#   cmake -DwxWidgets_DIR=/path/to/your/wxdragon/target/debug/build/wxdragon-sys-*/out/build/build/vendor/wxWidgets/lib/cmake/wxWidgets .
# Option 2: Or, hardcode the relative path if you know it (less flexible)
#   set(wxWidgets_DIR "../../../../target/debug/build/wxdragon-sys-SOMEHASH/out/build/build/vendor/wxWidgets/lib/cmake/wxWidgets")

# More robustly, try to find it relative to this example's build directory assuming a common structure.
# This might need adjustment based on your exact $OUT_DIR structure.
# This path assumes you are running cmake from examples/cpp/minimal_toolbar_cpp/build
# and your main target directory is at ../../../../target/

if(NOT wxWidgets_DIR)
    # Try to guess the location based on the typical output structure of the main project.
    # This assumes the C++ example is built in a subdirectory 'build' like:
    # wxdragon/examples/cpp/minimal_toolbar_cpp/build
    # And the main wxWidgets build artifacts are in:
    # wxdragon/target/debug/build/wxdragon-sys-*/out/build/build/vendor/wxWidgets/
    
    # To make this more reliable, we will require the user to pass it in or set an environment variable.
    # message(STATUS "wxWidgets_DIR not set. Trying to guess...")
    # file(GLOB WXDRAGON_SYS_OUT_DIR "../../../../target/debug/build/wxdragon-sys-*/out")
    # if(WXDRAGON_SYS_OUT_DIR)
    #     set(wxWidgets_DIR "${WXDRAGON_SYS_OUT_DIR}/build/build/vendor/wxWidgets/lib/cmake/wxWidgets")
    #     message(STATUS "Guessed wxWidgets_DIR: ${wxWidgets_DIR}")
    # endif()
    message(FATAL_ERROR "wxWidgets_DIR is not set. Please set it to point to the wxWidgets CMake configuration. \nExample: -DwxWidgets_DIR=/path/to/wxdragon/target/debug/build/wxdragon-sys-*/out/build/build/vendor/wxWidgets/lib/cmake/wxWidgets")
endif()

find_package(wxWidgets COMPONENTS core base REQUIRED)

include(${wxWidgets_USE_FILE})

# Add the executable
add_executable(minimal_toolbar_cpp main.cpp)

# Link to wxWidgets
target_link_libraries(minimal_toolbar_cpp PRIVATE ${wxWidgets_LIBRARIES})

# On macOS, ensure it's a bundle if using GUI
if(APPLE)
    set_target_properties(minimal_toolbar_cpp PROPERTIES MACOSX_BUNDLE ON)
endif()

message(STATUS "wxWidgets_FOUND: ${wxWidgets_FOUND}")
message(STATUS "wxWidgets_INCLUDE_DIRS: ${wxWidgets_INCLUDE_DIRS}")
message(STATUS "wxWidgets_LIBRARIES: ${wxWidgets_LIBRARIES}")
message(STATUS "wxWidgets_CXX_FLAGS: ${wxWidgets_CXX_FLAGS}") 