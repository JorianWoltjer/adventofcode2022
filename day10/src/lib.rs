use std::str::FromStr;

pub struct CPU {
    pub x: i64,
    cycle: usize,
    pub results: Vec<i64>,
}

impl CPU {
    pub fn new() -> Self {
        Self { x: 1, cycle: 0, results: Vec::new() }
    }

    fn cycle(&mut self) {
        self.cycle += 1;

        if (self.cycle + 20) % 40 == 0 {
            self.results.push(self.cycle as i64 * self.x);  // Save for retrieval later
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::NOOP => self.cycle(),
            Instruction::ADDX(value) => {
                self.cycle();
                self.cycle();
                self.x += value;
            }
        }
    }
}

pub enum Instruction {
    ADDX(i64),
    NOOP,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(" ");
        match s.next() {
            Some("noop") => Ok(Instruction::NOOP),
            Some("addx") => Ok(Instruction::ADDX(s.next().ok_or("No argument provided for `addx`")?.parse().map_err(|_| "Failed to parse int")?)),
            Some(instruction) => Err(format!("Instruction {instruction:?} not found")),
            None => Err("No instruction found".to_string()),
        }
    }
}
