use day24::Map;

fn main() {
    let input = include_str!("../../input.txt");
    
    let mut map: Map = input.parse().unwrap();

    let shortest_path = map.find_shortest_path_to_end().unwrap();
    println!("Shortest: {shortest_path}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_example() {
        let input = "\
#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#";

        let mut map: Map = input.parse().unwrap();
        println!("== Initial ==");
        map.print();
        for i in 1..=5 {
            println!("== Round {i} ==");
            map.do_round();
            map.print();
        }
    }

    #[test]
    fn example() {
        let input = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

        let mut map: Map = input.parse().unwrap();
        dbg!(&map);
        // println!("== Initial ==");
        // map.print();
        // for i in 1..=18 {
        //     println!("== Round {i} ==");
        //     map.do_round();
        //     map.print();
        // }

        let shortest_path = map.find_shortest_path_to_end();
        assert_eq!(shortest_path, Some(18));
    }
}
