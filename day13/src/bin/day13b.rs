use day13::get_decoder_key;
use serde_json::Value;
use serde_json::json;

fn main() {
    let input = include_str!("../../input.txt");

    let normalized = input.replace("\n\n", "\n");
    let list: Vec<Value> = normalized.split("\n").map(|line| serde_json::from_str(line).unwrap()).collect();

    // Additional divider packets
    let divider1 = json!([[2]]);
    let divider2 = json!([[6]]);

    let decoder_key = get_decoder_key(list, divider1, divider2);

    println!("Decoder key: {}", decoder_key);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        let normalized = input.replace("\n\n", "\n");
        let list: Vec<Value> = normalized.split("\n").map(|line| serde_json::from_str(line).unwrap()).collect();

        // Additional divider packets
        let divider1 = json!([[2]]);
        let divider2 = json!([[6]]);

        let decoder_key = get_decoder_key(list, divider1, divider2);

        assert_eq!(decoder_key, 140);
    }
}
