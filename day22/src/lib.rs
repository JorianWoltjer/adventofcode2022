#[macro_use]
extern crate lazy_static;

use regex::Regex;

use std::{str::FromStr, collections::HashMap};

lazy_static! {
    // Not used
    pub static ref PORTALS_A_TEST: HashMap<(isize, isize, Direction), (usize, usize, Direction)> = {
        let mut m = HashMap::new();
        // Vertical lines
        (0..4).for_each(|i| { m.insert((7, 0+i, Direction::RIGHT), (11, 0+i as usize, Direction::LEFT)); });
        (0..4).for_each(|i| { m.insert((12, 0+i, Direction::LEFT), (8, 0+i as usize, Direction::RIGHT)); });
        (0..4).for_each(|i| { m.insert((-1, 4+i, Direction::RIGHT), (11, 4+i as usize, Direction::LEFT)); });
        (0..4).for_each(|i| { m.insert((12, 4+i, Direction::LEFT), (0, 4+i as usize, Direction::RIGHT)); });
        (0..4).for_each(|i| { m.insert((7, 8+i, Direction::RIGHT), (15, 8+i as usize, Direction::LEFT)); });
        (0..4).for_each(|i| { m.insert((16, 8+i, Direction::LEFT), (8, 8+i as usize, Direction::RIGHT)); });
        // Horizontal lines
        (0..4).for_each(|i| { m.insert((0+i, 3, Direction::DOWN), (0+i as usize, 7, Direction::UP)); });
        (0..4).for_each(|i| { m.insert((0+i, 8, Direction::UP), (0+i as usize, 4, Direction::DOWN)); });
        (0..4).for_each(|i| { m.insert((4+i, 3, Direction::DOWN), (4+i as usize, 7, Direction::UP)); });
        (0..4).for_each(|i| { m.insert((4+i, 8, Direction::UP), (4+i as usize, 4, Direction::DOWN)); });
        (0..4).for_each(|i| { m.insert((8+i, -1, Direction::DOWN), (8+i as usize, 11, Direction::UP)); });
        (0..4).for_each(|i| { m.insert((8+i, 12, Direction::UP), (8+i as usize, 0, Direction::DOWN)); });
        (0..4).for_each(|i| { m.insert((12+i, 7, Direction::DOWN), (12+i as usize, 11, Direction::UP)); });
        (0..4).for_each(|i| { m.insert((12+i, 12, Direction::UP), (12+i as usize, 8, Direction::DOWN)); });
        m
    };
    // Portals for edges of the input cube
    pub static ref PORTALS_B_REAL: HashMap<(isize, isize, Direction), (usize, usize, Direction)> = {
        let mut m = HashMap::new();
        (0..50).for_each(|i| { m.insert((49, 0+i, Direction::LEFT), (0, 149-i as usize, Direction::RIGHT)); });
        (0..50).for_each(|i| { m.insert((-1, 149-i, Direction::LEFT), (50, 0+i as usize, Direction::RIGHT)); });
        (0..50).for_each(|i| { m.insert((150, 0+i, Direction::RIGHT), (99, 149-i as usize, Direction::LEFT)); });
        (0..50).for_each(|i| { m.insert((100, 149-i, Direction::RIGHT), (149, 0+i as usize, Direction::LEFT)); });
        (0..50).for_each(|i| { m.insert((49, 50+i, Direction::LEFT), (0+i as usize, 100, Direction::DOWN)); });
        (0..50).for_each(|i| { m.insert((0+i, 99, Direction::UP), (50, 50+i as usize, Direction::RIGHT)); });
        (0..50).for_each(|i| { m.insert((100, 50+i, Direction::RIGHT), (100+i as usize, 49, Direction::UP)); });
        (0..50).for_each(|i| { m.insert((100+i, 50, Direction::DOWN), (99, 50+i as usize, Direction::LEFT)); });
        (0..50).for_each(|i| { m.insert((50, 150+i, Direction::RIGHT), (50+i as usize, 149, Direction::UP)); });
        (0..50).for_each(|i| { m.insert((50+i, 150, Direction::DOWN), (49, 150+i as usize, Direction::LEFT)); });
        (0..50).for_each(|i| { m.insert((-1, 150+i, Direction::LEFT), (50+i as usize, 0, Direction::DOWN)); });
        (0..50).for_each(|i| { m.insert((50+i, -1, Direction::UP), (0, 150+i as usize, Direction::RIGHT)); });
        (0..50).for_each(|i| { m.insert((100+i, -1, Direction::UP), (0+i as usize, 199, Direction::UP)); });
        (0..50).for_each(|i| { m.insert((0+i, 200, Direction::DOWN), (100+i as usize, 0, Direction::DOWN)); });
        m
    };
}

