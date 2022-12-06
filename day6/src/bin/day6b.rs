use day6::find_start;


fn main() {
    let input = include_str!("../../input.txt");
    let start_offset = find_start(input, 14).unwrap();

    println!("Result: {start_offset}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(find_start(input, 14), Some(19));
    }

    #[test]
    fn example_2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(find_start(input, 14), Some(23));
    }

    #[test]
    fn example_3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(find_start(input, 14), Some(23));
    }

    #[test]
    fn example_4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(find_start(input, 14), Some(29));
    }

    #[test]
    fn example_5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(find_start(input, 14), Some(26));
    }
}
