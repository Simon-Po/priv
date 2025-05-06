use std::fmt;
use std::fs;
fn get_guard_position(map: &Vec<Vec<&str>>) -> Guard {
    let mut guard: Guard = Guard {
        pos: Vec2 { x: 0, y: 0 },
        looking_dir: Direction::Up,
        pos_list: vec![],
    };
    for (jdx, x) in map.iter().enumerate() {
        for (idx_index, _y) in x.iter().enumerate() {
            match map[jdx][idx_index] {
                "^" => {
                    guard.pos.x = idx_index;
                    guard.pos.y = jdx;
                }
                ">" => {
                    guard.pos.x = idx_index;
                    guard.pos.y = jdx;
                    guard.looking_dir = Direction::Right;
                }
                "<" => {
                    guard.pos.x = idx_index;
                    guard.pos.y = jdx;
                    guard.looking_dir = Direction::Left;
                }
                "v" => {
                    guard.pos.x = idx_index;
                    guard.pos.y = jdx;
                    guard.looking_dir = Direction::Down;
                }
                _ => continue,
            }
        }
    }
    guard
}
struct Vec2 {
    x: usize,
    y: usize,
}

struct Guard {
    pos: Vec2,
    looking_dir: Direction,
    pos_list: Vec<String>,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Up => write!(f, "Up"),
            Self::Right => write!(f, "Right"),
            Self::Down => write!(f, "Down"),
            Self::Left => write!(f, "Left"),
        }
    }
}

impl Guard {
    fn walk(&mut self, map: Vec<Vec<&str>>, counter: &mut usize) -> Result<(), &str> {
        match self.looking_dir {
            Direction::Up => {
                if self.check_for_wall(map.clone()) {
                    self.looking_dir = Direction::Right;
                } else {
                    if self.pos.y == 1 {
                        return Err("done");
                    }
                    self.pos.y -= 1;
                }
            }
            Direction::Right => {
                if self.check_for_wall(map.clone()) {
                    self.looking_dir = Direction::Down;
                } else {
                    if self.pos.x == map[0].len() {
                        return Err("done");
                    }
                    self.pos.x += 1;
                }
            }
            Direction::Down => {
                if self.check_for_wall(map.clone()) {
                    self.looking_dir = Direction::Left;
                } else {
                    if self.pos.y == map.len() {
                        return Err("done");
                    }
                    self.pos.y += 1;
                }
            }
            Direction::Left => {
                if self.check_for_wall(map.clone()) {
                    self.looking_dir = Direction::Up;
                } else {
                    if self.pos.y == 1 {
                        return Err("done");
                    }
                    self.pos.x -= 1;
                }
            }
        }
        let current_pos = format!("{},{}", self.pos.x, self.pos.y);

        if !self.pos_list.contains(&current_pos) {
            self.pos_list.push(current_pos);
            *counter += 1;
        }
        Ok(())
    }

    fn check_for_wall(&self, map: Vec<Vec<&str>>) -> bool {
        match self.looking_dir {
            Direction::Up => {
                let chr = map[self.pos.y - 1][self.pos.x];

                chr == "#"
            }
            Direction::Right => {
                let chr = map[self.pos.y][self.pos.x + 1];

                chr == "#"
            }
            Direction::Down => {
                let chr = map[self.pos.y + 1][self.pos.x];
                chr == "#"
            }
            Direction::Left => {
                let chr = map[self.pos.y][self.pos.x - 1];
                chr == "#"
            }
        }
    }
}

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let map: Vec<Vec<&str>> = binding.lines().map(|e| e.split("").collect()).collect();
    let mut guard: Guard = get_guard_position(&map);
    let mut counter: usize = 1;
    for _ in 0..100000000 {
        if guard.walk(map.clone(), &mut counter).is_ok() {
            println!("{} {} {}", guard.pos.x, guard.pos.y, &guard.looking_dir);
            println!("{}", counter);
        } else {
            println!("{}", counter + 1);
            break;
        }
    }
}
