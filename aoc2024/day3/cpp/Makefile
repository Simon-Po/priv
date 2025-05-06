# Variables
CXX = g++
CXXFLAGS = -std=c++20 -Wall
TARGET = main

# Rules
all: $(TARGET)

$(TARGET): main.cpp
	$(CXX) $(CXXFLAGS) -o $(TARGET) main.cpp

run: $(TARGET)
	./$(TARGET)

clean:
	rm -f $(TARGET)

