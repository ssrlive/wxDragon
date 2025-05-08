cmake_minimum_required(VERSION 3.15)
project(const_extractor LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find wxWidgets using the prefix path provided
# Request only core and base, which should be sufficient for wx/wx.h
find_package(wxWidgets REQUIRED COMPONENTS core base) 

add_executable(const_extractor main.cpp)

# Use variables set by find_package instead of imported targets
target_include_directories(const_extractor PRIVATE ${wxWidgets_INCLUDE_DIRS})
target_compile_definitions(const_extractor PRIVATE ${wxWidgets_DEFINITIONS})
target_link_libraries(const_extractor PRIVATE ${wxWidgets_LIBRARIES})

# Remove add_dependencies as it relies on target names which weren't found
# add_dependencies(const_extractor ...)

# Explicitly set the output directory for the executable
set_target_properties(const_extractor PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
)

message(STATUS "Configured const_extractor tool using wxWidgets variables and definitions") 
