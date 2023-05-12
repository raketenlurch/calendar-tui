use chrono::NaiveDate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let new_date = Date::new();

        let compare_date = Date {
            day: 0,
            month: 0,
            year: 0,
        };

        assert_eq!(new_date, compare_date);
    }

    #[test]
    fn test_get_date() {
        let date = Date {
            day: 1,
            month: 1,
            year: 2023,
        };
        let date_as_naive_date = date.get_date().unwrap();

        let compare_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();

        assert_eq!(date_as_naive_date, compare_date);
    }

    #[test]
    fn test_default() {
        let new_date = Date::default();
        let compare_date = Date {
            day: 1,
            month: 1,
            year: 2023,
        };

        assert_eq!(new_date, compare_date);
    }
}
