use crate::GenericDate;
use crate::invalid_year_err::CalendarError;
use crate::cal::Calendar;
use std::cmp;
use std::convert::TryInto;

#[derive(PartialEq, Clone)]
enum Era {
    AD,
    BC,
}

impl std::fmt::Display for Era {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let my_str: &str = match self {
            Era::AD => "AD",
            Era::BC => "BC",
        };
        write!(f, "{}", my_str)
    }
}

pub struct JulianCalendar {
    era: Era,
    year: u32,
    month: u8, // starts at month 1
    day: u8,   // day of month
}

impl JulianCalendar {
    fn valid(&self) -> Option<CalendarError> {
        if self.day <= 0 {
            return Some(CalendarError::InvalidInput);
        }
        if self.year <= 0 {
            return Some(CalendarError::InvalidInput);
        }
        if (self.month > 12) | (self.month <= 0) {
            return Some(CalendarError::InvalidInput);
        }

        if self.month == 2 {
            if self.is_leap_year() & (self.day > 29) {
                return Some(CalendarError::InvalidInput);
            }

            if !self.is_leap_year() & (self.day > 28) {
                return Some(CalendarError::InvalidInput);
            }
        } else {
            if (ROMAN_MONTH_LENGTHS[(self.month - 1) as usize] as u8) < self.day {
                return Some(CalendarError::InvalidInput);
            }
        }
        return None;
    }

    #[allow(dead_code)]
    fn is_leap_year(&self) -> bool {
        let year: u32;
        if self.era == Era::BC {
            year = self.year - 1;
        } else {
            year = self.year;
        }

        if year % 4 == 0 {
            return true;
        }
        return false;
    }
}



pub struct GregorianCalendar {
    era: Era,
    year: u32,
    month: u8, // starts at month 1
    day: u8,   // day of month
}

impl GregorianCalendar {

    #[allow(dead_code)]
    fn valid(&self) -> Option<CalendarError> {
        if self.day <= 0 {
            return Some(CalendarError::InvalidInput);
        }
        if self.year <= 0 {
            return Some(CalendarError::InvalidInput);
        }
        if (self.month > 12) | (self.month <= 0) {
            return Some(CalendarError::InvalidInput);
        }

        if self.month == 2 {
            if self.is_leap_year() & (self.day > 29) {
                return Some(CalendarError::InvalidInput);
            }

            if !self.is_leap_year() & (self.day > 28) {
                return Some(CalendarError::InvalidInput);
            }
        } else {
            if (ROMAN_MONTH_LENGTHS[(self.month - 1) as usize] as u8) < self.day {
                return Some(CalendarError::InvalidInput);
            }
        }
        return None;
    }

    

    #[allow(dead_code)]
    fn is_leap_year(&self) -> bool {
        let year: u32;
        if self.era == Era::BC {
            year = self.year - 1;
        } else {
            year = self.year;
        }

        if ((year % 4 == 0) & (year % 100 != 0)) | ((year % 4 == 0) & (year % 400 == 0)) {
            return true;
        }
        return false;
    }
}



impl std::fmt::Display for JulianCalendar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let era_string = self.era.to_string();
        let era = era_string.as_str();

        let year_string = self.year.to_string();
        let year = year_string.as_str();

        let month: &str = match self.month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "",
        };
        
        let day_string = self.day.to_string();
        let day = day_string.as_str();
        write!(
            f,
            "era: {}, year: {}, month: {}, day: {}",
            era, year, month, day
        )
    }
}

impl std::fmt::Display for GregorianCalendar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let temp = JulianCalendar {
            era: self.era.clone(),
            year: self.year,
            month: self.month,
            day: self.day
        };

        return temp.fmt(f);
    }
}

const ROMAN_MONTH_LENGTHS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

impl Calendar for GregorianCalendar {
    fn to_julian_day(&self) -> Result<i128, CalendarError> {
        return from_proleptic_gregorian(self);
    }

    fn from_julian_day(julian_day: i128) -> Result<Self, CalendarError> where Self: Sized {
        return to_proleptic_gregorian(julian_day);
    }

