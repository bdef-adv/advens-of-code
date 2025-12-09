use crate::classes::Point;

use memoize::memoize;
use geo_types::{LineString, Polygon};
use geo::{Contains, Rect, coord};

type Point32 = Point<i32>;

impl Point32 {
    fn area(&self, other: &Point32) -> u64 {
        (self.x - other.x + 1).abs() as u64 * (self.y - other.y + 1).abs() as u64
    }
}


fn part01(file_contents: &str) -> u64 {
    /*
        Part 1
     */
    let mut red_tiles: Vec<Point32> = Vec::with_capacity(500);
    for line in file_contents.lines() {
        if let Some((x, y)) = line.split_once(',') {
            red_tiles.push(Point32 {x: x.parse().unwrap(), y: y.parse().unwrap()});
        }
    }

    let mut largest_area: u64 = 0;
    for (i, a) in red_tiles.iter().enumerate() {
        for b in &red_tiles[i+1..] {
            let area: u64 = a.area(b);

            if area > largest_area {
                largest_area = area;
            }
        }
    }

    largest_area
}


#[memoize(Ignore: polygon)]
fn is_rectangle_in_polygon(rect_p1: Point32, rect_p2: Point32, polygon: &Polygon) -> bool {
    let rect = Rect::new(
        coord! { x: rect_p1.x as f64, y: rect_p1.y as f64 },
        coord! { x: rect_p2.x as f64, y: rect_p2.y as f64 },
    );

    is_rectangle_inside_or_touching(polygon, &rect)
}


fn is_rectangle_inside_or_touching(polygon: &Polygon, rect: &Rect) -> bool {
    let min = rect.min();
    let max = rect.max();
    let corners = vec![
        coord! { x: min.x, y: min.y },
        coord! { x: max.x, y: min.y },
        coord! { x: max.x, y: max.y },
        coord! { x: min.x, y: max.y },
    ];

    let all_corners_inside = corners.iter().all(|corner| {
        polygon.contains(corner) || polygon.exterior().contains(corner)
    });

    all_corners_inside && !polygon.contains(rect)
}


fn part02(file_contents: &str) -> u64 {
    /*
        Part 2
     */
    let mut red_tiles: Vec<Point32> = Vec::with_capacity(500);
    let mut green_tiles: Vec<(f64, f64)> = Vec::with_capacity(200000);

    let mut last_red_tile: Option<Point32> = None;

    let (mut size_x, mut size_y): (usize, usize) = (0, 0);

    for line in file_contents.lines() {
        if let Some((x, y)) = line.split_once(',') {
            let red_tile = Point32 {x: x.parse().unwrap(), y: y.parse().unwrap()};
            red_tiles.push(red_tile);

            size_x = size_x.max(red_tile.x as usize + 2);
            size_y = size_y.max(red_tile.y as usize + 2);

            if let Some(last_point) = last_red_tile {
                for x_ in last_point.x.min(red_tile.x)..=last_point.x.max(red_tile.x) {
                    for y_ in last_point.y.min(red_tile.y)..=last_point.y.max(red_tile.y) {
                        let green_tile = (x_ as f64, y_ as f64);
                        green_tiles.push(green_tile);
                    }
                }
            }

            last_red_tile = Some(red_tile);
        }
    }

    let last_red_tile = red_tiles[red_tiles.len()-1];
    let first_red_tile = red_tiles[0];
    for x_ in last_red_tile.x.min(first_red_tile.x)..=last_red_tile.x.max(first_red_tile.x) {
        for y_ in last_red_tile.y.min(first_red_tile.y)..=last_red_tile.y.max(first_red_tile.y) {
            let green_tile = (x_ as f64, y_ as f64);
            green_tiles.push(green_tile);
        }
    }

    let polygon: Polygon = Polygon::new(LineString::from(green_tiles), vec![]);

    let mut largest_area: u64 = 0;
    for (i, &a) in red_tiles.iter().enumerate() {
        for &b in &red_tiles[i+1..] {
            if is_rectangle_in_polygon(a, b, &polygon) {
                let area: u64 = a.area(&b);

                if area > largest_area {
                    largest_area = area;
                    println!("Current largest: {largest_area} ({a}->{b})")
                }
            }
        }
    }

    largest_area
}

pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    (format!("{}", part01(&file_contents)), format!("{}", part02(&file_contents)))
}