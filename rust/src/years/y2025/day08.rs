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


fn get_circuit_total(circuits: &mut Vec<HashSet<Point3D>>) -> usize {
    let mut sum: usize = 1;
    circuits.sort_by_key(|x| x.len());
    for &circuit in &circuits.iter().rev().collect::<Vec<&HashSet<Point3D>>>()[0..3] {
        sum *= circuit.len();
    }
    sum
}

fn connect_circuits(junction_boxes: Vec<Point3D>) -> usize {
    let junction_box_len: usize = junction_boxes.len();
    let mut connected: HashMap<Point3D, HashSet<Point3D>> = HashMap::with_capacity(junction_box_len);

    let mut circuits: Vec<HashSet<Point3D>> = junction_boxes
        .iter()
        .map(|x| HashSet::from([x.to_owned()]))
        .collect::<Vec<HashSet<Point3D>>>();

    while connected.len() < junction_box_len.min(10) {
        let mut smallest_distance: f64 = f64::MAX;
        let mut smallest_pair: Option<(Point3D, Point3D)> = None;
        for (i, junction_box) in junction_boxes[..].iter().enumerate() {
            for junction_box_2 in &junction_boxes[i+1..] {
                if let Some(j1) = connected.get(&junction_box) {
                    if j1.contains(&junction_box_2) {
                        continue;
                    }
                }

                let distance: f64 = junction_box.distance(&junction_box_2);
                if distance < smallest_distance {
                    smallest_pair = Some((junction_box.to_owned(), junction_box_2.to_owned()));
                    smallest_distance = distance;
                }
            }
        }

        if let Some((j1, j2)) = smallest_pair {
            connected.entry(j1.clone()).or_insert(HashSet::with_capacity(junction_box_len)).insert(j2.clone());
            connected.entry(j2.clone()).or_insert(HashSet::with_capacity(junction_box_len)).insert(j1.clone());

            let mut found: usize = 0;
            let mut circuit_index: usize = 0;
            let mut circuit_to_merge: usize = 0;
            for (i, c) in circuits.iter().enumerate() {
                if c.contains(&j1) {
                    circuit_index = i;
                    found += 1;
                    continue;
                }
                if c.contains(&j2) {
                    circuit_to_merge = i;
                    found += 1;
                }
                if found == 2 {
                    break;
                }
            }

            if found == 2 {
                circuits[circuit_index] = circuits[circuit_index].union(&circuits[circuit_to_merge]).cloned().collect();
                circuits.remove(circuit_to_merge);
            }
        }
    }

    get_circuit_total(&mut circuits)
}


#[allow(unused)]
fn get_last_connection(junction_boxes: Vec<Point3D>) -> u128 {
    let junction_box_len: usize = junction_boxes.len();
    let mut connected: HashMap<Point3D, HashSet<Point3D>> = HashMap::with_capacity(junction_box_len);

    let mut last_pair: Option<(Point3D, Point3D)> = None;

    while connected.len() < junction_box_len {
        let mut smallest_distance: f64 = f64::MAX;
        let mut smallest_pair: Option<(Point3D, Point3D)> = None;
        for (i, junction_box) in junction_boxes[..].iter().enumerate() {
            for junction_box_2 in &junction_boxes[i+1..] {
                if let Some(j1) = connected.get(&junction_box) {
                    if j1.contains(&junction_box_2) {
                        continue;
                    }
                }

                let distance: f64 = junction_box.distance(&junction_box_2);
                if distance < smallest_distance {
                    smallest_pair = Some((junction_box.to_owned(), junction_box_2.to_owned()));
                    last_pair = Some((junction_box.to_owned(), junction_box_2.to_owned()));
                    smallest_distance = distance;
                }
            }
        }


        if let Some((j1, j2)) = smallest_pair {
            connected.entry(j1.clone()).or_insert(HashSet::with_capacity(junction_box_len)).insert(j2.clone());
            connected.entry(j2.clone()).or_insert(HashSet::with_capacity(junction_box_len)).insert(j1.clone());
        }
    }

    if let Some((j1, j2)) = last_pair {
        return j1.x as u128 * j2.x as u128;
    }

    return 0;
}

#[allow(unused)]
fn part01(file_contents: &str) -> usize {
    /*
        Part 1
     */
    let mut junction_boxes: Vec<Point3D> = Vec::with_capacity(file_contents.lines().collect::<Vec<&str>>().len());

    for line in file_contents.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        junction_boxes.push(Point3D {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
            z: parts[2].parse().unwrap(),
        });
    }

    connect_circuits(junction_boxes)
}


#[allow(unused)]
fn part02_(file_contents: &str) -> u128 {
    /*
        Part 2
     */
    let mut junction_boxes: Vec<Point3D> = Vec::with_capacity(file_contents.lines().collect::<Vec<&str>>().len());

    for line in file_contents.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        junction_boxes.push(Point3D {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
            z: parts[2].parse().unwrap(),
        });
    }

    get_last_connection(junction_boxes)
}

fn part02(file_contents: &str) -> u128 {
    /*
        Part 2
     */
    let mut junction_boxes: Vec<Point3D> = Vec::with_capacity(file_contents.lines().collect::<Vec<&str>>().len());

    for line in file_contents.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        junction_boxes.push(Point3D {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
            z: parts[2].parse().unwrap(),
        });
    }

    let junction_box_len: usize = junction_boxes.len();
    let mut connections: HashMap<(&Point3D, &Point3D), f64> = HashMap::with_capacity(junction_box_len);

    while connections.len() < junction_box_len {
        for (i, junction_box) in junction_boxes.iter().enumerate() {
            for junction_box_2 in &junction_boxes[i+1..] {
                if connections.contains_key(&(junction_box, junction_box_2)) {
                    continue;
                }

                let distance: f64 = junction_box.distance(&junction_box_2);
                connections.insert((junction_box, junction_box_2), distance);
                connections.insert((junction_box_2, junction_box), distance);
            }
        }
    }

    let max_key = connections.iter()
        .max_by(|((_, _), &v), ((_, _), v2)| v.partial_cmp(v2).unwrap())
        .map(|(key, _value)| key).unwrap();

    //println!("{junction_boxes:?}");

    let &(j1, j2) = max_key;

    return j1.x as u128 * j2.x as u128;
}

pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    (format!(""), format!("{}", part02(&file_contents)))
}