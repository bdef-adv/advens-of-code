use crate::classes::{Maze, Point};
use itertools::Itertools;
use pathfinding::prelude::astar;

type Point32 = Point<i32>;
type MazeC = Maze<char>;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Vector {
    position: Point32,
    direction: Point32,
}

impl Vector {
    fn successors(&self, maze: &MazeC) -> Vec<(Vector, usize)> {
        let mut successors = vec![];

        for &direction in Point32::DIRECTIONS.iter() {
            let next_pos = self.position + direction;
            if let Some(ch) = maze.get(&next_pos) {
                if ch != ' ' {
                    let next_leaf = Vector {
                        position: next_pos,
                        direction,
                    };
                    let score = 1;
                    successors.push((next_leaf, score));
                }
            }
        }

        return successors;
    }
}

fn find_path(start: Point32, end: Point32, maze: &MazeC) -> Option<Vec<Vector>> {
    let vector = Vector {
        position: start,
        direction: Point32::default(),
    };
    if let Some(result) = astar(
        &vector,
        |p| p.successors(&maze),
        |_p| 0, //p.position.distance_from(&end),
        |p| p.position == end,
    ) {
        return Some(result.0[1..].to_vec());
    }

    return None;
}

#[allow(unused)]
fn get_direction_from_char(ch: char) -> Point32 {
    match ch {
        '^' => Point32::UP,
        'v' => Point32::DOWN,
        '>' => Point32::RIGHT,
        '<' => Point32::LEFT,
        'A' => Point32 { x: 0, y: 0 },
        ' ' => Point32 { x: -1, y: -1 },
        _ => Point32 { x: -15, y: -15 },
    }
}

fn get_char_from_direction(direction: Point32) -> char {
    match direction {
        Point32::UP => '^',
        Point32::DOWN => 'v',
        Point32::RIGHT => '>',
        Point32::LEFT => '<',
        Point32 { x: 0, y: 0 } => 'A',
        Point32 { x: -1, y: -1 } => ' ',
        _ => ' ',
    }
}

fn get_str_from_moves(moves: &Vec<Point32>) -> String {
    let mut result: String = String::new();

    for &direction in moves.iter() {
        result.push(get_char_from_direction(direction));
    }

    return result;
}

fn convert_input_to_int(combination: &str) -> usize {
    let s = combination.splitn(2, 'A').next().unwrap_or(combination);
    return s.parse().unwrap();
}

#[derive(Clone)]
struct Robot {
    position: Point32,
    maze: MazeC,
    moves: Vec<Point32>,
}

impl Robot {
    fn new(size_y: usize, size_x: usize, array_: Option<Vec<Vec<char>>>) -> Self {
        let array: Vec<Vec<char>>;
        if let Some(array__) = array_ {
            array = array__;
        } else {
            array = vec![vec![' '; size_x]; size_y];
        }

        Robot {
            position: Point32::default(),
            maze: MazeC {
                size_y,
                size_x,
                array,
            },
            moves: vec![],
        }
    }

    fn is_path_ok(&self, moves: &Vec<Point32>, start_position: &Point32) -> bool {
        let mut pos = start_position.clone();
        for &direction in moves.iter() {
            pos = pos + direction;
            if self.maze.get(&pos) == Some(' ') || self.maze.get(&pos) == None {
                return false;
            }
        }
        return true;
    }

