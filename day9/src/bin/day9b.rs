use day9::{parse_input, Rope};

fn main() {
    println!("Hello, day9b!");
}


#[cfg(test)]
mod tests {
    use day9::LongRope;

    use super::*;

    #[test]
    fn example() {
        let input = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        let movements = parse_input(input);
        
        let long_rope = LongRope::new(10);
        dbg!(long_rope);
    }
}

