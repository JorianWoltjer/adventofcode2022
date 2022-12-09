use day9::{parse_input, LongRope};

fn main() {
    let input = include_str!("../../input.txt");

    let movements = parse_input(input);

    let mut rope = LongRope::new(10);

    for (direction, amount) in movements {
        rope.move_head(&direction, amount);
    }

    println!("Visited: {}", rope.tail_visited.len());
}

#[cfg(test)]
mod tests {

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
        
        let mut long_rope = LongRope::new(10);
        
        for (direction, amount) in movements {
            long_rope.move_head(&direction, amount);
        }

        assert_eq!(long_rope.tail_visited.len(), 1);
    }

    #[test]
    fn example_2() {
        let input = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        let movements = parse_input(input);
                
        let mut long_rope = LongRope::new(10);

        for (direction, amount) in movements.iter() {
            long_rope.move_head(&direction, *amount);
        }

        assert_eq!(long_rope.tail_visited.len(), 36);
    }
}
