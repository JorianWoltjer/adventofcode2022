#![allow(unused_imports)]
use day19::Blueprint;

fn main() {
    unimplemented!("day19a");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]  // Takes too long
    #[test]
    fn example() {
        let input = "\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

        let blueprints: Vec<Blueprint> = input.lines().map(|line| line.parse().unwrap()).collect();

        let time = 24;  // Minutes
        let most_geodes_1 = blueprints[0].find_most_geodes(time);
        assert_eq!(most_geodes_1, 9);
        let most_geodes_1 = blueprints[1].find_most_geodes(time);
        assert_eq!(most_geodes_1, 12);
        
        let most_geodes = blueprints.iter().map(|blueprint| blueprint.find_most_geodes(time)).max();
        assert_eq!(most_geodes, Some(12));
    }
}
