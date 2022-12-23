use day23::Map;

fn main() {
    let input = include_str!("../../input.txt");

    let mut map: Map = input.parse().unwrap();
    
    // 10 rounds
    for _ in 0..10 {
        map.do_round();
    }

    let count = map.count_empty_tiles();
    println!("Count: {count}");
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
        
        println!("== Initial ==");
        map.print();
        for i in 0..5 {  // First 5
            map.do_round();
            println!("== Round {} ==", i+1);
            map.print();
        }
        for _ in 0..5 {  // 5 more
            map.do_round();
        }
        println!("== Final ==");
        map.print();

        assert_eq!(map.count_empty_tiles(), 110);
    }

    #[test]
    fn small_example() {
        let input = "\
.....
..##.
..#..
.....
..##.
.....";

        let mut map: Map = input.parse().unwrap();

        map.print();
        for _ in 0..3 {
            map.do_round();
            map.print();
        }
    }
}

