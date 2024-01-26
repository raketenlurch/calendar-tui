/// Helper-functions used in main.rs
use std::{cmp, fmt, str::FromStr};

use anyhow::{anyhow, Ok, Result};
use chrono::Local;
use colored::Colorize;

use crate::{date::Date, month::MonthString};

pub fn get_input<T: std::str::FromStr>() -> T
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buffer = String::new();
    let stdin = std::io::stdin();

    stdin.read_line(&mut buffer).expect("BROKEN STDIN");

    let input = buffer.trim();

    input.parse().expect("Peng")
}

pub fn print_month(mut dates: MonthString) -> Result<()> {
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

pub fn crop_dates(dates: MonthString, current_date: Date) -> Result<MonthString> {
    let monday = loop_through_day_vec(dates.monday.clone(), current_date)?;
    let tuesday = loop_through_day_vec(dates.tuesday.clone(), current_date)?;
    let wednesday = loop_through_day_vec(dates.wednesday.clone(), current_date)?;
    let thursday = loop_through_day_vec(dates.thursday.clone(), current_date)?;
    let friday = loop_through_day_vec(dates.friday.clone(), current_date)?;
    let saturday = loop_through_day_vec(dates.saturday.clone(), current_date)?;
    let sunday = loop_through_day_vec(dates.sunday.clone(), current_date)?;

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

fn loop_through_day_vec(
    day_vec: Vec<String>,
    current_date: Date,
) -> Result<Vec<String>, anyhow::Error> {
    let mut weekday: Vec<String> = Vec::new();

    for day in day_vec {
        let date = day.split("-").collect::<Vec<&str>>();

        if day.contains("0000-00-00") {
            weekday.push("--".to_string());
        } else if current_date.year == date[0].parse::<i32>()?
            && current_date.month == date[1].parse::<u32>()?
            && current_date.day == date[2].parse::<u32>()?
        {
            weekday.push(date[2].yellow().to_string());
        } else {
            weekday.push(date[2].to_string());
        }
    }

    Ok(weekday)
}

pub fn today() -> Result<Date> {
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
