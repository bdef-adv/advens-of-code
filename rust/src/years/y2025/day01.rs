fn part01(file_contents: &str) -> u32 {
    /*
        Part 1
     */
    let mut sum: u32 = 0;

    let mut position: i32 = 50;
    for line in file_contents.lines() {
        let (direction, num_str) = line.split_at(1);
        let num: i32 = num_str.parse::<i32>().unwrap();

        position = (position + num * if direction == "R" {1} else {-1}).rem_euclid(100);
        sum += (position == 0) as u32;
    }

    return sum;
}


fn part02(file_contents: &str) -> i32 {
    /*
        Part 2
     */
    let mut sum: i32 = 0;

    let mut position: i32 = 50;
    for line in file_contents.lines() {
        let (direction, num_str) = line.split_at(1);
        let num: i32 = num_str.parse::<i32>().unwrap();

        if direction == "L" && position - num <= 0 {
            sum += ((position - num) as f64 / 100.0).floor() as i32 * -1;
        } else if direction == "R" && position + num >= 100 {
            sum += ((position + num) as f64 / 100.0).floor() as i32;
        }

        position = (position + num * if direction == "R" {1} else {-1}).rem_euclid(100);
    }

    return sum;
}

pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    (format!("{}", part01(&file_contents)), format!("{}", part02(&file_contents)))
}