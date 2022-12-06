use day3::{find_shared_in_all, priority};


fn main() {
    let input = include_str!("../../input.txt");

    let list: Vec<&str> = input.lines().collect();

        let mut total_priority = 0;
        for chunk in list.chunks(3) {
            let shared = find_shared_in_all(chunk.to_vec()).unwrap();

            total_priority += priority(shared);
        }

    println!("Total: {}", total_priority);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg";

        let shared = find_shared_in_all(s.lines().collect());

        assert_eq!(shared, Some('r'));
    }

    #[test]
    fn example_2() {
        let s = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let shared = find_shared_in_all(s.lines().collect());

        assert_eq!(shared, Some('Z'));
    }

    #[test]
    fn all_examples() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let list: Vec<&str> = input.lines().collect();

        let mut total_priority = 0;
        for chunk in list.chunks(3) {
            let shared = find_shared_in_all(chunk.to_vec()).unwrap();

            total_priority += priority(shared);
        }

        assert_eq!(total_priority, 70);
    }
}
