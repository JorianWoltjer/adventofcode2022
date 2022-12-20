use day20::move_by_value;

fn main() {
    let input = include_str!("../../input.txt");

    let list: Vec<isize> = input.lines().map(|line| line.parse().unwrap()).collect();
    let moved = move_by_value(list);

    let zero_index = moved.iter().position(|value| value == &0).unwrap();

    let sum: isize = [1000, 2000, 3000].iter().map(|n| moved[(zero_index + n) % moved.len()]).sum();

    println!("Sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
1
2
-3
3
-2
0
4";

        let list: Vec<isize> = input.lines().map(|line| line.parse().unwrap()).collect();
        let moved = move_by_value(list);

        assert_eq!(moved, vec![1, 2, -3, 4, 0, 3, -2]);

        let zero_index = moved.iter().position(|value| value == &0).unwrap();

        let result: Vec<isize> = [1000, 2000, 3000].iter().map(|n| moved[(zero_index + n) % moved.len()]).collect();
        assert_eq!(result, vec![4, -3, 2]);
        assert_eq!(result.iter().sum::<isize>(), 3);
    }

}
