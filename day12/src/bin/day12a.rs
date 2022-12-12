use day12::Hill;

fn main() {
    let input = include_str!("../../input.txt");

    let hill: Hill = input.parse().unwrap();

    let shortest_path = hill.find_shortest_path_from_start().unwrap();
    println!("Shortest path: {shortest_path}");
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

        let shortest_path = hill.find_shortest_path_from_start();
        assert_eq!(shortest_path, Some(31));
    }
}
