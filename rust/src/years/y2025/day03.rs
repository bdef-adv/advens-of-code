fn find_max_in_slice(a: &[u32]) -> (usize, u32) {
    let mut max: u32 = 0;
    let mut max_idx: usize = 0;
    for (x, &d) in a.iter().enumerate() {
        if d > max {
            max = d;
            max_idx = x;
        }
    }
    (max_idx, max)
}


fn find_max_joltage(bank: &str, length: usize) -> u64 {
    let digits: Vec<u32> = bank
                            .chars()
                            .map(|c| c.to_digit(10).unwrap())
                            .collect();

    let bank_len: usize = bank.len();

    let mut joltage = 0;
    let mut start = 0;

    for x in 0..length {
        let remaining: usize = length - x - 1;
        let end: usize = bank_len - remaining;

        let (max_idx, next_digit): (usize, u32) = find_max_in_slice(&digits[start..end]);

        joltage = joltage * 10 + next_digit as u64;
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