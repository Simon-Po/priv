#include <raylib.h>
#include <raylib-cpp.hpp>
#include <vector>

constexpr int screenWidth = 800;
constexpr int screenHeight = 450;

class Line {
public:
    Line(raylib::Vector2 start, raylib::Vector2 end, raylib::Color c = raylib::Color::Black())
        : start(start), end(end), c(c) {}
    void Draw() const { DrawLineV(start, end, c); }
    raylib::Vector2 start;
    raylib::Vector2 end;
private:
    raylib::Color c;
};

class Shape {
public:
    void append(const Line &l) { lines.push_back(l); }
    void expand() {
        std::vector<Line> next;
        next.reserve(lines.size() * 2);
        for (auto &l : lines) {
            raylib::Vector2 v = l.end - l.start;
            raylib::Vector2 half{v.GetX() * 0.5f, v.GetY() * 0.5f};
            raylib::Vector2 fold{-half.GetY(), half.GetX()};
            raylib::Vector2 pivot = l.start + half + fold;
            next.emplace_back(l.start, pivot);
            next.emplace_back(pivot, l.end);
        }
        lines.swap(next);
    }
    void Draw() const {
        for (const auto &l : lines) l.Draw();
    }
private:
    std::vector<Line> lines;
};

int main() {
    raylib::Window w(screenWidth, screenHeight, "dragoncurver");
    SetTargetFPS(60);
    Camera2D camera{};
    camera.target = { screenWidth * 0.5f, screenHeight * 0.5f };
    camera.offset = { screenWidth * 0.5f, screenHeight * 0.5f };
    camera.rotation = 0.0f;
    camera.zoom = 1.0f;
    Shape s;
    s.append({ {300, 200}, {400, 200} });
    const float panSpeed = 10.0f;
    while (!w.ShouldClose()) {
        if (IsKeyDown(KEY_LEFT))    camera.target.x -= panSpeed;
        if (IsKeyDown(KEY_RIGHT))   camera.target.x += panSpeed;
        if (IsKeyDown(KEY_UP))      camera.target.y -= panSpeed;
        if (IsKeyDown(KEY_DOWN))    camera.target.y += panSpeed;
        if (IsKeyPressed(KEY_SPACE)) s.expand();
        if (IsKeyPressed(KEY_I))     camera.zoom *= 1.1f;
        if (IsKeyPressed(KEY_O))     camera.zoom *= 0.9f;
        w.BeginDrawing();
        ClearBackground(raylib::Color::LightGray());
        BeginMode2D(camera);
        s.Draw();
        EndMode2D();
        w.EndDrawing();
    }
    return 0;
}

