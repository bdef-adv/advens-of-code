use std::collections::HashSet;
use std::hash::{Hash, Hasher};

struct Direction {
    x: i32,
    y: i32
}

impl Direction {
    pub const RIGHT: Direction = Direction {x: 1, y: 0};
    pub const LEFT: Direction = Direction {x: -1, y: 0};
    pub const UP: Direction = Direction {x: 0, y: -1};
    pub const DOWN: Direction = Direction {x: 0, y: 1};
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Direction {}
impl Hash for Direction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
impl Clone for Direction {
    fn clone(&self) -> Self {
        Direction {
            x: self.x,
            y: self.y,
        }
    }
}


struct Guard {
    x: i32,
    y: i32,
    direction: Direction,
    visited: HashSet<(i32, i32)>,
    edge: bool,
    infinite: bool,
    rotations: HashSet<(i32, i32, Direction)>
}

impl Guard {
    fn advens(&mut self, maze: &mut Vec<Vec<char>>) {
        let new_x = self.x + self.direction.x;
        let new_y = self.y + self.direction.y;
        if new_x < 0 || new_y < 0 || new_x >= maze[0].len() as i32 || new_y >= maze.len() as i32 {
            self.edge = true;
            return;
        }
        if maze[new_y as usize][new_x as usize] == '#' {
            let rotation = (self.y, self.x, self.direction.clone());
            if self.rotations.contains(&rotation) {
                self.infinite = true;
            }
            self.rotations.insert(rotation);
            self.rotate();
        } else {
            self.visited.insert((new_y, new_x));
            self.x = new_x;
            self.y = new_y;
        }
    }

    fn rotate(&mut self) {
        if self.direction == Direction::UP {
            self.direction = Direction::RIGHT;
        } else if self.direction == Direction::RIGHT {
            self.direction = Direction::DOWN;
        } else if self.direction == Direction::DOWN {
            self.direction = Direction::LEFT;
        } else if self.direction == Direction::LEFT {
            self.direction = Direction::UP;
        }
    }
}


fn move_guard_through_maze(maze: &mut Vec<Vec<char>>, guard: &mut Guard) -> u32 {
    while !guard.edge {
        guard.advens(maze);
    }

    return guard.visited.len() as u32;
}


fn find_obstacles(maze: &mut Vec<Vec<char>>, guard: &mut Guard, init_position: &(i32, i32)) -> u32 {
    // Super slow, for each previously visited positions:
    // attempt to create obstacles and find out if rotations are repeated
    let mut sum: u32 = 0;    
    let visited = guard.visited.clone();
    for (visited_y, visited_x) in visited.iter() {
        maze[*visited_y as usize][*visited_x as usize] = '#';

        guard.x = init_position.0;
        guard.y = init_position.1;
        guard.infinite = false;
        guard.edge = false;
        guard.direction = Direction::UP;
        guard.rotations = HashSet::new();

        while !guard.edge && !guard.infinite {
            guard.advens(maze);
        }

        if guard.infinite {
            sum += 1;
        }

        maze[*visited_y as usize][*visited_x as usize] = '.';
    }

    return sum;
}


fn solve_day(file_contents: &str) -> (u32, u32) {
    /*
        Part 1
     */
    let mut maze: Vec<Vec<char>> = vec![];
    let mut guard: Guard = Guard {
        x: 0,
        y: 0,
        edge: false,
        direction: Direction::UP,
        visited: HashSet::new(),
        rotations: HashSet::new(),
        infinite: false
    };

    for (ln, line) in file_contents.lines().enumerate() {
        maze.push(Vec::new());
        for (cn, ch) in line.chars().enumerate() {
            maze[ln].push(ch);
            if ch == '^' {
                guard.x = cn as i32;
                guard.y = ln as i32;
            }
        }
    }

    guard.visited.insert((guard.x, guard.y));

    let init_position = (guard.x, guard.y);
    let sum_part1 = move_guard_through_maze(&mut maze, &mut guard);
    let sum_part2 = find_obstacles(&mut maze, &mut guard, &init_position);
    (sum_part1, sum_part2)
}


pub fn get_day_results(file_contents: &str) -> (u32, u32) {
    /*
        Return this day's results as a tuple
     */
    solve_day(&file_contents)
}