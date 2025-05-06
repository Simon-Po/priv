# Compiler and flags
CC = gcc
CFLAGS = -Wall -Wextra -std=c11 -O2

# Targets
TARGET = main
SRC = main.c

.PHONY: all clean run

# Default target to build the program
all: $(TARGET)

# Compile the program
$(TARGET): $(SRC)
	$(CC) $(CFLAGS) -o $(TARGET) $(SRC)

# Run the program
run: $(TARGET)
	./$(TARGET)

# Clean up build artifacts
clean:
	rm -f $(TARGET)
