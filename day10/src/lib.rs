use std::str::FromStr;

pub struct CPU {
    pub should_draw: bool,
    pub x: i64,
    pub results: Vec<i64>,
    cycle: usize,
}

impl CPU {
    pub fn new(should_draw: bool) -> Self {
        Self { 
            should_draw,
            x: 1, 
            cycle: 0, 
            results: Vec::new(),
        }
    }

    fn cycle(&mut self) {
        self.cycle += 1;  // Increment cycle

        if self.should_draw { self.draw() }  // For part B

        if (self.cycle + 20) % 40 == 0 {  // For part A
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

    pub fn draw(&self) {
        let position = (self.cycle-1) % 40;
            
        if self.x.abs_diff(position as i64) <= 1 {  // If close enough
            print!("#");
        } else {
            print!(".");
        }

        if position == 40-1 {  // New line
            println!();
        }
    }
}

#[derive(Debug)]
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
