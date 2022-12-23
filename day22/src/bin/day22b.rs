use day22::PORTALS_B_REAL;
use day22::MapCube;

fn main() {
    let input = include_str!("../../input.txt");
    let mut s = input.split("\n\n");

    let mut map = MapCube::new(s.next().unwrap(), PORTALS_B_REAL.clone());
    // map.print();

    let movement = s.next().unwrap();
    map.do_movement(movement);

    let password = map.get_password();
    println!("Password: {password}");
}
