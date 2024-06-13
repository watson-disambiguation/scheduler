use std::cmp::*;
use std::{io, error::Error, num::ParseIntError};
use std::fmt;

pub fn read_timespan() -> Result<TimeSpan,ParseTimeError> {
    println!("Start Time");
    let start = read_time()?;
    println!("End Time");
    let end = read_time()?;
    Ok(timespan_from_times(start, end)?)
}

fn read_time() -> Result<i32,ParseTimeError> {
    println!("Input a time in 24 hour format: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let time_mins = time_from_string(&input)?;
    Ok(time_mins)
}

fn timespan_from_times(start: i32, end: i32) -> Result<TimeSpan,ParseTimeError> {
    if end <= start {
        return Err(ParseTimeError::new("TimeSpan ends before it starts"));
    }

    let length = end - start;

    Ok(TimeSpan {
        start_time: start,
        length:  length,
    })
    
}

#[derive(Clone, Copy)]
pub struct TimeSpan {
    start_time: i32,
    length: i32,
}

impl TimeSpan {
    pub fn end_time(self) -> i32 {
        self.start_time + self.length
    }
    pub fn start_time(self) -> i32 {
        self.start_time
    }
    pub fn overlap(self, other: Self) -> Option<TimeSpan> {
        let end_time = min(self.end_time(), other.end_time());
        let start_time = max(self.start_time(),other.start_time());
        if start_time >= end_time {
            return None;
        }
        return Some(timespan_from_times(start_time, end_time).unwrap());
    } 
}

impl ToString for TimeSpan {
    fn to_string(&self) -> String {
        let start = time_to_string(self.start_time());
        let end = time_to_string(self.end_time());
        format!("{start} - {end}")
    }
}

pub fn time_to_string(time_minutes: i32) -> String{
    let minutes = time_minutes % 60;
    let hours = time_minutes / 60;
    format!("{hours}:{minutes:02}")
}




#[derive(Debug)]
pub struct ParseTimeError {
    details: String
}

impl ParseTimeError {
    fn new(msg: &str) -> Self {
        Self{details: msg.to_string()}
    }
}

impl fmt::Display for ParseTimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for ParseTimeError {
    fn description(&self) -> &str {
        &self.details
    }
}


impl From<ParseIntError> for ParseTimeError {
    fn from(err: ParseIntError) -> Self {
        Self::new(err.to_string().as_str())
    }
}

fn time_from_string(time_string: &str) -> Result<i32,ParseTimeError> {
    let mut split = time_string.split(':');
    let hours_string = match split.next() {
        Some(string) => string,
        None => return Err(ParseTimeError::new("String Improperly Formatted")),
    };
    let mins_string = match split.next() {
        Some(string) => string,
        None => return Err(ParseTimeError::new("String Improperly Formatted")),
    };

    let hours: i32 = hours_string.trim().parse()?;
    if hours < 0 || hours >= 24 {
        return Err(ParseTimeError::new("Hours outside range 0-23"));
    }
    let mins: i32 = mins_string.trim().parse()?;
    if mins < 0 || mins >= 60 {
        return Err(ParseTimeError::new("Minutes outside range 0-59"));
    }

    Ok(hours * 60 + mins)
}
