# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.30

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/sp/priv/screener

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/sp/priv/screener/build

# Include any dependencies generated for this target.
include CMakeFiles/screener.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/screener.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/screener.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/screener.dir/flags.make

CMakeFiles/screener.dir/src/main.cpp.o: CMakeFiles/screener.dir/flags.make
CMakeFiles/screener.dir/src/main.cpp.o: /home/sp/priv/screener/src/main.cpp
CMakeFiles/screener.dir/src/main.cpp.o: CMakeFiles/screener.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/home/sp/priv/screener/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/screener.dir/src/main.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/screener.dir/src/main.cpp.o -MF CMakeFiles/screener.dir/src/main.cpp.o.d -o CMakeFiles/screener.dir/src/main.cpp.o -c /home/sp/priv/screener/src/main.cpp

CMakeFiles/screener.dir/src/main.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/screener.dir/src/main.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /home/sp/priv/screener/src/main.cpp > CMakeFiles/screener.dir/src/main.cpp.i

CMakeFiles/screener.dir/src/main.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/screener.dir/src/main.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /home/sp/priv/screener/src/main.cpp -o CMakeFiles/screener.dir/src/main.cpp.s

# Object files for target screener
screener_OBJECTS = \
"CMakeFiles/screener.dir/src/main.cpp.o"

# External object files for target screener
screener_EXTERNAL_OBJECTS =

screener: CMakeFiles/screener.dir/src/main.cpp.o
screener: CMakeFiles/screener.dir/build.make
screener: CMakeFiles/screener.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --bold --progress-dir=/home/sp/priv/screener/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking CXX executable screener"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/screener.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/screener.dir/build: screener
.PHONY : CMakeFiles/screener.dir/build

CMakeFiles/screener.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/screener.dir/cmake_clean.cmake
.PHONY : CMakeFiles/screener.dir/clean

CMakeFiles/screener.dir/depend:
	cd /home/sp/priv/screener/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/sp/priv/screener /home/sp/priv/screener /home/sp/priv/screener/build /home/sp/priv/screener/build /home/sp/priv/screener/build/CMakeFiles/screener.dir/DependInfo.cmake "--color=$(COLOR)"
.PHONY : CMakeFiles/screener.dir/depend

