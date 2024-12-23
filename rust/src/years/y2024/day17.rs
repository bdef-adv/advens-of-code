#[derive(Debug,Clone)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv
}

impl Instruction {
    fn from_opcode(opcode: u8) -> Option<Instruction> {
        match opcode {
            0 => Some(Instruction::Adv),
            1 => Some(Instruction::Bxl),
            2 => Some(Instruction::Bst),
            3 => Some(Instruction::Jnz),
            4 => Some(Instruction::Bxc),
            5 => Some(Instruction::Out),
            6 => Some(Instruction::Bdv),
            7 => Some(Instruction::Cdv),
            _ => None
        }
    }

    fn get_combo_operand(computer: &Computer, operand: u8) -> u64 {
        match operand {
            0_u8..=3_u8 => operand as u64,
            4 => computer.register_a,
            5 => computer.register_b,
            6 => computer.register_c,
            _ => operand as u64 // shouldn't happen though
        }
    }

    fn adv(computer: &mut Computer, operand: u8, pointer: usize) -> usize {
        let numerator = computer.register_a;
        let denominator = 1 << Instruction::get_combo_operand(computer, operand);

        computer.register_a = numerator / denominator;
        return pointer + 2;
    }

    fn bxl(computer: &mut Computer, operand: u8, pointer: usize) -> usize {
        let result = computer.register_b ^ operand as u64;

        computer.register_b = result;
        return pointer + 2;
    }

    fn bst(computer: &mut Computer, operand: u8, pointer: usize) -> usize {
        computer.register_b = Instruction::get_combo_operand(computer, operand) % 8;
        return pointer + 2;
    }

    fn jnz(computer: &mut Computer, operand: u8, pointer: usize) -> usize {
        if computer.register_a == 0 {
            return pointer + 2;
        }
        return operand as usize;
    }

    fn bxc(computer: &mut Computer, _operand: u8, pointer: usize) -> usize {
        computer.register_b = computer.register_b ^ computer.register_c;
        return pointer + 2;
    }

    fn out(computer: &mut Computer, operand: u8, pointer: usize) -> usize {
        computer.output.push((Instruction::get_combo_operand(computer, operand) % 8) as u8);
        return pointer + 2;
    }

    fn bdv(computer: &mut Computer, operand: u8, pointer: usize) -> usize {
        let numerator = computer.register_a;
        let denominator = 1 << Instruction::get_combo_operand(computer, operand);

        computer.register_b = numerator / denominator;
        return pointer + 2;
    }

    fn cdv(computer: &mut Computer, operand: u8, pointer: usize) -> usize {
        let numerator = computer.register_a;
        let denominator = 1 << Instruction::get_combo_operand(computer, operand);

        computer.register_c = numerator / denominator;
        return pointer + 2;
    }

    fn run_instruction(&self, computer: &mut Computer, operand: u8, pointer: usize) -> usize {
        match *self {
            Instruction::Adv => {return Instruction::adv(computer, operand, pointer)},
            Instruction::Bxl => {return Instruction::bxl(computer, operand, pointer)},
            Instruction::Bst => {return Instruction::bst(computer, operand, pointer)},
            Instruction::Jnz => {return Instruction::jnz(computer, operand, pointer)},
            Instruction::Bxc => {return Instruction::bxc(computer, operand, pointer)},
            Instruction::Out => {return Instruction::out(computer, operand, pointer)},
            Instruction::Bdv => {return Instruction::bdv(computer, operand, pointer)},
            Instruction::Cdv => {return Instruction::cdv(computer, operand, pointer)},
        }
    }
}

#[derive(Clone)]
struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    program: Vec<u8>,
    output: Vec<u8>
}

impl Computer {
    fn from(input: &str) -> Self {
        let mut register_a: u64 = 0;
        let mut register_b: u64 = 0;
        let mut register_c: u64 = 0;
        let mut program: Vec<u8> = vec![];
        let output: Vec<u8> = vec![];

        for line in input.lines() {
            if line.starts_with("Register A:") {
                register_a = line.trim_start_matches("Register A: ").parse::<u64>().unwrap();
            } else if line.starts_with("Register B:") {
                register_b = line.trim_start_matches("Register B: ").parse::<u64>().unwrap();
            } else if line.starts_with("Register C:") {
                register_c = line.trim_start_matches("Register C: ").parse::<u64>().unwrap();
            } else if line.starts_with("Program:") {
                let parts: Vec<&str> = line.trim_start_matches("Program: ").split(',').collect();
                program = parts.iter().map(|x| x.parse::<u8>().unwrap()).collect();
            }
        }

        return Computer {
            register_a,
            register_b,
            register_c,
            program,
            output
        }
    }

    fn run_program(&mut self) {
        let program_length = self.program.len();
        let mut pointer = 0;
        loop {
            if pointer + 1 >= program_length {
                break;
            }

            if let Some(instruction) = Instruction::from_opcode(self.program[pointer]) {
                pointer = instruction.run_instruction(self, self.program[pointer + 1], pointer);
            }
        }
    }

    fn run_program_part2(&mut self) -> bool {
        let program_length = self.program.len();
        let mut pointer = 0;
        let mut checked: usize = 0;

        loop {
            if pointer + 1 >= program_length {
                break;
            }

            if let Some(instruction) = Instruction::from_opcode(self.program[pointer]) {
                pointer = instruction.run_instruction(self, self.program[pointer + 1], pointer);
                for (i, &out) in self.output[checked..].iter().enumerate() {
                    if checked + i >= program_length || out != self.program[checked + i] {
                        return false;
                    }
                    checked += 1;
                }
            }
        }

        return checked == program_length;
    }

    #[allow(unused)]
    fn print(&self) {
        println!("Register A: {}", self.register_a);
        println!("Register B: {}", self.register_b);
        println!("Register C: {}", self.register_c);
        println!("Program: {:?}", self.program);
    }

    fn output(&self) -> String {
        let mut string: String = String::new();
        for (i, &out) in self.output.iter().enumerate() {
            if i > 0 {
                string.push(',');
            }
            string.push(char::from_digit(out as u32, 10).unwrap());
        }
        return string;
    }

    fn find_part2(&mut self, start: u64) -> u64 {
        let mut a = start;

        while !self.run_program_part2() {
            a += 1;

            if a % 100000000 == 0 {
                println!("Trying register A={a}");
            }

            self.register_a = a;
            self.register_b = 0;
            self.register_c = 0;
            self.output.clear();
        }

        return a;
    }
}

fn solve_day(file_contents: &str) -> (String, u64) {
    /*
        Solve day
     */
    let mut computer = Computer::from(file_contents);
    computer.run_program();
    let out_part1 = computer.output();

    computer = Computer::from(file_contents);

    //let sum_part2: u64 = computer.find_part2_multi(600, 1_000_000_000);
    
    let sum_part2: u64 = computer.find_part2(125900000000);

    (out_part1, sum_part2)
}


pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    let results = solve_day(&file_contents);
    (format!("{}", results.0), format!("{}", results.1))
}
