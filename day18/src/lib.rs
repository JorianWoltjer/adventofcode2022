use std::{str::FromStr, collections::{HashSet, VecDeque}};

#[derive(Debug)]
pub struct Shape {
    cubes: HashSet<(isize, isize, isize)>,
    min: (isize, isize, isize),
    max: (isize, isize, isize),
}
impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse string
        let cubes = s.lines().map(|line| {
            let mut s = line.split(",");
            (s.next().unwrap().parse().unwrap(), 
             s.next().unwrap().parse().unwrap(), 
             s.next().unwrap().parse().unwrap())
        });

        // Calculate bounding box
        let min = (
            cubes.clone().map(|cube| cube.0).min().unwrap(),
            cubes.clone().map(|cube| cube.1).min().unwrap(),
            cubes.clone().map(|cube| cube.2).min().unwrap(),
        );
        let max = (
            cubes.clone().map(|cube| cube.0).max().unwrap(),
            cubes.clone().map(|cube| cube.1).max().unwrap(),
            cubes.clone().map(|cube| cube.2).max().unwrap(),
        );

        Ok(Self { cubes: HashSet::from_iter(cubes), min, max })
    }
}
impl Shape {
    pub fn surface_area(&self) -> usize {
        // Initial guess
        let mut area = self.cubes.len() * 6;

        for cube in &self.cubes {
            for adjacent in get_neighbors(cube) {
                if self.cubes.contains(&adjacent) {  // If another cube is in front of this side
                    area -= 1;
                }
            }
        }

        area
    }

    pub fn surface_area_without_gaps(&self) -> usize {
        // Initial guess
        let mut area = self.cubes.len() * 6;

        for cube in &self.cubes {
            for adjacent in get_neighbors(cube) {
                if self.cubes.contains(&adjacent) || !self.path_to_outside(adjacent) {  // If another cube is blocking this path
                    area -= 1;
                }
            }
        }

        area
    }

    /// Breadth first search to anywhere outside the bounding box
    fn path_to_outside(&self, root: (isize, isize, isize)) -> bool {
        let mut queue = VecDeque::from([root]);
        let mut explored = HashSet::new();

        while !queue.is_empty() {
            let cube = queue.pop_front().unwrap();  // Get from queue

            // If path lead to outside the bounding box
            if  cube.0 < self.min.0 || cube.1 < self.min.1 || cube.2 < self.min.2 || 
                cube.0 > self.max.0 || cube.1 > self.max.1 || cube.2 > self.max.2 {
                
                return true;
            }

            // Add next paths
            for adjacent in get_neighbors(&cube) {
                if !explored.contains(&adjacent) && !self.cubes.contains(&adjacent) {  // Ignore if already explored or is solid
                    explored.insert(adjacent.clone());
                    queue.push_back(adjacent.clone());
                }
            }
        }

        false // All paths ended
    }
}

pub fn get_neighbors(cube: &(isize, isize, isize)) -> Vec<(isize, isize, isize)> {
    vec![
        (cube.0 + 1, cube.1,     cube.2),
        (cube.0 - 1, cube.1,     cube.2),
        (cube.0,     cube.1 + 1, cube.2),
        (cube.0,     cube.1 - 1, cube.2),
        (cube.0,     cube.1,     cube.2 + 1),
        (cube.0,     cube.1,     cube.2 - 1),
    ]
}
