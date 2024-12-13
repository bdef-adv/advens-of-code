use crate::classes::{Point, Maze};
use std::collections::{HashMap, HashSet};

type Point32 = Point<i32>;

struct City {
    maze: Maze,
    pub antennas: HashMap<char, Vec<Point32>>,
    pub antinodes: HashSet<Point32>
}

impl City {
    fn from(input: &str) -> Self {
        let mut array: Vec<Vec<char>> = vec![];
        let mut antennas: HashMap<char, Vec<Point32>> = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            array.push(vec![]);
            for (x, ch) in line.chars().enumerate() {
                array[y].push(ch);
                if ch.is_alphanumeric() {
                    antennas.entry(ch).or_insert(vec![]).push(Point32::from(x as i32, y as i32));
                }
            }
        }

        let size_y = array.len();
        let size_x = array[0].len();

        let maze: Maze = Maze {
            size_y,
            size_x,
            array
        };

        return City {
            maze,
            antennas,
            antinodes: HashSet::new()
        }
    }

    fn get_antenna_pairs(&self, antenna: &char) -> Vec<(Point32, Point32)> {
        let mut pairs: Vec<(Point32, Point32)> = vec![];

        if let Some(antenna_positions) = self.antennas.get(antenna) {
            for i in 0..antenna_positions.len() {
                for j in i + 1..antenna_positions.len() {
                    pairs.push((antenna_positions[i].clone(), antenna_positions[j].clone()));
                }
            }
        }

        return pairs;
    }

    fn insert_antinode_from_pairs(&mut self, pairs: &Vec<(Point32, Point32)>) {
        for (point_a, point_b) in pairs.iter() {
            let left = point_a.clone();
            let right = point_b.clone();

            let distance = left.distance(&right);

            let antinode_a = left - distance.clone();
            let antinode_b = right + distance;

            if antinode_a.x < self.maze.size_x as i32 && antinode_a.x >= 0 &&
               antinode_a.y < self.maze.size_y as i32 && antinode_a.y >= 0 {
                self.antinodes.insert(antinode_a);
            }

            if antinode_b.x < self.maze.size_x as i32 && antinode_b.x >= 0 &&
               antinode_b.y < self.maze.size_y as i32 && antinode_b.y >= 0 {
                self.antinodes.insert(antinode_b);
            }
        }
    }

    fn insert_harmonics_from_pairs(&mut self, pairs: &Vec<(Point32, Point32)>) {
        for (point_a, point_b) in pairs.iter() {
            let left = point_a.clone();
            let right = point_b.clone();

            self.antinodes.insert(left.clone());
            self.antinodes.insert(right.clone());

            let distance = left.distance(&right);

            let mut antinode_a = left - distance.clone();
            let mut antinode_b = right + distance.clone();

            while antinode_a.x < self.maze.size_x as i32 && antinode_a.x >= 0 &&
                  antinode_a.y < self.maze.size_y as i32 && antinode_a.y >= 0 {
                self.antinodes.insert(antinode_a.clone());
                antinode_a = antinode_a - distance.clone();
            }

            while antinode_b.x < self.maze.size_x as i32 && antinode_b.x >= 0 &&
                  antinode_b.y < self.maze.size_y as i32 && antinode_b.y >= 0 {
                self.antinodes.insert(antinode_b.clone());
                antinode_b = antinode_b + distance.clone();
            }
        }
    }

    fn count_harmonics(&mut self) -> usize {
        for (antenna, _points) in self.antennas.clone().iter() {
            let pairs = self.get_antenna_pairs(&antenna);
            self.insert_harmonics_from_pairs(&pairs);
        }

        return self.antinodes.len();
    }

    fn count_antinodes(&mut self) -> usize {
        for (antenna, _points) in self.antennas.clone().iter() {
            let pairs = self.get_antenna_pairs(&antenna);
            self.insert_antinode_from_pairs(&pairs);
        }

        return self.antinodes.len();
    }
}

fn solve_day(file_contents: &str) -> (usize, usize) {
    /*
        Part 1
     */
    let mut maze: City = City::from(file_contents);
    let sum_part1 = maze.count_antinodes();
    maze.antinodes = HashSet::new();
    let sum_part2 = maze.count_harmonics();

    (sum_part1, sum_part2)
}


pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    let results = solve_day(&file_contents);
    (format!("{}", results.0), format!("{}", results.1))
}