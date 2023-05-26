#![feature(file_create_new)]
extern crate colored;

use crate::date::Date;
use crate::event::Event as EventInstance;
use crate::helper::{get_input, today};
use crate::mode::{mode_four, mode_one, mode_three, mode_two};
use crate::month::{MonthNaiveDate, MonthString};

use anyhow::{Ok, Result};

mod date;
mod event;
mod helper;
mod mode;
mod month;

fn main() -> Result<()> {
    let mut date = Date::new();
    let month = MonthString::new();
    let month_as_naive_date = MonthNaiveDate::new();
    let event = EventInstance::new();
    let dummy_date = "0000-00-00".to_string();
    let today = today()?;
    let event_file = EventInstance::create_filename()?;

    date.day = 1;

    println!("mode 1 (month only)/mode 2 (month + year)/mode 3 (create event)/mode 4 (show event)");
    let mode: u8 = get_input();

    if mode == 1 {
        mode_one(date, month, month_as_naive_date, dummy_date, today)?;
    } else if mode == 2 {
        mode_two(date, month, month_as_naive_date, today, dummy_date)?;
    } else if mode == 3 {
        mode_three(date, event, event_file)?;
    } else if mode == 4 {
        mode_four(event, event_file)?;
    }

    Ok(())
}
