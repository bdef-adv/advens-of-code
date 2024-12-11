use std::collections::HashMap;

// Merci chatgpt je suis trop un flemmard
fn split_number(n: u64) -> (u64, u64) {
    let mut temp = n;
    let mut digits = 0;
    while temp > 0 {
        digits += 1;
        temp /= 10;
    }

    if digits == 0 {
        return (0, 0);
    }
    let divisor = 10_u64.pow((digits / 2) as u32);

    let left = n / divisor;
    let right = n % divisor;

    (left, right)
}

fn has_even_length(n: u64) -> bool {
    let mut temp = n;
    let mut digits = 0;
    while temp > 0 {
        digits += 1;
        temp /= 10;
    }

    digits % 2 == 0
}

fn blink_stone(stone: u64) -> (u64, Option<u64>) {
    if stone == 0 {
        return (1, None);
    } else if has_even_length(stone) {
        let (left, right) = split_number(stone);
        return (left, Some(right));
    } else {
        return (stone*2024, None);
    }
}

// Les nombres se repetent pas mal, plutot que stocker tous les chiffres dans un vecteur 
// qui s'aggrandit beaucoup trop vite, on utilise un hashmap pour stocker les comptes des nombres qu'on a déjà vu.
// Abusé la diff parceque l'implementation d'avant arrivait en OOM sur mes 32Go de RAM à l'étape 49
fn blink_int_3(stones: &mut HashMap<u64, u64>) -> u64 {  
    let mut new_stones: HashMap<u64, u64> = HashMap::new();
    let mut total_count: u64 = 0;

    for (&stone, count) in stones.iter_mut() {
        let (left, r) = blink_stone(stone);

        *new_stones.entry(left).or_insert(0) += *count;
        total_count += *count;

        if let Some(right) = r {
            *new_stones.entry(right).or_insert(0) += *count;
            total_count += *count;
        }
    }

    *stones = new_stones;

    return total_count;
}


fn solve_day(file_contents: &str) -> (u64, u64) {
    let stones: Vec<u64> = file_contents.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut stone_map: HashMap<u64, u64> = HashMap::new();
    for &stone in stones.iter() {
        stone_map.entry(stone).or_insert(1);
    }

    let mut sum_part1: u64 = 0;
    for _ in 0..25 {
        sum_part1 = blink_int_3(&mut stone_map);
    }

    let mut sum_part2: u64 = 0;
    for _ in 0..50 {
        sum_part2 = blink_int_3(&mut stone_map);
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
