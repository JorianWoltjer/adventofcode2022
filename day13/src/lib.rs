use std::cmp::Ordering;

use serde_json::{Value};

pub fn compare(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => left.as_i64().cmp(&right.as_i64()),
        (Value::Array(left), Value::Array(right)) => {
            for i in 0..left.len() {
                let left_value = left.get(i).unwrap();
                
                if let Some(right_value) = right.get(i) {  // If there is a right item left
                    match compare(left_value, right_value) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => continue,
                        Ordering::Greater => return Ordering::Greater,
                    };
                } else {
                    break;
                }
            }
            return left.len().cmp(&right.len());  // If loop ended without conclusion, use length
        }
        (left @ Value::Array(_), right @ Value::Number(_)) => {
            compare(left, &Value::Array(Vec::from([right.clone()])))
        }
        (left @ Value::Number(_), right @ Value::Array(_)) => {
            compare(&Value::Array(Vec::from([left.clone()])), right)
        }
        (_, _) => unimplemented!()
    }
}

pub fn parse_input(input: &str) -> Vec<(Value, Value)> {
    input.split("\n\n").map(|lines| {
        let mut l = lines.lines();
        (serde_json::from_str(l.next().unwrap()).unwrap(), serde_json::from_str(l.next().unwrap()).unwrap())
    }).collect()
}

pub fn get_decoder_key(list: Vec<Value>, divider1: Value, divider2: Value) -> usize {
    assert_eq!(compare(&divider2, &divider1), Ordering::Greater);  // Make sure 2 > 1 for optimisation to work

    let mut divider1_index = 1;
    let mut divider2_index = 2;

    // Count number of items that would be before divider
    for value in list {
        if compare(&divider1, &value) == Ordering::Greater {
            divider1_index += 1;
            divider2_index += 1;  // divider2 is larger than divider1, so this will always be true
        } else if compare(&divider2, &value) == Ordering::Greater {
            divider2_index += 1;
        }
    }

    divider1_index * divider2_index
}
