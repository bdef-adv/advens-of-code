use crate::classes::{Point, Maze};

type Point32 = Point<i32>;
type MazeChar = Maze<char>;


fn part01(file_contents: &str) -> u64 {
    /*
        Part 1
     */
    let mut sum: u64 = 0;

    let maze = MazeChar::from(file_contents);
    for y in 0..maze.size_y {
        for x in 0..maze.size_x {
            let pos = Point32::from(x as i32, y as i32);
            if maze.get(&pos) == Some('@') && maze.count_neighbors(&pos, &'@') < 4 {
                sum += 1;
            }
        }
    }

    return sum;
}


fn part02(file_contents: &str) -> u64 {
    /*
        Part 2
     */
    let mut sum: u64 = 0;

    let mut maze = MazeChar::from(file_contents);

    loop {
        let mut removed: Vec<Point32> = Vec::with_capacity(maze.size_x * maze.size_y);
        let mut current_sum: u64 = 0;

        for y in 0..maze.size_y {
            for x in 0..maze.size_x {
                let pos = Point32::from(x as i32, y as i32);
                if maze.get(&pos) == Some('@') && maze.count_neighbors(&pos, &'@') < 4 {
                    removed.push(pos);
                    current_sum += 1;
                }
            }
        }

        sum += current_sum;
        if current_sum == 0 {
            break;
        }

        for pos in removed {
            maze.set(&pos, '.');
        }
    }

    return sum;
}


pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    (format!("{}", part01(&file_contents)), format!("{}", part02(&file_contents)))
}