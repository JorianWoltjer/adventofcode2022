use day25::SNAFU;

fn main() {
    let input = include_str!("../../input.txt");

    let numbers: Vec<isize> = input.lines().map(|line| 
        line.parse::<SNAFU>().unwrap().0
    ).collect();

    let sum: isize = numbers.iter().sum();

    let snafu = SNAFU(sum).to_string();
    println!("SNAFU: {snafu:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

        let numbers: Vec<isize> = input.lines().map(|line| 
            line.parse::<SNAFU>().unwrap().0
        ).collect();

        assert_eq!(numbers, vec![1747, 906, 198, 11, 201, 31, 1257, 32, 353, 107, 7, 3, 37]);

        let sum: isize = numbers.iter().sum();
        assert_eq!(sum, 4890);

        let tests = [
            (1747, "1=-0-2"), (906, "12111"), (198, "2=0="), 
            (11, "21"), (201, "2=01"), (31, "111"), 
            (1257, "20012"), (32, "112"), (353, "1=-1="),
            (107, "1-12"), (7, "12"), (3, "1="  ), (37, "122")
        ];

        for (test_dec, test_snafu) in tests {
            assert_eq!(SNAFU(test_dec).to_string(), test_snafu, "Decimal: {}", test_dec);
        }

        let snafu = SNAFU(sum).to_string();
        assert_eq!(snafu, String::from("2=-1=0"));
    }
}
