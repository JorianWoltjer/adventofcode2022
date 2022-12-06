use day1::sum_elves;

const TOP: usize = 3;  // Number of top items

fn main() {
    let input = include_str!("../../input.txt");

    let mut elves = sum_elves(input).expect("Should be formatted correctly");

    elves.sort();  // Biggest values will be at the end
    let top = &elves[elves.len()-TOP..];

    println!("Top 3: {:?}", top);

    let sum: i64 = top.iter().sum();

    println!("Sum: {}", sum);
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

