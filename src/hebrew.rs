use crate::invalid_year_err::CalendarError;
use crate::cal::Calendar;
use std::convert::TryInto;
use crate::GenericDate;

pub struct HebrewDate {
    pub year: u32,
    pub month: u8,
    pub day: u8,
}

impl Calendar for HebrewDate {
    fn to_julian_day(&self) -> Result<i128, CalendarError> {
        return from_hebrew(self);
    }

    fn from_julian_day(julian_day: i128) -> Result<Self, CalendarError> where Self: Sized {
        return to_hebrew(julian_day);
    }
    
    fn new(day: u32, month: String, year: i32, _era: String) -> Result<Self, CalendarError>  where Self: Sized{
        let day_error: Result<u8, _> = day.try_into();
        let day;
        match day_error {
            Ok(n) => day = n,
            Err(_) => return Err(CalendarError::InvalidInput),
        }

        let year_error: Result<u32, _> = year.try_into();
        let year: u32;
        match year_error{
            Ok(n) => year = n,
            Err(_) => return Err(CalendarError::InvalidInput),
        }

        let is_leap_year = is_leap_year(year);
        let is_leap_year_int = match is_leap_year {
            true => 1,
            false => 0,
        };

        if !is_leap_year && (month == String::from("Adar I")) {
            return Err(CalendarError::InvalidInput);
        }

       
        let month_num = match month.as_str() {
            "Tishrei" => 1,
            "Chesvan" => 2,
            "Kislev" => 3,
            "Tevet" => 4,
            "Shevat" => 5,
            "Adar I" => 6,
            "Adar/Adar II" => 6 + is_leap_year_int,
            "Nisan" => 7 + is_leap_year_int,
            "Iyar" => 8 + is_leap_year_int,
            "Sivan" => 9 + is_leap_year_int,
            "Tammuz" => 10 + is_leap_year_int,
            "Av" => 11 + is_leap_year_int,
            "Elul" => 12 + is_leap_year_int,
            _ => 100,
        };

        return Ok(HebrewDate  {
            year: year,
            month: month_num,
            day: day,
        });

    }
}

impl std::fmt::Display for HebrewDate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
            "year: {}, month: {}, day: {}",
            year, month, day
        )
    }
}

const DAY_LENGTH_PARTS: i128 = 24 * 60 * 18;
const MOLAD_LENGTH_PARTS: i128 = 765433;
const NINETEEN_YEAR_CYCLE_MONTHS: i128 = 19 * 12 + 7;

//const HEBREW_MONTH_NAMES: [&[u8]; 13] = ["Tishrei".as_bytes(), "Chesvan".as_bytes(), "Kislev".as_bytes(), "Tevet".as_bytes(), "Shevat".as_bytes(), "Adar I".as_bytes(), "Adar/Adar II".as_bytes(), "Nisan".as_bytes(), "Iyar".as_bytes(), "Sivan".as_bytes(), "Tammuz".as_bytes(), "Av".as_bytes(), "Elul".as_bytes()];

//const TISRHEI_1_JULIAN_DAY: i128 = 347997;
//const TISRHEI_1_PARTS: i128 = TISRHEI_1_JULIAN_DAY * DAY_LENGTH_PARTS;
// the number of "julian parts" at molad tishrei 5758
const MOLAD_TISHREI_5758_PARTS: i128 = 2450722 * DAY_LENGTH_PARTS + 23889;
// the number of "julian parts" at molad tishrei 1
const MOLAD_TISHREI_1_PARTS: i128 =
    MOLAD_TISHREI_5758_PARTS - (5758 / 19) * NINETEEN_YEAR_CYCLE_MONTHS * MOLAD_LENGTH_PARTS;
