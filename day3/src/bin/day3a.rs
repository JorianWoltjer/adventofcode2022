use day3::{split, find_shared_in_two, priority};

fn main() {
    let input = include_str!("../../input.txt");

    let mut total_priority: u64 = 0;
    for s in input.lines() {
        let (a, b) = split(s);

        let shared = find_shared_in_two(a, b).expect("Should have a shared character");

        total_priority += priority(shared) as u64;
    }

    println!("Total: {}", total_priority);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s = "vJrwpWtwJgWrhcsFMMfFFhFp";

        let (a, b) = split(s);

        assert_eq!(a, "vJrwpWtwJgWr");
        assert_eq!(b, "hcsFMMfFFhFp");

        let shared = find_shared_in_two(a, b);

        assert_eq!(shared, Some('p'));
    }

    #[test]
    fn all_examples() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let mut total_priority = 0;
        for s in input.lines() {
            let (a, b) = split(s);

            let shared = find_shared_in_two(a, b).unwrap();

            total_priority += priority(shared);
        }

        assert_eq!(total_priority, 157);
    }
}
