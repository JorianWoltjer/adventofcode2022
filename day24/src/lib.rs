use std::{str::FromStr, collections::{VecDeque, HashSet}};

#[derive(Debug)]
pub struct Map {
    obstacles: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    blizzards: Vec<Blizzard>,
    start_x: usize,
    end_x: usize,
}
impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let height = s.len() - 2;
        let width = s[0].len() - 2;

        let mut blizzards = Vec::new();
        
        for y in 1..=height {
            for x in 1..=width {
                let c = s[y][x];
                if c != '.' {
                    blizzards.push(Blizzard { x: x-1, y: y-1, direction: c.to_string().parse()? });
                }
            }
        }

        let start_x = s[0].iter().position(|c| c == &'.').ok_or("No start position found")? - 1;
        let end_x = s[height+1].iter().position(|c| c == &'.').ok_or("No end position found")? - 1;

        let mut self_ = Self { obstacles: Vec::new(), width, height, blizzards, start_x, end_x };
        self_.update_obstacles();

        Ok(self_)
    }
}
impl Map {
    fn update_obstacles(&mut self) {
        self.obstacles = vec![vec![false; self.width]; self.height];

        for blizzard in &self.blizzards {
            self.obstacles[blizzard.y][blizzard.x] = true;
        }
    }

    pub fn print(&self) {
        self.obstacles.iter().for_each(|row| {
            row.iter().for_each(|&val| {
                print!("{}", if val { '#' } else { '.' });
            });
            println!();
        });
        println!();
    }

    pub fn do_round(&mut self) {
        self.blizzards.iter_mut().for_each(|b| b.do_move(self.width, self.height));

        self.update_obstacles();
    }

    pub fn find_shortest_path_to_end(&mut self) -> Option<usize> {
        // Find first time to initially jump in
        self.do_round();
        let mut i: usize = 0;
        while self.obstacles[0][self.start_x] {
            // println!("No space, doing round...");
            self.do_round();
            i += 1;
        }

        let mut queue = VecDeque::from_iter(self.get_moves_for(self.start_x, 0).into_iter().map(|(x, y)| (x, y, i)));
        let mut explored = HashSet::<_>::from_iter(queue.iter().cloned());
        let mut current_depth = i;

        while !queue.is_empty() {
            let (x, y, depth) = queue.pop_front().unwrap();
            // println!("{}: Checking {:?}", depth, (x, y));

            // If map is not aligned with step count
            if depth > current_depth {
                self.do_round();
                current_depth += 1;
            }

            // If reached end
            if x == self.end_x && y == self.height-1 {
                return Some(depth + 1);
            }

            // Add next steps to the queue
            for (next_x, next_y) in self.get_moves_for(x, y) {
                let value = (next_x, next_y, depth + 1);
                if explored.insert(value.clone()) {  // Only if not explored yet
                    queue.push_back(value);
                }
            }
        }

        None
    }

    // For part B: Goes to the end, then back to the *start*, and finally back to the end
    // TODO: This does not give the right answer for the real input yet
    pub fn find_shortest_path_to_start(&mut self) -> Option<usize> {
        // Find first time to initially jump in
        self.do_round();
        let mut i: usize = 0;
        while self.obstacles[self.height-1][self.end_x] {
            println!("START: No space, doing round...");
            self.do_round();
            i += 1;
        }

        let mut queue = VecDeque::from_iter(self.get_moves_for(self.end_x, self.height-1).into_iter()
            .map(|(x, y)| (x, y, i)));
        let mut explored = HashSet::<_>::from_iter(queue.iter().cloned());
        let mut current_depth = i;

        while !queue.is_empty() {
            let (x, y, depth) = queue.pop_front().unwrap();
            // println!("{}: Checking {:?}", depth, (x, y));
            // self.print();

            // If map is not aligned with step count
            while depth > current_depth {
                self.do_round();
                current_depth += 1;
            }

            // If reached start
            if x == self.start_x && y == 0 {
                return Some(depth + 1);
            }

            // Add next steps to the queue
            for (next_x, next_y) in self.get_moves_for(x, y) {
                let value = (next_x, next_y, depth + 1);
                if explored.insert(value.clone()) {  // Only if not explored yet
                    queue.push_back(value);
                }
            }
        }

        None
    }

    /// Get all possible moves that don't have an obstacle
    fn get_moves_for(&self, from_x: usize, from_y: usize) -> Vec<(usize, usize)> {
        // TODO: can be optimized by checking early x > 0 for going left, for all directions. Only then apply and check obstacles
        [Direction::RIGHT, Direction::LEFT, Direction::UP, Direction::DOWN].iter()
            .map(|direction| direction.apply(from_x, from_y))
            .chain([(from_x as isize, from_y as isize)])
            .filter(|&(x, y)| x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize &&
                !self.obstacles[y as usize][x as usize])
            .map(|(x, y)| (x as usize, y as usize))
            .collect()
    }
}

#[derive(Debug)]
struct Blizzard {
    x: usize,
    y: usize,
    direction: Direction,
}
impl Blizzard {
    fn do_move(&mut self, width: usize, height: usize) {
        let (mut new_x, mut new_y) = self.direction.apply(self.x, self.y);
        new_x = new_x.rem_euclid(width as isize);
        new_y = new_y.rem_euclid(height as isize);
        (self.x, self.y) = (new_x as usize, new_y as usize);
    }
}

#[derive(Debug)]
enum Direction {
    RIGHT,
    LEFT,
    DOWN,
    UP,
}
impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Direction::RIGHT),
            "<" => Ok(Direction::LEFT),
            "v" => Ok(Direction::DOWN),
            "^" => Ok(Direction::UP),
            other => Err(format!("Direction {other:?} not recognized")),
        }
    }
}
impl Direction {
    fn apply(&self, x: usize, y: usize) -> (isize, isize) {
        match self {
            Direction::RIGHT => (x as isize + 1, y as isize),
            Direction::LEFT => (x as isize - 1, y as isize),
            Direction::DOWN => (x as isize, y as isize + 1),
            Direction::UP => (x as isize, y as isize - 1),
        }
    }
}
