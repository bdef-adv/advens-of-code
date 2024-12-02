mod day01;
use crate::utils;

fn unsigned_to_signed(src: &(u32, u32)) -> (i32, i32) {
    return (src.0 as i32, src.1 as i32)
}

pub fn run_day(day: u8) -> (i32, i32) {
    let contents = utils::get_day_input_content(day, 2023);

    match day {
        1 => {
            let results = day01::get_day_results(&contents.to_string());
            unsigned_to_signed(&results)
        },
        _ => {
            panic!("Day {day} has not yet been implemented");
        }
    }
}