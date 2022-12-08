use day8::Forest;

fn main() {
    let input = include_str!("../../input.txt");
    let forest: Forest = input.parse().unwrap();

    let best_score = forest.find_best_scenic_score();

    println!("Best: {best_score}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn example() {
        let forest: Forest = INPUT.parse().unwrap();

        let score = forest.get_scenic_score(2, 1);
        assert_eq!(score, 4);

        let score = forest.get_scenic_score(2, 3);
        assert_eq!(score, 8);

        let best = forest.find_best_scenic_score();
        assert_eq!(best, 8);
    }
}