//assumes deficiency in both chesvan and kislev
const COMMON_YEAR_MONTH_LENGTHS: [u8; 12] = [30, 29, 29, 29, 30, 29, 30, 29, 30, 29, 30, 29];
const LEAP_YEAR_MONTH_LENGTHS: [u8; 13] = [30, 29, 29, 29, 30, 30, 29, 30, 29, 30, 29, 30, 29];
//const LEAP_YEARS: [u8; 7] = [3, 6, 8, 11, 14, 17, 19];
const MOLAD_NUM_LENGTH: [u8; 19] = [
    12, 12, 13, 12, 12, 13, 12, 13, 12, 12, 13, 12, 12, 13, 12, 12, 13, 12, 13,
];
const MOLAD_NUM_LENGTH_SUM: [u8; 19] = [
    12,
    12 * 2,
    12 * 3 + 1,
    12 * 4 + 1,
    12 * 5 + 1,
    12 * 6 + 2,
    12 * 7 + 2,
    12 * 8 + 3,
    12 * 9 + 3,
    12 * 10 + 3,
    12 * 11 + 4,
    12 * 12 + 4,
    12 * 13 + 4,
    12 * 14 + 5,
    12 * 15 + 5,
    12 * 16 + 5,
    12 * 17 + 6,
    12 * 18 + 6,
    12 * 19 + 7,
];

fn is_leap_year(year: u32) -> bool {
    match year % 19 {
        3 => return true,
        6 => return true,
        8 => return true,
        11 => return true,
        17 => return true,
        0 => return true, // year 19 in cycle
        _ => return false,
    }
}

fn tishrei_1_julian_day(molad_tishrei_parts: i128, year: u32) -> (i128, i128) {
    let molad_tishrei_julian_day = molad_tishrei_parts / DAY_LENGTH_PARTS;
    let time_parts = molad_tishrei_parts % DAY_LENGTH_PARTS;
    let molad_tishrei_weekday = (molad_tishrei_julian_day + 2) % 7;
    //println!("molad_tishrei_weekday: {}", molad_tishrei_weekday);

    let mut postponement: i128 = 0;
    //postponement A
    if (molad_tishrei_weekday == 0) | (molad_tishrei_weekday == 3) | (molad_tishrei_weekday == 5) {
        postponement += 1;
    // postponement B
    } else if time_parts > (DAY_LENGTH_PARTS / 2) {
        // unclear what happens when the molad falls on sunday, wednesday or friday after noon
        postponement += 1;
        if (molad_tishrei_weekday == 6)
            | (molad_tishrei_weekday == 2)
            | (molad_tishrei_weekday == 4)
        {
            postponement += 1;
        }

    // postponement C
    } else if (time_parts > (3 * 60 * 18 + 204))
        & (molad_tishrei_weekday == 2)
        & !is_leap_year(year)
    {
        postponement += 2;
    // postponement D
    } else if (time_parts > (9 * 60 * 18 + 589))
        & (molad_tishrei_weekday == 1)
        & is_leap_year(year - 1)
    {
        postponement += 1;
    }

    return (postponement, postponement + molad_tishrei_julian_day);
}

