use crate::classes::{Point,Maze};
use std::fmt;

type Point32 = Point<i32>;

fn direction_from_char(ch: char) -> Point32 {
    match ch {
        '^' => Point32::UP,
        '>' => Point32::RIGHT,
        'v' => Point32::DOWN,
        '<' => Point32::LEFT,
        _ => Point32 {x: 0, y: 0}
    }
}

#[derive(PartialEq,Eq,Clone,Copy)]
enum Thing {
    Box,
    Wall,
    Robot,
    BoxLeft,
    BoxRight,
    Empty
}

impl fmt::Display for Thing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Thing::Box => write!(f, "O"),
            Thing::Wall => write!(f, "#"),
            Thing::Robot => write!(f, "@"),
            Thing::BoxLeft => write!(f, "["),
            Thing::BoxRight => write!(f, "]"),
            Thing::Empty => write!(f, "."),
        }
    }
}

impl Thing {
    fn from(ch: char) -> Thing {
        match ch {
            '@' => Thing::Robot,
            '#' => Thing::Wall,
            'O' => Thing::Box,
            '[' => Thing::BoxLeft,
            ']' => Thing::BoxRight,
            _ => Thing::Empty
        }
    }
}

type MazeThing = Maze<Thing>;

struct Warehouse {
    maze: MazeThing,
    robot: Point32,
    directions: Vec<Point32>
}

impl Warehouse {
    fn from(input: &str) -> Self {
        let mut array: Vec<Vec<Thing>> = vec![];
        let mut directions: Vec<Point32> = vec![];

        let mut robot: Point32 = Point32::default();

        let mut is_directions = false;
        for (y, line) in input.lines().enumerate() {
            if line.is_empty() {
                is_directions = true;
                continue;
            }

            if !is_directions {
                array.push(vec![]);
                for (x, ch) in line.chars().enumerate() {
                    let thing = Thing::from(ch);
                    if thing == Thing::Robot {
                        robot.y = y as i32;
                        robot.x = x as i32;
                    }
                    array[y].push(thing);
                }
            } else {
                for ch in line.chars() {
                    directions.push(direction_from_char(ch));
                }
            }
        }

        let size_y = array.len();
        let size_x = array[0].len();

        let maze =  MazeThing {
            size_y,
            size_x,
            array,
        };

        Warehouse {
            maze,
            directions,
            robot,
        }
    }

    fn move_boxes(&mut self, box_pos: Point32, direction: Point32) -> bool {
        let next_pos = box_pos + direction;

        if let Some(thing) = self.maze.get(&next_pos) {
            match thing {
                Thing::Empty => {
                    self.maze.array[box_pos.y as usize][box_pos.x as usize] = Thing::Empty;
                    self.maze.array[next_pos.y as usize][next_pos.x as usize] = Thing::Box;
                    return true;
                },
                Thing::Box => {
                    if self.move_boxes(next_pos, direction) {
                        self.maze.array[box_pos.y as usize][box_pos.x as usize] = Thing::Empty;
                        self.maze.array[next_pos.y as usize][next_pos.x as usize] = Thing::Box;
                        return true;
                    } else {
                        return false;
                    }
                },
                _ => {
                    return false;
                }
            }
        }

        return false;
    }

    fn move_one_step(&mut self, direction: Point32) -> bool {
        let new_pos = self.robot + direction;

        if let Some(thing) = self.maze.get(&new_pos) {
            match thing {
                Thing::Empty => {
                    self.maze.array[self.robot.y as usize][self.robot.x as usize] = Thing::Empty;
                    self.maze.array[new_pos.y as usize][new_pos.x as usize] = Thing::Robot;
                    self.robot = new_pos;
                },
                Thing::Box => {
                    if self.move_boxes(new_pos, direction) {
                        self.maze.array[self.robot.y as usize][self.robot.x as usize] = Thing::Empty;
                        self.maze.array[new_pos.y as usize][new_pos.x as usize] = Thing::Robot;
                        self.robot = new_pos;
                    }
                },
                _ => {}
            }
        }

        return false;
    }

    fn robot_go(&mut self) {
        for &direction in self.directions.clone().iter() {
            self.move_one_step(direction);
            //println!("");
            //self.maze.print();
        }
    }

    fn gps_score(&self) -> u32 {
        let mut score: u32 = 0;

        for (y, line) in self.maze.array.iter().enumerate() {
            for (x, &thing) in line.iter().enumerate() {
                if thing == Thing::Box {
                    score += 100 * y as u32 + x as u32;
                }
            }
        }

        return score;
    }
}

