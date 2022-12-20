use std::fmt;
use std::cmp::max;

pub struct Number {
    pub value: isize,
    pub moved: bool,
}
impl Number {
    pub fn new(value: isize) -> Self {
        Self { value, moved: false }
    }
}
impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub fn move_by_value(list: Vec<isize>) -> Vec<isize> {
    // Add `moved` attribute to items
    let mut list: Vec<Number> = list.iter().map(|item| Number::new(*item)).collect();
    let len = list.len();
    
    let mut i = 0;
    while i < list.len() {
        let number = &list[i];
        
        if !number.moved && number.value != 0 {  // If it needs to be moved
            // println!("List: {list:?} list[{i}] = {number:?}");

            let mut number = list.remove(i);  // Pop
            number.moved = true;
            // Calculate index
            let mut new_index = i as isize + number.value;
            if new_index <= 0 || new_index >= len as isize {  // Wrap around left
                new_index = new_index.rem_euclid(len as isize - 1);
                if new_index == 0 {  // Needs one more to overflow
                    new_index = (len-1) as isize;
                }
            }
            let new_index = new_index as usize;

            list.insert(new_index, number);  // Push
            
            // if new_index > i {  // If insertion messed up indexing
                // }
            if i > 0 {
                i -= 1;
            }
            continue;  // Don't increment `i`
        }
        i += 1;
    }

    // println!("FINAL {list:?}");

    // Convert back to plain list
    list.iter().map(|item| item.value).collect()
}
