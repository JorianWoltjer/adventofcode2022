use day13::{compare, parse_input};
use std::cmp::Ordering;

fn main() {
    let input = include_str!("../../input.txt");
    
    let matches = parse_input(input);

    let sum: usize = matches.iter().enumerate().map(|(i, (left, right))| {
        match compare(left, right) {
            Ordering::Less | Ordering::Equal => i+1,
            Ordering::Greater => 0
        }
    }).sum();

    println!("Total: {sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]

[1,[0,0,0],3]
[1,[0,0,0],2]";

        let answers = [true, true, false, true, false, true, false, false, false];

        let matches = parse_input(input);

        let mut sum = 0;
        for (i, (left, right)) in matches.iter().enumerate() {
            let answer = match compare(left, right) {
                Ordering::Less | Ordering::Equal => true,
                Ordering::Greater => false
            };
            assert_eq!(answer, answers[i], "{i}: {left} VS {right}");
            if answer {
                sum += i+1;  // 1-indexed
            }
        }
        assert_eq!(sum, 13);
    }
}
