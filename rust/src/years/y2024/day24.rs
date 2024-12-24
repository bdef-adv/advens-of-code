use rand::{thread_rng, Rng};
use std::collections::{HashMap, HashSet};

use std::hash::{Hash, Hasher};
use std::path::Iter;

#[derive(Debug, Clone)]
struct HashableSet {
    set: HashSet<(usize, usize)>,
}

impl Hash for HashableSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in &self.set {
            item.hash(state); // Hash each item in the set
        }
    }
}

impl PartialEq for HashableSet {
    fn eq(&self, other: &Self) -> bool {
        self.set == other.set
    }
}

impl Eq for HashableSet {}

impl HashableSet {
    fn new() -> Self {
        return HashableSet {
            set: HashSet::new(),
        };
    }

    fn clear(&mut self) {
        self.set.clear();
    }

    fn insert(&mut self, tu: (usize, usize)) {
        self.set.insert(tu);
    }

    fn contains(&mut self, tu: &(usize, usize)) -> bool {
        return self.set.contains(tu);
    }

    fn iter(&self) -> std::collections::hash_set::Iter<'_, (usize, usize)> {
        return self.set.iter();
    }
}

#[derive(Copy, Clone)]
struct Operation<'a> {
    left: &'a str,
    right: &'a str,
    operation: &'a str,
    destination: &'a str,
}

impl<'a> Operation<'a> {
    fn from(left: &'a str, right: &'a str, operation: &'a str, destination: &'a str) -> Self {
        return Operation {
            left,
            right,
            operation,
            destination,
        };
    }

    fn evaluate(&mut self, left: bool, right: bool) -> bool {
        match self.operation {
            "AND" => left && right,
            "OR" => left || right,
            "XOR" => left ^ right,
            _ => false,
        }
    }
}

struct JungleGrove<'a> {
    registries: HashMap<&'a str, Option<bool>>,
    operations: Vec<Operation<'a>>,
    done: usize,
}

impl<'a> JungleGrove<'a> {
    fn from(input: &str) -> JungleGrove {
        let mut registries = HashMap::with_capacity(200);
        let mut operations = vec![];

        let mut operations_ = false;
        for line in input.lines() {
            if line.is_empty() {
                operations_ = true;
                continue;
            }

            if operations_ {
                let parts: Vec<&str> = line.split_whitespace().collect();
                operations.push(Operation::from(parts[0], parts[2], parts[1], parts[4]));
            } else {
                let parts: Vec<&str> = line.split(": ").collect();
                let value: u8 = parts[1].parse().unwrap();
                registries.insert(parts[0], Some(value != 0));
            }
        }

        return JungleGrove {
            registries,
            operations,
            done: 0,
        };
    }

    fn refresh(&mut self, input: &'a str) {
        let mut registries = HashMap::with_capacity(200);
        let mut operations = vec![];

        let mut operations_ = false;
        for line in input.lines() {
            if line.is_empty() {
                operations_ = true;
                continue;
            }

            if operations_ {
                let parts: Vec<&str> = line.split_whitespace().collect();
                operations.push(Operation::from(parts[0], parts[2], parts[1], parts[4]));
            } else {
                let parts: Vec<&str> = line.split(": ").collect();
                let value: u8 = parts[1].parse().unwrap();
                registries.insert(parts[0], Some(value != 0));
            }
        }

        self.registries = registries;
        self.operations = operations;
    }

    fn get_num_from_registry(&mut self, registry: char) -> u64 {
        let mut number: u64 = 0;

        for (key, &value) in self.registries.iter() {
            if !key.starts_with(registry) {
                continue;
            }

            if let Some(val) = value {
                let index: u64 = key.trim_start_matches(registry).parse().unwrap();
                number |= (val as u64) << index;
            }
        }

        return number;
    }

    fn run_operations(&mut self) {
        let operations_len = self.operations.len();
        while self.done != operations_len {
            for operation in self.operations.iter_mut() {
                if let Some(Some(left)) = self.registries.get(operation.left) {
                    if let Some(Some(right)) = self.registries.get(operation.right) {
                        if self.registries.get(operation.destination).is_none() {
                            self.registries.insert(
                                operation.destination,
                                Some(operation.evaluate(*left, *right)),
                            );
                            self.done += 1;
                        }
                    }
                }
            }
        }
    }

    fn find_swaps(&mut self, swap_n: usize, input: &'a str) -> String {
        let mut swaps: Vec<&'a str> = vec![];

        let operations_len = self.operations.len();

        println!("X: {}", self.get_num_from_registry('x'));
        println!("Y: {}", self.get_num_from_registry('y'));
        println!("Z: {}", self.get_num_from_registry('z'));

        let mut swapped: HashSet<HashableSet> = HashSet::new();

        while self.get_num_from_registry('x') + self.get_num_from_registry('y')
            != self.get_num_from_registry('z')
        {
            swaps.clear();
            self.refresh(input);

            let mut current_swaps: HashableSet = HashableSet::new();
            let mut swaps_made = 0;
            for l in 0..operations_len {
                for r in (l + 1)..operations_len {
                    if !current_swaps.contains(&(l, r))
                        && !current_swaps.contains(&(r, l))
                        && self.operations[l].destination.starts_with('z')
                        && self.operations[r].destination.starts_with('z')
                    {
                        current_swaps.insert((l, r));
                        swaps_made += 1;

                        if swaps_made >= swap_n && !swapped.contains(&current_swaps) {
                            swapped.insert(current_swaps.clone());
                            break;
                        } else if swaps_made >= swap_n {
                            swaps_made = 0;
                            current_swaps.clear();
                        }
                    }
                }

                if swaps_made >= swap_n && !swapped.contains(&current_swaps) {
                    swapped.insert(current_swaps.clone());
                    break;
                } else if swaps_made >= swap_n {
                    swaps_made = 0;
                    current_swaps.clear();
                }
            }

            println!("Swapping {current_swaps:?} (swapped: {swapped:?})");

            for (swap_l, swap_r) in current_swaps.iter() {
                let left_dst = self.operations[*swap_l].destination;
                let right_dst = self.operations[*swap_r].destination;

                self.operations[*swap_l].destination = right_dst;
                self.operations[*swap_r].destination = left_dst;

                swaps.push(left_dst);
                swaps.push(right_dst);
            }

            self.run_operations();
        }

        swaps.sort();
        return swaps.join(",");
    }
}

fn solve_day(file_contents: &str) -> (u64, String) {
    /*
       Solve day
    */
    let mut grove = JungleGrove::from(file_contents);
    grove.run_operations();
    let sum_part1: u64 = grove.get_num_from_registry('z');

    let sum_part2: String = grove.find_swaps(2, file_contents);

    (sum_part1, sum_part2)
}

pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
       Return this day's results as a tuple
    */
    let results = solve_day(&file_contents);
    (format!("{}", results.0), format!("{}", results.1))
}
