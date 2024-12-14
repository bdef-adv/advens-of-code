use std::hash::Hash;
use std::ops::{Add, Sub, Mul};

#[derive(Debug,PartialEq,Copy,Eq,Hash,Default,Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T
}

impl Point<i32> {
    pub const RIGHT: Point<i32> = Point {x: 1, y: 0};
    pub const LEFT: Point<i32> = Point {x: -1, y: 0};
    pub const UP: Point<i32> = Point {x: 0, y: -1};
    pub const DOWN: Point<i32> = Point {x: 0, y: 1};
    pub const DIRECTIONS: [Point<i32>; 4] = [Point::<i32>::RIGHT, Point::<i32>::LEFT, Point::<i32>::UP, Point::<i32>::DOWN];

    pub const UP_RIGHT: Point<i32> = Point {x: 1, y: -1};
    pub const UP_LEFT: Point<i32> = Point {x: -1, y: -1};
    pub const DOWN_RIGHT: Point<i32> = Point {x: 1, y: 1};
    pub const DOWN_LEFT: Point<i32> = Point {x: -1, y: 1};
    pub const DIAGONALS: [Point<i32>; 4] = [
        Point::<i32>::UP_RIGHT,
        Point::<i32>::UP_LEFT,
        Point::<i32>::DOWN_RIGHT,
        Point::<i32>::DOWN_LEFT
    ];
}

impl Point<i64> {
    pub const RIGHT: Point<i64> = Point {x: 1, y: 0};
    pub const LEFT: Point<i64> = Point {x: -1, y: 0};
    pub const UP: Point<i64> = Point {x: 0, y: -1};
    pub const DOWN: Point<i64> = Point {x: 0, y: 1};
    pub const DIRECTIONS: [Point<i64>; 4] = [Point::<i64>::RIGHT, Point::<i64>::LEFT, Point::<i64>::UP, Point::<i64>::DOWN];

    pub const UP_RIGHT: Point<i64> = Point {x: 1, y: -1};
    pub const UP_LEFT: Point<i64> = Point {x: -1, y: -1};
    pub const DOWN_RIGHT: Point<i64> = Point {x: 1, y: 1};
    pub const DOWN_LEFT: Point<i64> = Point {x: -1, y: 1};
    pub const DIAGONALS: [Point<i64>; 4] = [
        Point::<i64>::UP_RIGHT,
        Point::<i64>::UP_LEFT,
        Point::<i64>::DOWN_RIGHT,
        Point::<i64>::DOWN_LEFT
    ];
}

impl<T> Point<T> {
    pub fn from(x: T, y: T) -> Self {
        return Point { x, y }
    }

    pub fn distance(&self, other: &Point<T>) -> Self where T: Sub<Output = T> + Clone  {
        return Point {
            x: other.x.clone() - self.x.clone(),
            y: other.y.clone() - self.y.clone()
        }
    }
}

impl<T: Add<Output=T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Point<T>) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl<T: Mul<Output = T> + Copy> Mul<T> for Point<T> {
    type Output = Self;

    fn mul(self, factor: T) -> Self::Output {
        Point {
            x: self.x * factor,
            y: self.y * factor,
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
    pub fn get_char(&self, pos: &Point<i32>) -> Option<char> {
        if pos.x < 0 || pos.x >= self.size_x as i32 ||
           pos.y < 0 || pos.y >= self.size_y as i32 {
            return None;
        }
        Some(self.array[pos.y as usize][pos.x as usize])
    }

    #[allow(unused)]
    pub fn set_char(&mut self, pos: &Point<i32>, ch: char) {
        if pos.x < 0 || pos.x >= self.size_x as i32 ||
           pos.y < 0 || pos.y >= self.size_y as i32 {
            return;
        }
        self.array[pos.y as usize][pos.x as usize] = ch;
    }

    #[allow(unused)]
    pub fn find_string(&self, pat: &str) -> bool {
        for line_vec in self.array.iter() {
            let line: String = line_vec.iter().collect();
            if let Some(index) = line.find(pat) {
                return true
            }
        }
        return false;
    }
}
