fn check_word(array: &Vec<Vec<char>>, word: &str, row: i16, col: i16, dir_y: i16, dir_x: i16) -> u32 {
    let length = word.len();
    let max_y = array.len();
    let max_x = array[0].len();
    let word_chars: Vec<char> = word.chars().collect();

    let mut y: i16 = row as i16;
    let mut x: i16 = col as i16;

    for i in 0..length {
        if y < 0 || y >= max_y as i16 || x < 0 || x >= max_x as i16 {
            return 0;
        }
        if array[y as usize][x as usize] != word_chars[i] {
            return 0
        }
        y += dir_y;
        x += dir_x;
    }

    return 1;
}


fn word_occurences(array: &Vec<Vec<char>>, word: &str) -> u32 {
    let mut sum: u32 = 0;

    let directions = [
        (0, 1),     // right
        (0, -1),    // left
        (1, 0),     // down
        (-1, 0),    // up
        (1, 1),     // down-right
        (1, -1),    // down-left
        (-1, 1),    // up-right
        (-1, -1),   // up-left
    ];

    for (y_index, y) in array.iter().enumerate() {
        for (x_index, _x) in y.iter().enumerate() {
            for (dy, dx) in directions {
                sum += check_word(&array, &word, y_index as i16, x_index as i16, dy, dx);
            }
        }
    }

    return sum;
}

fn cross_occurences(array: &Vec<Vec<char>>, word: &str) -> u32 {
    let mut sum: u32 = 0;

    let directions = [
        (1, 1),     // down-right
        (1, -1),    // down-left
        (-1, 1),    // up-right
        (-1, -1),   // up-left
    ];

    for (y_index, y) in array.iter().enumerate() {
        for (x_index, ch) in y.iter().enumerate() {
            if *ch != 'A' {
                continue;
            }
            let mut count_okay = 0;
            for (dy, dx) in directions {
                let new_y: i16 = y_index as i16 - dy;
                let new_x: i16 = x_index as i16 - dx;
                count_okay += check_word(&array, &word, new_y, new_x, dy, dx);
            }
            if count_okay >= 2 {
                sum += 1;
            }
        }
    }

    return sum;
}


fn solve_day(file_contents: &str) -> (u32, u32) {
    let mut array: Vec<Vec<char>> = vec![vec![]];

    let mut x: usize = 0;
    let mut y: usize = 0;
    for line in file_contents.lines() {
        for ch in line.chars() {
            if y >= array.len() {
                array.push(vec![]);
            }
            if x >= array[y].len() {
                array[y].push(ch);
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }

    let sum_part1 = word_occurences(&array, "XMAS");
    let sum_part2 = cross_occurences(&array, "MAS");

    (sum_part1, sum_part2)
}


pub fn get_day_results(file_contents: &str) -> (u32, u32) {
    /*
        Return this day's results as a tuple
     */
    solve_day(&file_contents)
}