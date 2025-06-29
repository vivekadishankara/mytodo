use std::fs;
use std::io::ErrorKind;
use std::collections::HashSet;
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
            }
        }

        Self {
            entries,
        }
    }

    pub fn enact(&mut self, command_line_args: &[String]) -> String {
        if command_line_args.len() < 2 {
            return self.list_entries();
        }
        let print_lines = match &command_line_args[1][..] {
            "list" => self.list_entries(),
            "add" => self.add(&command_line_args[2..]),
            "reset" => self.reset(),
            "done" => self.done(&command_line_args[2..]),
            "remove" => self.remove(&command_line_args[2..]),
            _ => Self::help(),
        };
        print_lines
    }

    fn write_to_file(&self) {
        let mut final_print = String::from("id,title,completed\n");
        for (idx, an_entry) in self.entries.iter().enumerate() {
            final_print += &an_entry.to_file_string(idx + 1);
            final_print += "\n";
        }
        if let Err(e) = fs::write("todo.csv", final_print) {
            eprintln!("Falied to write the file todo.csv due to error {}", e);
        }
    }

    fn list_entries(&self) -> String {
        let mut final_print = String::from("id entry");

        for (idx, an_entry) in self.entries.iter().enumerate() {
            final_print += "\n";
            final_print += &an_entry.to_screen_string(idx + 1);
        }
        
        final_print
    }

    fn publish(&self) -> String {
        self.write_to_file();
        self.list_entries()
    }

    // The command for adding an entry looks like:
    // mytodo add mango
    fn add(&mut self, args: &[String]) -> String {
        for an_arg in args {
            let new_entry = Entry::new(an_arg.clone(), false);
            self.entries.push(new_entry);
        }
        self.publish()
    }

    fn reset(&mut self) -> String {
        self.entries = Vec::new();
        self.publish()
    }

    fn arg_to_num(&self, an_arg: &String) -> usize {
        let num: usize = an_arg.parse::<usize>()
            .expect(&format!("Given entry [{}] is not a valid number", an_arg));
        if num < 1 || num > self.entries.len() {
            panic!("{}", format!("Given number [{}] is not valid as it is out of range", num));
        }
        num
    }

    fn done(&mut self, args: &[String]) -> String {
        if args.is_empty() {
            panic!("done takes at least one argument");
        }

        for an_arg in args {
            let num = self.arg_to_num(&an_arg);
            self.entries[num - 1].completed = !self.entries[num - 1].completed;
        }

        self.publish()
    }

    fn remove(&mut self, args: &[String]) -> String {
        if args.is_empty() {
            panic!("done takes at least one argument");
        }

        let mut indices_to_remove = HashSet::new();
        for an_arg in args {
            let num = self.arg_to_num(&an_arg);
            indices_to_remove.insert(num - 1);
        }

        let mut new_entries: Vec<Entry> = Vec::new();
        for (idx, an_entry) in self.entries.iter().enumerate() {
            if indices_to_remove.contains(&idx) {
                continue;
            }
            new_entries.push(an_entry.clone());
        }
        self.entries = new_entries;

        self.publish()
    }

    fn help() -> String {
        let mytodo_help = "this is help".to_string();
        mytodo_help
    }

}