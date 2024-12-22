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

#[derive(PartialEq,Eq,Clone,Copy,Debug)]
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
                    self.maze.set(&box_pos, Thing::Empty);
                    self.maze.set(&next_pos, Thing::Box);
                    return true;
                },
                Thing::Box => {
                    if self.move_boxes(next_pos, direction) {
                        self.maze.set(&box_pos, Thing::Empty);
                        self.maze.set(&next_pos, Thing::Box);
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
                    self.maze.set(&self.robot, Thing::Empty);
                    self.maze.set(&new_pos, Thing::Robot);
                    self.robot = new_pos;
                },
                Thing::Box => {
                    if self.move_boxes(new_pos, direction) {
                        self.maze.set(&self.robot, Thing::Empty);
                        self.maze.set(&new_pos, Thing::Robot);
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

    fn move_boxes(&mut self, box_pos: Point32, direction: Point32, set: bool, depth: usize) -> bool {
        let cur_thing = self.maze.get(&box_pos).unwrap();
        let _indent= "  ".repeat(depth);

        if direction == Point32::LEFT || direction == Point32::RIGHT {
            let next_pos = box_pos + direction;
            if let Some(thing) = self.maze.get(&next_pos) {
                match thing {
                    Thing::Empty => {
                        if set {
                            self.maze.set(&box_pos, Thing::Empty);
                            self.maze.set(&next_pos, cur_thing);
                        }
                        return true;
                    },
                    Thing::BoxLeft | Thing::BoxRight => {
                        if self.move_boxes(next_pos, direction, true, depth+1) {
                            if set {
                                self.maze.set(&box_pos, Thing::Empty);
                                self.maze.set(&next_pos, cur_thing);
                            }
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
        } else {
            if cur_thing == Thing::Empty {
                if set {
                    self.maze.set(&box_pos, Thing::Empty);
                }
                return true;
            }

            let mut boxes_to_check: Vec<Point32> = vec![];
            boxes_to_check.push(box_pos);

            if cur_thing == Thing::BoxRight {
                boxes_to_check.push(box_pos + Point32::LEFT);
            } else if cur_thing == Thing::BoxLeft {
                boxes_to_check.push(box_pos + Point32::RIGHT);
            }

            for &cbox_pos in boxes_to_check.iter() {
                let next_pos = cbox_pos + direction;
                if !self.move_boxes(next_pos, direction, false, depth+1) {
                    return false;
                }
            }

            if !set {
                return true;
            }

            for &cbox_pos in boxes_to_check.iter() {
                let next_pos = cbox_pos + direction;
                self.move_boxes(next_pos, direction, true, depth+1);
            }
            return true;


            /*println!("{indent}{set} Thing is: {:?} at position {box_pos}", cur_thing);
            println!("{indent}{set} We want to move {cur_thing:?} to {next_pos}");
            let mut cur_box_otherside: Point32 = box_pos;
            let mut box_otherside_next_pos: Point32 = next_pos;
            if cur_thing == Thing::BoxRight {
                cur_box_otherside = cur_box_otherside + Point32::LEFT;
                box_otherside_next_pos = box_otherside_next_pos + Point32::LEFT;
                println!("{indent}{set} BoxLeft is at position {cur_box_otherside}");
                println!("{indent}{set} We want to move BoxLeft to {box_otherside_next_pos}");
            } else if cur_thing == Thing::BoxLeft {
                cur_box_otherside = cur_box_otherside + Point32::RIGHT;
                box_otherside_next_pos = box_otherside_next_pos + Point32::RIGHT;
                println!("{indent}{set} BoxRight is at position {cur_box_otherside}");
                println!("{indent}{set} We want to move BoxRight to {box_otherside_next_pos}");
            }

            let mut can_move_current = false;
            let mut can_move_otherside = cur_box_otherside == box_pos;

            if let Some(next_thing) = self.maze.get(&next_pos) {
                match next_thing {
                    Thing::Empty => {
                        can_move_current = true;
                    },
                    Thing::BoxLeft | Thing::BoxRight => {
                        can_move_current = self.move_boxes(next_pos, direction, false, depth+1);
                    },
                    _ => {
                        can_move_current = false;
                    }
                }
            }

            if !can_move_otherside {
                if let Some(next_thing) = self.maze.get(&box_otherside_next_pos) {
                    match next_thing {
                        Thing::Empty => {
                            can_move_otherside = true;
                        },
                        Thing::BoxLeft | Thing::BoxRight => {
                            can_move_otherside = self.move_boxes(box_otherside_next_pos, direction, false, depth+1);
                        },
                        _ => {
                            can_move_otherside = false;
                        }
                    }
                }
            }

            if !set {
                return can_move_current && can_move_otherside;
            } else if can_move_current && can_move_otherside {

            }*/

            /*if set {
                println!("{}Attempting to move {cur_thing} from position {box_pos} to {next_pos}", " ".repeat(depth));
            }
            if let Some(thing) = self.maze.get(&next_pos) {
                

                println!("{}Next position to check={next_pos} {thing}     set={set}", " ".repeat(depth));
                match thing {
                    Thing::Empty => {
                        println!("{}Empty so it's okay", " ".repeat(depth));
                        if set {
                            println!("{}Setting {box_pos} to . and {next_pos} to {cur_thing}", " ".repeat(depth));
                            self.maze.set(&box_pos, Thing::Empty);
                            self.maze.set(&next_pos, cur_thing);
                            if cur_thing == Thing::BoxRight {
                                let box_left = box_pos - Point32::LEFT;

                                println!("{}Setting {box_left} to . and {} to {}", " ".repeat(depth), next_pos + Point32::LEFT, Thing::BoxLeft);
                                self.maze.set(&box_left, Thing::Empty);
                                self.maze.set(&(next_pos + Point32::LEFT), Thing::BoxLeft);
                            } else if cur_thing == Thing::BoxLeft {
                                let box_right = box_pos - Point32::RIGHT;
                                println!("{}Setting {box_right} to . and {} to {}", " ".repeat(depth), next_pos + Point32::RIGHT, Thing::BoxRight);
                                self.maze.set(&box_right, Thing::Empty);
                                self.maze.set(&(next_pos + Point32::RIGHT), Thing::BoxRight);
                            }
                        }
                        return true;
                    },
                    Thing::BoxLeft => {
                        let next_box_right: Point32 = next_pos + Point32 {x: 1, y: 0};
                        println!("{}Next box is BoxLeft, have to check. BoxRight={next_box_right}", " ".repeat(depth));

                        if self.move_boxes(next_pos, direction, false, depth+1) &&
                           self.move_boxes(next_box_right, direction, false, depth+1) {
                            println!("{}Able to move whole box at position {box_pos}", " ".repeat(depth));
                            if set {
                                println!("{}Move BoxLeft from {next_pos}", " ".repeat(depth));
                                self.move_boxes(next_pos, direction, true, depth+1);
                                println!("{}Move BoxRight from {next_box_right}", " ".repeat(depth));
                                self.move_boxes(next_box_right, direction, true, depth+1);

                                println!("{}Setting {box_pos} and {} to .", " ".repeat(depth), box_pos + Point32 {x: 1, y: 0});
                                self.maze.set(&box_pos, Thing::Empty);
                                self.maze.set(&(box_pos + Point32 {x: 1, y: 0}), Thing::Empty);

                                println!("{}Setting {next_pos} to {thing} and {} to {}", " ".repeat(depth), next_pos + Point32 {x: 1, y: 0}, Thing::BoxRight);
                                self.maze.set(&next_pos, thing);
                                self.maze.set(&(next_pos + Point32 {x: 1, y: 0}), Thing::BoxRight);
                            }
                            return true;
                        } else {
                            return false;
                        }
                    },
                    Thing::BoxRight => {
                        let box_left = next_pos + Point32 {x: -1, y: 0};
                        println!("{}Next box is BoxRight, have to check. BoxLeft={box_left}", " ".repeat(depth));

                        if self.move_boxes(next_pos, direction, false, depth+1) &&
                           self.move_boxes(box_left, direction, false, depth+1) {
                            println!("{}Able to move whole box at position {box_pos}", " ".repeat(depth));
                            if set {
                                println!("{}Move BoxRight from {next_pos}", " ".repeat(depth));
                                self.move_boxes(next_pos, direction, true, depth+1);
                                println!("{}Move BoxLeft from {box_left}", " ".repeat(depth));
                                self.move_boxes(box_left, direction, true, depth+1);

                                println!("{}Setting {box_pos} and {} to .", " ".repeat(depth), box_pos + Point32 {x: -1, y: 0});
                                self.maze.set(&box_pos, Thing::Empty);
                                self.maze.set(&(box_pos + Point32 {x: -1, y: 0}), Thing::Empty);

                                println!("{}Setting {next_pos} to {thing} and {} to {}", " ".repeat(depth), next_pos + Point32 {x: -1, y: 0}, Thing::BoxLeft);
                                self.maze.set(&next_pos, thing);
                                self.maze.set(&(next_pos + Point32 {x: - 1, y: 0}), Thing::BoxLeft);
                            }
                            return true;
                        } else {
                            return false;
                        }
                    }
                    _ => {
                        return false;
                    }
                }
            }*/
        }
        
        return false;
    }

    fn move_one_step(&mut self, direction: Point32) -> bool {
        let new_pos = self.robot + direction;

        if let Some(thing) = self.maze.get(&new_pos) {
            match thing {
                Thing::Empty => {
                    self.maze.set(&self.robot, Thing::Empty);
                    self.maze.set(&new_pos, Thing::Robot);
                    self.robot = new_pos;
                },
                Thing::BoxLeft | Thing::BoxRight => {
                    if self.move_boxes(new_pos, direction, true, 0) {
                        self.maze.set(&self.robot, Thing::Empty);
                        self.maze.set(&new_pos, Thing::Robot);
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
            println!("Direction: {direction}");
            self.move_one_step(direction);
            self.maze.print();
        }
    }

    fn gps_score(&self) -> u32 {
        let mut score: u32 = 0;

        for (y, line) in self.maze.array.iter().enumerate() {
            for (x, &thing) in line.iter().enumerate() {
                if thing == Thing::BoxLeft {
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