// TODO: maybe refactor to use the same method as B, by generating portals from the bounds
#[derive(Debug)]
pub struct Map {
    pub obstacles: Vec<Vec<bool>>,
    pub horizontal_bounds: Vec<(usize, usize)>,
    pub vertical_bounds: Vec<(usize, usize)>,
    pub walker_pos: (usize, usize),
    pub walker_facing: Direction,
}
impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

        // Map '#' to obstacle coordinates
        let mut obstacles: Vec<Vec<bool>> = s.iter().map(|line| line.iter().map(|c| {
            c == &'#'
        }).collect()).collect();
        // Calculate max width
        let width = obstacles.iter().map(|row| row.len()).max().unwrap();
        // Extend all rows to have the same max width
        obstacles.iter_mut().for_each(|row| row.extend(vec![false; width - row.len()]));

        // Calculate bounds
        let horizontal_bounds: Vec<(usize, usize)> = s.iter().map(|line| { (
            line.iter().position(|c| c != &' ').unwrap(),  // Start (first index)
            line.len() - 1  // End (last index)
        )}).collect();

        let mut vertical_bounds = Vec::new();
        for x in 0..width {
            let mut start = None;
            let mut end = None;
            for (y, row) in s.iter().enumerate() {
                let current = row.get(x);
                if start.is_none() && !(current.is_none() || current == Some(&' ')) {
                    // If looking for start, and current is not empty
                    start = Some(y);
                } else if start.is_some() && (current.is_none() || current == Some(&' ')) {
                    // If looking for end, and current is empty
                    end = Some(y - 1);
                    break;
                }
            }

            if end.is_none() { end = Some(s.len() - 1) }
            vertical_bounds.push((start.unwrap(), end.unwrap()));
        }

        // Leftmost open tile of the top row
        let min_x = horizontal_bounds.first().unwrap().0;
        let start_x = obstacles[0].iter().enumerate()
            .position(|(x, val)| x >= min_x && !*val).unwrap();

        Ok(Self { 
            obstacles, horizontal_bounds, vertical_bounds, 
            walker_pos: (start_x, 0), 
            walker_facing: Direction::RIGHT
        })
    }
}
impl Map {
    pub fn print(&self) {
        for (y, row) in self.obstacles.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if !self.is_in_bounds(&(x, y)) {
                    print!(" ");
                } else if (x, y) == self.walker_pos {
                    print!("@");
                } else {
                    print!("{}", if *cell { '#' } else { '.' });
                }
            }
            println!();
        }
    }

    pub fn go_forward(&mut self, amount: usize) {
        for _ in 0..amount {
            let new_position = self.move_position(&self.walker_pos, &self.walker_facing);

            if !self.obstacles[new_position.1][new_position.0] {
                self.walker_pos = new_position;
            } else {  // Hit wall
                break;
            }

            assert!(self.is_in_bounds(&self.walker_pos));
        }
    }

    pub fn turn(&mut self, turn: Turn) {
        self.walker_facing = match (&self.walker_facing, turn) {
            (Direction::RIGHT, Turn::RIGHT) | (Direction::LEFT, Turn::LEFT) => Direction::DOWN,
            (Direction::LEFT, Turn::RIGHT) | (Direction::RIGHT, Turn::LEFT) => Direction::UP,
            (Direction::UP, Turn::RIGHT) | (Direction::DOWN, Turn::LEFT) => Direction::RIGHT,
            (Direction::DOWN, Turn::RIGHT) | (Direction::UP, Turn::LEFT) => Direction::LEFT,
        };
    }

    /// Moves the position one into the direction, not checking walls but wrapping around bounds
    pub fn move_position(&self, position: &(usize, usize), direction: &Direction) -> (usize, usize) {
        match direction {
            Direction::RIGHT => {
                let bounds = self.horizontal_bounds[position.1];
                let mut x = position.0 + 1;
                if x > bounds.1 {  // Wrap around
                    x = bounds.0;  // Set to left edge
                }
                (x, position.1)
            }
            Direction::LEFT => {
                let bounds = self.horizontal_bounds[position.1];
                let mut x = position.0 as isize - 1;
                if x < bounds.0 as isize {  // Wrap around
                    x = bounds.1 as isize;  // Set to right edge
                }
                (x as usize, position.1)
            }
            Direction::DOWN => {
                let bounds = self.vertical_bounds[position.0];
                let mut y = position.1 + 1;
                if y > bounds.1 {  // Wrap around
                    y = bounds.0;  // Set to top
                }
                (position.0, y)
            }
            Direction::UP => {
                let bounds = self.vertical_bounds[position.0];
                let mut y = position.1 as isize - 1;
                if y < bounds.0 as isize {  // Wrap around
                    y = bounds.1 as isize;  // Set to bottom
                }
                (position.0, y as usize)
            }
        }
    }

    /// Used for printing
    fn is_in_bounds(&self, position: &(usize, usize)) -> bool {
        position.0 >= self.horizontal_bounds[position.1].0 &&
        position.0 <= self.horizontal_bounds[position.1].1 &&
        position.1 >= self.vertical_bounds[position.0].0 &&
        position.1 <= self.vertical_bounds[position.0].1
    }

    pub fn do_movement(&mut self, movement: &str) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d+").unwrap();
        }

        let mut i = 0;
        while i < movement.len() {
            let slice = &movement[i..];

            if let Some(capture) = RE.captures(slice) {  // If is digit
                let capture = capture.get(0).unwrap().as_str();
                let amount: usize = capture.parse().unwrap();
                
                self.go_forward(amount);

                i += capture.len() - 1;
            } else {
                let turn: Turn = slice[..1].parse().unwrap();
                
                self.turn(turn);
            }
            i += 1;
        }
    }

    pub fn get_password(&self) -> usize {
        let row = self.walker_pos.1 + 1;
        let column = self.walker_pos.0 + 1;
        let facing = self.walker_facing.value();

        row * 1000 + column * 4 + facing
    }
}

