#[macro_use]
extern crate lazy_static;

use std::{str::FromStr, cmp::{min, max}, ops::RangeInclusive, collections::HashSet};

use regex::Regex;

#[derive(Debug)]
pub struct Sensor {
    x: isize,
    y: isize,
    closest_beacon: Beacon,
    distance: usize,
}
impl FromStr for Sensor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
        }

        let cap = RE.captures(s).ok_or("No matches found")?;
        let (sensor_x, sensor_y, beacon_x, beacon_y): (isize, isize, isize, isize) = (
            cap[1].parse().map_err(|_| format!("Failed to parse {:?} as an int", &cap[1]))?, 
            cap[2].parse().map_err(|_| format!("Failed to parse {:?} as an int", &cap[2]))?,
            cap[3].parse().map_err(|_| format!("Failed to parse {:?} as an int", &cap[3]))?,
            cap[4].parse().map_err(|_| format!("Failed to parse {:?} as an int", &cap[4]))?,
        );

        let beacon = Beacon { x: beacon_x, y: beacon_y };
        let distance = beacon.distance_to(sensor_x, sensor_y);

        Ok(Self {
            x: sensor_x,
            y: sensor_y,
            closest_beacon: beacon,
            distance
        })
    }
}

#[derive(Debug)]
pub struct Beacon {
    x: isize,
    y: isize,
}
impl Beacon {
    /// Manhattan distance
    pub fn distance_to(&self, x: isize, y: isize) -> usize {
        self.x.abs_diff(x) + self.y.abs_diff(y)
    }
}

pub fn print_sensors(sensors: Vec<Sensor>) {
    let left = sensors.iter().map(|sensor| sensor.x).min().unwrap();
    let left = min(left, sensors.iter().map(|sensor| sensor.closest_beacon.x).min().unwrap());
    let right = sensors.iter().map(|sensor| sensor.x).max().unwrap();
    let right = max(right, sensors.iter().map(|sensor| sensor.closest_beacon.x).max().unwrap());
    let top = sensors.iter().map(|sensor| sensor.y).min().unwrap();
    let top = min(top, sensors.iter().map(|sensor| sensor.closest_beacon.y).min().unwrap());
    let bottom = sensors.iter().map(|sensor| sensor.y).max().unwrap();
    let bottom = max(bottom, sensors.iter().map(|sensor| sensor.closest_beacon.y).max().unwrap());

    let mut grid = Vec::new();
    for _ in top..=bottom {
        let mut row = Vec::new();
        for _ in left..=right {
            row.push('.');
        }
        grid.push(row);
    }

    for sensor in sensors {
        grid[top.abs_diff(sensor.y)][left.abs_diff(sensor.x)] = 'S';
        grid[top.abs_diff(sensor.closest_beacon.y)][left.abs_diff(sensor.closest_beacon.x)] = 'B';
    }

    for row in grid {
        for cell in row {
            print!("{cell}");
        }
        println!();
    }
}

/// For part A, calculates the number of filled squares in a certain row
pub fn calculate_for_y(sensors: &Vec<Sensor>, y: isize) -> usize {
    let mut ranges = CombinedRanges::new();

    for sensor in sensors {
        let overlap = sensor.distance as isize - sensor.y.abs_diff(y) as isize;  // y difference is falloff

        if overlap >= 0 {
            ranges.add(sensor.x-overlap..=sensor.x+overlap);
        }
    }

    ranges.reduce();
    ranges.total_size()
}

/// For part B, reduces ranges for row and returns Some(x, y) when it found a hole
pub fn find_hole(sensors: &Vec<Sensor>, y: isize) -> Option<(isize, isize)> {
    let mut ranges = CombinedRanges::new();

    for sensor in sensors {
        let overlap = sensor.distance as isize - sensor.y.abs_diff(y) as isize;  // y difference is falloff

        if overlap >= 0 {
            ranges.add(sensor.x-overlap..=sensor.x+overlap);
        }
    }

    ranges.reduce();

    if ranges.ranges.len() > 1 {
        let x = min(ranges.ranges[0].end(), ranges.ranges[1].end()) + 1;

        return Some((x, y));
    }

    None
}

#[derive(Debug)]
pub struct CombinedRanges {
    ranges: Vec<RangeInclusive<isize>>,
}
impl CombinedRanges {
    pub fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    pub fn add(&mut self, new_range: RangeInclusive<isize>) {
        self.ranges.push(new_range);
    }

    /// Remove all overlapping ranges
    pub fn reduce(&mut self) {
        loop {
            let len_before = self.ranges.len();
            let mut unique_ranges = HashSet::new();

            for range in &self.ranges {
                let mut range_start = *range.start();
                let mut range_end = *range.end();
                for compared_range in &self.ranges {
                    if range == compared_range { continue; }
                    
                    if compared_range.contains(&(range_start-1)) {
                        range_start = *compared_range.start();
                    }
                    if compared_range.contains(&(range_end+1)) {
                        range_end = *compared_range.end();
                    }
                }
                unique_ranges.insert(range_start..=range_end);
            }

            self.ranges = Vec::from_iter(unique_ranges.into_iter());

            if len_before == self.ranges.len() {  // If no change was made
                break;
            }
        }
    }

    pub fn total_size(&self) -> usize {
        self.ranges.iter().map(|range| range.start().abs_diff(*range.end())).sum()
    }
}
