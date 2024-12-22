use regex::Regex;

fn towel_ok(towel: &str, re: &Regex) -> bool {
    return re.is_match(towel);
}

static mut AVAILABLE: Vec<String> = vec![];

fn solve_day(file_contents: &str) -> (u32, u32) {
    /*
        Solve day
     */
    let mut models_to_try: Vec<&str> = vec![];
    let mut models_passed = false;
    for line in file_contents.lines() {
        if line.is_empty() {
            models_passed = true;
            continue;
        }
        if !models_passed {
            unsafe {
                AVAILABLE = line.split(", ").collect::<Vec<&str>>().iter().map(|s| s.to_string()).collect();
            }
        } else {
            models_to_try.push(line);
        }
    }

    let mut sum_part1: u32 = 0;
    let sum_part2: u32 = 0;
    unsafe {
        let re_pat = format!("^({})+$", AVAILABLE.join("|"));
        let re = Regex::new(&re_pat).unwrap();

        for &model in models_to_try.iter() {
            if towel_ok(model, &re) {
                sum_part1 += 1;
            }
        }
    }
    
    (sum_part1, sum_part2)
}


pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    let results = solve_day(&file_contents);
    (format!("{}", results.0), format!("{}", results.1))
}
