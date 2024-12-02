mod y2023;
mod y2024;


pub fn run_day(day: &mut u8, year: u16) -> (i32, i32) {
    match year {
        2023 => {
            y2023::run_day(day)
        },
        2024 => {
            y2024::run_day(day)
        },
        _ => {
            panic!("Year {year} has not yet been implemented");
        }
    }
}