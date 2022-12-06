use day5::{Stacks, Instruction};


fn main() {
    let input = include_str!("../../input.txt");

    let mut parts = input.split("\n\n");
        let mut stacks: Stacks = parts.next().unwrap().parse().unwrap();

    for instruction in parts.next().unwrap().lines() {
        let instruction: Instruction = instruction.parse().unwrap();

        stacks.execute_instruction_9001(instruction).unwrap();
    }

    let top_crates = stacks.get_top_crates();
    let top_crates: String = top_crates.iter().map(|maybe_char| maybe_char.unwrap()).collect();
    println!("Result: {top_crates:?}");
}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    use super::*;

    #[test]
    fn example() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let mut parts = input.split("\n\n");
        let mut stacks: Stacks = parts.next().unwrap().parse().unwrap();

        assert_eq!(*stacks.stacks.get(0).unwrap(), LinkedList::from(['Z', 'N']));
        assert_eq!(*stacks.stacks.get(1).unwrap(), LinkedList::from(['M', 'C', 'D']));
        assert_eq!(*stacks.stacks.get(2).unwrap(), LinkedList::from(['P']));

        for instruction in parts.next().unwrap().lines() {
            let instruction: Instruction = instruction.parse().unwrap();

            stacks.execute_instruction_9001(instruction).unwrap();
        }

        assert_eq!(*stacks.stacks.get(0).unwrap(), LinkedList::from(['M']));
        assert_eq!(*stacks.stacks.get(1).unwrap(), LinkedList::from(['C']));
        assert_eq!(*stacks.stacks.get(2).unwrap(), LinkedList::from(['P', 'Z', 'N', 'D']));

        let top_crates = stacks.get_top_crates();
        assert_eq!(top_crates, vec![Some('M'), Some('C'), Some('D')]);

        let top_crates: String = top_crates.iter().map(|maybe_char| maybe_char.unwrap()).collect();
        assert_eq!(top_crates, "MCD");
    }
}
