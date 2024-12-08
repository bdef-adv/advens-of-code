use itertools::Itertools;

fn concat(n1: u64, n2: u64) -> u64 {
    return format!("{n1}{n2}").parse().unwrap();
}

fn test_equation(total: u64, numbers: Vec<u64>, concat_enabled: bool) -> bool {
    let nlen = numbers.len() - 1;
    let mut operations = vec!['+', '*'];
    if concat_enabled {
        operations.push('|');
    }
    let permutations = Itertools::multi_cartesian_product(
        vec![operations.clone(); nlen].into_iter()
    );

    for operation_list in permutations {
        let mut result: u64 = numbers[0];

        for (index, &operation) in operation_list.iter().enumerate() {
            if operation == '+' {
                result += numbers[index+1];
            } else if operation == '*' {
                result *= numbers[index+1];
            } else if operation == '|' {
                result = concat(result, numbers[index+1]);
            }
        }

        if result == total {
            return true;
        }
    }

    return false;
}


fn solve_day(file_contents: &str) -> (u64, u64) {
    /*
        Part 1
     */
    let mut sum_part1: u64 = 0;
    let mut sum_part2: u64 = 0;

    for line in file_contents.lines() {
        let mut parts = line.split(":");
        let total = parts.next().unwrap().parse().unwrap();
        let numbers: Vec<u64> = parts.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        if test_equation(total, numbers.clone(), false) {
            sum_part1 += total;
        }
        if test_equation(total, numbers, true) {
            sum_part2 += total;
        }
    }

    return (sum_part1, sum_part2);
}


pub fn get_day_results(file_contents: &str) -> (u64, u64) {
    /*
        Return this day's results as a tuple
     */
    solve_day(file_contents)
}