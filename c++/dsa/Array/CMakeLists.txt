# Specify the minimum version of CMake
cmake_minimum_required(VERSION 3.20)

# Set the project name and version
project(MyCpp20Project VERSION 1.0 LANGUAGES CXX)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
# Specify the C++ standard to use
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Add the source files for the executable
add_executable(out Arrays.cpp)
add_custom_target(run
    DEPENDS out  # Ensure the 'out' target is built
    COMMAND $<TARGET_FILE:out>  # Run the 'out' executable
)