pub(crate) fn to_hebrew(julian_day: i128) -> Result<HebrewDate, CalendarError> {
    let parts_since_molad_tishrei_1;
    {
        let temp = julian_day.checked_mul(DAY_LENGTH_PARTS);
        if temp.is_none() {
            return Err(CalendarError::Overflow);
        }
        parts_since_molad_tishrei_1 = temp.unwrap() - MOLAD_TISHREI_1_PARTS;
    }

    // zeroeth: get rid of previous 19-year cycles
    let cycles = parts_since_molad_tishrei_1 / ((19 * 12 + 7) * MOLAD_LENGTH_PARTS); // if this is 1, that means it is year 20. if it is 2, it is year 39
    let mut parts_left_since_molad =
        parts_since_molad_tishrei_1 % ((19 * 12 + 7) * MOLAD_LENGTH_PARTS);

    // first: figure out molad tisrhei that precedes this day or falls on the same day as. both when it occurs and what year it is
    let mut year = cycles * 19 + 1;
    let mut i = 0;
    while parts_left_since_molad >= (MOLAD_NUM_LENGTH[i] as i128) * MOLAD_LENGTH_PARTS {
        if i >= 19 {
            println!("SOMETHING WENT WRONG");
        }
        parts_left_since_molad =
            parts_left_since_molad - (MOLAD_NUM_LENGTH[i] as i128) * MOLAD_LENGTH_PARTS;
        year += 1;
        i += 1;
    }

    let new_year = match year.try_into() {
        Ok(num) => Some(num),
        Err(_e) => None,
    };
    let mut year;
    match new_year {
        Some(num) => year = num,
        None => return Err(CalendarError::Overflow),
    }

    let last_tishrei_1_julian_day: i128;
    {
        /*let last_molad_tishrei_julian_day = (julian_day * DAY_LENGTH_PARTS - parts_left_since_molad) / DAY_LENGTH_PARTS;
        println!("last_molad_tishrei_julian_day: {}", last_molad_tishrei_julian_day);
        let time_parts = (julian_day * DAY_LENGTH_PARTS - parts_left_since_molad) % DAY_LENGTH_PARTS;*/
        let last_molad_tishrei_parts = julian_day * DAY_LENGTH_PARTS - parts_left_since_molad;
        (_, last_tishrei_1_julian_day) = tishrei_1_julian_day(last_molad_tishrei_parts, year);
    }

    //let next_tishrei_1_julian_day: i128;
    let next_tishrei_1_postponement: i128;
    {
        let next_molad_tishrei_parts: i128;
        if is_leap_year(year) {
            next_molad_tishrei_parts =
                julian_day * DAY_LENGTH_PARTS - parts_left_since_molad + MOLAD_LENGTH_PARTS * 13;
        } else {
            next_molad_tishrei_parts =
                julian_day * DAY_LENGTH_PARTS - parts_left_since_molad + MOLAD_LENGTH_PARTS * 12;
        }
        (next_tishrei_1_postponement, _) =
            tishrei_1_julian_day(next_molad_tishrei_parts, year + 1);
    }

    //println!("last_tishrei_1_julian_day: {}", last_tishrei_1_julian_day);
    //println!("next_tishrei_1_julian_day: {}", next_tishrei_1_julian_day);

    //let day: i128;

    // if we are at the end of a year but after molad tishrei of the next
    let mut days_left;
    let mut month;
    if last_tishrei_1_julian_day > julian_day {
        year = year - 1;
        if is_leap_year(year) {
            return Ok(HebrewDate {
                year: year,
                month: 13,
                day: (29 - (last_tishrei_1_julian_day - julian_day - 1)) as u8,
            });
        } else {
            return Ok(HebrewDate {
                year: year,
                month: 12,
                day: (29 - (last_tishrei_1_julian_day - julian_day - 1)) as u8,
            });
        }
    } else {
        days_left = julian_day - last_tishrei_1_julian_day;
        //println!("days_left: {}", days_left);
        month = 0;
        if is_leap_year(year) {
            let postponements: [u8; 13];
            if next_tishrei_1_postponement == 1 {
                postponements = [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            } else if next_tishrei_1_postponement == 2 {
                postponements = [0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            } else {
                postponements = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            }
            while days_left >= (LEAP_YEAR_MONTH_LENGTHS[month] + postponements[month]) as i128 {
                days_left =
                    days_left - (LEAP_YEAR_MONTH_LENGTHS[month] + postponements[month]) as i128;
                month = month + 1;
            }
        } else {
            let postponements: [u8; 12];
            if next_tishrei_1_postponement == 1 {
                postponements = [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            } else if next_tishrei_1_postponement == 2 {
                postponements = [0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            } else {
                postponements = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            }

            while days_left >= (COMMON_YEAR_MONTH_LENGTHS[month] + postponements[month]) as i128 {
                days_left =
                    days_left - (COMMON_YEAR_MONTH_LENGTHS[month] + postponements[month]) as i128;
                month = month + 1;
            }
        }

        return Ok(HebrewDate {
            year: year,
            month: month as u8 + 1,
            day: days_left as u8 + 1,
        });
    }
}

fn from_hebrew(date: &HebrewDate) -> Result<i128, CalendarError> {
    if date.month > 13 {
        return Err(CalendarError::InvalidInput);
    }
    if !is_leap_year(date.year) & (date.month == 13) {
        return Err(CalendarError::InvalidInput);
    }
    if (date.year < 1) | (date.day < 1) | (date.month < 1) {
        return Err(CalendarError::InvalidInput);
    }
    
    // find out julian day of the first day of the year
    let tishrei_1_julian_day_num: i128;
    let molad_tishrei_parts;
    {
        let cycles: i128 = ((date.year - 1) / 19).into();
        //let years_after_cycle: i128 = ((date.year - 1) % 19).into();
        let years_after_cycle: i128 = ((date.year - 1) % 19).into();

        let months_from_years_after_cycle: i128 = match years_after_cycle {
            0 => 0,
            _ => MOLAD_NUM_LENGTH_SUM[years_after_cycle as usize - 1] as i128
        };

        println!("estimated year: {}", cycles * 19 + years_after_cycle);
        let molad_tishrei_molad_num: i128 = cycles * (19 * 12 + 7) + months_from_years_after_cycle;
        molad_tishrei_parts = molad_tishrei_molad_num * MOLAD_LENGTH_PARTS + MOLAD_TISHREI_1_PARTS;

        (_, tishrei_1_julian_day_num) = tishrei_1_julian_day(
            molad_tishrei_parts,
            date.year,
        );

        // check if the day is too larger for the month
        
    }

    // find out how many days this date is from the first day of the year
    let mut days_since_tishrei_1: i128 = 0;
    {
        days_since_tishrei_1 += date.day as i128 - 1;
        let mut month = date.month as usize;
        if is_leap_year(date.year) {
            let (next_tishrei_1_postponement, _) = tishrei_1_julian_day(molad_tishrei_parts + 13 * MOLAD_LENGTH_PARTS, date.year);
            let postponements: [u8; 13];
            if next_tishrei_1_postponement == 1 {
                postponements = [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            } else if next_tishrei_1_postponement == 2 {
                postponements = [0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            } else {
                postponements = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            }

            if date.day > (LEAP_YEAR_MONTH_LENGTHS[date.month as usize - 1]  + postponements[date.month as usize - 1]) {
                return Err(CalendarError::InvalidInput);
            }

            while month - 1 >= 1 {
                days_since_tishrei_1 += (LEAP_YEAR_MONTH_LENGTHS[month - 1 - 1] + postponements[month - 1 - 1]) as i128;
                month -= 1;
            }
        } else {
            let (next_tishrei_1_postponement, _) = tishrei_1_julian_day(molad_tishrei_parts + 12 * MOLAD_LENGTH_PARTS, date.year);
            let postponements: [u8; 12];
            if next_tishrei_1_postponement == 1 {
                postponements = [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            } else if next_tishrei_1_postponement == 2 {
                postponements = [0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            } else {
                postponements = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            }

            if date.day > (COMMON_YEAR_MONTH_LENGTHS[date.month as usize - 1]  + postponements[date.month as usize - 1]) {
                return Err(CalendarError::InvalidInput);
            }

            while month > 1 {  
                days_since_tishrei_1 += (COMMON_YEAR_MONTH_LENGTHS[month - 1] + postponements[month - 1]) as i128;
                month -= 1;
            }
        }

        /*if is_leap_year(date.year) {
            
        } else {
            while month > 1 {
                days_since_tishrei_1 += COMMON_YEAR_MONTH_LENGTHS[month - 1] as i128;
                month -= 1;
            }
        }*/
    }
    return Ok(tishrei_1_julian_day_num + days_since_tishrei_1);
}

impl Into<GenericDate> for HebrewDate{
    fn into(self) -> GenericDate {
        // names: Tishrei (1), Chesvan (2), ... Adar (6), ... Adar I (13), Adar II (14)
        let month_name;
        {
            if is_leap_year(self.year) {
                if self.month <= 5 {
                    month_name = self.month;
                } else if self.month == 6 {
                    month_name = 13;
                } else if self.month == 7{
                    month_name = 14
                } else {
                    month_name = self.month - 1;
                }
            } else {
                month_name = self.month;
            }
        }

        GenericDate {
           is_valid: true,
           is_not_overflow: true,
           era: false,
           year: self.year,
           month_name: month_name,
           day: self.day 
        }
    }
}