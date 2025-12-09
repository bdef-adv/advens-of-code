mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
//mod day10;
//mod day11;
//mod day12;
//mod day13;
//mod day14;
//mod day15;
//mod day16;
//mod day17;
//mod day18;
//mod day19;
//mod day20;
//mod day21;
//mod day22;
//mod day23;
//mod day24;
use crate::utils;

pub fn run_day(day: &mut u8, test: bool) -> (String, String) {
    let contents = utils::get_day_input_content(day, 2025, test);

    println!("Running day {day} of year 2025");

    match day {
        1 => day01::get_day_results(&contents.to_string()),
        2 => day02::get_day_results(&contents.to_string()),
        3 => day03::get_day_results(&contents.to_string()),
        4 => day04::get_day_results(&contents.to_string()),
        5 => day05::get_day_results(&contents.to_string()),
        6 => day06::get_day_results(&contents.to_string()),
        7 => day07::get_day_results(&contents.to_string()),
        8 => day08::get_day_results(&contents.to_string()),
        9 => day09::get_day_results(&contents.to_string()),
        //10 => day10::get_day_results(&contents.to_string()),
        //11 => day11::get_day_results(&contents.to_string()),
        //12 => day12::get_day_results(&contents.to_string()),
        //13 => day13::get_day_results(&contents.to_string()),
        //14 => day14::get_day_results(&contents.to_string()),
        //15 => day15::get_day_results(&contents.to_string()),
        //16 => day16::get_day_results(&contents.to_string()),
        //17 => day17::get_day_results(&contents.to_string()),
        //18 => day18::get_day_results(&contents.to_string()),
        //19 => day19::get_day_results(&contents.to_string()),
        //20 => day20::get_day_results(&contents.to_string()),
        //21 => day21::get_day_results(&contents.to_string()),
        //22 => day22::get_day_results(&contents.to_string()),
        //23 => day23::get_day_results(&contents.to_string()),
        //24 => day24::get_day_results(&contents.to_string()),
        _ => {
            panic!("Day {day} has not yet been implemented");
        }
    }
}
