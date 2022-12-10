use day10::{Instruction, CPU};


fn main() {
    let input = include_str!("../../input.txt");

    let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut cpu = CPU::new(false);

    for instruction in instructions {
        cpu.execute(instruction);
    }

    let total: i64 = cpu.results.iter().sum();
    println!("Total: {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
noop
addx 3
addx -5";

        let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();

        let mut cpu = CPU::new(false);

        for instruction in instructions {
            cpu.execute(instruction);
        }

        assert_eq!(cpu.x, -1);
    }

    #[test]
    fn example_2() {
        let input = include_str!("../../test.txt");  // Too big to include here

        let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();

        let mut cpu = CPU::new(false);

        for instruction in instructions {
            cpu.execute(instruction);
        }

        assert_eq!(cpu.results, vec![420, 1140, 1800, 2940, 2880, 3960]);
        assert_eq!(cpu.results.iter().sum::<i64>(), 13140);
    }
}
