use day8::Forest;



fn main() {
    let input = include_str!("../../input.txt");
    let forest: Forest = input.parse().unwrap();

    let visible_count = forest.count_visible_trees().unwrap();

    println!("Count: {visible_count}");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let input = "30373
25512
65332
33549
35390";

        let forest: Forest = input.parse().unwrap();

        let visible_count = forest.count_visible_trees().unwrap();

        assert_eq!(visible_count, 21);
    }
}
