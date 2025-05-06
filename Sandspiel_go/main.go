package main

import (
	rl "github.com/gen2brain/raylib-go/raylib"
)

const WINDOWHEIGHT = int32(450)
const WINDOWWIDTH = int32(800)
const CELLSIZE = int32(10)
const CELLS_PER_COL = WINDOWHEIGHT / CELLSIZE
const CELLS_PER_ROW = WINDOWWIDTH / CELLSIZE




func main() {
	rl.InitWindow(WINDOWWIDTH, WINDOWHEIGHT, "raylib [core] example - basic window")
	defer rl.CloseWindow()

	rl.SetTargetFPS(30)
	// create new vector of booleans
	sand_map := make([][]bool, CELLS_PER_ROW)

	for i := range sand_map {
		sand_map[i] = make([]bool, CELLS_PER_COL)
	}
	for !rl.WindowShouldClose() {
		
		if rl.IsMouseButtonDown(rl.MouseButtonLeft) {
			var mousePosition rl.Vector2 = rl.GetMousePosition()
			cell_x := int(mousePosition.X / float32(CELLSIZE))
			cell_y := int(mousePosition.Y / float32(CELLSIZE))
			sand_map[cell_x][cell_y] = true
		}
		
		c := rl.Yellow
		c.G = 200
		rl.BeginDrawing()

		rl.ClearBackground(rl.RayWhite)
		for i := range sand_map {
			for j := range sand_map[i] {
				x := int32(i) * CELLSIZE
				y := int32(j) * CELLSIZE
				if sand_map[i][j] {
					// draw filled rect
					rl.DrawRectangle(x, y, CELLSIZE, CELLSIZE, c)
				} else {
					//rl.DrawRectangleLines(x, y, CELLSIZE, CELLSIZE, rl.Black)
				}
			}
		}
		rl.EndDrawing()
		sand_map = generate_next_gen(sand_map)
		
	}
}

func generate_next_gen(sand_map [][]bool) [][]bool {
	sand_map_next_gen := make([][]bool, len(sand_map))
	for i := range sand_map {
		sand_map_next_gen[i] = make([]bool, len(sand_map[i]))
		copy(sand_map_next_gen[i], sand_map[i])
	}

	for i := range sand_map {
		for j := len(sand_map[i]) - 1; j >= 0; j-- {
			if sand_map[i][j] {
				if j < len(sand_map[i])-1 {
					if !sand_map[i][j+1] {
						sand_map_next_gen[i][j] = false
						sand_map_next_gen[i][j+1] = true
					} else if i+1 < len(sand_map)-1 {
						if sand_map[i][j+1] && !sand_map[i+1][j+1] {
							sand_map_next_gen[i][j] = false
							sand_map_next_gen[i+1][j+1] = true
						} else if i-1 > 0 {
							if !sand_map[i-1][j+1] {
								sand_map_next_gen[i][j] = false
								sand_map_next_gen[i-1][j+1] = true
							}
						}
					}

				}
			}
		}
	}
	return sand_map_next_gen
}
