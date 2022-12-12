use std::{str::{FromStr}, collections::{VecDeque, HashMap}};

#[derive(Debug)]
pub struct Hill {
    start: Coordinate,
    end: Coordinate,
    width: usize,
    height: usize,
    map: Vec<Vec<u8>>,
}
impl FromStr for Hill {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let height = s.len();
        let width = s.first().ok_or("Empty input")?.len();

        let mut start = None;
        let mut end = None;
        let mut map = Vec::new();

        for y in 0..height {
            let mut row = Vec::new();

            for x in 0..width {
                let value = match s[y][x] {
                    'S' => {
                        start = Some(Coordinate { x, y });
                        1
                    }
                    'E' => {
                        end = Some(Coordinate { x, y });
                        26
                    }
                    other => {
                        other as u8 - b'a' + 1
                    }
                };

                row.push(value);
            }
            map.push(row);
        }

        Ok(Self { 
            start: start.ok_or("No start position found")?, 
            end: end.ok_or("No end position found")?, 
            width, height, map,
        })
    }
}
impl Hill {
    pub fn find_shortest_path(&self, roots: Vec<Coordinate>) -> Option<usize> {
        let mut queue = VecDeque::from(roots.clone());
        let mut explored: HashMap<Coordinate, usize> = HashMap::new();

        roots.into_iter().for_each(|root| { explored.insert(root, 0); });

        while !queue.is_empty() {
            let v = queue.pop_front().unwrap();
            if v == self.end {
                return explored.get(&v).copied();
            }
            for w in self.get_steps(&v) {
                if !explored.contains_key(&w) {
                    explored.insert(w.clone(), explored.get(&v).unwrap() + 1);
                    queue.push_back(w);
                }
            }
        }

        None
    }

    fn get_steps(&self, start: &Coordinate) -> Vec<Coordinate> {
        let value = self.map[start.y][start.x];

        [(start.x as isize + 1, start.y as isize),
        (start.x as isize - 1, start.y as isize),
        (start.x as isize, start.y as isize + 1),
        (start.x as isize, start.y as isize - 1)].iter().filter(|coord| {
            coord.0 >= 0 && coord.1 >= 0 && coord.0 < self.width as isize && coord.1 < self.height as isize &&
            self.map[coord.1 as usize][coord.0 as usize] <= value+1
        }).map(|coord| {
            Coordinate { x: coord.0 as usize, y: coord.1 as usize }
        }).collect()
    }

    pub fn find_shortest_path_from_start(&self) -> Option<usize> {
        let roots = Vec::from([self.start.clone()]);

        self.find_shortest_path(roots)
    }

    pub fn find_shortest_path_from_anywhere(&self) -> Option<usize> {
        let mut roots = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.map[y][x] == 1 {  // Start at lowest
                    roots.push(Coordinate { x, y });
                }
            }
        }

        self.find_shortest_path(roots)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Coordinate {
    x: usize,
    y: usize,
}
