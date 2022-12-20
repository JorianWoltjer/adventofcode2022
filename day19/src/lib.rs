#[macro_use]
extern crate lazy_static;

use std::{str::FromStr, collections::HashMap};

use regex::Regex;

#[derive(Debug)]
pub struct Blueprint {
    pub robots: Vec<Robot>,
}
impl FromStr for Blueprint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut robots = Vec::new();

        for robot_str in s.split(". ") {
            robots.push(robot_str.parse()?);
        }
        
        Ok(Self { robots })
    }
}
impl Blueprint {
    pub fn find_most_geodes(&self, time: usize) -> usize {
        let resources = HashMap::new();

        // Start recursive algorithm
        self.recursive_most_geodes(resources, Vec::from([&self.robots[0]]), time)
    }

    pub fn recursive_most_geodes(&self, mut resources: HashMap<Material, usize>, bought_robots: Vec<&Robot>, time: usize) -> usize {
        if time <= 0 {
            // Return number of mined geodes
            let geodes = *resources.get(&Material::GEODE).unwrap_or(&0);
                // println!("{}FINISHED with {resources:?} using {:?}", " ".repeat(24-time), 
                //     bought_robots.iter().map(|robot| &robot.collects).collect::<Vec<_>>());
            return geodes;
        }

        // println!("{}Resources: {resources:?} Robots: {:?}", " ".repeat(24-time), 
        //     bought_robots.iter().map(|robot| &robot.collects).collect::<Vec<_>>());

        // Apply bought robots
        bought_robots.iter().for_each(|robot| {
            if let Some(balance) = resources.get_mut(&robot.collects) {
                *balance += 1;  // Increment
            } else {
                resources.insert(robot.collects.clone(), 1);  // Initialize
            }
        });

        let resources_clone = resources.clone();
        // Explore all options
        self.robots.iter().map(|robot| {
            if time > 1 && robot.can_buy_with(&resources) {  // If enough resources / time
                // Buy robot
                let mut bought_robots = bought_robots.clone();
                bought_robots.push(robot);
                // Take away resources
                robot.costs.iter().for_each(|(material, amount)| {
                    let balance = resources.get_mut(material).unwrap();
                    *balance -= amount;
                });

                self.recursive_most_geodes(resources.clone(), bought_robots, time - 1)
            } else {  // Else don't consider
                0
            }
        }).chain([  // Option to not buy anything
            self.recursive_most_geodes(resources_clone, bought_robots.clone(), time - 1)
        ]).max().unwrap()
    }
}

#[derive(Debug)]
pub struct Robot {
    pub collects: Material,
    pub costs: HashMap<Material, usize>,
}
impl FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_COLLECTS: Regex = Regex::new(r"Each (\w+) robot").unwrap();
            static ref RE_COSTS: Regex = Regex::new(r"(\d+) (\w+)(?: and ((\d+) (\w+)))?").unwrap();
        }

        let collects = RE_COLLECTS.captures(s).ok_or("No material found")?[1].parse()?;
        let mut costs = HashMap::new();

        let captures = RE_COSTS.captures(s).unwrap();
        for (i, j) in [(1, 2), (4, 5)] {
            if let (Some(amount), Some(material)) = (captures.get(i), captures.get(j)) {
                let amount = amount.as_str().parse().map_err(|_| "Failed to parse int")?;
                let material = material.as_str().parse()?;

                costs.insert(material, amount);
            }
        }
        
        Ok(Self { collects, costs })
    }
}
impl Robot {
    pub fn can_buy_with(&self, resources: &HashMap<Material, usize>) -> bool {
        self.costs.iter().all(|(material, amount)| {
            resources.get(material) >= Some(amount)
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Material {
    ORE,
    CLAY,
    OBSIDIAN,
    GEODE,
}
impl FromStr for Material {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ore" => Ok(Material::ORE),
            "clay" => Ok(Material::CLAY),
            "obsidian" => Ok(Material::OBSIDIAN),
            "geode" => Ok(Material::GEODE),
            other => Err(format!("Material {other:?} not recognized"))
        }
    }
}
