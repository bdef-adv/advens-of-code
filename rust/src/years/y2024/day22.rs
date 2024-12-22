fn mix(secret_1: usize, secret_2: usize) -> usize {
    return secret_1 ^ secret_2;
}

fn prune(secret: usize) -> usize {
    return secret % 16777216;
}

fn next_secret(secret_number: usize) -> usize {
    let mut secret = secret_number;
    let mut next_secret = secret * 64;
    secret = mix(secret, next_secret);
    secret = prune(secret);

    next_secret = secret / 32;
    secret = mix(secret, next_secret);
    secret = prune(secret);

    next_secret = secret * 2048;
    secret = mix(secret, next_secret);
    secret = prune(secret);

    return secret;
}


fn solve_day(file_contents: &str) -> (usize, u32) {
    /*
        Solve day
     */
    let mut sum_part1: usize = 0;
    for line in file_contents.lines() {
        let mut secret: usize = line.parse().unwrap();
        
        for _i in 0..2000 {
            secret = next_secret(secret);
        }
        
        sum_part1 += secret;
    }

    let mut sum_part2: u32 = 0;
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
