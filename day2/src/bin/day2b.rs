use day2::{HandShape, Outcomes, find_move_for, parse_line};


fn main() {
    let input = include_str!("../../input.txt");

    let rounds: Vec<(HandShape, Outcomes)> = input.lines().map(|line| parse_line(line).unwrap()).collect();

    let mut player_total = 0;
    for round in rounds {
        let (opponent_move, desired_outcome) = round;

        let round = find_move_for(opponent_move, desired_outcome).ok_or("Could not find a move for the desired outcome").unwrap();

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

        let rounds: Vec<(HandShape, Outcomes)> = input.lines().map(|line| parse_line(line).unwrap()).collect();

        let mut player_total = 0;
        for round in rounds {
            let (opponent_move, desired_outcome) = round;

            let round = find_move_for(opponent_move, desired_outcome).unwrap();

            let outcome = round.outcome().get();
            player_total += outcome.player_points + round.player_move.points();
        }

        assert_eq!(player_total, 12);
    }
}
