
pub fn split(s: &str) -> (String, String) {
    let middle = s.len() / 2;
    (s[..middle].to_owned(), s[middle..].to_owned())
}

pub fn find_shared_in_two(a: String, b: String) -> Option<char> {
    for a_char in a.chars() {
        if b.contains(a_char) {
            return Some(a_char);
        }
    }

    None
}

/// Finds a character that is shared between all strings in `lines`
pub fn find_shared_in_all(lines: Vec<&str>) -> Option<char> {
    let first = lines.first()?;

    'outer: for char in first.chars() {
        for line in &lines[1..] {
            if !line.contains(char) {  // Not found in other line
                continue 'outer;  // Try next char
            }
        }

        return Some(char);  // Passed for all lines
    }

    None
}

pub fn priority(c: char) -> u64 {
    let c = c as u8;

    if c >= b'a' && c <= b'z' {  // Lowercase
        (c - b'a' + 1).into()
    } else if c >= b'A' && c <= b'Z' {  // Uppercase
        (c - b'A' + 27).into()
    } else {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        for (input, output) in [('p', 16), ('L', 38), ('P', 42), ('v', 22), ('t', 20), ('s', 19)] {
            assert_eq!(priority(input), output, "Input was: {:?}", input);
        }
    }
}

