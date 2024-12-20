use crate::classes::{Maze,Point};
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

type Point32 = Point<i32>;
type MazeC = Maze<char>;


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

impl MazeC {
    fn find_path(&self, start: Point32, end: Point32) -> Option<usize> {
        let vector = Vector {position: start, direction: Point32::default()};
        if let Some(result) = dijkstra(&vector, |p| p.successors(&self), |p| p.position == end) {
            return Some(result.1);
        }

        return None;
    }

    fn find_cheats(&mut self, start: Point32, end: Point32, init: usize) -> HashMap<Point32, Option<usize>> {
        let mut results: HashMap<Point32, Option<usize>> = HashMap::new();

        for y in 1..self.size_y-1 {
            for x in 1..self.size_x-1 {
                let cheat_start = Point32 {x: x as i32, y: y as i32};
                let ch_start = self.get(&cheat_start).unwrap();
                for &direction in Point32::DIRECTIONS.iter() {
                    let cheat_end = cheat_start + direction;
                    let ch_end = self.get(&cheat_end).unwrap();

                    if (ch_end != '.' && ch_start != '#') || results.contains_key(&cheat_start) {
                        continue;
                    }

                    self.set(&cheat_start, '.');

                    if let Some(count) = self.find_path(start, end) {
                        if count != init {
                            results.insert(cheat_start, Some(count));
                        }
                    }

                    self.set(&cheat_start, ch_start);
                }
            }
        }

        return results;
    }

    fn find_cheat_count(&mut self) -> usize {
        let start = self.find_first('S').unwrap();
        let end = self.find_first('E').unwrap();
        let initial_picoseconds = self.find_path(start, end).unwrap();

        let cheats = self.find_cheats(start, end, initial_picoseconds);
        let mut cheat_count: usize = 0;

        let mut differences_count: HashMap<i32, usize> = HashMap::new();

        for (_cheat_pos, &cheat_score) in cheats.iter() {
            if let Some(score) = cheat_score {
                *differences_count.entry(initial_picoseconds as i32 - score as i32).or_insert(0) += 1;
                if initial_picoseconds as i32 - score as i32 >= 100 {
                    cheat_count += 1;
                }
            }
        }


        return cheat_count;
    }
}

fn solve_day(file_contents: &str) -> (usize, u32) {
    /*
        Solve day
     */
    let mut maze = MazeC::from(file_contents);
    let sum_part1 = maze.find_cheat_count();

    let sum_part2: u32 = 0;
    
    (sum_part1, sum_part2)
}


pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    let results = solve_day(&file_contents);
    (format!("{}", results.0), format!("{}", results.1))
}
