use std::io;
use std::io::Write;
use crate::utils::config::{write_config, Config};

pub fn setup() {
    let mut config: Config = Default::default();

    println!("To find you cookie navigate to any puzzle page -> Press F12 -> Network Tab -> F5 to reload -> click the first request -> scroll to request headers -> copy the value for Cookie.");
    print!("Cookie > ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut config.cookies).unwrap();
    println!();

    println!("Below you will enter the path to your python code. E.g. C:/Users/username/Documents/code/advent of code");
    print!("Python Path > ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut config.python_path).unwrap();
    println!();

    println!("Below you will enter the path to your rust code. E.g. C:/Users/username/Documents/code/advent of code rust");
    print!("Rust Path > ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut config.rust_path).unwrap();
    println!();

    println!("Below you will enter the default language to use if one is not specified. (rust|python)");
    print!("Default (rust|python) > ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut config.default).unwrap();

    if config.default != String::from("python") || config.default != String::from("rust") {
        println!("Provided default is neither rust nor python, defaulting to python.");
        config.default = String::from("python")
    }

    println!("Got config \n    \x1b[32mcookies\x1b[0m={}\n    \x1b[32mpython_path\x1b[0m={}\n    \x1b[32mrust_path\x1b[0m={}\n    \x1b[32mdefault\x1b[0m={}", config.cookies, config.python_path, config.rust_path, config.default);
    println!("If any of these are incorrect re-run the command and start again.");

    match write_config(config) {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e);
            return
        }
    }

}