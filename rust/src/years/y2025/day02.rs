fn is_id_invalid_part1(id: String) -> bool {
    let len = id.len();
    if len % 2 != 0 {
        return false;
    }

    let (left, right) = id.split_at(len / 2);

    return left == right;
}


fn is_id_invalid_part2(id: String) -> bool {
    let len = id.len();

    for splice_len in 1..(len/2)+1 {
        if len % splice_len != 0 {
            continue;
        }

        let splice = &id[0..splice_len];
        let repeated = splice.repeat(len / splice_len);
        if repeated == id {
            return true;
        }
    }

    false
}


fn part01(file_contents: &str) -> u64 {
    /*
        Part 1
     */
    let ranges_str: Vec<&str> = file_contents.split(',').collect();
    let mut sum: u64 = 0;

    for range_str in ranges_str {
        let parts: Vec<&str> = range_str.split("-").collect();
        let start: u64 = parts[0].parse::<u64>().unwrap();
        let end: u64 = parts[1].parse::<u64>().unwrap();

        for n in start..end+1 {
            if is_id_invalid_part1(n.to_string()) {
                sum += n;
            }
        }
    }

    sum
}


fn part02(file_contents: &str) -> u64 {
    /*
        Part 2
     */
    let ranges_str: Vec<&str> = file_contents.split(',').collect();
    let mut sum: u64 = 0;

    for range_str in ranges_str {
        let parts: Vec<&str> = range_str.split("-").collect();
        let start: u64 = parts[0].parse::<u64>().unwrap();
        let end: u64 = parts[1].parse::<u64>().unwrap();

        for n in start..end+1 {
            if is_id_invalid_part2(n.to_string()) {
                sum += n;
            }
        }
    }

    sum
}

pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    (format!("{}", part01(&file_contents)), format!("{}", part02(&file_contents)))
}