    fn new(day: u32, month: String, year: i32, era: String) -> Result<Self, CalendarError>  where Self: Sized {
        let day_error: Result<u8, _> = day.try_into();
        let day;
        match day_error {
            Ok(n) => day = n,
            Err(_) => return Err(CalendarError::InvalidInput),
        }

        let month_num = match month.as_str() {
            "January" => 1,
            "February" => 2,
            "March" => 3,
            "April" => 4,
            "May" => 5,
            "June" => 6,
            "July" => 7,
            "August" => 8,
            "September" => 9,
            "October" => 10,
            "November" => 11,
            "December" => 12,
            _ => panic!(),
        };

        let year_error: Result<u32, _> = year.try_into();
        let year: u32;
        match year_error{
            Ok(n) => year = n,
            Err(_) => return Err(CalendarError::InvalidInput),
        }
       
        let era: Era = match era.as_str() {
            "AD" => Era::AD,
            _ => Era::BC,
        };


        return Ok(GregorianCalendar {
            era: era,
            year: year,
            month: month_num,
            day: day,
        });
        
    }
}

impl Calendar for JulianCalendar {
    fn to_julian_day(&self) -> Result<i128, CalendarError> {
        return from_proleptic_julian(self);
    }

    fn from_julian_day(julian_day: i128) -> Result<Self, CalendarError> where Self: Sized {
        return to_proleptic_julian(julian_day);
    }

    fn new(day: u32, month: String, year: i32, era: String) -> Result<Self, CalendarError>  where Self: Sized {
        let greg = GregorianCalendar::new(day, month, year, era);
        match greg {
            Ok(cal) => return Ok(JulianCalendar {
                era: cal.era,
                year: cal.year,
                month: cal.month,
                day: cal.day,
            }),
            Err(e) => return Err(e)
        };
    }
}

#[allow(dead_code)]
pub fn to_proleptic_gregorian(
    julian_day: i128,
) -> Result<GregorianCalendar, CalendarError> {
    const FIRST_JANUARY_1_AD: i128 = 1721425;
    const MIN_DAY_YEAR_ADJUSTMENT: i128 = 4400; // MUST BE DIVISIBLE BY 400!!!
                                                // 1721425-4400*365.2425 = 114358
    const MIN_DAY: i128 = FIRST_JANUARY_1_AD - (MIN_DAY_YEAR_ADJUSTMENT / 400) * (365 * 400 + 97);
    if julian_day < MIN_DAY {
        return Err(CalendarError::Overflow);
    }

    let shifted_date = julian_day - MIN_DAY;
    //let shifted_date = julian_day

    let mut days_left = shifted_date;
    let year: i128;

    // determine year and number of days into the year
    {
        //records number of cycles passed
        let big_cycles: i128 = days_left / (365 * 400 + 97);
        days_left = days_left - big_cycles * (365 * 400 + 97);

        // if on 31 dec on the last day of a big cycle, it could think 4 medium cycles have passed even though only 3 have
        let medium_cycles: i128 = cmp::min(days_left / (365 * 100 + 24), 3);
        days_left = days_left - medium_cycles * (365 * 100 + 24);

        let small_cycles: i128 = cmp::min(days_left / (365 * 4 + 1), 24);
        days_left = days_left - small_cycles * (365 * 4 + 1);

        //if the date is december 31 in the 4th year in the cycle, the years_in_cycle could come out to year 4 beacuse its a leap year even though it should be 3
        let years_in_cycle: i128 = cmp::min(days_left / 365, 3);
        days_left = days_left - years_in_cycle * 365;

        year = big_cycles * 400 + medium_cycles * 100 + small_cycles * 4 + years_in_cycle + 1;
    }

    let mut days_left = days_left as i32;
    let mut month = 0;
    if days_left >= ROMAN_MONTH_LENGTHS[0] {
        days_left = days_left - ROMAN_MONTH_LENGTHS[0];
        month = 1;
        if ((year % 4 == 0) & (year % 100 != 0)) | ((year % 4 == 0) & (year % 400 == 0)) {
            if days_left >= ROMAN_MONTH_LENGTHS[1] + 1 {
                days_left = days_left - (ROMAN_MONTH_LENGTHS[1] + 1);
                month = 2;
            }
            while days_left >= ROMAN_MONTH_LENGTHS[month] {
                days_left = days_left - ROMAN_MONTH_LENGTHS[month];
                month = month + 1;
            }
        } else {
            if days_left >= ROMAN_MONTH_LENGTHS[1] {
                days_left = days_left - (ROMAN_MONTH_LENGTHS[1]);
                month = 2;
            }
            while days_left >= ROMAN_MONTH_LENGTHS[month] {
                days_left = days_left - ROMAN_MONTH_LENGTHS[month];
                month = month + 1;
            }
        }
    }

    //final processing
    month = month + 1;
    let mut final_year = year - MIN_DAY_YEAR_ADJUSTMENT;
    //let mut final_year = year;
    let era;
    if final_year <= 0 {
        era = Era::BC;
        final_year = (-1 * final_year) + 1;
    } else {
        era = Era::AD;
        final_year = final_year;
    }

    let day = days_left + 1;

    let cal_date = GregorianCalendar {
        era: era,
        year: final_year as u32,
        month: month as u8,
        day: day as u8,
    };

    return Ok(cal_date);
}

