use std::{io, string::ParseError, convert::Infallible, error::Error, num::ParseIntError};
use std::fmt;
fn main() {

    let event = loop {
        match read_event() {
            Ok(event) => break event,
            Err(e) => {
                println!("{e}");
            }
        }
    };


}

fn read_event() -> Result<Event,ParseTimeError> {
    let start = read_time()?;
    let end = read_time()?;
    Ok(event_from_times(start, end)?)
}

fn read_time() -> Result<i32,ParseTimeError> {
    println!("Input a time in 24 hour format");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let time_mins = time_from_string(&input)?;
    Ok(time_mins)
}

fn event_from_times(start: i32, end: i32) -> Result<Event,ParseTimeError> {
    if end <= start {
        return Err(ParseTimeError::new("Event ends before it starts"));
    }

    let length = end - start;

    Ok(Event {
        start_time: start,
        length:  length,
    })
    
}

struct Event {
    start_time: i32,
    length: i32,
}



#[derive(Debug)]
struct ParseTimeError {
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
        Self::new(err.description())
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
    let mins: i32 = mins_string.trim().parse()?;

    Ok(hours * 60 + mins)
}
