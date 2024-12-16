mod utils;

use crate::utils::logger::{fatal, info};
use std::env;

fn run(year: usize, day: usize) {

}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        fatal!("You should provide the year and the day as inputs!")
    }

    for  arg in args{
        println!("{}", arg)
    }
}
