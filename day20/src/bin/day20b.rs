use day20::move_by_value_times;

fn main() {
    let input = include_str!("../../input.txt");

    let list: Vec<isize> = input.lines().map(|line| line.parse::<isize>().unwrap() * 811589153).collect();
    let moved = move_by_value_times(list, 10);

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

        let list: Vec<isize> = input.lines().map(|line| line.parse::<isize>().unwrap() * 811589153).collect();
        assert_eq!(list, vec![811589153, 1623178306, -2434767459, 2434767459, -1623178306, 0, 3246356612]);
        let moved = move_by_value_times(list, 10);

        assert_eq!(moved, vec![0, -2434767459, 1623178306, 3246356612, -1623178306, 2434767459, 811589153]);

        let zero_index = moved.iter().position(|value| value == &0).unwrap();

        let result: Vec<isize> = [1000, 2000, 3000].iter().map(|n| moved[(zero_index + n) % moved.len()]).collect();
        assert_eq!(result, vec![811589153, 2434767459, -1623178306]);
        assert_eq!(result.iter().sum::<isize>(), 1623178306);
    }

}
