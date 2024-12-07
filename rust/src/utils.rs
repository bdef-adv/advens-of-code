use std::fs::File;
use std::io::{self, Read};
use chrono::prelude::*;

pub fn unsigned_to_signed(src: &(u64, u64)) -> (i64, i64) {
    return (src.0 as i64, src.1 as i64)
}

pub fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_day_input_content(day: &mut u8, year: u16, test: bool) -> String {
    if *day == 0 {
        let date: u32 = Utc::now().day();
        *day = date.try_into().unwrap();
    }

    let file_name = format!("data/inputs/{year}/day{day:02}{}.txt", if test {".test"} else {""});
    match read_file(&file_name) {
        Ok(contents) => {
            return contents;
        },
        Err(err) => {
            panic!("File {file_name} could not be read: {err}");
        }
    }
}