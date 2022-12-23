use std::{str::FromStr, collections::HashSet};

const DIRECTIONS: [[(isize, isize); 3]; 4] = [
    [(0, -1), (1, -1), (-1, -1)],  // North
    [(0, 1), (1, 1), (-1, 1)],  // South
    [(-1, 0), (-1, -1), (-1, 1)],  // West
    [(1, 0), (1, -1), (1, 1)],  // East
];

#[derive(Debug)]
pub struct Map {
    pub elves: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    round: usize,
}
impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elves: Vec<Vec<bool>> = s.lines()
            .map(|line| line.chars()
            .map(|c| c == '#').collect())
            .collect();

        let height = elves.len();
        let width = elves.get(0).ok_or("Cannot find width".to_string())?.len();

        Ok(Self { elves, width, height, round: 0 })
    }
}
impl Map {
    pub fn print(&self) {
        self.elves.iter().for_each(|line| {
            line.iter().for_each(|&elf| {
                print!("{}", if elf { '#' } else { '.' });
            });
            println!();
        });
        println!();
    }

    /// A duplicate move is only possible if 
    /// .#.    ...
    /// ... or #.#
    /// .#.    ...
    /// We can make use of this by simply using these positions if a duplicate is found
    pub fn do_round(&mut self) {
        
        // Consider new position
        let mut all_positions = HashSet::new();

        let mut min = (isize::MAX, isize::MAX);
        let mut max = (isize::MIN, isize::MIN);

        for y in 0..self.height {
            for x in 0..self.width {
                if self.elves[y][x] {

                    let new_pos = self.get_new_position((x, y));
                    // Update bounds
                    if new_pos.0 < min.0 { min.0 = new_pos.0 };
                    if new_pos.1 < min.1 { min.1 = new_pos.1 };
                    if new_pos.0 > max.0 { max.0 = new_pos.0 };
                    if new_pos.1 > max.1 { max.1 = new_pos.1 };
                    
                    if !all_positions.insert(new_pos) {  // Try to insert
                        // Duplicate was found
                        let (x, y) = (x as isize, y as isize);
                        all_positions.remove(&new_pos);  // Don't move to duplicate
                        all_positions.insert((x, y));  // Stay at original position
                        // Update bounds
                        if x < min.0 { min.0 = x };
                        if y < min.1 { min.1 = y };
                        if x > max.0 { max.0 = x };
                        if y > max.1 { max.1 = y };
                        // Flip around duplicate
                        let other_x = new_pos.0 + (new_pos.0 - x);
                        let other_y = new_pos.1 + (new_pos.1 - y);
                        // Update bounds
                        if other_x < min.0 { min.0 = other_x };
                        if other_y < min.1 { min.1 = other_y };
                        if other_x > max.0 { max.0 = other_x };
                        if other_y > max.1 { max.1 = other_y };

                        all_positions.insert((other_x, other_y));  // Also let other elf that was collided with stay at the original position
                    }
                }
            }
        }

        let new_width = (max.0 - min.0) as usize + 1;
        let new_height = (max.1 - min.1) as usize + 1;

        self.width = new_width;
        self.height = new_height;
        self.elves = vec![vec![false; new_width]; new_height];
        for (x, y) in all_positions {
            // Place into vec with offset
            self.elves[(y - min.1) as usize][(x - min.0) as usize] = true;
        }

        self.round += 1;  // Increment counter
    }

    #[inline]
    fn empty_at(&self, (x, y): (isize, isize)) -> bool {
        if x < 0 || y < 0 || x > self.width as isize-1 || y > self.height as isize-1 {
            return true;  // Out of bounds
        }
        !self.elves[y as usize][x as usize]  // No elves
    }

    fn get_new_position(&self, (x, y): (usize, usize)) -> (isize, isize) {
        let mut count = 0;
        let mut first_valid = None;

        for check in 0..4 {
            let check = (check + self.round) % 4;
            // If all spots are empty
            if DIRECTIONS[check].iter().all(|&(offset_x, offset_y)| self.empty_at((x as isize + offset_x, y as isize + offset_y))) {
                count += 1;
                if first_valid.is_none() {  // Only for first
                    let (offset_x, offset_y) = DIRECTIONS[check][0];
                    first_valid = Some((x as isize + offset_x, y as isize + offset_y));
                }
            }
        }

        if count > 0 && count < 4 {
                // Move to first position in checks
                first_valid.unwrap()
        } else {
            (x as isize, y as isize)  // No movement
        }
    }

    pub fn count_empty_tiles(&self) -> usize {
        self.elves.iter().flat_map(|row| row).map(|&elf| !elf as usize).sum()
    }
}

#[derive(Debug)]
pub struct Elf {
    pub prev_pos: Option<(usize, usize)>,
}
impl Elf {
    pub fn new() -> Self {
        Self { prev_pos: None }
    }
}
