use std::collections::{HashMap,HashSet};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Hash)]
struct Point3D {
    x: u64,
    y: u64,
    z: u64
}

impl Point3D {
    fn distance(&self, other: &Point3D) -> f64 {
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2) + (other.z - self.z).pow(2)) as f64).sqrt()
    }
}

impl Eq for Point3D {}


fn part01(file_contents: &str) -> u32 {
    /*
        Part 1
     */
    let mut sum: u32 = 0;

    let mut junction_boxes: Vec<Point3D> = Vec::with_capacity(file_contents.lines().collect::<Vec<&str>>().len());

    for line in file_contents.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        junction_boxes.push(Point3D {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
            z: parts[2].parse().unwrap(),
        });
    }

    println!("Junction boxes: {junction_boxes:#?}");
    let junction_box_len: usize = junction_boxes.len();

    let mut connected: HashMap<Point3D, HashSet<Point3D>> = HashMap::with_capacity(junction_box_len);

    let mut smallest_distance: f64 = f64::MAX;
    let mut smallest_pair: Option<(Point3D, Point3D)> = None;
    for (i, junction_box) in junction_boxes.iter().enumerate() {
        for junction_box_2 in &junction_boxes[i+1..] {
            let distance: f64 = junction_box.distance(&junction_box_2);
            if distance < smallest_distance {
                smallest_pair = Some((junction_box.to_owned(), junction_box_2.to_owned()));
                smallest_distance = distance;

                connected.entry(junction_box.to_owned()).or_insert(HashSet::with_capacity(junction_box_len)).insert(junction_box_2.to_owned());
                connected.entry(junction_box_2.to_owned()).or_insert(HashSet::with_capacity(junction_box_len)).insert(junction_box.to_owned());
            }
        }
    }

    println!("Smallest distance: {smallest_distance}; pair: {smallest_pair:?}");
    println!("Connected: {connected:#?}");

    return sum;
}


fn part02(_file_contents: &str) -> i32 {
    /*
        Part 2
     */
    let mut sum: i32 = 0;
    sum += 0;

    return sum;
}

pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    (format!("{}", part01(&file_contents)), format!("{}", part02(&file_contents)))
}