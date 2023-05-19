use crate::date::Date;
use crate::month::{MonthNaiveDate, MonthString};

use anyhow::{anyhow, Ok, Result};
use std::{cmp, fmt};

mod date;
mod month;

fn main() {
    let mut date = Date::new();
    let month = MonthString::new();
    let mut month_as_naive_date = MonthNaiveDate::new();
    let dummy_date = "0000-00-00".to_string();

    date.day = 1;

    println!("month (as number):");
    date.month = get_u32();

    println!("year (as number):");
    date.year = get_u32().try_into().unwrap();

    let sorted_month_string = month
        .clone()
        .build_month_from_date_string(&mut month_as_naive_date, date);

    let sorted_month_string = sorted_month_string
        .clone()
        .push_dummy_dates_to_vectors(date, dummy_date);
    //let output = print_month(sorted_month_string);
    let cropped_string = crop_dates(sorted_month_string).unwrap();
    print_month(cropped_string).unwrap();
}

fn get_u32() -> u32 {
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

    for mut i in 0..length {
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

        i += 1;
    }

    Ok(())
}

fn crop_dates(dates: MonthString) -> Result<MonthString> {
    let mut monday: Vec<String> = Vec::new();
    let mut tuesday: Vec<String> = Vec::new();
    let mut wednesday: Vec<String> = Vec::new();
    let mut thursday: Vec<String> = Vec::new();
    let mut friday: Vec<String> = Vec::new();
    let mut saturday: Vec<String> = Vec::new();
    let mut sunday: Vec<String> = Vec::new();

    for day in dates.monday.clone() {
        if day.contains("0000-00-00") {
            monday.push("--".to_string());
        } else {
            let date = day.split("-").collect::<Vec<&str>>();
            monday.push(date[2].to_string());
        }
    }

    for day in dates.tuesday.clone() {
        if day.contains("0000-00-00") {
            tuesday.push("--".to_string());
        } else {
            let date = day.split("-").collect::<Vec<&str>>();
            tuesday.push(date[2].to_string());
        }
    }

    for day in dates.wednesday.clone() {
        if day.contains("0000-00-00") {
            wednesday.push("--".to_string());
        } else {
            let date = day.split("-").collect::<Vec<&str>>();
            wednesday.push(date[2].to_string());
        }
    }

    for day in dates.thursday.clone() {
        if day.contains("0000-00-00") {
            thursday.push("--".to_string());
        } else {
            let date = day.split("-").collect::<Vec<&str>>();
            thursday.push(date[2].to_string());
        }
    }

    for day in dates.friday.clone() {
        if day.contains("0000-00-00") {
            friday.push("--".to_string());
        } else {
            let date = day.split("-").collect::<Vec<&str>>();
            friday.push(date[2].to_string());
        }
    }

    for day in dates.saturday.clone() {
        if day.contains("0000-00-00") {
            saturday.push("--".to_string());
        } else {
            let date = day.split("-").collect::<Vec<&str>>();
            saturday.push(date[2].to_string());
        }
    }

    for day in dates.sunday.clone() {
        if day.contains("0000-00-00") {
            sunday.push("--".to_string());
        } else {
            let date = day.split("-").collect::<Vec<&str>>();
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
