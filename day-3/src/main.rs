use std::io::{Read, self};

fn main() {
    let mut buffer = String::new();

    // Get the input
    io::stdin().read_to_string(&mut buffer)
        .expect("Read stdin");
    let input : usize = buffer.trim().parse()
        .expect("Stdin is a single number");

    let result = day3a(input);
    println!("One: {}", result);
    let result = day3b(input);
    println!("Two: {}", result);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_dir(&self, dir : Direction) -> Point {
        match dir {
            Direction::Left => Point { x: self.x - 1, y: self.y },
            Direction::Right => Point { x: self.x + 1, y: self.y },
            Direction::Up => Point { x: self.x, y: self.y - 1 },
            Direction::Down => Point { x: self.x, y: self.y + 1 },
        }
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
enum BlockType {
    RightUp,
    LeftDown,
}

#[derive(Debug)]
struct MoveSequence {
    block_type: BlockType,
    block_count: u32,
    block_index: u32,
}

fn move_sequence() -> MoveSequence {
    MoveSequence { block_type: BlockType::RightUp, block_count: 1, block_index: 0 }
}

impl MoveSequence {
    fn direction(&self) -> Direction {
        if self.block_index < self.block_count {
            return match self.block_type {
                BlockType::RightUp => Direction::Right,
                BlockType::LeftDown => Direction::Left,
            }
        }

        match self.block_type {
            BlockType::RightUp => Direction::Up,
            BlockType::LeftDown => Direction::Down,
        }
    }
}

impl Iterator for MoveSequence {
    type Item = Direction;

    fn next(&mut self) -> Option<Direction> {
        let dir = self.direction();

        let next_index = self.block_index + 1;

        if next_index < self.block_count * 2 {
            self.block_index = next_index;
        }
        else {
            self.block_index = 0;
            self.block_count = self.block_count + 1;
            self.block_type = match self.block_type {
                BlockType::RightUp => BlockType::LeftDown,
                BlockType::LeftDown => BlockType::RightUp,
            };
        }

        Some(dir)
    }
}

fn manhatten_distance(a : Point, b : Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn coordinate_for_square(i : usize) -> Point {
    coordinate_for_index(i - 1)
}

fn coordinate_for_index(i : usize) -> Point {
    let mut point = Point { x: 0, y: 0 };

    if i == 0 {
        return point;
    }

    for dir in move_sequence().take(i) {
        point = point.move_dir(dir);
    }

    point
}

fn day3a(input : usize) -> i32 {
    let coord = coordinate_for_square(input);
    let zero = Point { x: 0, y: 0 };

    manhatten_distance(zero, coord)
}

fn day3b(_ : usize) -> i32 {
    0
}
