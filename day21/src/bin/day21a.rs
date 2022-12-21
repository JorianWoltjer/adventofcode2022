use day21::Monkeys;

fn main() {
    let input = include_str!("../../input.txt");
    
    let monkeys: Monkeys = input.parse().unwrap();

    let root_value = monkeys.get_value(&String::from("root")).unwrap();

    println!("Value: {root_value}");
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

        let root_value = monkeys.get_value(&String::from("root"));

        assert_eq!(root_value, Some(152));
    }
}

