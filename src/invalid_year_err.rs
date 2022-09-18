use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CalendarError {
    Overflow,
    InvalidInput,
}

impl fmt::Display for CalendarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let details: String = match self {
            Self::Overflow => String::from(crate::OVERFLOW_ERROR_STRING),
            Self::InvalidInput => String::from(crate::INVALID_DATE_ERROR_STRING),
        };
        write!(f, "{}", details)
    }
}

impl Error for CalendarError {
    fn description(&self) -> &str {
        match self {
            Self::Overflow => return crate::OVERFLOW_ERROR_STRING,
            Self::InvalidInput => return crate::INVALID_DATE_ERROR_STRING,
        }
    }
}