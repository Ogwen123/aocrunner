mod utils;

use crate::utils::logger::{fatal, info, warning};
use std::env;
use std::env::set_current_dir;
use std::process::Command;
use std::str::FromStr;
use std::time::{SystemTime};

type ParseResult = Result<i32, <i32 as FromStr>::Err>;

struct Result {
    part1: boolean,
    part2: boolean
}

fn submit(part1: i32, part2: i32) -> Result {
    return Result {
        part1: true,
        part2: true
    }
}

fn run(year: i32, day: i32) {
    info!("Running python code.");

    let cwd_result = set_current_dir("Z:\\Code\\python\\advent of code");
    let _ = match cwd_result {
        Ok(_) => (),
        Err(_) => {
            fatal!("Setting current working directory failed.");
            return;
        }
    };

    let now = SystemTime::now();

    let raw_output = Command::new("python3")
        .arg(format!(".\\{}\\{}\\main.py", year, day))
        .output()
        .expect("Failed to run solution.");

    let time_taken = match now.elapsed() {
        Ok(elapsed) => elapsed.as_millis(),
        Err(_) => {
            warning!("Timing failed.");
            0
        }
    };

    let output_format_result = String::from_utf8(raw_output.stdout);

    let output_str = match output_format_result {
        Ok(res) => res,
        Err(_) => {
            fatal!("Could not parse the solution output.");
            return;
        }
    };

    let mut output: Vec<&str> = Vec::new();

    for part in output_str.split("\r\n").collect::<Vec<&str>>() {
        if part.len() != 0 {
            output.push(part)
        }
    }

    if output.len() != 1 && output.len() != 2 {
        fatal!("Incorrect output. Make sure your output is 1 or 2 numbers on separate lines.");
        return;
    }

    if output.len() == 1 {
        output.push("N/A")
    }

    println!("\n-------------------- Solution ---------------");

    info!("Time Taken: {}ms", time_taken);
    info!("Part 1: {}", output[0]);
    info!("Part 2: {}", output[1]);
    println!("---------------------------------------------\n");

    info!("Submitting to advent of code.\n");

    let results: Result = sumbit(1, 2);

    println!("-------------------- Results ---------------");

    println!("---------------------------------------------");
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
    info!("Validated year and day.");
    run(year, day);
}
