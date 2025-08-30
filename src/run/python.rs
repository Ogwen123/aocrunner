use std::env::set_current_dir;
use std::process::{Command, Stdio};
use std::time::SystemTime;
use crate::{Ctx, RunResult};
use crate::utils::config::Config;
use crate::utils::logger::{info, warning};

pub fn run_python(config: Config, ctx: &Ctx) -> Result<RunResult, String> {
    info!("Running python code.");

    let cwd_result = set_current_dir(config.python_path);
    let _ = match cwd_result {
        Ok(_) => (),
        Err(_) => {
            return Err(String::from("Setting working directory failed. Make sure the path in the config file is valid."))
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

    let output_format_result = String::from_utf8(raw_output.stdout);

    let output_str = match output_format_result {
        Ok(res) => res,
        Err(_) => {
            return Err(String::from("Could not parse the solution output."))
        }
    };

    let mut output: Vec<String> = Vec::new();

    for part in output_str
        .split("\n")
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
        build_time: None
    })
}