use std::{collections::LinkedList, str::FromStr};

pub trait Monkey {
    fn do_round(&mut self, wrap_around: i64) -> Vec<(usize, i64)>;
    fn push_item(&mut self, item: i64);
}

#[derive(Debug)]
pub struct MonkeyA {
    pub items: LinkedList<i64>,
    pub operation: Operation,
    pub test: Test,
    pub inspected_count: usize,
}
impl Monkey for MonkeyA {
    fn do_round(&mut self, _wrap_around: i64) -> Vec<(usize, i64)> {
        let mut destinations = vec![];

        for _ in 0..self.items.len() {
            let mut item = self.items.pop_front().unwrap();
            self.inspected_count += 1;

            item = self.operation.apply(item);

            item /= 3;

            let new_index = self.test.test(item);
            
            destinations.push((new_index, item));
        }

        destinations
    }

    fn push_item(&mut self, item: i64) {
        self.items.push_back(item)
    }
}
impl FromStr for MonkeyA {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.lines();

        let items = s.next().ok_or("No starting items line found")?
            .split("Starting items: ")
            .last().ok_or("No starting items found")?
            .split(", ").map(|item| item.parse().unwrap()).collect();

        let operation = s.next().ok_or("No operation line found")?
            .split("Operation: new = old ")
            .last().ok_or("No operation found")?
            .parse()?;

        let test = Test::from_lines(s.take(3).collect())?;

        Ok(Self { items, operation, test, inspected_count: 0 })
    }
}

#[derive(Debug)]
pub struct MonkeyB {
    pub items: LinkedList<i64>,
    pub operation: Operation,
    pub test: Test,
    pub inspected_count: usize,
}
impl Monkey for MonkeyB {
    fn do_round(&mut self, wrap_around: i64) -> Vec<(usize, i64)> {
        let mut destinations = vec![];

        for _ in 0..self.items.len() {
            let mut item = self.items.pop_front().unwrap();
            self.inspected_count += 1;

            item = self.operation.apply(item);

            item %= wrap_around;  // Keep it reasonably low for efficiency

            let new_index = self.test.test(item);
            
            destinations.push((new_index, item));
        }

        destinations
    }

    fn push_item(&mut self, item: i64) {
        self.items.push_back(item)
    }
}
impl FromStr for MonkeyB {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.lines();

        let items = s.next().ok_or("No starting items line found")?
            .split("Starting items: ")
            .last().ok_or("No starting items found")?
            .split(", ").map(|item| item.parse().unwrap()).collect();

        let operation = s.next().ok_or("No operation line found")?
            .split("Operation: new = old ")
            .last().ok_or("No operation found")?
            .parse()?;

        let test = Test::from_lines(s.take(3).collect())?;

        Ok(Self { items, operation, test, inspected_count: 0 })
    }
}

#[derive(Debug)]
pub enum Operation {
    ADD(i64),  // new = old + i64
    MULTIPLY(i64),  // new = old * i64
    SQUARE,  // new = old * old
}
impl Operation {
    fn apply(&self, item: i64) -> i64 {
        match self {
            Operation::ADD(value) => item + value,
            Operation::MULTIPLY(value) => item * value,
            Operation::SQUARE => item * item,
        }
    }
}
impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(" ");
        
        match s.next() {
            Some("+") => Ok(Operation::ADD(s.next().ok_or("No argument found for add")?.parse().map_err(|_| "Failed to parse int")?)),
            Some("*") => match s.next() {
                Some("old") => Ok(Operation::SQUARE),
                Some(value) => Ok(Operation::MULTIPLY(value.parse().map_err(|_| "Failed to parse int")?)),
                None => Err("No argument found for multiply".to_string()),
            }
            Some(other) => Err(format!("Operation {other:?} not recognized")),
            None => Err("No operation found".to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Test {
    pub divisor: i64,
    pub if_true: usize,  // Index
    pub if_false: usize,  // Index
}

impl Test {
    fn from_lines(s: Vec<&str>) -> Result<Self, String> {
        let mut s = s.iter();
        let divisor = s.next().ok_or("No divisor found")?
            .split("Test: divisible by ")
            .last().ok_or("No divisor value found")?
            .parse().map_err(|_| "Failed to parse int")?;

        let if_true = s.next().ok_or("No if true statement found")?
            .split("If true: throw to monkey ")
            .last().ok_or("No index found")?
            .parse().map_err(|_| "Failed to parse int")?;

        let if_false = s.next().ok_or("No if true statement found")?
            .split("If false: throw to monkey ")
            .last().ok_or("No index found")?
            .parse().map_err(|_| "Failed to parse int")?;

        Ok(Self { divisor, if_true, if_false })
    }

    pub fn test(&self, item: i64) -> usize {
        if item % self.divisor == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

pub fn do_round<T: Monkey>(monkeys: &mut Vec<T>, wrap_around: i64) {
    for i in 0..monkeys.len() {
        // println!("=== Monkey {i} ===");
        let monkey = &mut monkeys[i];
        let destinations = monkey.do_round(wrap_around);

        for dest in destinations {  // Move items to corresponding other monkeys
            monkeys[dest.0].push_item(dest.1);
        }
    }
}
