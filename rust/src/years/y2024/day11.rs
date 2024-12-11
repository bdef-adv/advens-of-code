fn blink(stones: &mut Vec<String>) -> Vec<String> {
    let mut new_stones: Vec<String> = vec![];
    for stone in stones {
        if stone == "0" {
            new_stones.push("1".to_string());
        } else if stone.len() % 2 == 0 {
            let (left, right) = stone.split_at(stone.len() / 2);
            let left_n: u64 = left.parse().unwrap();
            let right_n: u64 = right.parse().unwrap();
            new_stones.push(left_n.to_string());
            new_stones.push(right_n.to_string());
        } else {
            let stone_as_int: u64 = stone.parse().unwrap();
            new_stones.push((stone_as_int*2024).to_string());
        }
    }
    return new_stones;
}

fn solve_day(file_contents: &str) -> (usize, usize) {
    /*
        Part 1
     */
    let mut stones: Vec<String> = file_contents.split_whitespace().map(|s| s.to_string()).collect();

    for _ in 0..25 {
        stones = blink(&mut stones);
    }

    let sum_part1: usize = stones.len();

    for _ in 0..50 {
        stones = blink(&mut stones);
    }

    let sum_part2: usize = stones.len();
    
    (sum_part1, sum_part2)
}


pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    let results = solve_day(&file_contents);
    (format!("{}", results.0), format!("{}", results.1))
}
