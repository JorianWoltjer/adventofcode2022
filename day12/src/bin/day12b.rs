use day12::Hill;

fn main() {
    let input = include_str!("../../input.txt");

    let hill: Hill = input.parse().unwrap();

    let shortest_path = hill.find_shortest_path_from_anywhere().unwrap();
    println!("Shortest path from anywhere: {shortest_path}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

        let hill: Hill = input.parse().unwrap();

        assert_eq!(hill.find_shortest_path_from_anywhere(), Some(29));
    }
}
