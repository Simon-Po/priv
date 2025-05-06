
#include <raylib.h>
#include <raylib-cpp.hpp>
#include <vector>
#include <inttypes.h>
static inline raylib::Vector2 operator*(const raylib::Vector2 &v, float s) {
    return { v.GetX() * s, v.GetY() * s };
}

constexpr int screenWidth = 1600;
constexpr int screenHeight = 900;

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
    void startExpandAnimation(float durationSec) {
        if (animating) return;
        oldLines = lines;
        newLines.clear();
        newLines.reserve(lines.size() * 2);
        for (auto &l : lines) {
            raylib::Vector2 v = l.end - l.start;
            raylib::Vector2 half{v.GetX() * 0.5f, v.GetY() * 0.5f};
            raylib::Vector2 fold{-half.GetY(), half.GetX()};
            raylib::Vector2 pivot = l.start + half + fold;
            newLines.emplace_back(l.start, pivot);
            newLines.emplace_back(pivot, l.end);
        }
        animating = true;
        t = 0.0f;
        duration = durationSec;
    }
    void update(float dt) {
        if (!animating) return;
        t += dt / duration;
        if (t >= 1.0f) {
            t = 1.0f;
            animating = false;
            lines.swap(newLines);
            oldLines.clear();
            newLines.clear();
        }
    }
    void Draw() const {
        if (!animating) {
            for (const auto &l : lines) l.Draw();
        } else {
            uint8_t a0 = static_cast<uint8_t>((1 - t) * 255);
            uint8_t a1 = static_cast<uint8_t>(t * 255);
            raylib::Color c0{0, 0, 0, a0};
            raylib::Color c1{0, 0, 0, a1};
            for (const auto &l : oldLines) DrawLineV(l.start, l.end, c0);
            for (const auto &l : newLines) DrawLineV(l.start, l.end, c1);
        }
    }
private:
    std::vector<Line> lines, oldLines, newLines;
    bool animating = false;
    float t = 0.0f;
    float duration = 0.0f;
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
        float dt = GetFrameTime();
        if (IsKeyDown(KEY_LEFT))    camera.target.x -= panSpeed;
        if (IsKeyDown(KEY_RIGHT))   camera.target.x += panSpeed;
        if (IsKeyDown(KEY_UP))      camera.target.y -= panSpeed;
        if (IsKeyDown(KEY_DOWN))    camera.target.y += panSpeed;
        if (IsKeyPressed(KEY_SPACE)) s.startExpandAnimation(0.5f);
        if (IsKeyPressed(KEY_I))     camera.zoom *= 1.1f;
        if (IsKeyPressed(KEY_O))     camera.zoom *= 0.9f;
        s.update(dt);
        BeginDrawing();
        ClearBackground(raylib::Color::LightGray());
        BeginMode2D(camera);
        s.Draw();
        EndMode2D();
        EndDrawing();
    }
    return 0;
}

