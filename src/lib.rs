mod utils;

use wasm_bindgen::prelude::*;

use invalid_year_err::CalendarError;

use crate::hebrew::HebrewDate;
use crate::julian_gregorian::JulianCalendar;
use crate::julian_gregorian::GregorianCalendar;
use crate::cal::Calendar;

pub mod hebrew;
pub mod invalid_year_err;
pub mod julian_gregorian;
pub mod cal;

pub const OVERFLOW_ERROR_STRING: &str = "The entered date was too early or late to handle"; 
pub const INVALID_DATE_ERROR_STRING: &str = "The entered date was not valid";


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn log_to_console(contents: &str) {
    web_sys::console::log_1(&contents.into());
}


/*
Generic date format which can be returned to javascript code via web
assembly. Does not use error or result types for web assembly compatibility

is_not_overflow: false when input date was too early or too late to be
converted. otherwise true. this is used because result and error types
do not work as web assembly returns

is_valid: true when input date was a valid date. otherwise false

era: indicates the era. false represents first era (e.g. BCE). true
represents second era (e.g. CE). For calendars without era, this
defaults to false

year: year

month_name: index corresponding to the name of the month. In some calendars,
this may not always correspond to the month's number (e.g. in Hebrew leap 
years)

day: day in month
 */
#[wasm_bindgen]
pub struct GenericDate {
    is_valid: bool,
    is_not_overflow: bool, 
    era: bool,
    year: u32,
    month_name: u8,
    day: u8,
}

#[wasm_bindgen]
impl GenericDate {
    
    pub fn get_is_not_overflow (&self) -> bool {
        self.is_not_overflow
    }

    pub fn get_is_valid (&self) -> bool {
        self.is_valid
    }

    pub fn get_era (&self) -> bool {
        self.era
    }

    pub fn get_year (&self) -> u32 {
        self.year
    }

    pub fn get_month_name (&self) -> u8 {
        self.month_name
    }

    pub fn get_day (&self) -> u8 {
        self.day
    }
}

#[wasm_bindgen]
pub fn js_api(from: &str, to: &str,  day: u32, month: &str, year: i32, era: &str) -> GenericDate {
    log_to_console("");
    log_to_console("");
    utils::set_panic_hook();
    //log_to_console("log test");
    let from = String::from(from);
    let to = String::from(to);
    let month = String::from(month);
    let era = String::from(era);

    match from.as_str() {
        "Hebrew" => {
            let cal = HebrewDate::new(day, month, year, era);
            return js_api_helper(cal, to);
        },
        "Julian" => {
            let cal = JulianCalendar::new(day, month, year, era);
            return js_api_helper(cal, to);
        },
        "Gregorian" => {
            let cal = GregorianCalendar::new(day, month, year, era);
            return js_api_helper(cal, to);
        },
        _ => {
            panic!()
        }
    }
}

fn js_api_helper<T1: Calendar + std::fmt::Display>(from_cal: Result<T1, CalendarError>, to: String) -> GenericDate{
    let date;
    match from_cal {
        Err(CalendarError::InvalidInput) => return GenericDate { is_valid: false, is_not_overflow: true, era: false, year: 0, month_name: 0, day: 0 },
        Err(CalendarError::Overflow) => return GenericDate { is_valid: true, is_not_overflow: false, era: false, year: 0, month_name: 0, day: 0 },
        Ok(d) => date = d,
    };

    match to.as_str() {
        "Hebrew" => {
            let result = convert::<T1, HebrewDate>(date);
            return js_api_helper_helper(result);
        }
        "Julian" => {
            let result = convert::<T1, JulianCalendar>(date);
            return js_api_helper_helper(result);
        }
        "Gregorian" => {
            let result = convert::<T1, GregorianCalendar>(date);
            return js_api_helper_helper(result);
        }
        _ => {
            panic!()
        }
    };
}

fn js_api_helper_helper<T1: Into<GenericDate> + Calendar + std::fmt::Display>(input: Result<T1, CalendarError>) -> GenericDate {
    match input {
        Ok(date) => return date.into(),
        Err(CalendarError::InvalidInput) => GenericDate { is_valid: false, is_not_overflow: true, era: false, year: 0, month_name: 0, day: 0 },
        Err(CalendarError::Overflow) => GenericDate { is_valid: true, is_not_overflow: false, era: false, year: 0, month_name: 0, day: 0 }
    }
}

fn convert<T1: Calendar + std::fmt::Display, T2: Calendar + std::fmt::Display>(from_cal: T1) -> Result<T2, CalendarError> {
    //let julian_day = ;
    let julian_day;
    match from_cal.to_julian_day() {
        Ok(day) => julian_day = day,
        Err(e) => return Err(e)
    }
    
    log_to_console(format!("julian_day: {}", julian_day).as_str());
    return T2::from_julian_day(julian_day);
}



/*fn js_api_helper<T1: Calendar + std::fmt::Display, T2: Calendar + std::fmt::Display>(day: u128, month: String, year: i128, era: String) -> String{

    let from_cal = T1::new(day, month, year, era);
    let julian_day;
    match from_cal {
        Err(e) => return e.to_string(),
        Ok(date) => julian_day = date.to_julian_day()
    }

    let to_cal;
    match julian_day {
        Err(e) => return e.to_string(),
        Ok(day) => to_cal = T2::from_julian_day(day)
    }

    match to_cal {
        Err(e) => return e.to_string(),
        Ok(date) => return date.to_string()
    }
}*/

