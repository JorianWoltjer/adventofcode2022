use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub struct Monkeys {
    monkeys: HashMap<String, Monkey>,
}
impl FromStr for Monkeys {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut monkeys = HashMap::new();
        
        for line in s.lines() {
            let mut line = line.split(": ");
            let (name, job): (&str, Job) = (
                line.next().ok_or("No name found")?,
                line.next().ok_or("No job found")?.parse()?,
            );
            monkeys.insert(name.to_string(), Monkey { job });
        }
        
        Ok(Self { monkeys })
    }
}
impl Monkeys {
    pub fn get_value(&self, monkey: &String) -> Option<isize> {
        let job = &self.monkeys.get(monkey)?.job;
        
        match job {
            Job::CONSTANT(value) => Some(*value),
            Job::OPERATION(name1, operation, name2) => {
                let value1 = self.get_value(name1)?;
                let value2 = self.get_value(name2)?;

                Some(operation.apply(value1, value2))
            }
        }
    }

    /// Only returns Some(value) if the calculation does not include "humn"
    pub fn get_value_without_human(&self, monkey: &String) -> Option<isize> {
        if monkey == "humn" {
            return None;
        }

        let job = &self.monkeys.get(monkey)?.job;
        
        match job {
            Job::CONSTANT(value) => Some(*value),
            Job::OPERATION(name1, operation, name2) => {
                let value1 = self.get_value_without_human(name1)?;
                let value2 = self.get_value_without_human(name2)?;

                Some(operation.apply(value1, value2))
            }
        }
    }

    pub fn find_human_value(&self) -> isize {
        let job = &self.monkeys.get("root").unwrap().job;

        match job {
            Job::OPERATION(name1, _operation, name2) => {
                // One of these will be None
                let value1 = self.get_value_without_human(name1);
                let value2 = self.get_value_without_human(name2);

                match (value1, value2) {
                    (None, Some(value)) => self.solve_unknown(name1, value),
                    (Some(value), None) => self.solve_unknown(name2, value),
                    (_, _) => panic!("No or multiple unknowns found, should only be one"),
                }
            }
            Job::CONSTANT(_) => panic!("root cannot have a constant value")
        }
    }

    pub fn solve_unknown(&self, unknown: &String, prev_value: isize) -> isize {
        let job = &self.monkeys.get(unknown).unwrap().job;

        match job {
            Job::OPERATION(name1, operation, name2) => {
                // One of these will be None
                let value1 = self.get_value_without_human(name1);
                let value2 = self.get_value_without_human(name2);

                // Do inverse operation
                match (value1, operation, value2) {
                    (None, Operation::ADD, Some(constant)) => self.solve_unknown(name1, prev_value - constant),
                    (Some(constant), Operation::ADD, None) => self.solve_unknown(name2, prev_value - constant),
                    (None, Operation::SUBTRACT, Some(constant)) => self.solve_unknown(name1, prev_value + constant),
                    (Some(constant), Operation::SUBTRACT, None) => self.solve_unknown(name2, constant - prev_value),
                    (None, Operation::MULTIPLY, Some(constant)) => self.solve_unknown(name1, prev_value / constant),
                    (Some(constant), Operation::MULTIPLY, None) => self.solve_unknown(name2, prev_value / constant),
                    (None, Operation::DIVIDE, Some(constant)) => self.solve_unknown(name1, prev_value * constant),
                    (Some(constant), Operation::DIVIDE, None) => self.solve_unknown(name2, constant / prev_value),
                    (_, _, _) => panic!("No or multiple unknowns found, should only be one")
                }
            }
            Job::CONSTANT(_value) => {
                prev_value
            }
        }
    }
}

#[derive(Debug)]
pub struct Monkey {
    job: Job,
}

#[derive(Debug)]
pub enum Job {
    CONSTANT(isize),
    OPERATION(String, Operation, String)
}
impl FromStr for Job {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(" ");
        match (s.next(), s.next(), s.next()) {
            (Some(value), None, None) => Ok(
                Job::CONSTANT(value.parse().map_err(|_| "Failed to parse int")?)
            ),
            (Some(name1), Some(operation), Some(name2)) => Ok(
                Job::OPERATION(name1.to_string(), operation.parse()?, name2.to_string())
            ),
            (_, _, _) => Err("Not recognized".to_string())
        }
    }
}

#[derive(Debug)]
pub enum Operation {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}
impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::ADD),
            "-" => Ok(Operation::SUBTRACT),
            "*" => Ok(Operation::MULTIPLY),
            "/" => Ok(Operation::DIVIDE),
            other => Err(format!("Operation {other:?} not regognized"))
        }
    }
}
impl Operation {
    pub fn apply(&self, left: isize, right: isize) -> isize {
        match self {
            Operation::ADD => left + right,
            Operation::SUBTRACT => left - right,
            Operation::MULTIPLY => left * right,
            Operation::DIVIDE => left / right,
        }
    }
}
