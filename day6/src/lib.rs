use std::{collections::VecDeque, cmp::max};


pub fn find_start(stream: &str, buffer_length: usize) -> Option<usize> {
    if stream.len() < buffer_length {  // If not long enough
        return None;
    }
    // Initialize buffer
    let mut buffer = VecDeque::from((0..buffer_length-1).map(|_| '\x00').collect::<Vec<char>>());

    let mut ignore = 0;

    // Go through rest of the stream
    for (i, c) in stream.chars().enumerate() {
        // println!("Buffer: {buffer:?} {c:?}");
        
        if let Some(index) = buffer.iter().rev().position(|item| item == &c) {
            let real_index = buffer_length-2 - index;
            // println!(" = Duplicate ({real_index})");
            ignore = max(ignore, real_index+1);  // Overwrite if needs to skip more
        } else if ignore <= 0 && i >= buffer_length {
            // println!("Found!");
            return Some(i+1);
        }

        if ignore > 0 {
            // println!("Skipping ({ignore} left)...");
            ignore -= 1;
        }

        buffer.pop_front();
        buffer.push_back(c);
    }

    None
}
