use anyhow::{Ok, Result};

use std::fs;

use crate::helper::{crop_dates, print_month};
use crate::{
    date::Date,
    event::Event,
    get_input,
    month::{MonthNaiveDate, MonthString},
};

pub fn mode_one(
    mut date: Date,
    month: MonthString,
    mut month_as_naive_date: MonthNaiveDate,
    dummy_date: String,
    current_date: Date,
) -> Result<()> {
    println!("month ():");
    date.month = get_input();

    date.year = current_date.year;

    let sorted_month_string = month
        .clone()
        .build_month_from_date_string(&mut month_as_naive_date, date);
    let sorted_month_string = sorted_month_string
        .clone()
        .push_dummy_dates_to_vectors(date, dummy_date);
    let cropped_string = crop_dates(sorted_month_string, current_date)?;

    print_month(cropped_string)?;

    Ok(())
}

pub fn mode_two(
    mut date: Date,
    month: MonthString,
    mut month_as_naive_date: MonthNaiveDate,
    current_date: Date,
    dummy_date: String,
) -> Result<()> {
    println!("month (as number):");
    date.month = get_input();

    println!("year (as number):");
    date.year = get_input::<i32>().try_into()?;

    let sorted_month_string = month
        .clone()
        .build_month_from_date_string(&mut month_as_naive_date, date);
    let sorted_month_string = sorted_month_string
        .clone()
        .push_dummy_dates_to_vectors(date, dummy_date);
    let cropped_string = crop_dates(sorted_month_string, current_date)?;

    print_month(cropped_string)?;

    Ok(())
}

pub fn mode_three(mut date: Date, mut event: Event, filename: String) -> Result<()> {
    println!("day:");
    date.day = get_input();

    println!("month:");
    date.month = get_input();

    let mut event = Event::create_event(&mut event, date);

    if event.write_event_to_file(&filename).is_err() {
        fs::remove_file(&filename)?;
        event.write_event_to_file(&filename)?;
    } else {
        event.write_event_to_file(&filename)?;
    }

    Ok(())
}

pub fn mode_four(mut event: Event, filename: String) -> Result<()> {
    let event_content = event.read_event_from_file(&filename)?;

    println!("title: {}", event_content.title);
    println!("date: {}.{}", event_content.day, event_content.month);
    println!("description: {}", event_content.content);

    Ok(())
}