struct WarehouseDeBatard {
    maze: MazeThing,
    robot: Point32,
    directions: Vec<Point32>
}

impl WarehouseDeBatard {
    fn from(input: &str) -> Self {
        let mut array: Vec<Vec<Thing>> = vec![];
        let mut directions: Vec<Point32> = vec![];

        let mut robot: Point32 = Point32::default();

        let mut is_directions = false;
        for (y, line) in input.lines().enumerate() {
            if line.is_empty() {
                is_directions = true;
                continue;
            }

            if !is_directions {
                array.push(vec![]);
                for ch in line.chars() {
                    let thing = Thing::from(ch);

                    match thing {
                        Thing::Robot => {
                            array[y].push(thing);
                            array[y].push(Thing::Empty);
                        },
                        Thing::Box => {
                            array[y].push(Thing::BoxLeft);
                            array[y].push(Thing::BoxRight);
                        },
                        _ => {
                            array[y].push(thing);
                            array[y].push(thing);
                        }
                    }
                }
            } else {
                for ch in line.chars() {
                    directions.push(direction_from_char(ch));
                }
            }
        }

        'outer: for (y, line) in array.iter().enumerate() {
            for (x, &thing) in line.iter().enumerate() {
                if thing == Thing::Robot {
                    robot.y = y as i32;
                    robot.x = x as i32;
                    break 'outer;
                }
            }
        }

        let size_y = array.len();
        let size_x = array[0].len();

        let maze =  MazeThing {
            size_y,
            size_x,
            array,
        };

        WarehouseDeBatard {
            maze,
            directions,
            robot,
        }
    }

    fn move_boxes(&mut self, box_pos: Point32, direction: Point32) -> bool {
        let next_pos = box_pos + direction;

        if let Some(thing) = self.maze.get(&next_pos) {
            match thing {
                Thing::Empty => {
                    self.maze.array[box_pos.y as usize][box_pos.x as usize] = Thing::Empty;
                    self.maze.array[next_pos.y as usize][next_pos.x as usize] = Thing::Box;
                    return true;
                },
                Thing::BoxLeft | Thing::BoxRight => {
                    if self.move_boxes(next_pos, direction) {
                        self.maze.array[box_pos.y as usize][box_pos.x as usize] = Thing::Empty;
                        self.maze.array[next_pos.y as usize][next_pos.x as usize] = Thing::Box;
                        return true;
                    } else {
                        return false;
                    }
                },
                _ => {
                    return false;
                }
            }
        }

        return false;
    }

    fn move_one_step(&mut self, direction: Point32) -> bool {
        let new_pos = self.robot + direction;

        if let Some(thing) = self.maze.get(&new_pos) {
            match thing {
                Thing::Empty => {
                    self.maze.array[self.robot.y as usize][self.robot.x as usize] = Thing::Empty;
                    self.maze.array[new_pos.y as usize][new_pos.x as usize] = Thing::Robot;
                    self.robot = new_pos;
                },
                Thing::BoxLeft | Thing::BoxRight => {
                    if self.move_boxes(new_pos, direction) {
                        self.maze.array[self.robot.y as usize][self.robot.x as usize] = Thing::Empty;
                        self.maze.array[new_pos.y as usize][new_pos.x as usize] = Thing::Robot;
                        self.robot = new_pos;
                    }
                },
                _ => {}
            }
        }

        return false;
    }

    fn robot_go(&mut self) {
        for &direction in self.directions.clone().iter() {
            self.move_one_step(direction);
            println!("");
            self.maze.print();
        }
    }

    fn gps_score(&self) -> u32 {
        let mut score: u32 = 0;

        for (y, line) in self.maze.array.iter().enumerate() {
            for (x, &thing) in line.iter().enumerate() {
                if thing == Thing::Box {
                    score += 100 * y as u32 + x as u32;
                }
            }
        }

        return score;
    }
}


fn solve_day(file_contents: &str) -> (u32, u32) {
    /*
        Solve day
     */
    let mut warehouse = Warehouse::from(file_contents);
    //warehouse.maze.print();
    warehouse.robot_go();

    let sum_part1: u32 = warehouse.gps_score();

    let mut warehouse_de_batard = WarehouseDeBatard::from(file_contents);
    warehouse_de_batard.maze.print();
    warehouse_de_batard.robot_go();
    let sum_part2: u32 = warehouse_de_batard.gps_score();
    
    (sum_part1, sum_part2)
}


pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    let results = solve_day(&file_contents);
    (format!("{}", results.0), format!("{}", results.1))
}
