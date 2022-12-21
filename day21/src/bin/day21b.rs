use day21::Monkeys;

fn main() {
    let input = include_str!("../../input.txt");
    
    let monkeys: Monkeys = input.parse().unwrap();

    let human_value = monkeys.find_human_value();

    println!("Value: {human_value}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

        let monkeys: Monkeys = input.parse().unwrap();

        let human_value = monkeys.find_human_value();

        assert_eq!(human_value, 301);
    }
}
