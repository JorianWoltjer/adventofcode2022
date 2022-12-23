use day23::Map;

fn main() {
    let input = include_str!("../../input.txt");

    let mut map: Map = input.parse().unwrap();
        
    let mut prev_map = Vec::new();
    
    let mut i = 0;
    while map.elves != prev_map {
        prev_map = map.elves.clone();
        map.do_round();
        i += 1;
    }
    println!("Round: {i}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

        let mut map: Map = input.parse().unwrap();
        
        let mut prev_map = Vec::new();
        
        let mut i = 0;
        while map.elves != prev_map {
            prev_map = map.elves.clone();
            map.do_round();
            i += 1;
        }

        assert_eq!(i, 20);
    }
}
