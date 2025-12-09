use std::hash::Hash;
use std::ops::{Add, Sub, Mul};
use std::fmt;

#[derive(Debug,PartialEq,Copy,Eq,Hash,Default,Clone,Ord,PartialOrd)]
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

    pub fn inverse(&self) -> Point<i32> {
        return Point::<i32> {
            x: self.x * -1,
            y: self.y * -1
        }
    }

    pub fn is_rotated(&self, other: Point<i32>) -> bool {
        return (*self == Point::<i32>::LEFT && (other == Point::<i32>::DOWN || other == Point::<i32>::UP)) ||
               (*self == Point::<i32>::RIGHT && (other == Point::<i32>::DOWN || other == Point::<i32>::UP)) ||
               (*self == Point::<i32>::DOWN && (other == Point::<i32>::RIGHT || other == Point::<i32>::LEFT)) ||
               (*self == Point::<i32>::UP && (other == Point::<i32>::RIGHT || other == Point::<i32>::LEFT));
    }

    pub fn distance_from(&self, other: &Point<i32>) -> usize  {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as usize
    }
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

impl<T: Mul<Output = T> + Copy> Point<T> {
    pub fn from(x: T, y: T) -> Self {
        return Point { x, y }
    }

    pub fn distance(&self, other: &Point<T>) -> Self where T: Sub<Output = T> + Clone  {
        return Point {
            x: other.x - self.x,
            y: other.y - self.y
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
impl<T: std::fmt::Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Clone)]
pub struct Maze<T> {
    pub size_y: usize,
    pub size_x: usize,
    #[allow(unused)]
    pub array: Vec<Vec<T>>,
}

impl<T: Copy + std::fmt::Display + std::cmp::PartialEq> Maze<T> {
    #[allow(unused)]
    pub fn new() -> Self {
        return Maze {
            size_y: 0,
            size_x: 0,
            array: vec![]
        }
    }

    #[allow(unused)]
    pub fn from_size(size_x: usize, size_y: usize, init: T) -> Self {
        let mut array: Vec<Vec<T>> = Vec::with_capacity(size_y);
        for y in 0..size_y {
            array.push(Vec::with_capacity(size_x));
            for _x in 0..size_x {
                array[y].push(init);
            }
        }

        return Self {
            size_x,
            size_y,
            array
        }
    }

    #[allow(unused)]
    pub fn print(&self) {
        for row in self.array.iter() {
            for &col in row.iter() {
                print!("{}", col);
            }
            println!("");
        }
    }

    #[allow(unused)]
    pub fn get(&self, pos: &Point<i32>) -> Option<T> {
        if pos.x < 0 || pos.x >= self.size_x as i32 ||
           pos.y < 0 || pos.y >= self.size_y as i32 {
            return None;
        }
        Some(self.array[pos.y as usize][pos.x as usize])
    }

    #[allow(unused)]
    pub fn set(&mut self, pos: &Point<i32>, ch: T) {
        if pos.x < 0 || pos.x >= self.size_x as i32 ||
           pos.y < 0 || pos.y >= self.size_y as i32 {
            return;
        }
        self.array[pos.y as usize][pos.x as usize] = ch;
    }

    #[allow(unused)]
    pub fn find_first(&self, to_find: T) -> Option<Point<i32>> {
        for (y, line) in self.array.iter().enumerate() {
            for (x, &ch) in line.iter().enumerate() {
                if ch == to_find {
                    return Some(Point::<i32> {x: x as i32, y: y as i32});
                }
            }
        }
        return None
    }

    #[allow(unused)]
    pub fn count(&self, to_count: T) -> usize {
        let mut count: usize = 0;
        for (y, line) in self.array.iter().enumerate() {
            for (x, &ch) in line.iter().enumerate() {
                if ch == to_count {
                    count += 1;
                }
            }
        }
        return count;
    }
}

impl Maze<char> {
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
    pub fn count_neighbors(&self, pos: &Point<i32>, neighbor: &char) -> u8 {
        let mut count: u8 = 0;

        if let Some(v) = self.get(pos) {
            let directions: [Point<i32>; 8] = [
                Point::from(0, 1),
                Point::from(0, -1),
                Point::from(1, 0),
                Point::from(1, -1),
                Point::from(1, 1),
                Point::from(-1, 0),
                Point::from(-1, 1),
                Point::from(-1, -1),
            ];

            for direction in directions {
                if let Some(neigh) = self.get(&(*pos + direction)) {
                    if neigh == *neighbor {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    #[allow(unused)]
    pub fn fill_rectangles(&mut self, fill_char: char) {
        if self.size_y == 0 || self.size_x == 0 {
            return;
        }

        // First, mark all cells reachable from the border (outside the rectangles)
        let mut visited = vec![vec![false; self.size_x]; self.size_y];

        // Start flood fill from all border cells that are not 'X'
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                // Check if on border
                if y == 0 || y == self.size_y - 1 || x == 0 || x == self.size_x - 1 {
                    if self.array[y][x] != 'X' && !visited[y][x] {
                        self.flood_fill_mark(&mut visited, x, y);
                    }
                }
            }
        }

        // Now fill all unvisited cells that are not 'X' (these are inside rectangles)
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                if !visited[y][x] && self.array[y][x] != 'X' {
                    self.array[y][x] = fill_char;
                }
            }
        }
    }

    fn flood_fill_mark(&self, visited: &mut Vec<Vec<bool>>, x: usize, y: usize) {
        let mut stack = vec![(x, y)];

        while let Some((cx, cy)) = stack.pop() {
            if visited[cy][cx] || self.array[cy][cx] == 'X' {
                continue;
            }

            visited[cy][cx] = true;

            // Check all 4 neighbors
            if cx > 0 {
                stack.push((cx - 1, cy));
            }
            if cx < self.size_x - 1 {
                stack.push((cx + 1, cy));
            }
            if cy > 0 {
                stack.push((cx, cy - 1));
            }
            if cy < self.size_y - 1 {
                stack.push((cx, cy + 1));
            }
        }
    }
}
