use std::fs;
use std::io::ErrorKind;
use crate::entry::Entry;

pub struct Todo {
    pub entries: Vec<Entry>,
}

impl Todo {
    pub fn new() -> Self {
        let content = match fs::read_to_string("todo.csv") {
            Ok(content) => content,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                String::new()
            }
            Err(e) => {
                panic!("Error reading todo.csv: {}", e)
            }
        };

        let mut entries: Vec<Entry> = Vec::new();
        for line in content.lines().skip(1) {
            if let Some(entry) = Entry::from_file_line(line) {
                entries.push(entry);
                println!("Made an entry");
            }
        }

        Self {
            entries,
        }
    }

    pub fn enact(&self, command_line_args: &Vec<String>) {
        if command_line_args.len() < 2 {
            self.list_entries();
            return;
        }
        match &command_line_args[1][..] {
            "add" => self.add(&command_line_args[2..]),
            _ => self.list_entries(),
        };
    }

    // The command for adding an entry looks like:
    // mytodo add mango
    fn add(&self, args: &[String]) {
        panic!("This is add");
    }

    fn list_entries(&self) {
        panic!("This is list_entries");
    }
}