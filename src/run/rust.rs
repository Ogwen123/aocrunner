use std::env::set_current_dir;
use std::process::{Command, Stdio};
use std::time::Instant;
use crate::{Ctx, RunResult};
use crate::utils::config::Config;
use crate::utils::logger::{info};

// run command example: rustc ./2017/1/main.rs --out-dir ./2017/1 && .\2017\1\main.exe
pub fn run_rust(config: Config, ctx: &Ctx) -> Result<RunResult, String> {
    info!("Running python code.");

    let cwd_result = set_current_dir(config.rust_path);
    let _ = match cwd_result {
        Ok(_) => (),
        Err(_) => {
            return Err(String::from("Setting working directory failed. Make sure the path in the config file is valid."))
        }
    };

    let mut now = Instant::now();

    let build_output = Command::new("rustc")
        .arg(format!("./{}/{}/main.rs", ctx.year, ctx.day))
        .arg("--out-dir")
        .arg(format!("./{}/{}", ctx.year, ctx.day))
        .arg("--color=always")// keeps the colour when printing the command below
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| e.to_string())?;

    let build_time = now.elapsed().as_millis();

    let build_error_parse = String::from_utf8(build_output.stderr);

    let output_str = match build_error_parse {
        Ok(res) => res,
        Err(_) => {
            return Err(String::from("Could not parse the possible build error message."))
        }
    };
    if output_str.contains("error[") {
        println!("{}", output_str);
        return Err(String::from("Submission stopped due to above build failure."))
    }

    now = Instant::now();

    let raw_output = Command::new(format!(".\\{}\\{}\\main.exe", ctx.year, ctx.day))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to run solution.");

    let time_taken = now.elapsed().as_millis();

    let output_format_result = String::from_utf8(raw_output.stdout);

    let output_str = match output_format_result {
        Ok(res) => res,
        Err(_) => {
            return Err(String::from("Could not parse the solution output."))
        }
    };

    let mut output: Vec<String> = Vec::new();

    for part in output_str
        .trim().split("\n")
        .map(|x| x.replace("\r", ""))
        .collect::<Vec<String>>() {

        if part.len() != 0 {
            output.push(part)
        }
    }

    if output.len() != 1 && output.len() != 2 {
        return Err(String::from("Incorrect output format. Make sure your output is 1 or 2 number on separate lines."))
    }

    if output.len() == 1 {
        output.push(String::from("N/A"))
    }

    Ok(RunResult {
        output,
        time_taken,
        build_time: Some(build_time)
    })
}