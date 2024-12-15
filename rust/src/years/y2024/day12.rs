use crate::classes::{Maze, Point};

type Point32 = Point<i32>;
type MazeChar = Maze<char>;

struct Region {
    plots: Vec<Point32>
}

impl Region {
    fn area(&self) -> usize {
        return self.plots.len();
    }

    fn count_neighbors(&self, plot: &Point32, maze: &MazeChar) -> usize {
        let mut neighbors: usize = 0;

        for direction in Point32::DIRECTIONS {
            let ch: char = maze.array[plot.y as usize][plot.x as usize];
            let new_start = *plot + direction;

            if new_start.x < 0 || new_start.x >= maze.size_x as i32 ||
               new_start.y < 0 || new_start.y >= maze.size_y as i32 {
                continue;
            }

            if maze.array[new_start.y as usize][new_start.x as usize] == ch {
                neighbors += 1;
            }
        }
        
        return neighbors;
    }

    fn perimeter(&self, maze: &MazeChar) -> usize {
        let mut perimeter: usize = 0; 
        for plot in self.plots.iter() {
            perimeter += 4 - self.count_neighbors(plot, maze);
        }
        return perimeter;
    }

    fn count_corners(&self, plot: &Point32, maze: &MazeChar) -> usize {
        let mut corners: usize = 0;

        let directions = [
            (Point32::DOWN, Point32::RIGHT, Point32::DOWN_RIGHT),
            (Point32::DOWN, Point32::LEFT, Point32::DOWN_LEFT),
            (Point32::UP, Point32::RIGHT, Point32::UP_RIGHT),
            (Point32::UP, Point32::LEFT, Point32::UP_LEFT),
        ];

        for (d1, d2, d3) in directions {
            let p1 = maze.get(&(d1 + *plot));
            let p2 = maze.get(&(d2 + *plot));
            let p3 = maze.get(&(d3 + *plot));
            
            /* Coins classiques; ex:
            RR -> on check si le R en haut à gauche n'a pas de voisin direct en haut et à gauche
            R
            */
            if p1 != maze.get(&plot) &&
               p2 != maze.get(&plot) {
                corners += 1;
            }

            /* Coins classiques; ex:
            RR
            RR -> on check si le deuxieme R de ses sales morts a un voisin à gauche et en bas mais pas en diagonal
            AR
            quel batard ce R de merde putain
            */
            if p1 == maze.get(&plot) &&
               p2 == maze.get(&plot) &&
               p3 != maze.get(&plot) {
                corners += 1;
            }
        }

        return corners;
    }

    fn corners(&self, maze: &MazeChar) -> usize {
        let mut corners = 0;

        for plot in self.plots.iter() {
            corners += self.count_corners(plot, maze);
        }

        return corners;
    }
}

struct Garden {
    maze: MazeChar,
    regions: Vec<Region>
}

impl Garden {
    fn from(input: &str) -> Self {
        let regions: Vec<Region> = vec![];

        let maze = MazeChar::from(input);

        return Garden {
            maze,
            regions
        }
    }

    fn find_neighbors(&mut self, start: Point32, region_plots: &mut Vec<Point32>, visited: &mut Vec<Vec<bool>>) {
        if visited[start.y as usize][start.x as usize] {
            return;
        }

        let ch = self.maze.array[start.y as usize][start.x as usize];
        
        region_plots.push(start);
        visited[start.y as usize][start.x as usize] = true;

        for direction in Point32::DIRECTIONS {
            let new_start = start + direction;

            if new_start.x < 0 || new_start.x >= self.maze.size_x as i32 ||
               new_start.y < 0 || new_start.y >= self.maze.size_y as i32 {
                continue;
            }

            if self.maze.array[new_start.y as usize][new_start.x as usize] == ch {
                self.find_neighbors(new_start, region_plots, visited);
            }
        }
    }

    fn find_regions(&mut self) {
        let mut visited = vec![vec![false; self.maze.size_x]; self.maze.size_y];

        for y in 0..self.maze.size_y {
            for x in 0..self.maze.size_x {
                if visited[y][x] {
                    continue;
                }

                let mut region_plots: Vec<Point32> = vec![];
                self.find_neighbors(Point32 {x: x as i32, y: y as i32}, &mut region_plots, &mut visited);

                if !region_plots.is_empty() {
                    self.regions.push(Region {plots: region_plots});
                }
            }
        }
    }

    fn get_cost(&self) -> usize {
        let mut cost: usize = 0;
        for region in self.regions.iter() {
            cost += region.area() * region.perimeter(&self.maze);
        }
        return cost;
    }

    fn get_bulk_cost(&self) -> usize {
        let mut cost: usize = 0;
        for region in self.regions.iter() {
            cost += region.area() * region.corners(&self.maze);
        }
        return cost;
    }
}


fn solve_day(file_contents: &str) -> (usize, usize) {
    /*
        Solve day
     */
    let mut garden = Garden::from(file_contents);
    garden.find_regions();
    let sum_part1: usize = garden.get_cost();
    let sum_part2: usize = garden.get_bulk_cost();
    
    (sum_part1, sum_part2)
}


pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    let results = solve_day(&file_contents);
    (format!("{}", results.0), format!("{}", results.1))
}
