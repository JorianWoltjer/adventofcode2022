#![feature(iter_advance_by)]

use std::{collections::HashSet, cmp::{min, max}};

#[derive(Debug)]
pub struct Cave {
    pub obstacles: HashSet<(usize, usize)>,
    pub area: Area,
}

impl Cave {
    pub fn new() -> Self {
        Self { 
            obstacles: HashSet::new(), 
            area: Area { top: 0, bottom: 0, left: 0, right: 0 }
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        let mut cave = Cave::new();

        s.lines().for_each(|line| {
            let coordinates_str: Vec<&str> = line.split(" -> ").collect();

            for i in 1..coordinates_str.len() {
                let start = parse_coordinate(coordinates_str[i-1]).unwrap();
                let end = parse_coordinate(coordinates_str[i]).unwrap();

                cave.add_line(start, end);
            }
        });

        cave.calculate_area();

        Ok(cave)
    }

    fn calculate_area(&mut self) {
        let top = 0;
        let (mut bottom, mut left, mut right) = (usize::MIN, usize::MAX, usize::MIN);

        for point in &self.obstacles {
            left = min(left, point.0);
            right = max(right, point.0);
            bottom = max(bottom, point.1);
        }

        self.area = Area { top, bottom, left, right };
    }

    fn add_line(&mut self, start: (usize, usize), end: (usize, usize)) {
        if start.0 == end.0 {  // x = same, vertical line
            let min = min(start.1, end.1);
            let max = max(start.1, end.1);
            for y in min..=max {
                self.obstacles.insert((start.0, y));
            }
        } else if start.1 == end.1 {  // y = same, horizontal line
            let min = min(start.0, end.0);
            let max = max(start.0, end.0);
            for x in min..=max {
                self.obstacles.insert((x, start.1));
            }
        } else {
            unimplemented!();
        }
    }

    pub fn drop_max_sand(&mut self) -> usize {
        let mut count = 0;
        while self.drop_sand() {  // Breaks if sand falls into abyss
            count += 1;
        }
        count
    }

    /// Returns true if the sand was placed somewhere, and false if the sand fell into the abyss
    pub fn drop_sand(&mut self) -> bool {
        let mut sand_y = 0;
        let mut sand_x = 500;

        loop {
            if !self.obstacles.contains(&(sand_x, sand_y+1)) {  // If square below is empty
                sand_y += 1;
                if sand_y > self.area.bottom {  // If fell into the abyss
                    return false;  // Not placed
                }
            } else if !self.obstacles.contains(&(sand_x-1, sand_y+1)) {  // Check below + left
                sand_y += 1;
                sand_x -= 1;
            } else if !self.obstacles.contains(&(sand_x+1, sand_y+1)) {  // Check below + right
                sand_y += 1;
                sand_x += 1;
            } else {  // No more space
                self.obstacles.insert((sand_x, sand_y));
                return true;
            }
        }
    }

    pub fn drop_max_sand_floor(&mut self) -> usize {
        let mut count = 0;
        while self.drop_sand_floor() {  // Breaks if sand blocks start
            count += 1;
        }
        count
    }

    /// For part B, sand stops at the floor at bottom+2 and returns false if the starting 500,0 is blocked
    pub fn drop_sand_floor(&mut self) -> bool {
        let mut sand_y = 0;
        let mut sand_x = 500;

        if self.obstacles.contains(&(sand_x, sand_y)) {  // If start blocked
            return false;
        }

        loop {
            if sand_y+1 >= self.area.bottom+2 {  // If sand will hit floor
                self.obstacles.insert((sand_x, sand_y));
                return true;
            }

            if !self.obstacles.contains(&(sand_x, sand_y+1)) {  // If square below is empty
                sand_y += 1;
            } else if !self.obstacles.contains(&(sand_x-1, sand_y+1)) {  // Check below + left
                sand_y += 1;
                sand_x -= 1;
            } else if !self.obstacles.contains(&(sand_x+1, sand_y+1)) {  // Check below + right
                sand_y += 1;
                sand_x += 1;
            } else {  // No more space
                self.obstacles.insert((sand_x, sand_y));
                return true;
            }
        }
    }

    pub fn print(&self) {
        for y in self.area.top..=self.area.bottom {
            for x in self.area.left..=self.area.right {
                if self.obstacles.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

pub fn parse_coordinate(s: &str) -> Result<(usize, usize), String> {
    let mut split = s.split(",");

    Ok((split.next().ok_or("No first coordinate found")?.parse().map_err(|_| "Failed to parse int")?, 
        split.next().ok_or("No second coordinate found")?.parse().map_err(|_| "Failed to parse int")?))
}

#[derive(Debug)]
pub struct Area {
    pub top: usize,
    pub bottom: usize,
    pub left: usize,
    pub right: usize,
}
