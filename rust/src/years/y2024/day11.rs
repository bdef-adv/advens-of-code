fn split_number(n: u64) -> (u64, u64) {
    // Step 1: Find the number of digits in `n`
    let mut temp = n;
    let mut digits = 0;
    while temp > 0 {
        digits += 1;
        temp /= 10;
    }

    // If `n` is zero, just return (0, 0)
    if digits == 0 {
        return (0, 0);
    }

    // Step 2: Calculate the divisor for splitting the number
    let divisor = 10_u64.pow((digits / 2) as u32);

    // Step 3: Split the number into left and right parts
    let left = n / divisor;  // Left half
    let right = n % divisor; // Right half

    (left, right)
}

fn has_even_length(n: u64) -> bool {
    // Step 1: Find the number of digits in `n`
    let mut temp = n;
    let mut digits = 0;
    while temp > 0 {
        digits += 1;
        temp /= 10;
    }

    // Step 2: Check if the number of digits is even
    digits % 2 == 0
}

fn blink_int(stones: &mut Vec<u64>) -> Vec<u64> {
    let mut new_stones: Vec<u64> = vec![];
    for &stone in stones.iter() {
        if stone == 0 {
            new_stones.push(1);
        } else if has_even_length(stone) {
            let (left, right) = split_number(stone);
            new_stones.push(left);
            new_stones.push(right);
        } else {
            new_stones.push(stone*2024)
        }
    }
    return new_stones;
}

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
    let mut stones: Vec<u64> = file_contents.split_whitespace().map(|s| s.parse().unwrap()).collect();

    for i in 0..25 {
        stones = blink_int(&mut stones);
        println!("blink {i}: {:?}", stones);
    }

    let sum_part1: usize = stones.len();

    println!("PART 1: {sum_part1}");

    for i in 0..50 {
        println!("Blink {}", i+26);
        stones = blink_int(&mut stones);
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
