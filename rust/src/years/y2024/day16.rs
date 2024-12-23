use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use crate::classes::{Maze,Point};
// Sans utiliser ça je pourrais chauffer la planète pendant 100000 ans ngl
use pathfinding::prelude::{dijkstra,astar};

type MazeC = Maze<char>;
type Point32 = Point<i32>;

#[derive(Debug,Copy,Clone,Hash,Eq)]
struct Leaf {
    position: Point32,
    direction: Point32,
    score: usize
}

impl Ord for Leaf {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score) // Min-heap (lowest score has highest priority)
    }
}

impl PartialOrd for Leaf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Leaf {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Vector {
    position: Point32,
    direction: Point32,
}

impl Vector {
    fn successors(&self, maze: &MazeC) -> Vec<(Vector, usize)> {
        let mut successors = vec![];

        for &direction in Point32::DIRECTIONS.iter() {
            let next_pos = self.position + direction;
            if maze.get(&next_pos) != Some('#') {
                let move_score = get_move_score(self.direction, direction);
                let next_leaf = Vector {position: next_pos, direction};               
                successors.push((next_leaf, move_score));
            }
        }

        return successors;
    }
}

fn get_move_score(dir1: Point32, dir2: Point32) -> usize {
    if dir1.is_rotated(dir2) {
        return 1001;
    } else {
        return 1;
    }
}

impl Leaf {
    fn successors(&self, maze: &MazeC, visited: &mut HashMap<Point32, usize>) -> Vec<(Leaf, usize)> {
        let mut successors = vec![];

        for &direction in Point32::DIRECTIONS.iter() {
            let next_pos = self.position + direction;
            if maze.get(&next_pos) != Some('#') {
                let move_score = get_move_score(self.direction, direction);
                let next_leaf = Leaf {position: next_pos, direction, score: self.score + move_score};
                if let Some(&prev_score) = visited.get(&next_leaf.position) {
                    if prev_score <= move_score {
                        continue;
                    }
                }

                
                successors.push((next_leaf, move_score));
            }
        }

        return successors;
    }
}


impl MazeC {
    #[allow(unused)]
    fn solve_djikstra(&mut self, start: Point32, end: Point32) -> Option<usize> {
        let mut visited: HashMap<Point32, usize> = HashMap::new();
        let leaf = Leaf {position: start, direction: Point32::default(), score: 0};
        visited.insert(start, 0);
        if let Some(result) = dijkstra(&leaf, |p| p.successors(self, &mut visited), |p| p.position == end) {
            return Some(result.1 + 1000); // je rajoute + 1000 ici, pour l'exemple ça passe mais pas pour mon input
        }

        return None;
    }

    #[allow(unused)]
    fn solve_djikstra_vector(&mut self, start: Point32, end: Point32) -> Option<usize> {
        let mut visited: HashMap<Point32, usize> = HashMap::new();
        let leaf = Vector {position: start, direction: Point32::default()};
        visited.insert(start, 0);
        if let Some(result) = dijkstra(&leaf, |p| p.successors(self), |p| p.position == end) {
            return Some(result.1);
        }

        return None;
    }


    #[allow(unused)]
    fn solve_astar(&mut self, start: Point32, end: Point32) -> Option<usize> {
        let mut visited: HashMap<Point32, usize> = HashMap::new();
        let leaf = Leaf {position: start, direction: Point32::default(), score: 0};
        visited.insert(start, 0);
        if let Some(result) = astar(&leaf, |p| p.successors(self, &mut visited), |_| 0, |p| p.position == end) {
            return Some(result.1 + 1000);
        }

        return None;
    }

    #[allow(unused)]
    fn solve_astar_vector(&mut self, start: Point32, end: Point32) -> Option<usize> {
        let mut visited: HashMap<Point32, usize> = HashMap::new();
        let leaf = Vector {position: start, direction: Point32::default()};
        visited.insert(start, 0);
        if let Some(result) = astar(&leaf, |p| p.successors(self), |_| 0, |p| p.position == end) {
            return Some(result.1);
        }

        return None;
    }

    #[allow(unused)]
    fn solve_slow(&mut self, start: Point32, rotations: &mut usize, prev_direction: Point32, path_max_length: usize, path_length: usize, visited: &Vec<Vec<bool>>) -> Option<usize> {
        // Path too long mate, go back
        if path_length > path_max_length {
            return None;
        }

        if let Some(ch) = self.get(&start) {
            if ch == 'E' {
                return Some((*rotations + 1) * 1000 + path_length);
            }
        }

        let mut directions_available: Vec<Point32> = vec![];
        for &direction in Point32::DIRECTIONS.iter() {
            let dest = start + direction;
            if let Some(ch) = self.get(&dest) {
                if ch == '.' || ch == 'E' {
                    directions_available.push(direction);
                }
            }
        }

        let mut new_visited = visited.clone();
        new_visited[start.y as usize][start.x as usize] = true;

        let mut min_score = usize::MAX;
        for &direction in directions_available.iter() {
            let next_start = start + direction;

            if visited[next_start.y as usize][next_start.x as usize] {
                continue;
            }

            let mut rotation: usize = 0;
            if direction.is_rotated(prev_direction) {
                rotation += 1;
            }

            if let Some(next_score) = self.solve_slow(next_start, &mut (*rotations + rotation), direction, path_max_length, path_length + 1, &new_visited) {
                if next_score < min_score {
                    min_score = next_score;
                }
            }
        }

        if min_score == usize::MAX {
            return None;
        }
        return Some(min_score);
    }

    #[allow(unused)]
    fn solve_faster(&mut self, start: Point32, end: Point32) -> Option<usize> {
        let mut visited: HashMap<Leaf, usize> = HashMap::new();
        let mut queue: BinaryHeap<Leaf> = BinaryHeap::new();

        let start_leaf = Leaf {position: start, direction: Point32::default(), score: 0};
        queue.push(start_leaf);
        visited.insert(start_leaf, 0);

        while let Some(leaf) = queue.pop() {
            if self.get(&leaf.position) == Some('E') {
                println!("Found the fucking end mate: {} -> {}", leaf.position, leaf.score);
                return Some(leaf.score + 1000);
            }

            for &direction in Point32::DIRECTIONS.iter() {
                let next_start = leaf.position + direction;
                let new_score = leaf.score + get_move_score(leaf.direction, direction);

                if self.get(&next_start) != Some('#') {
                    let next_leaf = Leaf {position: next_start, direction, score: new_score};

                    if let Some(&prev_score) = visited.get(&next_leaf) {
                        if prev_score <= new_score {
                            continue;
                        }
                    }

                    queue.push(Leaf {
                        position: next_start,
                        direction,
                        score: new_score
                    });
                    visited.insert(next_leaf, new_score);
                }
            }
        }

        return None;     
    }
}


fn solve_day(file_contents: &str) -> (usize, u32) {
    /*
        Solve day
     */
    let mut sum_part1: usize = 0;
    let mut maze = MazeC::from(file_contents);
    if let Some(start_position) = maze.find_first('S') {
        /*sum_part1 = maze.solve_slow(
            start_position,
            &mut 0,
            Point32 {x: 0, y: 0},
            maze.count('.'),
            0,
            &vec![vec![false; maze.size_x]; maze.size_x]
        ).expect("Part 1 did not return any result.");*/
        sum_part1 = maze.solve_djikstra_vector(start_position, maze.find_first('E').unwrap()).expect("Part 1 did not return any result.");
    }
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
