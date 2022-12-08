use std::{collections::HashSet, str::FromStr};

const DEBUG: bool = false;

pub struct Forest {
    trees: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl FromStr for Forest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees: Vec<Vec<u32>> = s.lines()
            .map(|line| line.chars()
            .map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        
        let height = trees.len();
        let width = trees.get(0).ok_or("Failed to get width")?.len();

        Ok(Self { trees, width, height })
    }
}

impl Forest {
    pub fn count_visible_trees(&self) -> Result<usize, String> {
        let mut visible = HashSet::new();  // Unique

        // Visible from left
        for y in 1..self.height-1 {
            let mut max = self.trees[y][0];
            
            for x in 1..self.width-1 {  // Left to right
                let value = self.trees[y][x];
                if value > max {
                    max = value;
                    if DEBUG { println!("({x},{y}) is visible from the left"); }
                    visible.insert(x * self.width + y);
                }
            }
        }
        // Visible from right
        for y in 1..self.height-1 {
            let mut max = self.trees[y][self.height-1];
            
            for x in (1..self.width-1).rev() {  // Right to left
                let value = self.trees[y][x];
                if value > max {
                    max = value;
                    if DEBUG { println!("({x},{y}) is visible from the right"); }
                    visible.insert(x * self.width + y);
                }
            }
        }
        // Visible from top
        for x in 1..self.width-1 {
            let mut max = self.trees[0][x];
            
            for y in 1..self.height-1 {  // Top to bottom
                let value = self.trees[y][x];
                if value > max {
                    max = value;
                    if DEBUG { println!("({x},{y}) is visible from the top"); }
                    visible.insert(x * self.width + y);
                }
            }
        }
        // Visible from bottom
        for x in 1..self.width-1 {
            let mut max = self.trees[self.width-1][x];
            
            for y in (1..self.height-1).rev() {  // Bottom to top
                let value = self.trees[y][x];
                if value > max {
                    max = value;
                    if DEBUG { println!("({x},{y}) is visible from the bottom"); }
                    visible.insert(x * self.width + y);
                }
            }
        }
    
        let mut total = visible.len();
        total += 4 + (self.width-2)*2 + (self.height-2)*2;  // 4 corners, and 2 edges are always visible
    
        Ok(total)
    }

    
    pub fn get_scenic_score(&self, x: usize, y: usize) -> u64 {
        if x == 0 || y == 0 || x == self.width-1 || y == self.height-1 {
            return 0;  // If on edge, one factor will be 0 so we can return instantly
        }

        let own_value = self.trees[y][x];

        // Looking left
        let mut count_left = 0;
        for x_iter in (0..x).rev() {
            count_left += 1;
            if self.trees[y][x_iter] >= own_value {  // If value greater than own value
                if DEBUG { println!("({x_iter}, {y}) is blocking view to the left"); }
                break;
            }
        }
        if DEBUG { println!("Count left: {count_left}"); }
        
        // Looking right
        let mut count_right = 0;
        for x_iter in x+1..self.width {
            count_right += 1;
            if self.trees[y][x_iter] >= own_value {
                if DEBUG { println!("({x_iter}, {y}) is blocking view to the right"); }
                break;
            }
        }
        if DEBUG { println!("Count right: {count_right}"); }
        
        // Looking up
        let mut count_up = 0;
        for y_iter in (0..y).rev() {
            count_up += 1;
            if self.trees[y_iter][x] >= own_value {
                if DEBUG { println!("({x}, {y_iter}) is blocking view up"); }
                break;
            }
        }
        if DEBUG { println!("Count up: {count_up}"); }
        
        // Looking down
        let mut count_down = 0;
        for y_iter in y+1..self.height {
            count_down += 1;
            if self.trees[y_iter][x] >= own_value {
                if DEBUG { println!("({x}, {y_iter}) is blocking view down"); }
                break;
            }
        }
        if DEBUG { println!("Count down: {count_down}"); }

        let score = count_left * count_right * count_up * count_down;

        score
    }

    pub fn find_best_scenic_score(&self) -> u64 {
        let mut best = 0;

        for y in 1..self.height-1 {
            for x in 1..self.width-1 {
                if DEBUG { println!("CALCULATING ({x}, {y})"); }
                let score = self.get_scenic_score(x, y);

                if score > best {
                    best = score;
                    if DEBUG { println!("NEW BEST of {score} at ({x}, {y})"); }
                }
            }
        }

        best
    }
}
