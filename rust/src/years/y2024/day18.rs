use crate::classes::{Maze,Point};
use pathfinding::prelude::dijkstra;

type Point32 = Point<i32>;
type MazeC = Maze<char>;

struct Grid {
    maze: MazeC,
    fallen: Vec<Point32>
}

#[derive(Debug,PartialEq,Eq,Hash,Copy,Clone)]
struct Vector {
    position: Point32,
    direction: Point32
}

impl Vector {
    fn successors(&self, maze: &MazeC) -> Vec<(Vector, usize)> {
        let mut successors = vec![];

        for &direction in Point32::DIRECTIONS.iter() {
            let next_pos = self.position + direction;
            if let Some(ch) = maze.get(&next_pos) {
                if ch != '#' {
                    let next_leaf = Vector {position: next_pos, direction};               
                    successors.push((next_leaf, 1));
                }
            }
        }

        return successors;
    }
}

impl Grid {
    fn from(input: &str, size_y: usize, size_x: usize) -> Self {
        let maze: MazeC = MazeC {
            size_y,
            size_x,
            array: vec![vec!['.'; size_x]; size_y]
        };

        let mut fallen: Vec<Point32> = vec![];
        for line in input.lines() {
            let parts: Vec<&str> = line.split(",").collect();
            let x: i32 = parts[0].parse().unwrap();
            let y: i32 = parts[1].parse().unwrap();
            fallen.push(Point32 {x, y});
        }

        return Grid {
            maze,
            fallen
        }
    }

    fn fill_maze(&mut self, max_length: Option<usize>) {
        if let Some(length) = max_length {
            let length = std::cmp::min(length, self.fallen.len());
            for i in 0..length {
                let point = self.fallen[i];
                self.maze.set(&point, '#');
            }
        } else {
            for i in 0..self.fallen.len() {
                let point = self.fallen[i];
                self.maze.set(&point, '#');
            }
        }
    }

    fn find_path(&self, start: Point32, end: Point32) -> Option<usize> {
        let vector = Vector {position: start, direction: Point32::default()};
        if let Some(result) = dijkstra(&vector, |p| p.successors(&self.maze), |p| p.position == end) {
            return Some(result.1);
        }

        return None;
    }

    fn find_block(&mut self, start: Point32, end: Point32) -> Point32 {
        let mut blocks: Vec<Point32> = vec![];

        for i in 1024..self.fallen.len() {
            let point = self.fallen[i];
            self.maze.set(&point, '#');
            let vector = Vector {position: start, direction: Point32::default()};
            if let None = dijkstra(&vector, |p| p.successors(&self.maze), |p| p.position == end) {
                blocks.push(point)
            }
            self.maze.set(&point, '#');
        }

        return blocks[0];
    }
}

fn solve_day(file_contents: &str) -> (usize, Point32) {
    /*
        Solve day
     */
    let mut grid = Grid::from(file_contents, 71, 71);
    grid.fill_maze(Some(1024));
    let sum_part1 = grid.find_path(Point32 {x: 0, y: 0}, Point32 {x: 70, y: 70}).unwrap();
    let sum_part2: Point32 = grid.find_block(Point32 {x: 0, y: 0}, Point32 {x: 70, y: 70});
    
    (sum_part1, sum_part2)
}


pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    let results = solve_day(&file_contents);
    (format!("{}", results.0), format!("{}", results.1))
}