#[derive(Debug)]
pub struct MapCube {
    pub obstacles: Vec<Vec<bool>>,
    pub walker_pos: (usize, usize),
    pub walker_facing: Direction,
    //                   (x_in , y_in , facing   ): (x_out, y_out, facing   )
    pub portals: HashMap<(isize, isize, Direction), (usize, usize, Direction)>,
}
impl MapCube {
    pub fn new(s: &str, portals: HashMap<(isize, isize, Direction), (usize, usize, Direction)>) -> Self {
        let s: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

        // Map '#' to obstacle coordinates
        let mut obstacles: Vec<Vec<bool>> = s.iter().map(|line| line.iter().map(|c| {
            c == &'#'
        }).collect()).collect();
        // Calculate max width
        let width = obstacles.iter().map(|row| row.len()).max().unwrap();
        // Extend all rows to have the same max width
        obstacles.iter_mut().for_each(|row| row.extend(vec![false; width - row.len()]));

        // Leftmost open tile of the top row
        let start_x = s[0].iter().position(|c| c == &'.').unwrap();

        Self { 
            obstacles, 
            walker_pos: (start_x, 0), 
            walker_facing: Direction::RIGHT,
            portals
        }
    }

    pub fn print(&self) {
        let mut out = Vec::new();
        for y in -1..self.obstacles.len() as isize + 1 {
            let mut line = Vec::new();
            for x in -1..self.obstacles[0].len() as isize + 1 {
                if y >= 0 && x >= 0 {
                    let (x, y) = (x as usize, y as usize);
                    if (x, y) == self.walker_pos {
                        line.push('@');
                    } else if let Some(cell) = self.obstacles.get(y).and_then(|row| row.get(x)) {
                        line.push(if *cell { '#' } else { '.' });
                    } else {
                        line.push(' ');
                    }
                } else {
                    line.push(' ');
                }
            }
            out.push(line);
        }

        // Portals
        for ((in_x, in_y, in_facing), (out_x, out_y, out_facing)) in &self.portals {
            if ['V', '^', '<', '>'].iter().any(|c| c == &out[(in_y+1) as usize][(in_x+1) as usize]) {
                out[(in_y+1) as usize][(in_x+1) as usize] = '+';
            } else {
                out[(in_y+1) as usize][(in_x+1) as usize] = match in_facing {
                    Direction::DOWN => 'V',
                    Direction::UP => '^',
                    Direction::LEFT => '<',
                    Direction::RIGHT => '>',
                };
            }
            if ['V', '^', '<', '>'].iter().any(|c| c == &out[out_y+1][out_x+1]) {
                out[out_y+1][out_x+1] = '+';
            } else {
                out[out_y+1][out_x+1] = match out_facing {
                    Direction::DOWN => 'V',
                    Direction::UP => '^',
                    Direction::LEFT => '<',
                    Direction::RIGHT => '>',
                };
            }
        }

        // 2D vec of chars to string seperated by newlines
        let s = out.into_iter()
            .map(|line| line.iter()
                .map(|c| c.to_string()).collect::<Vec<_>>()
                .join("")
            ).collect::<Vec<_>>()
            .join("\n");

        // Use `$ watch -n 0.1 cat /tmp/out` to see each step
        // fs::write("/tmp/out", s).unwrap();
        // thread::sleep(Duration::from_millis(100));
        print!("{}", s);
    }