    fn propagate(&mut self, moves: &Vec<Point32>, depth: usize) {
        let _indent = "  ".repeat(depth);
        for &direction in moves.iter() {
            let next_char = get_char_from_direction(direction);
            let end_position: Point<i32> = self.maze.find_first(next_char).unwrap();
            let mut next_moves = vec![];
            if let Some(path) = find_path(self.position, end_position, &self.maze) {
                next_moves.extend(path.iter().map(|x| x.direction));

                if next_moves.is_empty() {
                    self.position = end_position;
                    self.moves.push(Point32 { x: 0, y: 0 });
                    continue;
                }

                let next_moves_len = next_moves.len();
                let mut best_moves = next_moves.clone();

                //println!("{_indent}Found moves: {}", get_str_from_moves(&next_moves));

                if depth < 2 {
                    // Check if moves are in correct order
                    let mut new_robot = Robot::new(2, 3, None);
                    new_robot.maze.set(&Point32 { x: 2, y: 0 }, 'A');
                    new_robot.maze.set(&Point32 { x: 1, y: 0 }, '^');
                    new_robot.maze.set(&Point32 { x: 0, y: 1 }, '<');
                    new_robot.maze.set(&Point32 { x: 1, y: 1 }, 'v');
                    new_robot.maze.set(&Point32 { x: 2, y: 1 }, '>');
                    new_robot.position.y = 0;
                    new_robot.position.x = 2;

                    let mut shortest_move = usize::MAX;

                    let permutations = next_moves.into_iter().permutations(next_moves_len);
                    for perm in permutations {
                        if !self.is_path_ok(&perm, &self.position)
                            || !new_robot.is_path_ok(&perm, &new_robot.position)
                        {
                            continue;
                        }

                        //println!("{_indent}Checking permutation: {}", get_str_from_moves(&perm));
                        new_robot.propagate(&perm, depth + 1);
                        if new_robot.moves.len() <= shortest_move {
                            shortest_move = new_robot.moves.len();
                            best_moves = perm.clone();
                        }

                        new_robot.moves.clear();
                        new_robot.position.y = 0;
                        new_robot.position.x = 2;
                    }
                }

                //println!("{_indent}Best moves: {}", get_str_from_moves(&best_moves));

                self.position = end_position;
                self.moves.extend(best_moves);
                self.moves.push(Point32 { x: 0, y: 0 });
            }
        }
    }

    fn find_number(&mut self, number: char, depth: usize) {
        let _indent = "  ".repeat(depth);
        let end_position: Point<i32> = self.maze.find_first(number).unwrap();
        let mut next_moves = vec![];
        if let Some(path) = find_path(self.position, end_position, &self.maze) {
            next_moves.extend(path.iter().map(|x| x.direction));

            if next_moves.is_empty() {
                self.position = end_position;
                self.moves.push(Point32 { x: 0, y: 0 });
                return;
            }

            let next_moves_len = next_moves.len();
            let mut best_moves = next_moves.clone();

            let mut new_robot = Robot::new(2, 3, None);
            new_robot.maze.set(&Point32 { x: 2, y: 0 }, 'A');
            new_robot.maze.set(&Point32 { x: 1, y: 0 }, '^');
            new_robot.maze.set(&Point32 { x: 0, y: 1 }, '<');
            new_robot.maze.set(&Point32 { x: 1, y: 1 }, 'v');
            new_robot.maze.set(&Point32 { x: 2, y: 1 }, '>');
            new_robot.position.y = 0;
            new_robot.position.x = 2;

            let mut shortest_move = usize::MAX;

            let permutations = next_moves.into_iter().permutations(next_moves_len);
            for perm in permutations {
                if !self.is_path_ok(&perm, &self.position) {
                    continue;
                }

                new_robot.propagate(&perm, depth + 1);

                if new_robot.moves.len() < shortest_move {
                    shortest_move = new_robot.moves.len();
                    best_moves = perm.clone();
                }

                new_robot.moves.clear();
            }

            self.position = end_position;
            self.moves.extend(best_moves);
            self.moves.push(Point32 { x: 0, y: 0 });
        }
    }

    #[allow(unused)]
    fn print(&self) {
        println!("Position: {}", self.position);
        self.maze.print();
    }
}

struct Keypad {
    remotes: Vec<Robot>,
}

