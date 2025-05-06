
# Project name
PROJECT = screener

# Build directory
BUILD_DIR = build

# Source files and CMakeLists.txt (add all source files as dependencies here)
SRC = $(wildcard src/*.cpp) CMakeLists.txt

# Default rule: build and run
all: build run

# Build rule (depends on source files and CMakeLists.txt)
build: $(SRC)
	cmake -S . -B $(BUILD_DIR)
	cmake --build $(BUILD_DIR)

# Run rule
run: build
	./$(BUILD_DIR)/$(PROJECT)

# Clean rule
clean:
	rm -rf $(BUILD_DIR)
