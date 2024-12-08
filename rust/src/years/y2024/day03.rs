use regex::Regex;

fn part01(file_contents: &str) -> u32 {
    /*
        Part 1
     */
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum: u32 = 0;

    for (_, [left, right]) in re_mul.captures_iter(file_contents).map(|c| c.extract()) {
        sum += left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
    }

    return sum;
}


fn part02(file_contents: &str) -> u32 {
    /*
        Part 2
     */
    let mut sum: u32 = 0;
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut disabled_buffer = String::from("");
    let mut buffer = String::from("");
    let mut enabled = true;
    for char in file_contents.chars() {
        if enabled {
            buffer.push(char);
        } else {
            disabled_buffer.push(char);
        }
        if buffer.ends_with("don't()") {
            enabled = false;
        }
        if disabled_buffer.ends_with("do()") {
            enabled = true;
            disabled_buffer = String::from("");
        }
    }

    for (_, [left, right]) in re_mul.captures_iter(buffer.as_str()).map(|c| c.extract()) {
        sum += left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
    }

    return sum;
}


pub fn get_day_results(file_contents: &str) -> (u32, u32) {
    /*
        Return this day's results as a tuple
     */
    (part01(&file_contents).try_into().unwrap(), part02(&file_contents).try_into().unwrap())
}