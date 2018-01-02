use std::io::{Read, self};
use std::collections::HashMap;

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

#[derive(Debug, PartialEq, Eq, Hash)]
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

    fn surrounding_points(&self) -> Vec<Point> {
        let mut vec = Vec::new();
        vec.push(Point { x: self.x - 1, y: self.y - 1 });
        vec.push(Point { x: self.x - 1, y: self.y     });
        vec.push(Point { x: self.x - 1, y: self.y + 1 });
        vec.push(Point { x: self.x    , y: self.y - 1 });
        vec.push(Point { x: self.x    , y: self.y     });
        vec.push(Point { x: self.x    , y: self.y + 1 });
        vec.push(Point { x: self.x + 1, y: self.y - 1 });
        vec.push(Point { x: self.x + 1, y: self.y     });
        vec.push(Point { x: self.x + 1, y: self.y + 1 });

        vec
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

#[derive(Debug)]
struct CoordinateSequence {
    point: Point,
    move_seq: MoveSequence,
}

fn coordinate_sequence() -> CoordinateSequence {
    CoordinateSequence { point: Point { x: 0, y: 0 }, move_seq: move_sequence() }
}

impl Iterator for CoordinateSequence {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        let p = Point { x: self.point.x, y: self.point.y };
        let dir = self.move_seq.next().unwrap();
        let new_point = self.point.move_dir(dir);
        self.point = new_point;

        Some(p)
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

fn day3b(input : usize) -> usize {
    let mut values = HashMap::new();
    let mut last_value = 0;

    for point in coordinate_sequence().take(input) {
        if point.x == 0 && point.y == 0 {
            values.insert(point, 1);
            last_value = 1;
        }
        else {
            let mut v = 0;
            for surrounding_point in point.surrounding_points() {
                v += match values.get(&surrounding_point) {
                    Some(&number) => number,
                    _ => 0,
                }
            }
            values.insert(point, v);
            last_value = v;
        }

        if last_value > input {
            return last_value;
        }
    }

    last_value
}
