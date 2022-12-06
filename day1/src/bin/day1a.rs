use day1::sum_elves;

fn main() {
    let input = include_str!("../../input.txt");

    let elves = sum_elves(input).expect("Should be formatted correctly");

    let max = elves.iter().max().unwrap();

    println!("Max: {}", max);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let elves = sum_elves(input).unwrap();

        let max = elves.iter().max();

        assert_eq!(max, Some(&24000));
    }
}
