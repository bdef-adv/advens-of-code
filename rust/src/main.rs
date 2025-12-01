use clap::Parser;
use std::time::Instant;
mod utils;
mod years;
pub mod classes;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    day: u8,
    #[arg(short, long, default_value_t = 2025)]
    year: u16,
    #[arg(short, long, default_value_t = false)]
    test: bool,
}


fn main() {
    let args: Args = Args::parse();
    let mut day: u8 = args.day;

    let timer: Instant = Instant::now();
    let (part_one, part_two): (String, String) = years::run_day(&mut day, args.year, args.test);

    println!("Part 01: {part_one}\nPart 02: {part_two}");

    let millis_limit = 2;
    let duration = timer.elapsed();
    let time_unit: char = if duration.as_millis() <= millis_limit {'µ'} else {'m'};
    let duration_displayed: u128 = if duration.as_millis() <= millis_limit {
        duration.as_micros()
    } else {
        duration.as_millis()
    };
    println!("Took: {}{time_unit}s to calculate both parts", duration_displayed);
}
