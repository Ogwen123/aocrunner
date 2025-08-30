mod utils;
mod run;

use crate::utils::logger::{fatal, info, success, warning};
use crate::utils::config::{Config, load_config, write_config};
use std::env;
use std::str::FromStr;
use std::time::{SystemTime};
use reqwest;
use reqwest::header::COOKIE;
use crate::run::python::run_python;
use crate::run::rust::run_rust;
use crate::utils::setup::setup;

type ParseResult = Result<i32, <i32 as FromStr>::Err>;

struct Ctx {
    cookies: String,
    year: String,
    day: String
}

pub struct RunResult {
    output: Vec<String>,
    time_taken: u128,
    build_time: Option<u128>
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

fn submit(answer: String, part: &str, ctx: &Ctx) -> ResultType {

    let params = [("level", part), ("answer", answer.as_str())];

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

fn run(args: Vec<String>, lang_provided: bool) {
    info!("Loading config.");
    let config: Config = match load_config() {
        Ok(res) => res,
        Err(e) => {
            fatal!("{}", e);
            return;
        }
    };
    println!("{}", config);

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
        cookies: config.cookies.clone(),
        year: year.to_string(),
        day: day.to_string()
    };

    let mut output_result: Result<RunResult, String>;

    const PYTHON_MATCHES: [&str; 3] = ["python", "py", "p"];
    const RUST_MATCHES: [&str; 2] = ["rust", "r"];

    if lang_provided == false {
        if config.default == "python" {
            output_result = run_python(config, &ctx);
        } else if config.default == "rust" {
            output_result = run_rust(config, &ctx);
        } else {
            warning!("Could not read default language, defaulting to python.");
            output_result = run_python(config, &ctx);

        }
    } else {
        if PYTHON_MATCHES.contains(&args[3].to_lowercase().as_str()) {
            output_result = run_python(config, &ctx);
        } else if RUST_MATCHES.contains(&args[3].to_lowercase().as_str()) {
            output_result = run_rust(config, &ctx);
        } else {
            warning!("Could not read specified language, defaulting to python.");
            output_result = run_python(config, &ctx);

        }
    }

    let run_result = match output_result {
        Ok(res) => res,
        Err(err) => {
            fatal!("{}", err);
            return;
        }
    };

    let time_taken = run_result.time_taken;
    let output = run_result.output;

    println!("\n-------------------- Solution ---------------");

    match run_result.build_time {
        Some(time) => {info!("Build Time: {}ms", time)}
        None => {}
    }

    info!("Time Taken: {}ms", time_taken);
    info!("Part 1: {}", &output[0]);
    info!("Part 2: {}", &output[1]);
    println!("---------------------------------------------\n");

    info!("Submitting to advent of code.\n");

    let part1_result = submit(output[0].clone(), "1", &ctx);
    let part2_result;
    if output[1] != "N/A" {
        part2_result = submit(output[1].clone(), "2", &ctx)
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
            run(args, false)
        }
    } else if args.len() == 4 {
        run(args, true);
    } else {
        fatal!("You provided too many inputs.Command syntax: 'aocrunner <day> <year> <optional: rust|python>")
    }
}
