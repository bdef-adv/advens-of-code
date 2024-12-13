/*
Friday the 13th - Oh shit oh fuck
*/
use crate::classes::Point;

type Point64 = Point<i64>;

#[derive(Debug)]
struct ClawMachine {
    button_a: Point64,
    button_b: Point64,
    prize: Point64
}

fn get_point_from_str(input: &str) -> Point64 {
    let parts: Vec<&str> = input.split(", ").collect();

    let x: i64 = parts[0].trim_start_matches("X+").parse().unwrap();
    let y: i64 = parts[1].trim_start_matches("Y+").parse().unwrap();

    return Point64 {x, y};
}

impl ClawMachine {
    fn from(input: &str) -> Vec<ClawMachine> {
        let mut machines: Vec<ClawMachine> = vec![];

        let mut button_a = Point64::default();
        let mut button_b = Point64::default();

        #[allow(unused_assignments)]
        let mut prize = Point64::default();

        for line in input.lines() {
            if line.starts_with("Button A:") {
                button_a = get_point_from_str(line.trim_start_matches("Button A: ").parse::<String>().unwrap().as_str());
            } else if line.starts_with("Button B:") {
                button_b = get_point_from_str(line.trim_start_matches("Button B: ").parse::<String>().unwrap().as_str());
            } else if line.starts_with("Prize:") {
                prize = get_point_from_str(line.replace("=", "+").trim_start_matches("Prize: ").parse::<String>().unwrap().as_str());
                machines.push(ClawMachine {
                    button_a: button_a,
                    button_b: button_b,
                    prize: prize
                });
            }
        }

        return machines;
    }

    // merci chatGPT de me rappeler chaque année ce que c'est un determinant de matrice et des équations linéaires
    fn cheapest(&self) -> Option<i64> {
        let det_ab = self.button_a.x * self.button_b.y - self.button_a.y * self.button_b.x;

        if det_ab == 0 {
            return None;
        }

        let det_a_p = self.prize.x * self.button_b.y - self.prize.y * self.button_b.x;
        let det_p_b = self.button_a.x * self.prize.y - self.button_a.y * self.prize.x;

        let a = det_a_p / det_ab;
        let b = det_p_b / det_ab;

        if self.button_a*a + self.button_b*b == self.prize {
            return Some(3 * a + b);
        }

        return None;
    }

    fn increase_prize(&mut self) {
        self.prize.x += 10000000000000;
        self.prize.y += 10000000000000;
    }
}


fn solve_day(file_contents: &str) -> (i64, i64) {
    /*
        Solve day
     */

    let mut claw_machines: Vec<ClawMachine> = ClawMachine::from(file_contents);

    let mut tokens_part1 = 0;
    let mut tokens_part2 = 0;
    for cm in claw_machines.iter_mut() {
        if let Some(cheapest) = cm.cheapest() {
            tokens_part1 += cheapest;
        }
        cm.increase_prize();
        if let Some(cheapest) = cm.cheapest() {
            tokens_part2 += cheapest;
        }
    }
    
    (tokens_part1, tokens_part2)
}


pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    let results = solve_day(&file_contents);
    (format!("{}", results.0), format!("{}", results.1))
}
