/*
Friday the 13th - Oh shit oh fuck
*/

fn solve_day(_file_contents: &str) -> (u32, u32) {
    /*
        Solve day
     */
    let mut sum_part1: u32 = 0;
    let mut sum_part2: u32 = 0;
    sum_part1 += 0;
    sum_part2 += 0;
    
    (sum_part1, sum_part2)
}


pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    let results = solve_day(&file_contents);
    (format!("{}", results.0), format!("{}", results.1))
}
