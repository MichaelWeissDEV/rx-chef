/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parse DateTime operation.
 * -----------------------------------------------------------------------------
 */

use chrono::{Datelike, NaiveDateTime, Timelike};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Parse DateTime operation
///
/// Parses a date/time string using chrono's strftime format specifiers
/// (e.g. "%d/%m/%Y %H:%M:%S") and displays detailed information about the
/// parsed date.  Note: unlike the original CyberChef which uses moment.js
/// format tokens, this implementation uses strftime format strings.
pub struct ParseDateTime;

impl Operation for ParseDateTime {
    fn name(&self) -> &'static str {
        "Parse DateTime"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Parses a DateTime string using strftime format specifiers and displays detailed date/time information including day of year, week number, quarter, and leap year status. Format uses strftime tokens (e.g. %d/%m/%Y %H:%M:%S)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Input format string",
                description: "strftime format string (e.g. %d/%m/%Y %H:%M:%S)",
                default_value: "%d/%m/%Y %H:%M:%S",
            },
            ArgSchema {
                name: "Input timezone",
                description: "Timezone name (currently UTC only)",
                default_value: "UTC",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let text = String::from_utf8_lossy(&input);
        let text = text.trim();

        let fmt = args
            .first()
            .and_then(|a| a.as_str())
            .unwrap_or("%d/%m/%Y %H:%M:%S");

        let dt = NaiveDateTime::parse_from_str(text, fmt).map_err(|e| {
            OperationError::InvalidInput(format!(
                "Invalid format. Could not parse '{}' with format '{}': {}",
                text, fmt, e
            ))
        })?;

        let date = dt.date();
        let time = dt.time();

        let day_of_year = date.ordinal();
        let week_number = date.iso_week().week();
        let quarter = (date.month() - 1) / 3 + 1;
        let is_leap = is_leap_year(date.year());
        let days_in_month = days_in_month(date.year(), date.month());
        let period = if time.hour() < 12 { "AM" } else { "PM" };

        let weekday_name = weekday_to_str(date.weekday());
        let month_name = month_to_str(date.month());
        let day_ordinal = ordinal_suffix(date.day());

        let output = format!(
            "Date: {} {} {} {}\nTime: {:02}:{:02}:{:02}\nPeriod: {}\nTimezone: UTC\nUTC offset: +0000\n\nDaylight Saving Time: false\nLeap year: {}\nDays in this month: {}\n\nDay of year: {}\nWeek number: {}\nQuarter: {}",
            weekday_name,
            day_ordinal,
            month_name,
            date.year(),
            time.hour(),
            time.minute(),
            time.second(),
            period,
            is_leap,
            days_in_month,
            day_of_year,
            week_number,
            quarter,
        );

        Ok(output.into_bytes())
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

fn ordinal_suffix(day: u32) -> String {
    let suffix = match day {
        1 | 21 | 31 => "st",
        2 | 22 => "nd",
        3 | 23 => "rd",
        _ => "th",
    };
    format!("{}{}", day, suffix)
}

fn weekday_to_str(wd: chrono::Weekday) -> &'static str {
    match wd {
        chrono::Weekday::Mon => "Monday",
        chrono::Weekday::Tue => "Tuesday",
        chrono::Weekday::Wed => "Wednesday",
        chrono::Weekday::Thu => "Thursday",
        chrono::Weekday::Fri => "Friday",
        chrono::Weekday::Sat => "Saturday",
        chrono::Weekday::Sun => "Sunday",
    }
}

fn month_to_str(month: u32) -> &'static str {
    match month {
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
        _ => "Unknown",
    }
}
