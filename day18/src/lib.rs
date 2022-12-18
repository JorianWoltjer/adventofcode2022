use std::{str::FromStr, collections::HashSet};

#[derive(Debug)]
pub struct Shape {
    cubes: HashSet<(isize, isize, isize)>,
}
impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cubes = s.lines().map(|line| {
            let mut s = line.split(",");
            (s.next().unwrap().parse().unwrap(), 
             s.next().unwrap().parse().unwrap(), 
             s.next().unwrap().parse().unwrap())
        });

        Ok(Self { cubes: HashSet::from_iter(cubes) })
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
