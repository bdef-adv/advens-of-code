fn report_safe(line_values: &Vec<i32>) -> bool {
    let mut last_value: i32 = line_values[1];
    let desc: bool = line_values[1] < line_values[0];

    let mut diff: i32 = (line_values[0] - line_values[1]).abs();

    if diff > 3 || diff == 0 {
        return false;
    }

    for value in line_values[2..].iter() {
        if *value < last_value && !desc {
            return false;
        } else if *value > last_value && desc {
            return false;
        }

        diff = (*value - last_value).abs();

        if diff > 3 || diff == 0 {
            return false;
        }

        last_value = *value;
    }

    return true;
}


fn part01(file_contents: &str) -> u32 {
    /*
        Part 1
     */
    let mut sum: u32 = 0;

    for line in file_contents.lines() {
        let line_values: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        if report_safe(&line_values) {
            sum += 1
        }
    }

    return sum;
}


fn problem_dampener(line_values: &Vec<i32>) -> bool {
    let new_len = line_values.len() - 1;
    for index in 0..(new_len+1) {
        let new_values: Vec<i32> = line_values.iter().enumerate()
            .filter_map(|(i, &value)| {
                if i != index {
                    Some(value)
                } else {
                    None
                }
            })
            .collect();
        if report_safe(&new_values) {
            return true;
        }
    }

    return false;
}


fn part02(file_contents: &str) -> u32 {
    /*
        Part 2
     */
    let mut sum: u32 = 0;
    
    for line in file_contents.lines() {
        let line_values: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        if problem_dampener(&line_values) {
            sum += 1
        }
    }
    
    return sum;
}


pub fn get_day_results(file_contents: &str) -> (u64, u64) {
    /*
        Return this day's results as a tuple
     */
    (part01(&file_contents).try_into().unwrap(), part02(&file_contents).try_into().unwrap())
}