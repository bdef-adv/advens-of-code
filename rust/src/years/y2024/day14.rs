use crate::classes::{Point,Maze};

type Point32 = Point<i32>;

struct Robot {
    position: Point32,
    velocity: Point32
}

impl Robot {
    fn next(&mut self, maze: &Maze) {
        let mut next_pos = self.position + self.velocity;
        if next_pos.x < 0 {
            next_pos.x = maze.size_x as i32 + next_pos.x;
        }
        if next_pos.y < 0 {
            next_pos.y = maze.size_y as i32 + next_pos.y;
        }
        if next_pos.x >= maze.size_x as i32 {
            next_pos.x = next_pos.x - maze.size_x as i32;
        }
        if next_pos.y >= maze.size_y as i32 {
            next_pos.y = next_pos.y - maze.size_y as i32;
        }
        self.position = next_pos;
    }
}

fn get_point_from_str(input: &str, split_str: &str) -> Point32 {
    let parts: Vec<&str> = input.split(split_str).collect();

    let x: i32 = parts[0].parse().unwrap();
    let y: i32 = parts[1].parse().unwrap();

    return Point32 {x, y};
}

struct Bathroom {
    maze: Maze,
    robots: Vec<Robot>
}

impl Bathroom {
    fn from(input: &str, size_x: usize, size_y: usize) -> Self {
        let array: Vec<Vec<char>> = vec![vec!['.'; size_x]; size_y];
        let maze = Maze {size_x, size_y, array};

        let mut robots: Vec<Robot> = vec![];

        for line in input.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let position = get_point_from_str(parts[0].trim_start_matches("p=").parse::<String>().unwrap().as_str(), ",");
            let velocity = get_point_from_str(parts[1].trim_start_matches("v=").parse::<String>().unwrap().as_str(), ",");
            robots.push(Robot {position, velocity});
        }

        return Bathroom {
            maze,
            robots
        }
    }

    fn elapse(&mut self, seconds: usize) {
        for _s in 0..seconds {
            for robot in self.robots.iter_mut() {
                robot.next(&self.maze);
            }
        }
    }

    fn elapse_tree(&mut self) -> u64 {
        let mut seconds: u64 = 0;
        loop {
            seconds += 1;
            for robot in self.robots.iter_mut() {
                robot.next(&self.maze);
            }
            self.reset_array();
            if self.maze.find_string("xxxxxxxxx") {
                self.maze.print();
                return seconds;
            }
        }
    }

    fn quadrant_from_position(&self, position: Point32) -> Option<usize> {
        let left = position.x < self.maze.size_x as i32 / 2;
        let top = position.y < self.maze.size_y as i32 / 2;
        if position.x == self.maze.size_x as i32 / 2 || position.y == self.maze.size_y as i32 / 2 {
            return None;
        }
        if left && top {
            return Some(0);
        } else if !left && top {
            return Some(1);
        } else if left && !top {
            return Some(2);
        } else if !left && !top {
            return Some(3);
        }
        return None;
    }

    fn reset_array(&mut self) {
        for y in 0..self.maze.size_y {
            for x in 0..self.maze.size_x {
                self.maze.set_char(&Point32 {x: x as i32, y: y as i32}, '.');
            }
        }

        for robot in self.robots.iter() {
            if let Some(ch) = self.maze.get_char(&robot.position) {
                match ch {
                    '.' => {
                        self.maze.set_char(&robot.position, 'x');
                    },
                    _ => {}
                }
            }
        }
    }

    #[allow(unused)]
    fn print(&mut self) {
        for y in 0..self.maze.size_y {
            for x in 0..self.maze.size_x {
                self.maze.set_char(&Point32 {x: x as i32, y: y as i32}, '.');
            }
        }

        for robot in self.robots.iter() {
            if let Some(ch) = self.maze.get_char(&robot.position) {
                match ch {
                    '.' => {
                        self.maze.set_char(&robot.position, '1');
                    },
                    '1' => {
                        self.maze.set_char(&robot.position, '2');
                    },
                    '2' => {
                        self.maze.set_char(&robot.position, '3');
                    },
                    '3' => {
                        self.maze.set_char(&robot.position, '4');
                    },
                    '4' => {
                        self.maze.set_char(&robot.position, '5');
                    },
                    '5' => {
                        self.maze.set_char(&robot.position, '6');
                    },
                    '6' => {
                        self.maze.set_char(&robot.position, '7');
                    },
                    '7' => {
                        self.maze.set_char(&robot.position, '8');
                    },
                    '8' => {
                        self.maze.set_char(&robot.position, '9');
                    },
                    '9' => {
                        self.maze.set_char(&robot.position, '*');
                    },
                    _ => {}
                }
            }
        }
        self.maze.print();
    }

    fn score_quadrants(&self) -> u64 {
        let mut quadrants: Vec<u64> = vec![0; 4];

        for robot in self.robots.iter() {
            if let Some(quadrant) = self.quadrant_from_position(robot.position) {
                quadrants[quadrant] += 1;
            }
        }

        return quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];
    }
}

fn solve_day(file_contents: &str) -> (u64, u64) {
    /*
        Solve day
     */
    let mut bathroom = Bathroom::from(file_contents, 101, 103);
    bathroom.elapse(100);
    let sum_part1 = bathroom.score_quadrants();
    let sum_part2: u64 = bathroom.elapse_tree() + 100;
    
    (sum_part1, sum_part2)
}


pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    let results = solve_day(&file_contents);
    (format!("{}", results.0), format!("{}", results.1))
}
