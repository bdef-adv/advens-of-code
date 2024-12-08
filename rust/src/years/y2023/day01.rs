use std::collections::HashMap;

fn part01(file_contents: &str) -> u32 {
    /*
        Part 1
     */
    let mut sum: u32 = 0;
    for line in file_contents.lines() {
        let mut first_digit: u8 = 0;
        let mut last_digit: u8 = 0;
        for cur_char in line.chars() {
            if cur_char.is_digit(10) {
                if first_digit == 0 {
                    first_digit = cur_char.to_digit(10).unwrap() as u8;
                }
                last_digit = cur_char.to_digit(10).unwrap() as u8;
            }
        }
        let number  = 10 * first_digit + last_digit;
        sum += number as u32;
    }
    return sum;
}


fn buffer_digits(buffer: &String) -> Vec<u8> {
    /*
        Get alpha digits from buffer
     */
    let digits: HashMap<&str, u8> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    let mut results: Vec<u8> = Vec::new();
    let buffer_len: u8 = buffer.len() as u8;
    for index in 0..buffer_len {
        for size in 3..6 {
            if index + size > buffer_len {
                continue;
            }
            let cur_slice = &buffer[index as usize..(index+size) as usize];
            if digits.contains_key(&cur_slice) {
                let digit = digits.get(&cur_slice).unwrap();
                results.push(*digit);
            }
        }
    }

    results
}


fn part02(file_contents: &str) -> u32 {
    /*
        Part 2
     */
    let mut sum: u32 = 0;
    for line in file_contents.lines() {
        let mut first_digit: u8 = 0;
        let mut last_digit: u8 = 0;
        let mut buffer: String = "".to_string();
        for cur_char in line.chars() {
            if cur_char.is_digit(10) {
                if first_digit == 0 {
                    first_digit = cur_char.to_digit(10).unwrap().try_into().unwrap();
                }
                last_digit = cur_char.to_digit(10).unwrap().try_into().unwrap();
                buffer = "".to_string();
            } else {
                buffer += &cur_char.to_string();
                let digits: Vec<u8> = buffer_digits(&buffer);
                let digits_len = digits.len();
                if digits_len > 0 {
                    if first_digit == 0 {
                        first_digit = digits[0];
                    }
                    last_digit = digits[digits_len - 1];
                }
            }
        }
        let number  = 10 * first_digit + last_digit;
        sum += number as u32;
    }
    return sum;
}

pub fn get_day_results(file_contents: &str) -> (u64, u64) {
    /*
        Return this day's results as a tuple
     */
    (part01(&file_contents).try_into().unwrap(), part02(&file_contents).try_into().unwrap())
}