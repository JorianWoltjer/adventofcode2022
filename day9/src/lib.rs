use std::{str::FromStr, collections::{HashSet, LinkedList}};

const DEBUG: bool = false;

#[derive(Debug)]
pub struct Knot {
    pub x: isize,
    pub y: isize,
}

impl Knot {
    pub fn is_in_range_of(&self, other: &Knot) -> bool {
        self.x.abs_diff(other.x) <= 1 &&
        self.y.abs_diff(other.y) <= 1
    }

    pub fn needs_diagonal_move_to(&self, other: &Knot) -> bool {
        self.x != other.x && self.y != other.y
    }

    pub fn move_into(&mut self, direction: &Direction) {
        match direction {
            Direction::RIGHT => self.x += 1,
            Direction::LEFT => self.x -= 1,
            Direction::UP => self.y += 1,
            Direction::DOWN => self.y -= 1,
        }
    }
}

#[derive(Debug)]
pub struct Rope {
    head: Knot,
    tail: Knot,
    pub visited_positions: HashSet<(isize, isize)>,
}

impl Rope {
    pub fn new() -> Self {
        Self {
            head: Knot { x: 0, y: 0 },
            tail: Knot { x: 0, y: 0 },
            visited_positions: HashSet::new(),
        }
    }

    pub fn update_tail(&mut self, head_direction: &Direction) {
        if !self.tail.is_in_range_of(&self.head) {  // If not in range, needs to move
            if self.tail.needs_diagonal_move_to(&self.head) {  // Edge case for if the movement needs to be diagonal to catch up
                match head_direction {  // Diagonal move
                    Direction::RIGHT | Direction::LEFT => self.tail.y = self.head.y,
                    Direction::UP | Direction::DOWN => self.tail.x = self.head.x,
                }
            }
            self.tail.move_into(head_direction);  // Do the same
        }
    }

    pub fn move_head(&mut self, direction: &Direction, amount: usize) {
        if DEBUG { println!("Movement: {amount} {direction:?}"); }
        
        for _ in 0..amount {
            self.head.move_into(direction);
            
            self.update_tail(&direction);
            self.visited_positions.insert((self.tail.x, self.tail.y));
            if DEBUG { self.print(); }
        }
    }

    pub fn print(&self) {
        let mut grid: Vec<Vec<char>> = Vec::new();
        for y in 0..5 {
            let mut row: Vec<char> = Vec::new();
            for x in 0..6 {
                if y == self.head.y && x == self.head.x {
                    row.push('H');
                } else if y == self.tail.y && x  == self.tail.x {
                    row.push('T');
                } else if self.visited_positions.contains(&(x, y)) {
                    row.push('#');
                } else {
                    row.push('.');
                }
            }
            grid.push(row);
        }

        for row in grid.iter().rev() {
            row.iter().for_each(|c| print!("{c}"));
            println!();
        }
        println!();
    }
}

#[derive(Debug)]
pub struct LongRope {
    pieces: LinkedList<Rope>,
}

impl LongRope {
    pub fn new(length: usize) -> Self {
        Self {
            pieces: (0..length).map(|_| Rope::new()).collect()
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    RIGHT,
    LEFT,
    UP,
    DOWN,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::UP),
            "D" => Ok(Direction::DOWN),
            "L" => Ok(Direction::LEFT),
            "R" => Ok(Direction::RIGHT),
            _ => Err(format!("Direction {s:?} not recognized"))
        }
    }
}

pub fn parse_input(input: &str) -> Vec<(Direction, usize)> {
    input.lines().map(|line| {
        line.split(" ")
    }).map(|mut split| {
        (split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap())
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_range() {
        let a = Knot { x: 0, y: 0 };
        let b = Knot { x: 1, y: 1 };
        assert_eq!(a.is_in_range_of(&b), true);
        let b = Knot { x: 1, y: 2 };
        assert_eq!(a.is_in_range_of(&b), false);
        let a = Knot { x: 2, y: 3 };
        assert_eq!(a.is_in_range_of(&b), true);
    }
}
