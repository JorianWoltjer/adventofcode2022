use std::fmt;

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
            let mut new_index = (i as isize + number.value).rem_euclid(len as isize - 1);

            if new_index == 0 {  // Needs one more to overflow
                new_index = (len-1) as isize;
            }
            let new_index = new_index as usize;

            list.insert(new_index, number);  // Push
            
            if new_index > i {  // If insertion messed up indexing
                continue;  // Don't increment `i`
            }
        }
        i += 1;
    }

    // println!("FINAL {list:?}");

    // Convert back to plain list
    list.iter().map(|item| item.value).collect()
}

/// Number for part B, allowing a number of moves
pub struct NumberB {
    pub value: isize,
    pub moved: usize,
    pub i: usize,
}
impl NumberB {
    pub fn new(i: usize, value: isize) -> Self {
        Self { value, moved: 0, i }
    }
}
impl fmt::Debug for NumberB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.value, self.moved)
    }
}

pub fn move_by_value_times(list: Vec<isize>, times: usize) -> Vec<isize> {
    // Add `moved` attribute to items
    let mut list: Vec<NumberB> = list.iter().enumerate().map(|(i, item)| NumberB::new(i, *item)).collect();
    let len = list.len();

    for round in 0..times {
        let mut i = 0;
    
        while i < list.len() {
            // Make sure to use the same indexes as decided at the start
            // Really inefficient but it works ¯\_(ツ)_/¯
            let list_index = list.iter().position(|item| item.i == i).unwrap();
            let number = &list[list_index];

            if number.moved <= round && number.value != 0 {  // If it needs to be moved
                // println!("List: {list:?} list[{i}] = {number:?}");

                let mut number = list.remove(list_index);  // Pop
                number.moved += 1;
                // Calculate index
                let mut new_index = (list_index as isize + number.value).rem_euclid(len as isize - 1);

                if new_index == 0 {  // Needs one more to overflow
                    new_index = (len-1) as isize;
                }
                let new_index = new_index as usize;

                list.insert(new_index, number);  // Push
                
                if new_index > list_index {  // If insertion messed up indexing
                    continue;  // Don't increment `i`
                }
            }
            i += 1;
        }

        // println!("End of round {round}: {list:?}");
        assert!(list.iter().all(|item| item.moved == round+1 || item.value == 0));
    }

    // println!("FINAL {list:?}");

    // Convert back to plain list
    list.iter().map(|item| item.value).collect()
}
