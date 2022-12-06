#[macro_use]
extern crate lazy_static;

use std::{collections::LinkedList, str::FromStr};

use regex::Regex;


pub struct Stacks {
    pub stacks: Vec<LinkedList<char>>
}

impl Stacks {
    pub fn execute_instruction_9000(&mut self, instruction: Instruction) -> Result<(), String> {
        for _ in 0..instruction.amount {
            let from_stack = self.stacks.get_mut(instruction.from)
                .ok_or(format!("Stack index {} not found", instruction.from))?;
            
            let item = from_stack.pop_back().ok_or(format!("Stack {} is empty", instruction.from))?;

            let to_stack = self.stacks.get_mut(instruction.to)
                .ok_or(format!("Stack index {} not found", instruction.to))?;
            
            to_stack.push_back(item);
        }

        Ok(())
    }

    pub fn execute_instruction_9001(&mut self, instruction: Instruction) -> Result<(), String> {
        let from_stack = self.stacks.get_mut(instruction.from)
            .ok_or(format!("Stack index {} not found", instruction.from))?;
        
        let mut items = Vec::new();
        for _ in 0..instruction.amount {
            items.push(from_stack.pop_back().ok_or(format!("Stack {} is empty", instruction.from))?);
        }

        let to_stack = self.stacks.get_mut(instruction.to)
            .ok_or(format!("Stack index {} not found", instruction.to))?;

        for item in items.iter().rev() {
            to_stack.push_back(*item);
        }

        Ok(())
    }

    pub fn get_top_crates(&self) -> Vec<Option<char>> {
        let mut top = Vec::new();
        for stack in &self.stacks {
            top.push(stack.back().cloned());
        }
        top
    }
}

impl FromStr for Stacks {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().rev().collect();
        let length = lines.get(1).ok_or("No first line")?.len();
        
        let mut stacks = Vec::new();
    
        for i in (0..length).step_by(4) {
            let mut items = LinkedList::new();
            for line in lines.iter() {
                let part = &line[i..i+3];
                if part.starts_with('[') {
                    items.push_back(part.chars().nth(1).ok_or("No letter found in brackets")?);
                }
            }
        
            stacks.push(items);
        }
        
        Ok(Stacks { stacks })
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        }
        let cap = RE.captures(s).ok_or(format!("{s:?} did not match Regex"))?;
        let (amount, from, to): (usize, usize, usize) = (
            cap[1].parse().map_err(|_| format!("Failed to parse {:?} as an int", &cap[1]))?, 
            cap[2].parse().map_err(|_| format!("Failed to parse {:?} as an int", &cap[2]))?,
            cap[3].parse().map_err(|_| format!("Failed to parse {:?} as an int", &cap[3]))?,
        );
        
        Ok(Self { amount, from: from - 1, to: to - 1 })  // `to` and `from` are 0-indexed
    }
}
