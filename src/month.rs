use chrono::{Datelike, NaiveDate};
use itertools::Itertools;

use crate::date::Date;
use crate::helper::calculate_leap_year;
use crate::month_string::MonthString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonthNaiveDate {
    pub monday: Vec<NaiveDate>,
    pub tuesday: Vec<NaiveDate>,
    pub wednesday: Vec<NaiveDate>,
    pub thursday: Vec<NaiveDate>,
    pub friday: Vec<NaiveDate>,
    pub saturday: Vec<NaiveDate>,
    pub sunday: Vec<NaiveDate>,
}

impl MonthNaiveDate {
    pub fn new() -> Self {
        Self {
            monday: Vec::new(),
            tuesday: Vec::new(),
            wednesday: Vec::new(),
            thursday: Vec::new(),
            friday: Vec::new(),
            saturday: Vec::new(),
            sunday: Vec::new(),
        }
    }

    #[allow(unused)]
    pub fn get_fields_as_array(&self) -> Vec<String> {
        let mut days = Vec::new();

        days.push("monday".to_string());
        days.push("tuesday".to_string());
        days.push("wednesday".to_string());
        days.push("thursday".to_string());
        days.push("friday".to_string());
        days.push("saturday".to_string());
        days.push("sunday".to_string());

        days
    }

    #[allow(unused)]
    pub fn check_if_months_are_all_equal(&mut self) -> bool {
        let vectors = vec![
            self.monday.clone(),
            self.tuesday.clone(),
            self.wednesday.clone(),
            self.thursday.clone(),
            self.friday.clone(),
            self.saturday.clone(),
            self.sunday.clone(),
        ];

        vectors.iter().map(|v| v.len()).all_equal()
    }

    #[allow(unused)]
    pub fn push_dummy_dates_to_vectors(mut self, mut dates: Date, dummy_date: NaiveDate) -> Self {
        let len = self.get_length_of_month(dates);

        // Beginning of the month
        dates.day = 1;
        let beginning_month = NaiveDate::from_ymd_opt(dates.year, dates.month, dates.day).unwrap();

        match beginning_month.weekday() {
            chrono::Weekday::Mon => (),
            chrono::Weekday::Tue => self.monday.insert(0, dummy_date),
            chrono::Weekday::Wed => {
                self.monday.insert(0, dummy_date);
                self.tuesday.insert(0, dummy_date);
            }
            chrono::Weekday::Thu => {
                self.monday.insert(0, dummy_date);
                self.tuesday.insert(0, dummy_date);
                self.wednesday.insert(0, dummy_date);
            }
            chrono::Weekday::Fri => {
                self.monday.insert(0, dummy_date);
                self.tuesday.insert(0, dummy_date);
                self.wednesday.insert(0, dummy_date);
                self.thursday.insert(0, dummy_date);
            }
            chrono::Weekday::Sat => {
                self.monday.insert(0, dummy_date);
                self.tuesday.insert(0, dummy_date);
                self.wednesday.insert(0, dummy_date);
                self.thursday.insert(0, dummy_date);
                self.friday.insert(0, dummy_date);
            }
            chrono::Weekday::Sun => {
                self.monday.insert(0, dummy_date);
                self.tuesday.insert(0, dummy_date);
                self.wednesday.insert(0, dummy_date);
                self.thursday.insert(0, dummy_date);
                self.friday.insert(0, dummy_date);
                self.saturday.insert(0, dummy_date);
            }
        }

        // End of the month
        dates.day = len;
        let end_month = NaiveDate::from_ymd_opt(dates.year, dates.month, dates.day).unwrap();

        match end_month.weekday() {
            chrono::Weekday::Mon => {
                self.tuesday.push(dummy_date);
                self.wednesday.push(dummy_date);
                self.thursday.push(dummy_date);
                self.friday.push(dummy_date);
                self.saturday.push(dummy_date);
                self.sunday.push(dummy_date);
            }
            chrono::Weekday::Tue => {
                self.wednesday.push(dummy_date);
                self.thursday.push(dummy_date);
                self.friday.push(dummy_date);
                self.saturday.push(dummy_date);
                self.sunday.push(dummy_date);
            }
            chrono::Weekday::Wed => {
                self.thursday.push(dummy_date);
                self.friday.push(dummy_date);
                self.saturday.push(dummy_date);
                self.sunday.push(dummy_date);
            }
            chrono::Weekday::Thu => {
                self.friday.push(dummy_date);
                self.saturday.push(dummy_date);
                self.sunday.push(dummy_date);
            }
            chrono::Weekday::Fri => {
                self.saturday.push(dummy_date);
                self.sunday.push(dummy_date);
            }
            chrono::Weekday::Sat => self.sunday.push(dummy_date),
            chrono::Weekday::Sun => (),
        }

        Self {
            monday: self.monday,
            tuesday: self.tuesday,
            wednesday: self.wednesday,
            thursday: self.thursday,
            friday: self.friday,
            saturday: self.saturday,
            sunday: self.sunday,
        }
    }

