use crate::invalid_year_err::CalendarError;

pub trait Calendar {
    fn to_julian_day(&self) -> Result<i128, CalendarError>;

    fn from_julian_day(julian_day: i128) -> Result<Self, CalendarError> where Self: Sized;

    fn new(day: u32, month: String, year: i32, era: String) -> Result<Self, CalendarError>  where Self: Sized;
}