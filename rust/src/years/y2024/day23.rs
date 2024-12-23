use std::collections::{HashSet,HashMap};


fn solve_day(file_contents: &str) -> (u32, String) {
    /*
        Solve day
     */
    let mut computers: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in file_contents.lines() {
        let parts: Vec<&str> = line.split('-').collect();
        computers.entry(parts[0]).or_insert(HashSet::new()).insert(parts[1]);
        computers.entry(parts[1]).or_insert(HashSet::new()).insert(parts[0]);
    }

    let mut connection_set: HashSet<Vec<&str>> = HashSet::new();
    for (&computer, connections) in computers.iter() {
        for &connected in connections.iter() {
            let others = computers.get(&connected).unwrap();
            for &other in others.iter() {
                if connections.contains(other) {
                    let mut group = vec![computer, connected, other];
                    group.sort();
                    connection_set.insert(group);
                }
            }
        }
    }

    let mut sum_part1: u32 = 0;
    for set in connection_set.iter() {
        if set.iter().any(|x| x.starts_with('t')) {
            sum_part1 += 1;
        }
    }
    
    let mut largest: Vec<&str> = vec![];
    largest.sort();
    let sum_part2: String = largest.join(",");
    
    (sum_part1, sum_part2)
}


pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    let results = solve_day(&file_contents);
    (format!("{}", results.0), format!("{}", results.1))
}
