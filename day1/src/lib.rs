use std::{num::ParseIntError};

pub fn sum_elves(input: &str) -> Result<Vec<i64>, ParseIntError> {
    let mut elves = vec![];

    let mut running_total = 0;
    for line in input.lines() {
        if line == "" {
            elves.push(running_total);
            running_total = 0;
        } else {
            running_total += line.parse::<i64>()?;
        }
    }

    elves.push(running_total);  // Last

    Ok(elves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        
        let input = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let elves = sum_elves(input).unwrap();
        assert_eq!(elves, vec![6000, 4000, 11000, 24000, 10000]);
    }
}

