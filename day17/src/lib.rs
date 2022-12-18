#[macro_use]
extern crate lazy_static;

use std::{str::FromStr, cmp::max, collections::HashMap};


lazy_static! {
    pub static ref PIECES: Vec<Piece> = vec![
        Piece { offsets: vec![(-1, 0), (0, 0), (1, 0), (2, 0)], top_y: 0 },  // Dash
        Piece { offsets: vec![(-1, 1), (1, 1), (0, 0), (0, 2), (0, 1)], top_y: 2 },  // Plus
        Piece { offsets: vec![(-1, 0), (0, 0), (1, 0), (1, 1), (1, 2)], top_y: 2 },  // L
        Piece { offsets: vec![(-1, 0), (-1, 1), (-1, 2), (-1, 3)], top_y: 3 },  // Bar
        Piece { offsets: vec![(0, 0), (-1, 0), (-1, 1), (0, 1)], top_y: 1 },  // Square
    ];
}

pub struct Game {
    pub board: Vec<Vec<bool>>,  // Dynamic height
    pub width: usize,
    pub piece_index: usize,
    pub piece_position: (isize, isize),
    pub movement: Vec<Movement>,
    pub movement_index: usize,
}
impl Game {
    pub fn new(movement: Vec<Movement>) -> Self {
        Self { 
            board: Vec::new(), 
            width: 7,
            piece_index: 0,
            piece_position: (3, 3), 
            movement,
            movement_index: 0,
        }
    }

    pub fn drop_n_rocks(&mut self, n: usize) -> usize {
        let mut count = 0;
        // Loop until n have dropped
        loop {
            if self.next_tick() {  // If returns true, a piece was placed
                count += 1;
                if count >= n {
                    break;
                }
            }
        }

        self.board.len()  // Return height
    }

    pub fn next_tick(&mut self) -> bool {
        // Push sideways
        let new_position = self.get_movement().apply(self.piece_position);
        if self.fits_on_board(new_position) {
            self.piece_position = new_position; 
        }
        self.movement_index = (self.movement_index + 1) % self.movement.len();
        // Move down
        let new_position = Movement::DOWN.apply(self.piece_position);
        if self.fits_on_board(new_position) {
            self.piece_position = new_position;
            return false;
        } else {  // Can't move down, needs to stop
            self.drop_new_piece();
            return true;
        }
    }

    pub fn drop_new_piece(&mut self) {
        self.update_board_height();  // Make board fit piece

        // Place piece on board
        self.get_piece().offsets.to_owned().iter().for_each(|(offset_x, offset_y)| {
            let x = self.piece_position.0 + offset_x;
            let y = self.piece_position.1 + offset_y;
            self.board[y as usize][x as usize] = true;
        });

        // Choose new piece
        let board_height = self.board.len();
        self.piece_index = (self.piece_index + 1) % PIECES.len();  // Cycle next piece
        self.piece_position = (3, board_height as isize + 3);  // Set to 3 above highest point
    }

    /// Make sure board is the same height as the highest falling piece right now
    fn update_board_height(&mut self) {
        let board_height = self.board.len() as isize;
        let piece_top = self.piece_position.1 + self.get_piece().top_y;
        for _ in board_height..piece_top+1 {
            self.board.extend([[false].repeat(self.width)]);
        }
    }

    fn get_movement(&self) -> &Movement {
        self.movement.get(self.movement_index).unwrap()
    }

    fn get_piece(&self) -> &Piece {
        PIECES.get(self.piece_index).unwrap()
    }

    fn is_piece(&self, (x, y): (usize, usize)) -> bool {
        self.get_piece().offsets.iter().any(|(offset_x, offset_y)| {
            self.piece_position.0 + offset_x == x as isize && self.piece_position.1 + offset_y == y as isize
        })
    }

