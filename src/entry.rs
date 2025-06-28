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
        let completed = parts[2].trim().parse::<u8>().ok()?;
        let completed = completed != (0 as u8);


        Some(Self{
            title: parts[1].trim().to_string(),
            completed,
        })
    }

    pub fn to_screen_string(&self, num: usize) -> String {
        if self.completed {
            format!("{}  {}", num, self.title.strikethrough())
        } else {
            format!("{}  {}", num, self.title)
        }
    }

    pub fn to_file_string(&self, num: usize) -> String {
        let completed = if self.completed { 1 } else { 0 };
        format!("{},{},{}", num, self.title, completed)
    }
}