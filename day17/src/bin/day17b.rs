use day17::{predict_many_rocks, Movement};

fn main() {
    let input = include_str!("../../input.txt");
    
    let movement: Vec<Movement> = input.chars().map(|c| c.to_string().parse().unwrap()).collect();

    let prediction = predict_many_rocks(&movement, 1000000000000);

    println!("Prediction: {prediction}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    
        let movement: Vec<Movement> = input.chars().map(|c| c.to_string().parse().unwrap()).collect();

        let prediction = predict_many_rocks(&movement, 1000000000000);

        assert_eq!(prediction, 1514285714288);
    }
}
