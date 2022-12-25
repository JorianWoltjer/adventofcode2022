use std::str::FromStr;

#[derive(Debug)]
pub struct SNAFU (pub isize);

impl FromStr for SNAFU {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = 0;
        let len = s.len();
        
        for (i, c) in s.chars().enumerate() {  // Left to right (biggest first)
            let power = 5_isize.pow((len-1 - i) as u32);
            let digit = match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,  // Minus
                '=' => -2,  // Double minus
                other => { return Err(format!("{other:?} is not a valid SNAFU digit")) }
            };

            value += digit * power;
        }

        Ok(Self(value))
    }
}
impl SNAFU {
    pub fn to_string(&self) -> String {
        let n = self.0;

        let mut result = String::new();

        // Calculate the highest power needed
        let mut i = 0;
        while n > calculate_max_for(i) {
            i += 1;
        }

        // Calculate needed other digits to get exact n
        let mut needed = n;
        for power in (0..=i).rev() {

            let max_below = calculate_max_for(power-1);
            let value = 5_isize.pow(power as u32);

            if needed > max_below {  // If nothing else can make this big of a number
                if needed - value > max_below {  // If one is not enough
                    result.push('2');
                    needed -= value*2;
                } else {  // One is enough
                    result.push('1');
                    needed -= value;
                }
            } else if needed < -max_below {  // If nothing else can make this small of a number
                if needed + value < -max_below {  // If one is not enough
                    result.push('=');  // Double minus
                    needed += value*2;
                } else {  // One is enough
                    result.push('-');  // Minus
                    needed += value;
                }
            } else {  // This number is not needed
                result.push('0');
            }
        }

        result
    }
}

// The highest number you can make only using this power and below
pub fn calculate_max_for(power: isize) -> isize {
    let mut total = 0;
    let mut fives = 1;
    for _ in 0..=power {
        total += 2 * fives;
        fives *= 5;
    }
    total
}