#[allow(dead_code)]
pub fn from_proleptic_gregorian(
    cal_date: &GregorianCalendar,
) -> Result<i128, CalendarError> {
    const FIRST_JANUARY_1_AD: i128 = 1721425;
    const MIN_DAY_YEAR_ADJUSTMENT: i128 = 4400; // MUST BE DIVISIBLE BY 400!!!
                                                // 1721425-4400*365.2425 = 114358
    const MIN_DAY: i128 = FIRST_JANUARY_1_AD - (MIN_DAY_YEAR_ADJUSTMENT / 400) * (365 * 400 + 97); // the minimumum julian day allowed
    if (cal_date.year > MIN_DAY_YEAR_ADJUSTMENT as u32) & (cal_date.era == Era::BC) {
        return Err(CalendarError::Overflow);
    }

    if cal_date.valid().is_some() {
        return Err(CalendarError::InvalidInput);
    }

    let year: i128;
    //let year = cal_date.year as i128;
    if cal_date.era == Era::BC {
        year = (-1 * (cal_date.year as i128 - 1)) + MIN_DAY_YEAR_ADJUSTMENT;
    } else {
        year = cal_date.year as i128 + MIN_DAY_YEAR_ADJUSTMENT;
    }

    //let year_days = (((year - 1) / 4) * (365 * 4 + 1)) + ((((year - 1) % 4))) * 365;
    let mut year_days: i128;
    {
        let mut years_left = year - 1;

        year_days = (years_left / 400) * (365 * 400 + 97);
        years_left = years_left % 400;

        year_days += (years_left / 100) * (365 * 100 + 24);
        years_left = years_left % 100;

        year_days += (years_left / 4) * (365 * 4 + 1);
        years_left = years_left % 4;

        year_days += years_left * 365;
    }

    let mut month_days = 0;
    let mut monthi: u8 = 1;
    while monthi < cal_date.month {
        month_days = month_days + ROMAN_MONTH_LENGTHS[(monthi - 1) as usize];
        monthi = monthi + 1;
    }

    return Ok((year_days as i128) + (month_days as i128) + (cal_date.day as i128 - 1) + MIN_DAY);
}

/*
 * Takes in Julian date and outputs the date on the proleptic Julian calendar (they are different)
 */
