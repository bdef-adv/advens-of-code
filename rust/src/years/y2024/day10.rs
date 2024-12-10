use crate::classes::{Maze,Point};

impl Point {
    pub const RIGHT: Point = Point {x: 1, y: 0};
    pub const LEFT: Point = Point {x: -1, y: 0};
    pub const UP: Point = Point {x: 0, y: -1};
    pub const DOWN: Point = Point {x: 0, y: 1};
    pub const DIRECTIONS: [Point; 4] = [Point::RIGHT, Point::LEFT, Point::UP, Point::DOWN];
}

struct Trailmap {
    maze: Maze,
    trails: Vec<Point>
}

impl Trailmap {
    fn from(input: &str) -> Self {
        let mut array: Vec<Vec<char>> = vec![];
        let mut trails: Vec<Point> = vec![];

        for (y, line) in input.lines().enumerate() {
            array.push(vec![]);
            for (x, ch) in line.chars().enumerate() {
                array[y].push(ch);
                if ch == '0' {
                    trails.push(Point {x: x as i32, y: y as i32});
                }
            }
        }

        let size_y = array.len();
        let size_x = array[0].len();

        let maze = Maze {
            size_y,
            size_x,
            array,
        };

        return Trailmap {
            maze,
            trails
        }
    }

    fn run_trailhead(&mut self, start: Point, position: u64, visited: &mut Vec<Vec<bool>>) -> u64 {
        if start.x < 0 || start.x >= self.maze.size_x as i32 ||
           start.y < 0 || start.y >= self.maze.size_y as i32 {
            return 0;
        }

        if visited[start.y as usize][start.x as usize] {
            return 0;
        }
        visited[start.y as usize][start.x as usize] = true;
    

        let current_value = self.maze.array[start.y as usize][start.x as usize].to_digit(10).unwrap() as u64;

        if current_value == 9 {
            return 1;
        }

        if current_value != position {
            return 0;
        }

        let mut score = 0;
        for direction in Point::DIRECTIONS {
            let new_start = start.clone() + direction;

            if new_start.x < 0 || new_start.x >= self.maze.size_x as i32 ||
               new_start.y < 0 || new_start.y >= self.maze.size_y as i32 {
                continue;
            }

            if self.maze.array[new_start.y as usize][new_start.x as usize].to_digit(10).unwrap() == position as u32 + 1 {
                score += self.run_trailhead(new_start, position+1, visited);
            }
        }

        return score;
    }

    fn run_trailhead_rating(&mut self, start: Point, position: u64, score: u64) -> u64 {
        if start.x < 0 || start.x >= self.maze.size_x as i32 ||
           start.y < 0 || start.y >= self.maze.size_y as i32 {
            return score;
        }
    
        let current_value = self.maze.array[start.y as usize][start.x as usize].to_digit(10).unwrap() as u64;
    
        if current_value == 9 {
            return score + 1;
        }
    
        if current_value != position {
            return score;
        }
    
        let mut new_score = score;
        for direction in Point::DIRECTIONS {
            let new_start = start.clone() + direction;
    
            if new_start.x < 0 || new_start.x >= self.maze.size_x as i32 ||
               new_start.y < 0 || new_start.y >= self.maze.size_y as i32 {
                continue;
            }
    
            if self.maze.array[new_start.y as usize][new_start.x as usize].to_digit(10).unwrap() == position as u32 + 1 {
                new_score = self.run_trailhead_rating(new_start, position + 1, new_score);
            }
        }
    
        return new_score; // Return the accumulated score
    }

    fn run_map(&mut self) -> u64 {
        let mut score_sum: u64 = 0;
    
        for trail in self.trails.clone().iter() {
            score_sum += self.run_trailhead(trail.clone(), 0, &mut vec![vec![false; self.maze.size_x]; self.maze.size_y]);
        }
    
        return score_sum;
    }

    fn run_map_rating(&mut self) -> u64 {
        let mut score_sum: u64 = 0;
    
        for trail in self.trails.clone().iter() {
            score_sum += self.run_trailhead_rating(trail.clone(), 0, 0);
        }
    
        return score_sum;
    }
}


fn solve_day(file_contents: &str) -> (u64, u64) {
    /*
        Part 1
     */
    let mut trailmap = Trailmap::from(file_contents);

    let sum_part1: u64 = trailmap.run_map();
    let sum_part2: u64 = trailmap.run_map_rating();
    
    (sum_part1, sum_part2)
}


pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    let results = solve_day(&file_contents);
    (format!("{}", results.0), format!("{}", results.1))
}
