fn is_id_invalid_part1(id: u64) -> u64 {
    let id_str = id.to_string();
    let len = id_str.len();
    if len % 2 != 0 {
        return 0;
    }

    let (left, right) = id_str.split_at(len / 2);

    if left == right {
        return id;
    } else {
        return 0;
    }
}


fn is_id_invalid_part2(id: u64) -> u64 {
    let id_str = id.to_string();
    let len = id_str.len();

    for splice_len in 1..(len/2)+1 {
        if len % splice_len != 0 {
            continue;
        }

        let splice = &id_str[0..splice_len];
        let repeated = splice.repeat(len / splice_len);
        if repeated == id_str {
            return id;
        }
    }

    0
}


fn part01(file_contents: &str) -> u64 {
    /*
        Part 1
     */
    let mut sum: u64 = 0;

    for range_str in file_contents.split(',') {
        let (start, end): (u64, u64) = range_str
            .split_once("-")
            .map(|(s, e)| (s.parse().unwrap(), e.parse().unwrap()))
            .unwrap();

        for n in start..end+1 {
            sum += is_id_invalid_part1(n);
        }
    }

    sum
}


fn part02(file_contents: &str) -> u64 {
    /*
        Part 2
     */
    let mut sum: u64 = 0;

    for range_str in file_contents.split(',') {
        let (start, end): (u64, u64) = range_str
            .split_once("-")
            .map(|(s, e)| (s.parse().unwrap(), e.parse().unwrap()))
            .unwrap();

        for n in start..end+1 {
            sum += is_id_invalid_part2(n);
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
