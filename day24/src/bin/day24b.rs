use day24::Map;

fn main() {
    let input = include_str!("../../input.txt");
    
    let mut map: Map = input.parse().unwrap();

    let mut total = map.find_shortest_path_to_end().unwrap();
    total += map.find_shortest_path_to_start().unwrap();
    total += map.find_shortest_path_to_end().unwrap();
    println!("Total: {total}");  // 844 too low
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let mut total = map.find_shortest_path_to_end().unwrap();
        assert_eq!(total, 18);
        for _ in 0..5 {  // Can't find a path when instantly going, so needs to wait a bit
            map.do_round();
            total += 1;
        }
        total += map.find_shortest_path_to_start().unwrap();
        assert_eq!(total, 18 + 23);  // 41
        total += map.find_shortest_path_to_end().unwrap();
        assert_eq!(total, 18 + 23 + 13);  // 54
    }
}