    pub fn fits_on_board(&self, (piece_x, piece_y): (isize, isize)) -> bool {
        self.get_piece().offsets.iter().all(|(offset_x, offset_y)| {
            let x = piece_x + offset_x;
            let y = piece_y + offset_y;
            (x >= 0 && x < 7) &&  // If in width bounds
            (y >= self.board.len() as isize ||  // If higher than top of board
            (y >= 0 && !self.board[y as usize][x as usize]))  // If above the floor, and board is empty there
        })
    }

    /// For part B:
    /// Get all variables, where if they are the same with some other offset you can predict future values
    pub fn get_variables(&self) -> (Vec<usize>, usize, usize) {
        let height_offsets = (0..self.width).map(|x| {
            for y in (0..self.board.len()).rev() {  // Go down from top
                if self.board[y][x] {
                    return self.board.len() - y - 1;  // Offset from top
                }
            }
            self.board.len()
        }).collect();

        (height_offsets, self.piece_index, self.movement_index)
    }
    
    pub fn print(&self) {
        let max_height = max(self.board.len(), (self.piece_position.1 + self.get_piece().top_y + 1) as usize);
        for y in (0..max_height).rev() {
            print!("|");
            for x in 0..self.width {
                if y >= self.board.len() {
                    if self.is_piece((x, y)) {
                        print!("@");
                    } else {
                        print!(".");
                    }
                } else {
                    let cell = self.board[y][x];
                    if cell {
                        print!("#");
                    } else if self.is_piece((x, y)) {
                        print!("@");
                    } else {
                        print!(".");
                    }
                }
            }
            println!("|");
        }
        println!("+{}+", "-".repeat(self.width));
    }
}

pub struct Piece {
    pub offsets: Vec<(isize, isize)>,
    pub top_y: isize,
}

#[derive(Debug, Clone)]
pub enum Movement {
    LEFT,
    RIGHT,
    DOWN,
}
impl FromStr for Movement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Movement::LEFT),
            ">" => Ok(Movement::RIGHT),
            other => Err(format!("{other:?} is not a valid direction"))
        }
    }
}
impl Movement {
    pub fn apply(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Movement::LEFT => (x - 1, y),
            Movement::RIGHT => (x + 1, y),
            Movement::DOWN => (x, y - 1),
        }
    }
}

/// For part B, finds a cycle/pattern that it can simply multiply to get any arbitrary number of rocks in the future
pub fn predict_many_rocks(movement: &Vec<Movement>, amount: usize) -> usize {
    let (cycle_start, cycle_multiple) = find_cycle(&movement);

    let mut simulated_game = Game::new(movement.to_vec());

    // Sync with cycle
    simulated_game.drop_n_rocks(cycle_start);
    let height_start = simulated_game.board.len();
    
    // Do one multiple to get height difference
    simulated_game.drop_n_rocks(cycle_multiple);
    let height_multiple = simulated_game.board.len() - height_start;
    
    // Rought estimate only using cycle multiples
    let needed_multiples = (amount - cycle_start) / cycle_multiple;
    let height_estimate = height_multiple * needed_multiples;

    // Do last offset to get exact height
    let rocks_left = (amount - cycle_start) % cycle_multiple;
    simulated_game.drop_n_rocks(rocks_left);
    let height_correction = simulated_game.board.len() - height_multiple;

    // Final value
    height_estimate + height_correction
}

/// Returns (start, multiple)
pub fn find_cycle(movement: &Vec<Movement>) -> (usize, usize) {
    let mut simulated_game = Game::new(movement.to_vec());

    let mut map = HashMap::new();
    let mut rocks_dropped = 0;
    loop {
        simulated_game.drop_n_rocks(1);
        rocks_dropped += 1;
        let offsets = simulated_game.get_variables();
        // If all variables are the same as a previous iteration, we found a cycle
        if let Some(start) = map.insert(offsets, rocks_dropped) {
            return (start, rocks_dropped - start);
        }
    }
}
