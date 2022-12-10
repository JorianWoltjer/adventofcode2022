use day10::{Instruction, CPU};

fn main() {
    let input = include_str!("../../input.txt");

    let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut cpu = CPU::new(true);

    for instruction in instructions {
        cpu.execute(instruction);
    }
}
