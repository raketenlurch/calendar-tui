use crate::date::Date;
use crate::month::{MonthNaiveDate, MonthString};

use std::cmp;

mod date;
mod month;

fn main() {
    let mut date = Date::new();
    let mut month = MonthNaiveDate::new();

    date.day = 1;

    println!("month (as number): ");
    date.month = get_u32();

    println!("year (as number):");
    date.month = get_u32();

    let mut sorted_month_string = month
        .clone()
        .build_month_from_date_naive_date(&mut month, date);

    dbg!(&month);
    sorted_month_string.equalize_vector_length(date);
    dbg!(&month);
}

fn get_u32() -> u32 {
    let mut buffer = String::new();
    let stdin = std::io::stdin();

    stdin.read_line(&mut buffer).expect("BROKEN STDIN");

    let input = buffer.trim();

    input.parse().expect("Peng")
}

fn build_string_from_vecs(mut dates: MonthString) -> Result<String, &'static str> {
    if !dates.check_if_month_are_all_equal() {
        return Err("All fields need to have the same length");
    }

    let mut output = String::new();
    let month_as_string = dates.get_fields_as_array();

    output.push_str(" monday | tuesday | wednesday | thursday | friday | saturday | sunday ");
    output.push_str("\n");
    output.push_str("------------|-------------|---------------|--------------|------------|--------------|------------");
    output.push_str("\n");

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
        output.push_str(" ");
        output.push_str(&dates.monday[i]);
        output.push_str(" | ");
        output.push_str(&dates.tuesday[i]);
        output.push_str(" | ");
        output.push_str(&dates.wednesday[i]);
        output.push_str(" | ");
        output.push_str(&dates.thursday[i]);
        output.push_str(" | ");
        output.push_str(&dates.friday[i]);
        output.push_str(" | ");
        output.push_str(&dates.saturday[i]);
        output.push_str(" | ");
        output.push_str(&dates.sunday[i]);
        output.push_str("\n");

        i += 1;
    }

    Ok(output)
}
