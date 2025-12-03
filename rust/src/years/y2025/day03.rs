fn find_max_in_slice(a: &str) -> (usize, char) {
    let mut max: char = '0';
    let mut max_idx: usize = 0;
    for (x, d) in a.chars().enumerate() {
        if d > max {
            max = d;
            max_idx = x;
        }
    }
    (max_idx, max)
}


fn find_max_joltage(bank: &str, length: usize) -> u64 {
    let bank_len: usize = bank.len();

    let mut joltage: u64 = 0;
    let mut start: usize = 0;

    for x in 0..length {
        let remaining: usize = length - x - 1;
        let end: usize = bank_len - remaining;

        let (max_idx, next_digit): (usize, char) = find_max_in_slice(&bank[start..end]);

        joltage = joltage * 10 + next_digit.to_digit(10).unwrap() as u64;
        start += max_idx + 1;
    }

    joltage
}


fn part01(file_contents: &str) -> u64 {
    /*
        Part 1
     */
    let mut sum: u64 = 0;

    for bank in file_contents.lines() {
        sum += find_max_joltage(bank, 2);
    }

    return sum;
}


fn part02(file_contents: &str) -> u64 {
    /*
        Part 2
     */
    let mut sum: u64 = 0;

    for bank in file_contents.lines() {
        sum += find_max_joltage(bank, 12);
    }

    return sum;
}


pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    (format!("{}", part01(&file_contents)), format!("{}", part02(&file_contents)))
}