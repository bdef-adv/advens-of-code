fn part01(file_contents: &str) -> u64 {
    /*
        Part 1
     */
    let mut sum: u64 = 0;

    let mut numbers: Vec<Vec<u64>> = Vec::with_capacity(200);
    let mut operations: Vec<&str> = Vec::with_capacity(0);

    for line in file_contents.lines() {
        let elements: Vec<&str> = line.split_whitespace().collect();
        if ["*", "+"].contains(&elements[0]) {
            operations = elements.clone();
        } else {
            let line_numbers: Vec<u64> = elements.iter().map(|x| x.parse::<u64>().unwrap()).collect();
            numbers.push(line_numbers);
        }
    }

    for (idx, &operation) in operations.iter().enumerate() {
        let mut result: u64 = 0;
        for n_list in numbers.iter() {
            let current_number = n_list[idx];
            match operation {
                "*" => {
                    if result == 0 {
                        result = 1;
                    }
                    result *= current_number;
                }
                "+" => {
                    result += current_number;
                }
                _ => {}
            }
        }

        sum += result;
    }

    return sum;
}


#[derive(Debug, Clone)]
enum OperationType {
    Mul,
    Add
}


impl OperationType {
    fn from(s: &str) -> Self {
        if s.contains('+') {
            return OperationType::Add;
        }

        OperationType::Mul
    }
}


#[derive(Debug, Clone)]
struct Operation {
    operation_type: OperationType,
    start_idx: usize,
    end_idx: usize
}


#[derive(Debug)]
struct Number {
    number: String,
    start_idx: usize,
    end_idx: usize
}


fn part02(file_contents: &str) -> u64 {
    /*
        Part 2
        Garbage code
     */
    let mut sum: u64 = 0;

    let mut numbers: Vec<Number> = Vec::with_capacity(50);
    let mut operations: Vec<Operation> = Vec::with_capacity(50);

    for line in file_contents.lines() {
        let mut current_number: String = String::new();
        let mut current_number_start_idx: usize = 0;

        let mut idx: usize = 0;
        for ch in line.chars() {
            if ch == ' ' && !current_number.trim().is_empty() && !["*", "+"].contains(&current_number.trim()) {
                numbers.push(Number {
                    number: current_number,
                    start_idx: current_number_start_idx,
                    end_idx: idx
                });

                current_number_start_idx = idx + 1;
                current_number = String::new();
            } else if ['*', '+'].contains(&ch) {
                if !current_number.trim().is_empty() {
                    operations.push(Operation {
                        operation_type: OperationType::from(current_number.as_str()),
                        start_idx: current_number_start_idx,
                        end_idx: idx - 1
                    });

                    current_number = String::new();
                    current_number_start_idx = idx;
                }

                current_number.push(ch);
            } else if ch == ' ' && !["*", "+"].contains(&current_number.trim()) {
                current_number_start_idx += 1;
            } else {
                current_number.push(ch);
            }

            idx += 1;
        }

        if !current_number.trim().is_empty() {
            if ["*", "+"].contains(&current_number.trim()) {
                operations.push(Operation {
                    operation_type: OperationType::from(current_number.as_str()),
                    start_idx: current_number_start_idx,
                    end_idx: idx
                });
            } else {
                numbers.push(Number {
                    number: current_number,
                    start_idx: current_number_start_idx,
                    end_idx: idx
                });
            }
        }
    }

    for operation in operations {
        let mut real_numbers: Vec<u64> = Vec::with_capacity(50);
        for number in &numbers {
            if number.start_idx >= operation.start_idx && number.end_idx <= operation.end_idx {
                for digit in operation.start_idx..operation.end_idx {
                    if digit >= number.start_idx {
                        let dig: char = number.number.chars().nth(digit - number.start_idx).unwrap_or(' ');
                        if let Some(dig) = dig.to_digit(10) {
                            if digit - operation.start_idx >= real_numbers.len() {
                                for _ in digit - operation.start_idx..=real_numbers.len() {
                                    real_numbers.push(0);
                                }
                            }

                            real_numbers[digit - operation.start_idx] *= 10;
                            real_numbers[digit - operation.start_idx] += dig as u64;
                        }
                    } else {
                        real_numbers.push(0);
                    }
                }
            }
        }

        let mut result: u64 = 0;
        for &num in real_numbers.iter() {
            if num == 0 {
                continue;
            }
            match operation.operation_type {
                OperationType::Mul => {
                    if result == 0 {
                        result = 1;
                    }
                    result *= num;
                }
                OperationType::Add => {
                    result += num;
                }
            }
        }

        sum += result;
    }

    return sum;
}


pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    (format!("{}", part01(&file_contents)), format!("{}", part02(&file_contents)))
}
