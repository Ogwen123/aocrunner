mod utils;

use crate::utils::logger::{fatal, info, success, warning};
use crate::utils::config::{Config, load_config, write_config};
use std::env;
use std::env::set_current_dir;
use std::process::{Stdio, Command};
use std::str::FromStr;
use std::time::{SystemTime};
use reqwest;
use reqwest::header::COOKIE;
use crate::utils::setup::setup;

type ParseResult = Result<i32, <i32 as FromStr>::Err>;

struct Ctx {
    cookies: String,
    year: String,
    day: String
}

#[derive(PartialEq)]
enum ResultType {
    Success,
    Failure,
    Cooldown,
    AlreadySubmitted
}

fn result_type_to_message(res_type: ResultType, part: &str) -> String {
    if res_type == ResultType::Success {
        format!("\u{001b}[102mPart {} SUCCESS\u{001b}[0m", part)
    } else if res_type == ResultType::Failure {
        return format!("\u{001b}[41mPart {} FAILURE\u{001b}[0m", part)
    } else if res_type == ResultType::AlreadySubmitted {
        return format!("\u{001b}[44mPart {} ALREADY SUBMITTED\u{001b}[0m", part)
    } else if res_type == ResultType::Cooldown {
        return format!("\u{001b}[43mPart {} ON COOLDOWN\u{001b}[0m", part)
    } else {
        return "".to_string()
    }
}

fn submit(answer: &str, part: &str, ctx: &Ctx) -> ResultType {

    let params = [("level", part), ("answer", answer)];

    let client = reqwest::blocking::Client::new();
    let res_result = client.post(format!("https://adventofcode.com/{}/day/{}/answer", ctx.year, ctx.day))
        .header(COOKIE, &ctx.cookies)
        .form(&params)
        .send();

    let res = match res_result {
        Ok(res) => res.text().unwrap(),
        Err(_) => {
            warning!("Failed to submit part {} answer.", part);
            return ResultType::Failure;
        }
    };
    println!("{}", res);

    if res.contains("not the right answer") {
        ResultType::Failure
    } else if res.contains("left to wait") {
        ResultType::Cooldown
    } else if res.contains("you already complete") {
        ResultType::AlreadySubmitted
    } else {
        ResultType::Success
    }
}

fn run(ctx: Ctx) {
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

    let raw_output = Command::new("python")
        .arg(format!(".\\{}\\{}\\main.py", ctx.year, ctx.day))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to run solution.");

    let time_taken = match now.elapsed() {
        Ok(elapsed) => elapsed.as_millis(),
        Err(_) => {
            warning!("Timing failed.");
            0
        }
    };
    println!("{:?}", raw_output);
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

    let part1_result = submit(output[0], "1", &ctx);
    let part2_result;
    if output[1] != "N/A" {
        part2_result = submit(output[1], "2", &ctx)
    } else {
        part2_result = ResultType::Failure;
    }

    let part1_string = result_type_to_message(part1_result, "1");
    let part2_string = if output[1] == "N/A" {"N/A".to_string()} else { result_type_to_message(part2_result, "2") };
    println!("-------------------- Results ---------------");
    println!("{}", part1_string);
    println!("{}", part2_string);
    println!("--------------------------------------------");
}

fn pre_run(args: Vec<String>) {
    info!("Loading config.");
    let config: Config = match load_config() {
        Ok(res) => res,
        Err(e) => {
            fatal!("{}", e);
            return;
        }
    };

    // validate the args
    if args.len() != 3 { // the command itself is part of the args array
        fatal!("You should provide the year and the day as inputs. 'aocrunner <day> <year> <optional: rust|python>");
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

    let ctx = Ctx {
        cookies: config.cookies,
        year: year.to_string(),
        day: day.to_string()
    };

    run(ctx);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        if args[1] == "setup" {
            setup();
            return
        } else {
            fatal!("Invalid args.");
        }
    } else if args.len() == 3 {
        if args[1] == "cookie" {
            let mut old_config = match load_config() {
                Ok(res) => res,
                Err(e) => {
                    fatal!("{}", e);
                    return;
                }
            };
            
            old_config.cookies = args[2].clone();
            
            match write_config(old_config) {
                Ok(_) => success!("Updated cookie"),
                Err(e) => {
                    fatal!("{}", e);
                    return;
                }
            }
        } else {
            pre_run(args)
        }
    }


}
