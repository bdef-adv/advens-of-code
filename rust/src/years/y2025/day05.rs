use std::collections::HashSet;


fn part01(file_contents: &str) -> usize {
    /*
        Part 1
     */
    let mut ingredients: HashSet<u128> = HashSet::with_capacity(50);
    let mut ranges: Vec<(u128, u128)> = Vec::with_capacity(50);

    for line in file_contents.lines() {
        if line.contains('-') {
            let parts: Vec<&str> = line.split('-').collect();
            ranges.push((parts[0].parse().unwrap(), parts[1].parse().unwrap()));
        } else if !line.is_empty() {
            let n: u128 = line.parse().unwrap();
            for &(start, end) in ranges.iter() {
                if start <= n && n <= end {
                    ingredients.insert(n);
                }
            }
        }
    }

    ingredients.len()
}


fn merge_overlaps(ranges: &mut Vec<(u128, u128)>) -> Vec<(u128, u128)> {
    ranges.sort_by_key(|&(start, _)| start);

    let mut merged: Vec<(u128, u128)> = Vec::with_capacity(ranges.len());
    let mut current: (u128, u128) = ranges[0];

    for &(start, end) in ranges[1..].iter() {
        if start <= current.1 {
            current.1 = current.1.max(end);
        } else {
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);

    merged
}


fn part02(file_contents: &str) -> u128 {
    /*
        Part 2
     */
    let mut sum: u128 = 0;
    let mut ranges: Vec<(u128, u128)> = Vec::with_capacity(50);

    for line in file_contents.lines() {
        if line.contains('-') {
            let parts: Vec<&str> = line.split('-').collect();
            ranges.push((parts[0].parse().unwrap(), parts[1].parse().unwrap()));
        }
    }

    let ranges: Vec<(u128, u128)> = merge_overlaps(&mut ranges);

    for &(start, end) in ranges.iter() {
        sum += (end-start) + 1;
    }

    sum
}


pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    (format!("{}", part01(&file_contents)), format!("{}", part02(&file_contents)))
}