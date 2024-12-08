use clap::Parser;
mod utils;
mod years;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    day: u8,
    #[arg(short, long, default_value_t = 2024)]
    year: u16,
}


fn main() {
    let args = Args::parse();

    let (part_one, part_two): (i32, i32) = years::run_day(args.day, args.year);

    println!("Part 01: {part_one}\nPart 02: {part_two}")
}
