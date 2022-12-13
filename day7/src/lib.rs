#![feature(iter_advance_by)]

use std::{collections::{LinkedList, HashMap}, cell::RefCell, rc::Rc, str::FromStr};

#[derive(Debug)]
pub struct Shell {
    pub levels: LinkedList<Rc<RefCell<Directory>>>,
}

#[derive(Debug)]
pub enum Item {
    Directory(Rc<RefCell<Directory>>),
    File(File),
}

#[derive(Debug)]
pub struct Directory {
    children: HashMap<String, Item>,
}

#[derive(Debug)]
pub struct File {
    pub size: u64,
}

impl Directory {
    pub fn new() -> Self {
        Directory { children: HashMap::new() }
    }

    pub fn get_total_size(&self) -> u64 {
        self.children.values().map(|item| {
            match item {
                Item::File(file) => file.size,
                Item::Directory(directory) => directory.borrow().get_total_size(),
            }
        }).sum()
    }

    pub fn get_all_sizes(&self) -> Vec<u64> {
        let mut result = vec![];

        // Push itself
        result.push(self.get_total_size());
        
        // Push any children recusively
        for item in self.children.values() {
            match item {
                Item::File(_) => {},
                Item::Directory(directory) => {
                    for dirsize in directory.borrow().get_all_sizes() {
                        result.push(dirsize);
                    }
                }
            }
        }
        result
    }
}

impl Shell {
    pub fn new() -> Self {
        // Empty root directory
        Self { 
            levels: LinkedList::from([
                Rc::new(RefCell::new(Directory { 
                    children: HashMap::new()
                }))
            ]) 
        }
    }

    pub fn change_directory(&mut self, directory: &str) -> Result<(), String> {
        match directory {
            "/" => {  // Go to root directory
                while self.levels.len() > 1 {
                    self.levels.pop_back();
                }
            },
            ".." => {  // Go one directory up
                self.levels.pop_back();
            }
            directory => {  // Move into directory if exists
                let pwd = Rc::clone(self.levels.back().unwrap());
                let pwd = pwd.borrow();
                let directory_to_enter = pwd.children.get(directory).ok_or(format!("{directory:?} no such File or Directory"))?;

                match directory_to_enter {
                    Item::File(_) => return Err("Cannot enter a file".to_string()),
                    Item::Directory(directory) => {
                        self.levels.push_back(Rc::clone(directory));
                    }
                }
            }
        }

        Ok(())
    }

    fn create_item(&mut self, name: &str, item: Item) {
        self.levels.back_mut().unwrap()
            .borrow_mut().children.insert(name.to_string(), item);
    }

    /// Create a directory in the current working directory
    pub fn create_directory(&mut self, name: &str) {
        let item = Item::Directory(Rc::new(RefCell::new(Directory::new())));
        self.create_item(name, item)
    }

    /// Create a file in the current working directory
    pub fn create_file(&mut self, size: u64, name: &str) {
        let item = Item::File(File { size });
        self.create_item(name, item)
    }
}

impl FromStr for Shell {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut shell = Self::new();

        let mut input = s.lines().peekable();
        loop {
            let mut command = match input.next() {
                Some(command) => {
                    // println!("COMMAND: {command:?}");
                    command.split(" ")
                }
                None => break
            };

            if let Some("$") = command.next() {
                match (command.next(), command.next()) {
                    (Some("cd"), Some(directory)) => {
                        // println!("Changing directory to {directory:}");
                        shell.change_directory(directory)?;
                    }
                    (Some("ls"), None) => {
                        // println!("Reading from ls");
                        loop {
                            match input.peek() {
                                Some(peeked_item) => {
                                    let mut split_item = peeked_item.split(" ");
                                    match split_item.next() {
                                        Some("$") => {
                                            // println!("- Found another command with '$', stopping ls");
                                            break;  // Don't advance input, so next command can execute
                                        }
                                        Some("dir") => {
                                            let name = split_item.next().ok_or("No directory name found")?;
                                            // println!("- Creating directory {name:?}");
                                            shell.create_directory(name);
                                            input.next();  // This peek was correct, so take it
                                        }
                                        Some(size) => {
                                            let name = split_item.next().ok_or("No filename found")?;
                                            // println!("- Creating file {name:?} with size {size}");
                                            shell.create_file(
                                                size.parse().map_err(|_| "Failed to parse size int")?,
                                                name
                                            );
                                            input.next();  // This peek was correct, so take it
                                        }
                                        None => {
                                            return Err("ls item is empty".to_string());
                                        }
                                    }
                                }
                                None => break
                            }
                        }
                    }
                    (Some(other), arg) => {
                        return Err(format!("Unrecognized command {other:?} with argument {arg:?}"));
                    }
                    (None, _) => {
                        return Err("No command found".to_string());
                    }
                }
            } else {
                return Err(format!("No '$' found at start of command ({command:?})"));
            }
        }

        Ok(shell)
    }
}
