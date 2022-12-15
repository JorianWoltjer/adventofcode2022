use day15::{Sensor, find_hole};

fn main() {
    let input = include_str!("../../input.txt");

    let sensors: Vec<Sensor> = input.lines().map(|line| line.parse().unwrap()).collect();

    for y in 0..=4_000_000 {
        if let Some((x, y)) = find_hole(&sensors, y) {
            println!("Found ({x}, {y}) = {}", x * 4_000_000 + y);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"; 

        let sensors: Vec<Sensor> = input.lines().map(|line| line.parse().unwrap()).collect();

        for y in 0..=20 {
            if y == 11 {
                assert_eq!(find_hole(&sensors, y), Some((14, 11)));
            } else {
                assert_eq!(find_hole(&sensors, y), None, "y={y}");
            }
        }
    }
}
