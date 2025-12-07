use crate::classes::{Point, Maze};
use std::collections::HashSet;

type Point32 = Point<i32>;
type MazeChar = Maze<char>;

const DIRECTION_DOWN: Point32 = Point32 {x: 0, y: 1};
const DIRECTION_RIGHT: Point32 = Point32 {x: 1, y: 0};
const DIRECTION_LEFT: Point32 = Point32 {x: -1, y: 0};


fn part01(file_contents: &str) -> usize {
    /*
        Part 1
     */
    let mut sum: usize = 0;
    sum += 0;

    let mut maze = MazeChar::from(file_contents);
    let mut tachyons: Vec<Point32> = vec![maze.find_first('S').unwrap()];

    while !tachyons.iter().all(|t| t.y == maze.size_y as i32 - 1) {
        let mut next_tachyons: HashSet<Point32> = HashSet::with_capacity(tachyons.len()*2);
        for &tachyon in tachyons.iter()  {
            let next_position = tachyon + DIRECTION_DOWN;
            if let Some(next_char) = maze.get(&next_position) {
                match next_char {
                    '.' => {
                        next_tachyons.insert(next_position);
                        maze.set(&next_position, '|');
                    }
                    '^'  => {
                        next_tachyons.insert(next_position + DIRECTION_LEFT);
                        next_tachyons.insert(next_position + DIRECTION_RIGHT);
                        maze.set(&(next_position + DIRECTION_LEFT), '|');
                        maze.set(&(next_position + DIRECTION_RIGHT), '|');
                        sum += 1;
                    }
                    _ => {}
                }
            }
        }

        tachyons = next_tachyons.iter().map(|t| t.to_owned()).collect();
    }

    sum
}


fn part02(_file_contents: &str) -> usize {
    /*
        Part 2
     */
    let mut sum: usize = 0;
    sum += 0;

    sum
}


pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    (format!("{}", part01(&file_contents)), format!("{}", part02(&file_contents)))
}