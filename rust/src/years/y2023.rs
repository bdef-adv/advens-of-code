mod day01;
use crate::utils;


pub fn run_day(day: &mut u8) -> (i32, i32) {
    let contents = utils::get_day_input_content(day, 2023);

    match day {
        1 => {
            let results = day01::get_day_results(&contents.to_string());
            utils::unsigned_to_signed(&results)
        },
        _ => {
            panic!("Day {day} has not yet been implemented");
        }
    }
}