use day17::{Game, Movement};

fn main() {
    let input = include_str!("../../input.txt");

    let movement: Vec<Movement> = input.chars().map(|c| c.to_string().parse().unwrap()).collect();
    let mut game = Game::new(movement);

    let height = game.drop_n_rocks(2022);

    println!("Height: {height}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        let movement: Vec<Movement> = input.chars().map(|c| c.to_string().parse().unwrap()).collect();
        let mut game = Game::new(movement);

        let height = game.drop_n_rocks(2022);

        assert_eq!(height, 3068);
    }
}
