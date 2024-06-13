use std::io;

use crate::day_schedule::DaySchedule;

mod timespan;

mod day_schedule;


fn main() {
    
    println!("First Schedule:");
    let schedule1 = read_schedule();
    println!("{}",schedule1.to_string());
    println!("Second Schedule:");
    let schedule2 = read_schedule();
    println!("{}",schedule2.to_string());
    println!("Common Availabilities");
    let schedule3 = schedule1.intersect(&schedule2);
    println!("{}",schedule3.to_string());



}

fn read_int() -> i32 {
    
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        match input.trim().parse() {
            Ok(int) => return int,
            Err(_) => println!("Failed to read int, try again.")
        }
    }
    
}

fn read_schedule() -> DaySchedule {
    let mut schedule = DaySchedule::new();
    println!("Input number of available times to add: ");
    let num_times = read_int();
    for _i in 0..num_times {
        loop {
            match timespan::read_timespan() {
                Ok(span) => {
                   if schedule.add(span) {
                    break;
                   }
                   else {
                    println!("Time span overlaps existing time");
                   }
                },
                Err(e) => {
                    println!("{e}");
                }
            }
        };

    }
    schedule
}