#[allow(dead_code)]
fn to_proleptic_julian(
    julian_day: i128,
) -> Result<JulianCalendar, CalendarError> {
    const FIRST_JANUARY_1_AD: i128 = 1721423;
    const MIN_DAY_YEAR_ADJUSTMENT: i128 = 4712; // MUST BE DIVISIBLE BY 4!!!
                                                // 1721423-4712*365.25
    const MIN_DAY: i128 = FIRST_JANUARY_1_AD - (MIN_DAY_YEAR_ADJUSTMENT / 4) * (365 * 4 + 1);
    if julian_day < MIN_DAY {
        return Err(CalendarError::Overflow);
    }

    let shifted_date = julian_day - MIN_DAY;
    //let shifted_date = julian_day

    let mut days_left = shifted_date;
    let year: i128;

    // determine year and number of days into the year
    {
        let cycles: i128 = days_left / (365 * 4 + 1);
        days_left = days_left - cycles * (365 * 4 + 1);

        //if the date is december 31 in the 4th year in the cycle, the years_in_cycle could come out to year 4 beacuse its a leap year even though it should be 3
        let years_in_cycle: i128 = cmp::min(days_left / 365, 3);
        days_left = days_left - years_in_cycle * 365;

        year = cycles * 4 + years_in_cycle + 1;
    }

    let mut days_left = days_left as i32;
    let mut month = 0;
    if days_left >= ROMAN_MONTH_LENGTHS[0] {
        days_left = days_left - ROMAN_MONTH_LENGTHS[0];
        month = 1;
        if ((year % 4 == 0) & (year % 100 != 0)) | ((year % 4 == 0) & (year % 400 == 0)) {
            if days_left >= ROMAN_MONTH_LENGTHS[1] + 1 {
                days_left = days_left - (ROMAN_MONTH_LENGTHS[1] + 1);
                month = 2;
            }
            while days_left >= ROMAN_MONTH_LENGTHS[month] {
                days_left = days_left - ROMAN_MONTH_LENGTHS[month];
                month = month + 1;
            }
        } else {
            if days_left >= ROMAN_MONTH_LENGTHS[1] {
                days_left = days_left - (ROMAN_MONTH_LENGTHS[1]);
                month = 2;
            }
            while days_left >= ROMAN_MONTH_LENGTHS[month] {
                days_left = days_left - ROMAN_MONTH_LENGTHS[month];
                month = month + 1;
            }
        }
    }

    //final processing
    month = month + 1;
    let mut final_year = year - MIN_DAY_YEAR_ADJUSTMENT;
    //let mut final_year = year;
    let era;
    if final_year <= 0 {
        era = Era::BC;
        final_year = (-1 * final_year) + 1;
    } else {
        era = Era::AD;
        final_year = final_year;
    }

    let day = days_left + 1;

    let cal_date = JulianCalendar {
        era: era,
        year: final_year as u32,
        month: month as u8,
        day: day as u8,
    };

    return Ok(cal_date);
}

#[allow(dead_code)]
fn from_proleptic_julian(
    cal_date: &JulianCalendar,
) -> Result<i128, CalendarError> {
    const FIRST_JANUARY_1_AD: i128 = 1721423;
    const MIN_DAY_YEAR_ADJUSTMENT: i128 = 4712; // MUST BE DIVISIBLE BY 4!!!
                                                // 1721423-4712*365.25
    const MIN_DAY: i128 = FIRST_JANUARY_1_AD - (MIN_DAY_YEAR_ADJUSTMENT / 4) * (365 * 4 + 1);
    if (cal_date.year < MIN_DAY_YEAR_ADJUSTMENT as u32) & (cal_date.era == Era::BC) {
        return Err(CalendarError::Overflow);
    }

    if cal_date.valid().is_some() {
        return Err(CalendarError::InvalidInput);
    }

    let year: i128;
    //let year = cal_date.year as i128;
    if cal_date.era == Era::BC {
        year = (-1 * (cal_date.year as i128 - 1)) + MIN_DAY_YEAR_ADJUSTMENT;
    } else {
        year = cal_date.year as i128 + MIN_DAY_YEAR_ADJUSTMENT;
    }
    let year_days = (((year - 1) / 4) * (365 * 4 + 1)) + ((year - 1) % 4) * 365;

    let mut month_days = 0;
    let mut monthi: u8 = 1;
    while monthi < cal_date.month {
        month_days = month_days + ROMAN_MONTH_LENGTHS[(monthi - 1) as usize];
        monthi = monthi + 1;
    }

    return Ok((year_days as i128) + (month_days as i128) + (cal_date.day as i128 - 1) + MIN_DAY);
}


impl Into<GenericDate> for JulianCalendar {
    fn into(self) -> GenericDate {
        GenericDate {
            is_not_overflow: true,
            is_valid: true,
            era: {match self.era {
                Era::AD => true,
                Era::BC => false
            }},
            year: self.year,
            month_name: self.month,
            day: self.day
        }
    }
}

impl Into<GenericDate> for GregorianCalendar {
    fn into(self) -> GenericDate {
        GenericDate {
            is_not_overflow: true,
            is_valid: true,
            era: {match self.era {
                Era::AD => true,
                Era::BC => false
            }},
            year: self.year,
            month_name: self.month,
            day: self.day
        }
    }
}