    pub fn get_length_of_month(&self, dates: Date) -> u32 {
        match dates.month {
            1 => 31,
            2 => match calculate_leap_year(dates.year) {
                true => 29,
                false => 28,
            },
            3 => 31,
            4 => 30,
            5 => 31,
            6 => 30,
            7 => 31,
            8 => 31,
            9 => 30,
            10 => 31,
            11 => 30,
            12 => 31,
            _ => panic!("invalid month"),
        }
    }

    #[allow(unused)]
    pub fn get_month_as_string(&self, date: Date) -> String {
        match date.month {
            1 => "january".to_string(),
            2 => "february".to_string(),
            3 => "march".to_string(),
            4 => "april".to_string(),
            5 => "may".to_string(),
            6 => "june".to_string(),
            7 => "july".to_string(),
            8 => "august".to_string(),
            9 => "september".to_string(),
            10 => "october".to_string(),
            11 => "november".to_string(),
            12 => "december".to_string(),
            _ => panic!("invalid month"),
        }
    }

    #[allow(unused)]
    pub fn build_month_from_date_naive_date(
        self,
        mut month: &mut MonthNaiveDate,
        mut dates: Date,
    ) -> MonthNaiveDate {
        let len = self.get_length_of_month(dates);

        for _i in 1..=len {
            match dates.get_date().unwrap().weekday() {
                chrono::Weekday::Mon => month.monday.push(dates.get_date().unwrap()),
                chrono::Weekday::Tue => month.tuesday.push(dates.get_date().unwrap()),
                chrono::Weekday::Wed => month.wednesday.push(dates.get_date().unwrap()),
                chrono::Weekday::Thu => month.thursday.push(dates.get_date().unwrap()),
                chrono::Weekday::Fri => month.friday.push(dates.get_date().unwrap()),
                chrono::Weekday::Sat => month.saturday.push(dates.get_date().unwrap()),
                chrono::Weekday::Sun => month.sunday.push(dates.get_date().unwrap()),
            }

            dates.day += 1;
        }

        month.to_owned()
    }
}

impl From<MonthNaiveDate> for MonthString {
    fn from(weekday: MonthNaiveDate) -> Self {
        let mut monday = Vec::new();
        let mut tuesday = Vec::new();
        let mut wednesday = Vec::new();
        let mut thursday = Vec::new();
        let mut friday = Vec::new();
        let mut saturday = Vec::new();
        let mut sunday = Vec::new();

        for i in weekday.monday {
            let monday_as_string = i.to_string();
            monday.push(monday_as_string);
        }

        for i in weekday.tuesday {
            let tuesday_as_string = i.to_string();
            tuesday.push(tuesday_as_string);
        }

        for i in weekday.wednesday {
            let wednesday_as_string = i.to_string();
            wednesday.push(wednesday_as_string);
        }

        for i in weekday.thursday {
            let thursday_as_string = i.to_string();
            thursday.push(thursday_as_string);
        }

        for i in weekday.friday {
            let friday_as_string = i.to_string();
            friday.push(friday_as_string);
        }

        for i in weekday.saturday {
            let saturday_as_string = i.to_string();
            saturday.push(saturday_as_string);
        }

        for i in weekday.sunday {
            let sunday_as_string = i.to_string();
            sunday.push(sunday_as_string);
        }

        Self {
            monday,
            tuesday,
            wednesday,
            thursday,
            friday,
            saturday,
            sunday,
        }
    }
}