    pub fn go_forward(&mut self, amount: usize) {
        for _ in 0..amount {
            let (new_x, new_y, new_facing) = self.move_position(&self.walker_pos, &self.walker_facing);

            if !self.obstacles[new_y][new_x] {
                // Set position
                self.walker_pos = (new_x, new_y);
                self.walker_facing = new_facing;
            } else {  // Hit wall
                break;
            }
        }
    }

    pub fn turn(&mut self, turn: Turn) {
        self.walker_facing = match (&self.walker_facing, turn) {
            (Direction::RIGHT, Turn::RIGHT) | (Direction::LEFT, Turn::LEFT) => Direction::DOWN,
            (Direction::LEFT, Turn::RIGHT) | (Direction::RIGHT, Turn::LEFT) => Direction::UP,
            (Direction::UP, Turn::RIGHT) | (Direction::DOWN, Turn::LEFT) => Direction::RIGHT,
            (Direction::DOWN, Turn::RIGHT) | (Direction::UP, Turn::LEFT) => Direction::LEFT,
        };
    }

    /// Moves the position one into the direction, not checking walls but wrapping around bounds
    pub fn move_position(&self, position: &(usize, usize), direction: &Direction) -> (usize, usize, Direction) {
        let new_pos = match direction {
            Direction::RIGHT => (position.0 as isize + 1, position.1 as isize),
            Direction::LEFT => (position.0 as isize - 1, position.1 as isize),
            Direction::DOWN => (position.0 as isize, position.1 as isize + 1),
            Direction::UP => (position.0 as isize, position.1 as isize - 1),
        };
        // Teleport through portal if any
        if let Some((out_x, out_y, out_facing)) = self.portals.get(&(new_pos.0, new_pos.1, self.walker_facing.clone())) {
            return (*out_x, *out_y, out_facing.clone());
        } else {
            return (new_pos.0 as usize, new_pos.1 as usize, self.walker_facing.clone());
        }
    }

    pub fn do_movement(&mut self, movement: &str) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d+").unwrap();
        }

        let mut i = 0;
        while i < movement.len() {
            let slice = &movement[i..];

            if let Some(capture) = RE.captures(slice) {  // If is digit
                let capture = capture.get(0).unwrap().as_str();
                let amount: usize = capture.parse().unwrap();
                
                self.go_forward(amount);

                i += capture.len() - 1;
            } else {
                let turn: Turn = slice[..1].parse().unwrap();
                
                self.turn(turn);
            }
            i += 1;
        }
    }

    pub fn get_password(&self) -> usize {
        let row = self.walker_pos.1 + 1;
        let column = self.walker_pos.0 + 1;
        let facing = self.walker_facing.value();

        row * 1000 + column * 4 + facing
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Direction {
    LEFT,
    RIGHT,
    DOWN,
    UP,
}
impl Direction {
    pub fn value(&self) -> usize {
        match self {
            Direction::RIGHT => 0,
            Direction::DOWN => 1,
            Direction::LEFT => 2,
            Direction::UP => 3,
        }
    }
}

#[derive(Debug)]
pub enum Turn {
    RIGHT,
    LEFT,
}
impl FromStr for Turn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Turn::RIGHT),
            "L" => Ok(Turn::LEFT),
            other => Err(format!("Turn {other:?} not recognized"))
        }
    }
}
