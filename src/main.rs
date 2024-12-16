mod utils;

use crate::utils::logger::{fatal};
use std::env;
use std::str::FromStr;

type ParseResult = Result<i32, <i32 as FromStr>::Err>;

fn run(year: i32, day: i32) {
    println!("doing stuff")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // validate the args
    if args.len() != 3 { // the command itself is part of the args array
        fatal!("You should provide the year and the day as inputs.");
        return;
    }

    // make sure the 2 provided args are valid e.g. year - 20** and day is between 1 and 25 inclusive
    let year_result: ParseResult = args[1].parse();

    let year = match year_result {
        Ok(year) => year,
        Err(_) => {
            fatal!("Could not convert the year provided to an int.");
            return;
        }
    };

    let day_result: ParseResult = args[2].parse();

    let day = match day_result {
        Ok(day) => day,
        Err(_) => {
            fatal!("Could not convert the day provided to an int.");
            return;
        }
    };

    if year < 2000 || year > 3000 {
        fatal!("Invalid year.");
        return;
    }

    if day < 1 || day > 25 {
        fatal!("Invalid day.");
        return;
    }

    run(year, day);
}
