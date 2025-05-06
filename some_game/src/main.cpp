#include "Rectangle.hpp"
#include "raylib.h"
#include <raylib-cpp.hpp>

int main() {
    
    // Initialization
    constexpr int screenWidth = 800;
    constexpr int screenHeight = 450;
    
    constexpr const char* window_name = "some_game";

    raylib::Color textColor(LIGHTGRAY);
    raylib::Window w(screenWidth, screenHeight, window_name);
        
    SetTargetFPS(10);
    int i = 0;
    while (!w.ShouldClose()) // Detect window close button or ESC key
    {
        // Update
        raylib::Rectangle r(i,120,200,200) ;
        
        // Draw
        w.BeginDrawing();
        r.Draw(raylib::Color());
        w.ClearBackground(raylib::Color::RayWhite());
        w.EndDrawing();
        i++;
    }

    return 0;
}
