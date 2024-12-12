use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub};

#[derive(Debug,PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub const RIGHT: Point = Point {x: 1, y: 0};
    pub const LEFT: Point = Point {x: -1, y: 0};
    pub const UP: Point = Point {x: 0, y: -1};
    pub const DOWN: Point = Point {x: 0, y: 1};
    pub const DIRECTIONS: [Point; 4] = [Point::RIGHT, Point::LEFT, Point::UP, Point::DOWN];

    pub const UP_RIGHT: Point = Point {x: 1, y: -1};
    pub const UP_LEFT: Point = Point {x: -1, y: -1};
    pub const DOWN_RIGHT: Point = Point {x: 1, y: 1};
    pub const DOWN_LEFT: Point = Point {x: -1, y: 1};
    pub const DIAGONALS: [Point; 4] = [
        Point::UP_RIGHT,
        Point::UP_LEFT,
        Point::DOWN_RIGHT,
        Point::DOWN_LEFT
    ];
}

impl Point {

    #[allow(unused)]
    pub fn new() -> Self {
        return Point { x: 0, y: 0 }
    }

    pub fn from(x: i32, y: i32) -> Self {
        return Point { x, y }
    }

    pub fn distance(&self, other: &Point) -> Point {
        return Point {
            x: other.x - self.x,
            y: other.y - self.y
        }
    }
}

impl Eq for Point {}
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}
impl Add for Point {
    type Output = Self;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub struct Maze {
    pub size_y: usize,
    pub size_x: usize,
    #[allow(unused)]
    pub array: Vec<Vec<char>>,
}

impl Maze {
    #[allow(unused)]
    pub fn new() -> Self {
        return Maze {
            size_y: 0,
            size_x: 0,
            array: vec![]
        }
    }

    pub fn from(input: &str) -> Self {
        let mut array: Vec<Vec<char>> = vec![];

        for (y, line) in input.lines().enumerate() {
            array.push(vec![]);
            for ch in line.chars() {
                array[y].push(ch);
            }
        }

        let size_y = array.len();
        let size_x = array[0].len();

        return Maze {
            size_y,
            size_x,
            array,
        }
    }

    #[allow(unused)]
    pub fn print(&self) {
        for row in self.array.iter() {
            for &col in row.iter() {
                print!("{col}");
            }
            println!("");
        }
    }

    #[allow(unused)]
    pub fn get_char(&self, pos: &Point) -> Option<char> {
        if pos.x < 0 || pos.x >= self.size_x as i32 ||
           pos.y < 0 || pos.y >= self.size_y as i32 {
            return None;
        }
        Some(self.array[pos.y as usize][pos.x as usize])
    }
}
