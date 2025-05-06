use std::fs;

fn main() {
    

    let x = fs::read_to_string("src/input.txt").unwrap();
    // Convert the input string into a grid (2D vector of characters)
    let grid: Vec<Vec<char>> = x
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Iterate through each cell in the grid
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if is_xmas(&grid, row, col) {
                count += 1;
            }
        }
    }

    println!("Total X-MAS patterns found: {}", count);
}

// Function to check if the X-MAS pattern exists with the given center
fn is_xmas(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    // Check the diagonals for "MAS" or "SAM" in both orientations
    let top_left = (grid[row - 1][col - 1], grid[row][col], grid[row + 1][col + 1]);
    let top_right = (grid[row - 1][col + 1], grid[row][col], grid[row + 1][col - 1]);

    // Valid MAS patterns (forwards and backwards)
    let mas = [('M', 'A', 'S'), ('S', 'A', 'M')];

    // Check if both diagonals form a valid MAS pattern
    mas.contains(&top_left) && mas.contains(&top_right)
}
