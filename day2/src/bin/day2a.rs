use day2::Round;

fn main() {
    let input = include_str!("../../input.txt");

    let rounds: Vec<Round> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut player_total = 0;
    for round in rounds {
        let outcome = round.outcome().get();
        player_total += outcome.player_points + round.player_move.points();
    }

    println!("Total: {}", player_total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
A Y
B X
C Z";

        let rounds: Vec<Round> = input.lines().map(|line| line.parse().unwrap()).collect();

        let mut player_total = 0;
        for round in rounds {
            let outcome = round.outcome().get();
            player_total += outcome.player_points + round.player_move.points();
        }

        assert_eq!(player_total, 15);
    }
    
}
