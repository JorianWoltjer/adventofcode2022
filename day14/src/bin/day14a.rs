use day14::Cave;

fn main() {
    let input = include_str!("../../input.txt");

    let mut cave = Cave::from_str(input).unwrap();

    let count = cave.drop_max_sand();

    println!("Count: {count}");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        let mut cave = Cave::from_str(input).unwrap();
        // cave.print();

        let count = cave.drop_max_sand();
        // cave.print();

        assert_eq!(count, 24);
    }
}

