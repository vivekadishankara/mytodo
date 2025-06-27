use colored::*;

pub struct Entry {
    pub title: String,
    pub completed: bool
}

impl Entry {
    pub fn new(title: String, completed: bool) -> Self {
        Entry {
            title,
            completed
        }
    }

    pub fn from_file_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() < 3 {
            return None;
        }
        let completed = parts[2].trim().parse::<u8>().unwrap();
        let completed = completed != (0 as u8);


        Some(Self{
            title: parts[1].to_string(),
            completed,
        })
    }

    pub fn to_string(&self, num: usize) -> String {
        if self.completed {
            format!("{} {}", num, self.title.strikethrough())
        } else {
            format!("{} {}", num, self.title)
        }
    }
}