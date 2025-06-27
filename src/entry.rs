#[derive(Debug)]
pub struct Entry {
    pub id: u32,
    pub title: String,
    pub completed: bool
}

impl Entry {
    pub fn new(id: u32, title: String, completed: bool) -> Self {
        Entry {
            id,
            title,
            completed
        }
    }

    pub fn from_file_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() < 3 {
            return None;
        }
        let id = parts[0].trim().parse::<u32>().unwrap();
        let completed = parts[2].trim().parse::<u8>().unwrap();
        let completed = completed != (0 as u8);


        Some(Self{
            id,
            title: parts[1].to_string(),
            completed,
        })
    }
}