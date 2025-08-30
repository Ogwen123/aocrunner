use std::fs;
use std::env::home_dir;
use std::fmt::{Display, Formatter};
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub struct Config {
    pub cookies: String,
    pub python_path: String,
    pub rust_path: String,
    pub default: String
}

impl Default for Config {
    fn default() -> Self {
        Config {
            cookies: String::new(),
            python_path: String::new(),
            rust_path: String::new(),
            default: String::from("python")
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Got config \n    \x1b[32mcookies\x1b[0m={}\n    \x1b[32mpython_path\x1b[0m={}\n    \x1b[32mrust_path\x1b[0m={}\n    \x1b[32mdefault\x1b[0m={}", self.cookies, self.python_path, self.rust_path, self.default)
    }
}

pub fn config_directory() -> Result<PathBuf, String> {
    let mut home = match home_dir() {
        Some(res) => res,
        None => {
            return Err(String::from("Could not get home directory."));
        }
    };
    let os = std::env::consts::OS;

    if os == "linux" {
        home.push(".config");
        home.push("aocrunner");
    } else if os == "windows" {
        home.push("AppData");
        home.push("Local");
        home.push("aocrunner")
    } else {
        return Err(String::from("Unsupported OS"))
    }

    Ok(home)
}

pub fn write_config(config: Config) -> Result<(), String> {
    let write_string = format!("cookies={}\npython_path={}\nrust_path={}\ndefault={}", config.cookies, config.python_path, config.rust_path, config.default);

    let mut path = config_directory().map_err(|e| e)?;

    let _ = create_dir_all(path.clone()).map_err(|e| e.to_string());
    
    path.push("aocrunner");
    path.set_extension("txt");
    println!("{}", path.display());
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(|_| {
            return "Could not open config file in write mode (write, create, truncate).";
        })?;

    file.write_all(write_string.as_bytes()).map_err(|e| e.to_string())
}

pub fn load_config() -> Result<Config, String> {
    let mut path = config_directory().map_err(|e| e)?;

    path.push("aocrunner");
    path.set_extension("txt");

    let config = fs::read_to_string(path)
        .expect("Could not find config file. Run 'aocrunner setup' to generate one.");

    let mut default_config: Config = Default::default();

    for line in config.lines() {
        match line.split_once("=") {
            Some((k, v)) =>  {
                match k {
                    "cookies" => default_config.cookies = String::from(v),
                    "python_path" => default_config.python_path = String::from(v),
                    "rust_path" => default_config.rust_path = String::from(v),
                    "default" => default_config.default = String::from(v),
                    _ => {}
                }
            },
            None => {}
        }
    }

    Ok(default_config)
}