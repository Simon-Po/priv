
# Specify the minimum version of CMake
cmake_minimum_required(VERSION 3.20)

# Set the project name and version
project(MyProject VERSION 1.0 LANGUAGES CXX)

# Specify the C++ standard
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Find and include GLib and GIO libraries for GDBus
find_package(PkgConfig REQUIRED)
pkg_check_modules(GLIB REQUIRED glib-2.0)
pkg_check_modules(GIO REQUIRED gio-2.0)

# Add the executable
add_executable(screener src/main.cpp)

# Link the GDBus-related libraries
target_include_directories(screener PRIVATE ${GLIB_INCLUDE_DIRS} ${GIO_INCLUDE_DIRS})
target_link_libraries(screener PRIVATE ${GLIB_LIBRARIES} ${GIO_LIBRARIES})
target_compile_options(screener PRIVATE ${GLIB_CFLAGS_OTHER} ${GIO_CFLAGS_OTHER})

# Optional: Enable additional warnings
if (MSVC)
    target_compile_options(screener PRIVATE /W4 /WX)
else()
    target_compile_options(screener PRIVATE -Wall -Wextra -pedantic -Werror)
endif()
