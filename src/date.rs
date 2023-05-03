use chrono::NaiveDate;

#[derive(Debug, Clone, Copy)]
pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: i32,
}

impl Date {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            day: 0,
            month: 0,
            year: 0,
        }
    }

    #[allow(unused)]
    pub fn get_date(self) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(self.year, self.month, self.day)
    }
}

impl Default for Date {
    fn default() -> Self {
        Self {
            day: 1,
            month: 1,
            year: 2023,
        }
    }
}
