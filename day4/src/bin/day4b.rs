use day4::Pair;


fn main() {
    let input = include_str!("../../input.txt");

    let pairs: Vec<Pair> = input.lines().map(|line| line.parse().unwrap()).collect();

    let total: u64 = pairs.iter().map(|pair| pair.is_any_overlapped() as u64).sum();

    println!("Total: {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
8-9,1-2";  // <-- Extra edge case in input.txt

        let pairs: Vec<Pair> = input.lines().map(|line| line.parse().unwrap()).collect();

        let total: u64 = pairs.iter().map(|pair| pair.is_any_overlapped() as u64).sum();

        assert_eq!(total, 4);
    }
}
