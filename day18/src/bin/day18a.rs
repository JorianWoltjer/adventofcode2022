use day18::Shape;

fn main() {
    let input = include_str!("../../input.txt");

    let shape: Shape = input.parse().unwrap();
    let surface_area = shape.surface_area();

    println!("Area: {surface_area}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_example() {
        let input = "\
1,1,1
2,1,1";

        let shape: Shape = input.parse().unwrap();
        let surface_area = shape.surface_area();

        assert_eq!(surface_area, 10);
    }
    
    #[test]
    fn larger_example() {
        let input = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

        let shape: Shape = input.parse().unwrap();
        let surface_area = shape.surface_area();

        assert_eq!(surface_area, 64);
    }
}
