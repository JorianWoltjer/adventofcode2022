use day11::{MonkeyB, do_round};

fn main() {
    let input = include_str!("../../input.txt");

    let mut monkeys: Vec<MonkeyB> = input.split("\n\n").map(|monkey| {
        let monkey = monkey.split_once("\n").unwrap().1;  // Remove first "Monkey" line
        monkey.parse().unwrap()  // Parse attributes
    }).collect();

    // Calculate maximum modulo to make calculations smaller and faster
    let wrap_around = monkeys.iter().map(|monkey| monkey.test.divisor)
        .reduce(|accum, item| accum * item).unwrap();

    // Do 10000 rounds
    for _ in 0..10000 {
        do_round(&mut monkeys, wrap_around);
    }

    // Highest first
    monkeys.sort_by(|a, b| b.inspected_count.cmp(&a.inspected_count));

    // Mulitply top 2 together
    let monkey_business = monkeys.iter().map(|monkey| monkey.inspected_count)
        .take(2).reduce(|accum, item| accum * item).unwrap();
    
    println!("Monkey business: {monkey_business}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        // Parse into monkey structs
        let mut monkeys: Vec<MonkeyB> = input.split("\n\n").map(|monkey| {
            let monkey = monkey.split_once("\n").unwrap().1;  // Remove first "Monkey" line
            monkey.parse().unwrap()  // Parse attributes
        }).collect();

        // Calculate maximum modulo to make calculations smaller and faster
        let wrap_around = monkeys.iter().map(|monkey| monkey.test.divisor)
            .reduce(|accum, item| accum * item).unwrap();

        // Do 10000 rounds
        for _ in 0..10000 {
            do_round(&mut monkeys, wrap_around);
        }

        // Highest first
        monkeys.sort_by(|a, b| b.inspected_count.cmp(&a.inspected_count));

        // Mulitply top 2 together
        let monkey_business = monkeys.iter().map(|monkey| monkey.inspected_count)
            .take(2).reduce(|accum, item| accum * item).unwrap();
        
        assert_eq!(monkey_business, 2713310158);
    }
}
