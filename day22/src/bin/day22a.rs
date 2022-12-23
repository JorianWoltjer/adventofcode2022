use day22::Map;

fn main() {
    let input = include_str!("../../input.txt");
    let mut s = input.split("\n\n");

    let mut map: Map = s.next().unwrap().parse().unwrap();

    let movement = s.next().unwrap();
    map.do_movement(movement);

    let password = map.get_password();
    println!("Password: {password}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

        let mut s = input.split("\n\n");

        let mut map: Map = s.next().unwrap().parse().unwrap();
        
        let movement = s.next().unwrap();
        map.do_movement(movement);

        let password = map.get_password();
        assert_eq!(password, 6032);
    }

    #[test]
    fn more_wrapping() {
        let input = " ......
.......
......
......
 .....

L1L1L1L1L1R1";
let mut s = input.split("\n\n");

        let mut map: Map = s.next().unwrap().parse().unwrap();
        println!("Horizontal: {:?}", map.horizontal_bounds);
        println!("Vertical: {:?}", map.vertical_bounds);

        let movement = s.next().unwrap();
        map.do_movement(movement);

        map.print();
        assert_eq!(map.walker_pos, (0, 1));
    }
}
