#![feature(linked_list_cursors)]

use std::{str::FromStr, collections::{HashSet}};

const DEBUG: bool = false;

#[derive(Debug, Clone)]
pub struct Knot {
    pub x: isize,
    pub y: isize,
}

impl Knot {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    /// If other is in one of the 8 neighbouring squares
    pub fn is_neighbouring(&self, other: &Knot) -> bool {
        self.x.abs_diff(other.x) <= 1 &&
        self.y.abs_diff(other.y) <= 1
    }

    /// If self is not on the same axis as other
    pub fn needs_diagonal_move_to(&self, other: &Knot) -> bool {
        self.x != other.x && self.y != other.y
    }

    /// Returns the number of squares that need to be walked to get to other
    pub fn distance_to(&self, other: &Knot) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn move_into(&mut self, direction: &Direction) {
        match direction {
            Direction::RIGHT => self.x += 1,
            Direction::LEFT => self.x -= 1,
            Direction::UP => self.y += 1,
            Direction::DOWN => self.y -= 1,
        }
    }

    /// Moves into other if it is too far away
    pub fn follow(&mut self, other: &Knot) {
        if !self.is_neighbouring(other) {  // If not in range, needs to move

            if self.distance_to(other) > 2 {  // If other is more than 2 away, step diagonally into the direction (move 1,1)
                self.x += 1 * (other.x - self.x).signum();
                self.y += 1 * (other.y - self.y).signum();
            } else if self.distance_to(other) == 2 {  // If other is not in range and exactly 2 away, only step one into the direction
                if self.x == other.x {  // If x is equal, y axis needs a step
                    self.y += 1 * (other.y - self.y).signum();
                } else if self.y == other.y {  // If y is equal, x axis need a step
                    self.x += 1 * (other.x - self.x).signum();
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct LongRope {
    pieces: Vec<Knot>,
    pub tail_visited: HashSet<(isize, isize)>,
}

impl LongRope {
    pub fn new(length: usize) -> Self {
        Self {
            pieces: (0..length).map(|_| Knot::new()).collect(),
            tail_visited: HashSet::new(),
        }
    }

    pub fn move_head(&mut self, direction: &Direction, amount: usize) {
        if DEBUG { println!("Movement: {amount} {direction:?}"); }

        for _ in 0..amount {
            // Move head
            let head = self.pieces.first_mut().unwrap();
            head.move_into(direction);

            // Move tail
            for i in 1..self.pieces.len() {
                let prev = &self.pieces[i-1].clone();  // We have to clone here because borrow checker doesn't know we don't mutate [i-1]
                self.pieces[i].follow(prev);
            }

            // Save position of last element for result
            let tail = self.pieces.last().unwrap();
            self.tail_visited.insert((tail.x, tail.y));

            if DEBUG { self.print(); }
        }
    }
    
    pub fn print(&self) {  // Only for debugging
        let mut grid: Vec<Vec<char>> = Vec::new();
        for y in -5..=15 {
            let mut row: Vec<char> = Vec::new();
            'x_loop: for x in -11..=14 {
                if y == self.pieces.first().unwrap().y && x == self.pieces.first().unwrap().x {
                    row.push('H');
                } else {
                    for (i, knot) in self.pieces.iter().enumerate() {
                        if y == knot.y && x == knot.x {
                            let c = i.to_string().chars().next().unwrap();
                            row.push(c);
                            continue 'x_loop;
                        }
                    }
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
    fn test_is_neighbouring() {
        let a = Knot { x: 0, y: 0 };
        let b = Knot { x: 1, y: 1 };
        assert_eq!(a.is_neighbouring(&b), true);
        let b = Knot { x: 1, y: 2 };
        assert_eq!(a.is_neighbouring(&b), false);
        let a = Knot { x: 2, y: 3 };
        assert_eq!(a.is_neighbouring(&b), true);
    }
}
