fn split_number(n: u64) -> (u64, u64) {
    let mut temp = n;
    let mut digits = 0;
    while temp > 0 {
        digits += 1;
        temp /= 10;
    }

    if digits == 0 {
        return (0, 0);
    }
    let divisor = 10_u64.pow((digits / 2) as u32);

    let left = n / divisor;
    let right = n % divisor;

    (left, right)
}

fn has_even_length(n: u64) -> bool {
    let mut temp = n;
    let mut digits = 0;
    while temp > 0 {
        digits += 1;
        temp /= 10;
    }

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

fn blink_int_2(stones: &mut Vec<u64>) {
    let stones_len: usize = stones.len();
    
    for index in 0..stones_len {
        let stone = stones[index];
        if stone == 0 {
            stones[index] = 1;
        } else if has_even_length(stone) {
            let (left, right) = split_number(stone);
            stones[index] = left;
            stones.push(right);
        } else {
            stones[index] = stone*2024;
        }
    }
}


fn solve_day(file_contents: &str) -> (usize, usize) {
    /*
        Part 1
     */
    let mut stones: Vec<u64> = file_contents.split_whitespace().map(|s| s.parse().unwrap()).collect();

    for i in 0..25 {
        blink_int_2(&mut stones);
        println!("blink {i}: {:?}", stones);
    }

    let sum_part1: usize = stones.len();

    println!("PART 1: {sum_part1}");

    for i in 0..50 {
        println!("Blink {}", i+26);
        blink_int_2(&mut stones);
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
