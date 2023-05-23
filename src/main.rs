extern crate colored;

use crate::date::Date;
use crate::month::{MonthNaiveDate, MonthString};

use std::str::FromStr;
use std::{cmp, fmt};

use anyhow::{anyhow, Ok, Result};
use chrono::Local;
use colored::Colorize;

mod date;
mod month;

fn main() -> Result<()> {
    let mut date = Date::new();
    let month = MonthString::new();
    let mut month_as_naive_date = MonthNaiveDate::new();
    let dummy_date = "0000-00-00".to_string();
    let today = today()?;

    date.day = 1;

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
    let cropped_string = crop_dates(sorted_month_string, today)?;

    print_month(cropped_string)?;

    Ok(())
}

fn get_input<T: std::str::FromStr>() -> T
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buffer = String::new();
    let stdin = std::io::stdin();

    stdin.read_line(&mut buffer).expect("BROKEN STDIN");

    let input = buffer.trim();

    input.parse().expect("Peng")
}

fn print_month(mut dates: MonthString) -> Result<()> {
    if !dates.check_if_month_are_all_equal() {
        return Err(anyhow!("All fields need to have the same length"));
    }

    println!(" monday | tuesday | wednesday | thursday | friday | saturday | sunday ");
    println!("--------|---------|-----------|----------|--------|----------|--------");

    let length = cmp::max(
        dates.monday.len(),
        cmp::max(
            dates.tuesday.len(),
            cmp::max(
                dates.wednesday.len(),
                cmp::max(
                    dates.thursday.len(),
                    cmp::max(
                        dates.friday.len(),
                        cmp::max(dates.saturday.len(), dates.sunday.len()),
                    ),
                ),
            ),
        ),
    );

    for i in 0..length {
        let output_string = fmt::format(format_args!(
            " {}     | {}      | {}        | {}       | {}     | {}       | {} {}",
            &dates.monday[i],
            &dates.tuesday[i],
            &dates.wednesday[i],
            &dates.thursday[i],
            &dates.friday[i],
            &dates.saturday[i],
            &dates.sunday[i],
            "\n"
        ));

        print!("{}", output_string);
    }

    Ok(())
}

fn crop_dates(dates: MonthString, current_date: Date) -> Result<MonthString> {
    let mut monday: Vec<String> = Vec::new();
    let mut tuesday: Vec<String> = Vec::new();
    let mut wednesday: Vec<String> = Vec::new();
    let mut thursday: Vec<String> = Vec::new();
    let mut friday: Vec<String> = Vec::new();
    let mut saturday: Vec<String> = Vec::new();
    let mut sunday: Vec<String> = Vec::new();

    for day in dates.monday.clone() {
        let date = day.split("-").collect::<Vec<&str>>();

        if day.contains("0000-00-00") {
            monday.push("--".to_string());
        } else if current_date.year == date[0].parse::<i32>()?
            && current_date.month == date[1].parse::<u32>()?
            && current_date.day == date[2].parse::<u32>()?
        {
            monday.push(date[2].yellow().to_string());
        } else {
            monday.push(date[2].to_string());
        }
    }

    for day in dates.tuesday.clone() {
        let date = day.split("-").collect::<Vec<&str>>();

        if day.contains("0000-00-00") {
            tuesday.push("--".to_string());
        } else if current_date.year == date[0].parse::<i32>()?
            && current_date.month == date[1].parse::<u32>()?
            && current_date.day == date[2].parse::<u32>()?
        {
            tuesday.push(date[2].yellow().to_string());
        } else {
            tuesday.push(date[2].to_string());
        }
    }

    for day in dates.wednesday.clone() {
        let date = day.split("-").collect::<Vec<&str>>();

        if day.contains("0000-00-00") {
            wednesday.push("--".to_string());
        } else if current_date.year == date[0].parse::<i32>()?
            && current_date.month == date[1].parse::<u32>()?
            && current_date.day == date[2].parse::<u32>()?
        {
            wednesday.push(date[2].yellow().to_string());
        } else {
            wednesday.push(date[2].to_string());
        }
    }

    for day in dates.thursday.clone() {
        let date = day.split("-").collect::<Vec<&str>>();

        if day.contains("0000-00-00") {
            thursday.push("--".to_string());
        } else if current_date.year == date[0].parse::<i32>()?
            && current_date.month == date[1].parse::<u32>()?
            && current_date.day == date[2].parse::<u32>()?
        {
            thursday.push(date[2].yellow().to_string());
        } else {
            thursday.push(date[2].to_string());
        }
    }

    for day in dates.friday.clone() {
        let date = day.split("-").collect::<Vec<&str>>();

        if day.contains("0000-00-00") {
            friday.push("--".to_string());
        } else if current_date.year == date[0].parse::<i32>()?
            && current_date.month == date[1].parse::<u32>()?
            && current_date.day == date[2].parse::<u32>()?
        {
            friday.push(date[2].yellow().to_string());
        } else {
            friday.push(date[2].to_string());
        }
    }

    for day in dates.saturday.clone() {
        let date = day.split("-").collect::<Vec<&str>>();

        if day.contains("0000-00-00") {
            saturday.push("--".to_string());
        } else if current_date.year == date[0].parse::<i32>()?
            && current_date.month == date[1].parse::<u32>()?
            && current_date.day == date[2].parse::<u32>()?
        {
            saturday.push(date[2].yellow().to_string());
        } else {
            saturday.push(date[2].to_string());
        }
    }

    for day in dates.sunday.clone() {
        let date = day.split("-").collect::<Vec<&str>>();

        if day.contains("0000-00-00") {
            sunday.push("--".to_string());
        } else if current_date.year == date[0].parse::<i32>()?
            && current_date.month == date[1].parse::<u32>()?
            && current_date.day == date[2].parse::<u32>()?
        {
            sunday.push(date[2].yellow().to_string());
        } else {
            sunday.push(date[2].to_string());
        }
    }

    Ok(MonthString {
        monday,
        tuesday,
        wednesday,
        thursday,
        friday,
        saturday,
        sunday,
    })
}

fn today() -> Result<Date> {
    let current_date_with_time = Local::now().to_string();
    let tmp = current_date_with_time
        .chars()
        .into_iter()
        .take(10)
        .collect::<String>();
    let current_date_stripped = tmp.split("-").collect::<Vec<&str>>();

    let mut date = Date::new();
    date.year = current_date_stripped[0].parse::<i32>()?;
    date.month = current_date_stripped[1].parse::<u32>()?;
    date.day = current_date_stripped[2].parse::<u32>()?;

    Ok(date)
}
