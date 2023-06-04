#![allow(unused)]

mod files;
mod run;

use std::cmp::Ordering::*;
use std::env;

use run::run;

fn main() {
        let args: Vec<String> = env::args().collect();
        match args.len().cmp(&3) {
            Less => {
                println!("Too few arguments (entered {} must be 2)", args.len() - 1);
                return
            },
            Greater => {
                println!("Too many arguments (entered {} must be 2)", args.len() - 1);
                return
            },
            Equal => (),
        }

        let command = args[1].clone();
        let file = args[2].clone();

        match command.as_str() {
            "run" => run(&file).unwrap(),
            "compile" => println!("Compiling"),
            _ => println!("Unknown command")
        };
}
