use day6::find_start;


fn main() {
    let input = include_str!("../../input.txt");
    let start_offset = find_start(input, 4).unwrap();

    println!("Result: {start_offset}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(find_start(input, 4), Some(7));
    }

    #[test]
    fn example_2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(find_start(input, 4), Some(5));
    }

    #[test]
    fn example_3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(find_start(input, 4), Some(6));
    }

    #[test]
    fn example_4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(find_start(input, 4), Some(10));
    }

    #[test]
    fn example_5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(find_start(input, 4), Some(11));
    }

}
