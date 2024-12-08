mod day01;
mod day02;
mod day03;
mod day04;
use crate::utils;


pub fn run_day(day: &mut u8) -> (i32, i32) {
    let contents = utils::get_day_input_content(day, 2024);

    println!("Running day {day} of year 2024");

    match day {
        1 => {
            let results = day01::get_day_results(&contents.to_string());
            utils::unsigned_to_signed(&results)
        },
        2 => {
            let results = day02::get_day_results(&contents.to_string());
            utils::unsigned_to_signed(&results)
        },
        3 => {
            let results = day03::get_day_results(&contents.to_string());
            utils::unsigned_to_signed(&results)
        },
        4 => {
            let results = day04::get_day_results(&contents.to_string());
            utils::unsigned_to_signed(&results)
        },
        _ => {
            panic!("Day {day} has not yet been implemented");
        }
    }
}