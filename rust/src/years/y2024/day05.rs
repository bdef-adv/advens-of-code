use std::collections::HashMap;

fn is_page_update_valid(ordering: &HashMap<u32, Vec<u32>>, page_update: &Vec<u32>) -> bool {
    let len = page_update.len();
    for (index_one, page_one) in page_update.iter().rev().enumerate() {
        if let Some(after) = ordering.get(&page_one) {
            for after_page in after.iter() {
                for page_two in page_update[0..len-index_one].iter() {
                    if *page_two == *after_page {
                        return false;
                    }
                }
            }
        }
    }

    return true;
}


fn get_middle_value(page_update: &Vec<u32>) -> u32 {
    let len = page_update.len();
    if len == 0 {
        return 0;
    }
    return page_update[(len as f32/2.0).floor() as usize];
}


fn fix_page_update_order(ordering: &HashMap<u32, Vec<u32>>, page_update: &Vec<u32>) -> Vec<u32> {
    let mut new_page_update: Vec<u32> = page_update.clone();
    let len = page_update.len();

    // on en est là, c'est degueulasse, salut la team
    while !is_page_update_valid(&ordering, &new_page_update) || new_page_update.len() == 0 {
        let mut errors: HashMap<u32, Vec<u32>> = HashMap::new(); 
        for (index_one, page_one) in new_page_update.iter().rev().enumerate() {
            if let Some(after) = ordering.get(&page_one) {
                for after_page in after.iter() {
                    for page_two in new_page_update[0..len-index_one].iter() {
                        if *page_two == *after_page {
                            errors.entry(*page_one).or_insert(Vec::new()).push(*page_two);
                        }
                    }
                }
            }
        }

        for (found_after, before_rules) in errors.iter() {
            for rule in before_rules.iter() {
                if let Some(index_one) = new_page_update.iter().position(|&x| x == *found_after) {
                    if let Some(index_two) = new_page_update.iter().position(|&x| x == *rule) {
                        new_page_update.swap(index_one, index_two);
                    }
                }
            }
        }
    }

    return new_page_update;
}


fn solve_day(file_contents: &str) -> (u32, u32) {
    /*
        Part 1
     */
    let mut sum_part1: u32 = 0;
    let mut sum_part2: u32 = 0;
    let mut ordering_done = false;

    let mut ordering: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut page_updates: Vec<Vec<u32>> = vec![];

    for line in file_contents.lines() {
        if line.trim().is_empty() {
            ordering_done = true;
            continue;
        }
        if !ordering_done {
            let pages_order: Vec<u32> = line.split('|').map(|s| s.trim().parse().unwrap()).collect();
            ordering.entry(pages_order[0]).or_insert(Vec::new()).push(pages_order[1]);
        } else {
            let page_update: Vec<u32> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            page_updates.push(page_update.clone());
        }
    }

    for page_update in page_updates.iter() {
        if is_page_update_valid(&ordering, page_update) {
            sum_part1 += get_middle_value(page_update);
        } else {
            let new_page_update: Vec<u32> = fix_page_update_order(&ordering, page_update);
            sum_part2 += get_middle_value(&new_page_update);
        }
    }

    return (sum_part1, sum_part2);
}


pub fn get_day_results(file_contents: &str) -> (u32, u32) {
    /*
        Return this day's results as a tuple
     */
    solve_day(&file_contents)
}