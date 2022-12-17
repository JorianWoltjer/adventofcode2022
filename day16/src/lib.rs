#![feature(hash_drain_filter)]

#[macro_use]
extern crate lazy_static;

use std::{str::FromStr, collections::{HashMap, VecDeque, HashSet}};

use regex::Regex;

#[derive(Debug)]
pub struct Cave {
    pub valves: HashMap<String, Valve>,
    pub start: String,
}
impl FromStr for Cave {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut valves = HashMap::new();

        lazy_static! {
            static ref RE_VALVE: Regex = Regex::new(r"^Valve ([A-Z]{2}) has flow rate=(-?\d+); tunnels? leads? to valves? (.*)$").unwrap();
        }
        for line in s.lines() {
            let matches = RE_VALVE.captures(line).ok_or("No match found with regex")?;
            let name = matches[1].to_string();
            let flow_rate: usize = matches[2].parse().map_err(|_| "Could not parse int")?;
            let tunnels = matches[3].split(", ").map(|s| s.to_string()).collect();

            let mut distances = HashMap::new();
            distances.insert(name.clone(), 0);  // Distance to self is 0

            let valve = Valve { flow_rate, tunnels, distances };
            valves.insert(name, valve);
        }

        let mut cave = Self { valves, start: String::from("AA") };

        cave.calculate_distances();

        Ok(cave)
    }
}
impl Cave {
    /* 
    ! IDEA !
    - Save distance_to cache FAST (precomputed)
    - only consider rate>0 as nodes, others just make the path cost more
        - edge case: AA is a node with rate=0
    - Then do recursive algorithm
        - Try to go to a target node, keep going recursively
        - Don't look at visited nodes anymore
            - Pass a list of nodes left? remove from list when used
        - Score is calculated at end condition when time<=0
            - Needs a bit of calculation to get the correct score if it went multiple past the timer
    */
    fn calculate_distances(&mut self) {
        let valves: Vec<String> = self.valves.keys().cloned().collect();
        for valve in valves {
            self.calculate_distances_for(&valve);
        }
    }

    /// Breadth first search to find distances
    fn calculate_distances_for(&mut self, valve: &String) {
        let root = self.valves.get(valve).unwrap();
        let mut queue = VecDeque::from([String::from(valve)]);
        let mut explored: HashMap<String, usize> = root.distances.clone();

        while !queue.is_empty() {
            let v = queue.pop_front().unwrap();  // Get from the queue

            // Add children to the queue
            for tunnel in &self.valves.get(&v).unwrap().tunnels {
                if !explored.contains_key(tunnel) {
                    let depth = *explored.get(&v).unwrap();

                    // Insert into distances
                    explored.insert(tunnel.clone(), depth + 1);
                    queue.push_back(tunnel.clone());
                }
            }
        }

        // Filter out rates of 0
        explored.drain_filter(|valve, _distance| {
            self.valves.get(valve).unwrap().flow_rate == 0
        });
        let root = self.valves.get_mut(valve).unwrap();
        root.distances = explored;
    }

    pub fn find_highest_score(&self) -> usize {
        let start = self.start.clone();
        let start_valve = self.valves.get(&start).unwrap();
        let closed_valves = HashSet::from_iter(start_valve.distances.keys().cloned());

        let (total, rate, time) = (0, 0, 30);

        // First round
        closed_valves.iter().map(|valve| {
            let time_passed = *start_valve.distances.get(valve).unwrap() + 1;
            let time = time - time_passed as isize;
            // let total = total + rate * time_passed;
            self.recursive_highest_score(valve.clone(), closed_valves.clone(), total, rate, time, 0)
        }).max().unwrap()
    }

    fn recursive_highest_score(&self, current: String, mut closed_valves: HashSet<String>, mut total: usize, mut rate: usize, time: isize, depth: usize) -> usize {
        if time <= 0 {
            total -= rate * time.abs() as usize;  // Back to zero
            return total;
        } else if closed_valves.len() <= 1 {
            // Add to rate one last time
            let current_valve = self.valves.get(&current).unwrap();
            rate += current_valve.flow_rate;

            total += rate * time as usize;  // Predict for future (at max rate)
            return total;
        }

        let current_valve = self.valves.get(&current).unwrap();
        closed_valves.remove(&current);  // Open valve
        rate += current_valve.flow_rate;

        // Possible next valves
        closed_valves.iter().map(|valve| {
            let time_passed = *current_valve.distances.get(valve).unwrap() + 1;
            let time = time - time_passed as isize;
            let total = total + rate * time_passed;
            self.recursive_highest_score(valve.clone(), closed_valves.clone(), total, rate, time, depth+1)
        }).max().unwrap()
    }
}

#[derive(Debug)]
pub struct Valve {
    pub flow_rate: usize,
    pub tunnels: Vec<String>,
    pub distances: HashMap<String, usize>,
}
