use std::{str::{FromStr}, collections::HashSet};

#[derive(Debug)]
pub struct Hill {
    start: Coordinate,
    end: Coordinate,
    width: usize,
    height: usize,
    map: Vec<Vec<u8>>,
}
impl FromStr for Hill {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let height = s.len();
        let width = s.first().ok_or("Empty input")?.len();

        let mut start = None;
        let mut end = None;
        let mut map = Vec::new();

        for y in 0..height {
            let mut row = Vec::new();

            for x in 0..width {
                let value = match s[y][x] {
                    'S' => {
                        start = Some(Coordinate { x, y });
                        1
                    }
                    'E' => {
                        end = Some(Coordinate { x, y });
                        26
                    }
                    other => {
                        other as u8 - b'a' + 1
                    }
                };

                row.push(value);
            }
            map.push(row);
        }

        Ok(Self { 
            start: start.ok_or("No start position found")?, 
            end: end.ok_or("No end position found")?, 
            width, height, map,
        })
    }
}
impl Hill {
    // TODO: implement from here: https://en.wikipedia.org/wiki/Breadth-first_search
    // TODO: for part B, simply pass all 'a' nodes as root
    pub fn find_shortest_path(&self) -> Option<usize> {
        let mut visited = HashSet::new();
        let mut next_depth;

        visited.insert((self.start.x, self.start.y));
        next_depth = visited.clone();

        for i in 1..self.width*self.height {  // Depths
            let mut new_next_depth = HashSet::new();
            for coord in next_depth.iter() {
                // Find next steps
                let steps = self.get_steps(coord.0, coord.1, &visited);
                for step in steps {
                    if step.0 == self.end.x && step.1 == self.end.y {  // If found end
                        return Some(i);
                    }
                    visited.insert(step);
                    new_next_depth.insert(step);
                }
            }
            next_depth = new_next_depth;
        }

        None
    }

    fn get_steps(&self, x: usize, y: usize, visited: &HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
        let value = self.map[y][x];

        [(x as isize + 1, y as isize),
        (x as isize - 1, y as isize),
        (x as isize, y as isize + 1),
        (x as isize, y as isize - 1)].iter().filter(|coord| {
            coord.0 >= 0 && coord.1 >= 0 && coord.0 < self.width as isize && coord.1 < self.height as isize &&
            self.map[coord.1 as usize][coord.0 as usize] <= value+1 &&
            !visited.contains(&(coord.0 as usize, coord.1 as usize))
        }).map(|coord| {
            (coord.0 as usize, coord.1 as usize)
        }).collect()
    }

    pub fn find_shortest_path_from_anywhere(&mut self) -> Option<usize> {
        let mut min = None;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.map[y][x] == 1 {  // Start at lowest
                    self.start = Coordinate { x, y };
                    let score = self.find_shortest_path();
                    // If new best
                    if score.is_some() && (min.is_none() || score.unwrap() < min.unwrap()) {
                        min = score;
                    }
                }
            }
        }

        min
    }
}

#[derive(Debug)]
pub struct Coordinate {
    x: usize,
    y: usize,
}
