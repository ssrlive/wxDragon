cmake_minimum_required(VERSION 3.15)
project(const_extractor LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Assume wxWidgets has already been found and configured 
# by the parent CMakeLists.txt (using add_subdirectory or FetchContent)
# We just need to link against the imported target or variables.

add_executable(const_extractor main.cpp)

# *** ADD EXPLICIT INCLUDE DIRECTORIES ***
target_include_directories(const_extractor PRIVATE
    ${CMAKE_PROJECT_SOURCE_DIR}/vendor/wxWidgets/include # Explicitly add main wx headers
    # Generated setup.h dir should be added by target_link_libraries
)

# Link against wxWidgets components needed for the headers
# This should add compile definitions and specific include paths (like setup.h)
target_link_libraries(const_extractor PRIVATE wxcore wxbase wxadv wxaui wxgl wxmedia wxpropgrid)

# *** ADD EXPLICIT BUILD DEPENDENCY ***
# Ensure wxWidgets targets are built *before* const_extractor is built,
# especially since wxWidgets might be added with EXCLUDE_FROM_ALL.
add_dependencies(const_extractor wxcore wxbase wxadv wxaui wxgl wxmedia wxpropgrid)

# Explicitly set the output directory for the executable
set_target_properties(const_extractor PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
)

# Ensure the executable is placed in a predictable location relative
# to the main build directory (e.g., ${CMAKE_BINARY_DIR}/bin)
# The main CMakeLists.txt should set CMAKE_RUNTIME_OUTPUT_DIRECTORY

message(STATUS "Configured const_extractor tool") 