impl Keypad {
    fn from(robots: usize) -> Keypad {
        let mut remotes: Vec<Robot> = vec![];

        for _i in 0..robots - 1 {
            let mut new_robot = Robot::new(2, 3, None);
            new_robot.maze.set(&Point32 { x: 2, y: 0 }, 'A');
            new_robot.maze.set(&Point32 { x: 1, y: 0 }, '^');
            new_robot.maze.set(&Point32 { x: 0, y: 1 }, '<');
            new_robot.maze.set(&Point32 { x: 1, y: 1 }, 'v');
            new_robot.maze.set(&Point32 { x: 2, y: 1 }, '>');
            new_robot.position.y = 0;
            new_robot.position.x = 2;

            remotes.push(new_robot);
        }

        let mut new_robot = Robot::new(4, 3, None);
        new_robot.maze.set(&Point32 { x: 2, y: 3 }, 'A');
        new_robot.maze.set(&Point32 { x: 1, y: 3 }, '0');
        new_robot.maze.set(&Point32 { x: 0, y: 2 }, '1');
        new_robot.maze.set(&Point32 { x: 1, y: 2 }, '2');
        new_robot.maze.set(&Point32 { x: 2, y: 2 }, '3');
        new_robot.maze.set(&Point32 { x: 0, y: 1 }, '4');
        new_robot.maze.set(&Point32 { x: 1, y: 1 }, '5');
        new_robot.maze.set(&Point32 { x: 2, y: 1 }, '6');
        new_robot.maze.set(&Point32 { x: 0, y: 0 }, '7');
        new_robot.maze.set(&Point32 { x: 1, y: 0 }, '8');
        new_robot.maze.set(&Point32 { x: 2, y: 0 }, '9');
        new_robot.position.y = 3;
        new_robot.position.x = 2;

        remotes.push(new_robot);

        return Keypad { remotes };
    }

    fn solve(&mut self, combination: &str) -> usize {
        let robots: usize = self.remotes.len();

        let mut remotes_ = self.remotes.clone();
        let main_keypad: &mut Robot = &mut self.remotes[robots - 1];

        let mut last_moves: Vec<Point32> = vec![];
        let mut next_moves;
        for number in combination.chars() {
            println!(
                "Finding {number} on keypad starting from position {}",
                main_keypad.position
            );
            main_keypad.find_number(number, 0);
            println!(
                "Main keypad moves: {} {}",
                get_str_from_moves(&main_keypad.moves),
                main_keypad.position
            );

            let mut robot_index: isize = robots as isize - 2;
            next_moves = main_keypad.moves.clone();
            while robot_index >= 0 {
                let next_robot: &mut Robot = &mut remotes_[robot_index as usize];
                next_robot.propagate(&next_moves, 1);

                println!(
                    "Next robot moves: {} {} (length={})",
                    get_str_from_moves(&next_robot.moves),
                    next_robot.position,
                    next_robot.moves.len()
                );

                next_moves = next_robot.moves.clone();
                robot_index -= 1;
                next_robot.moves.clear();
            }

            main_keypad.moves.clear();
            last_moves.extend(next_moves.iter());
            println!("");
        }

        println!("Last moves: {}", get_str_from_moves(&last_moves));
        println!(
            "Length: {} * input= {}\n",
            last_moves.len(),
            convert_input_to_int(combination)
        );

        return last_moves.len();
    }
}

fn solve_day(file_contents: &str) -> (usize, usize) {
    /*
       Solve day
    */
    let mut sum_part1: usize = 0;
    for line in file_contents.lines() {
        println!("{line}");
        let ncomb = convert_input_to_int(line);
        let mut keypad = Keypad::from(3);
        let result = keypad.solve(line);
        sum_part1 += result * ncomb;
    }

    let sum_part2: usize = 0;
    /*for line in file_contents.lines() {
        println!("{line}");
        let ncomb = convert_input_to_int(line);
        let mut keypad = Keypad::from(26);
        let result = keypad.solve(line);
        sum_part2 += result * ncomb;
    }*/

    (sum_part1, sum_part2)
}

pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
       Return this day's results as a tuple
    */
    let results = solve_day(&file_contents);
    (format!("{}", results.0), format!("{}", results.1))
}
