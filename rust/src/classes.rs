use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub, Mul};

#[derive(Debug,PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T
}

impl Point<i32> {
    pub const RIGHT: Point<i32> = Point {x: 1, y: 0};
    pub const LEFT: Point<i32> = Point {x: -1, y: 0};
    pub const UP: Point<i32> = Point {x: 0, y: -1};
    pub const DOWN: Point<i32> = Point {x: 0, y: 1};
    pub const DIRECTIONS: [Point<i32>; 4] = [Point::RIGHT, Point::LEFT, Point::UP, Point::DOWN];

    pub const UP_RIGHT: Point<i32> = Point {x: 1, y: -1};
    pub const UP_LEFT: Point<i32> = Point {x: -1, y: -1};
    pub const DOWN_RIGHT: Point<i32> = Point {x: 1, y: 1};
    pub const DOWN_LEFT: Point<i32> = Point {x: -1, y: 1};
    pub const DIAGONALS: [Point<i32>; 4] = [
        Point::UP_RIGHT,
        Point::UP_LEFT,
        Point::DOWN_RIGHT,
        Point::DOWN_LEFT
    ];
}

impl<T> Point<T> {

    #[allow(unused)]
    pub fn new() -> Self where T: Default {
        return Point { x: T::default(), y: T::default() }
    }

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

impl<T: PartialEq> Eq for Point<T> {}
impl<T> Hash for Point<T> where T: Hash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
impl<T> Clone for Point<T> where T: Clone {
    fn clone(&self) -> Self {
        Point {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}
impl<T> Add for Point<T> where T: Add<Output = T> {
    type Output = Self;

    fn add(self, other: Point<T>) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<T> Sub for Point<T> where T: Sub<Output = T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Mul<i32> for Point<i32> {
    type Output = Self;

    fn mul(self, factor: i32) -> Self::Output {
        Point {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl Mul<i64> for Point<i64> {
    type Output = Self;

    fn mul(self, factor: i64) -> Self::Output {
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
}
