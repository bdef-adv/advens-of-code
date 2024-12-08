fn part01(file_contents: &str) -> u32 {
    /*
        Part 1
     */
    let mut sum: u32 = 0;
    let mut left_map: Vec<i32> = Vec::new();
    let mut right_map: Vec<i32> = Vec::new();
    for line in file_contents.lines() {
        let line_values: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        left_map.push(line_values[0]);
        right_map.push(line_values[1]);
    }

    left_map.sort();
    right_map.sort();

    for (index, left_item) in left_map.iter().enumerate() {
        sum += (left_item - right_map[index]).abs() as u32
    }

    return sum;
}


fn occurrences(vector: &Vec<i32>, item: &i32) -> u16 {
    /*
        Find count of occurences of item in vector.
    */
    let mut count = 0;
    for vec_item in vector.iter() {
        if vec_item == item {
            count += 1
        }
        // vector needs to be sorted for this to work
        if vec_item > item {
            return count;
        }
    }
    count
}


fn part02(file_contents: &str) -> u32 {
    /*
        Part 2
     */
    let mut sum: u32 = 0;
    let mut left_map: Vec<i32> = Vec::new();
    let mut right_map: Vec<i32> = Vec::new();
    for line in file_contents.lines() {
        let line_values: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        left_map.push(line_values[0]);
        right_map.push(line_values[1]);
    }

    left_map.sort();
    right_map.sort();

    for left_item in left_map.iter() {
        let count = occurrences(&right_map, &left_item);
        sum += (left_item * count as i32) as u32;
    }

    return sum;
}

pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    (format!("{}", part01(&file_contents)), format!("{}", part02(&file_contents)))
}