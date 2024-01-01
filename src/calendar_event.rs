use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

use anyhow::{Ok, Result};

use crate::{date::Date, get_input, today};

#[derive(Debug)]
pub struct Event {
    pub day: u32,
    pub month: u32,
    pub title: String,
    pub content: String,
}

impl Event {
    pub fn new() -> Self {
        Self {
            day: 0,
            month: 0,
            title: String::new(),
            content: String::new(),
        }
    }

    pub fn create_event(&mut self, date: Date) -> Self {
        println!("title:");
        let title = get_input();

        println!("description:");
        let content = get_input();

        Self {
            day: date.day,
            month: date.month,
            title,
            content,
        }
    }

    pub fn create_filename() -> Result<String> {
        let today = today()?;

        let mut filename = String::new();

        filename.push_str(&today.day.to_string());
        filename.push_str("-");
        filename.push_str(&today.month.to_string());
        filename.push_str("-");
        filename.push_str(&today.year.to_string());
        filename.push_str(".txt");

        Ok(filename)
    }

    pub fn write_event_to_file(&mut self, filename: &String) -> Result<()> {
        let mut file = File::create_new(filename)?;

        write!(file, "{}", &self.day)?;
        write!(file, "{}", "\n")?;
        write!(file, "{}", &self.month)?;
        write!(file, "{}", "\n")?;
        write!(file, "{}", &self.title)?;
        write!(file, "{}", "\n")?;
        write!(file, "{}", &self.content)?;

        Ok(())
    }

    pub fn read_event_from_file(&mut self, path: &str) -> Result<Self> {
        let mut lines = Vec::new();

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            lines.push(line?);
        }

        Ok(Self {
            day: lines[0].parse::<u32>()?,
            month: lines[1].parse::<u32>()?,
            title: lines[2].clone(),
            content: lines[3].clone(),
        })
    }
}
