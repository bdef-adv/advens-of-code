mod y2023;
mod y2024;
mod y2025;


pub fn run_day(day: &mut u8, year: u16, test: bool) -> (String, String) {
    match year {
        2023 => {
            y2023::run_day(day, test)
        },
        2024 => {
            y2024::run_day(day, test)
        },
        2025 => {
            y2025::run_day(day, test)
        },
        _ => {
            panic!("Year {year} has not yet been implemented");
        }
    }
}