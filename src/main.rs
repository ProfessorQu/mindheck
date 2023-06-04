#![allow(unused)]

mod files;
mod run;
mod compile;

use std::cmp::Ordering::*;
use std::env;

use run::run;
use compile::compile;

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
        let filename = args[2].clone();

        match command.as_str() {
            "run" => run(&filename).unwrap(),
            "compile" => compile(&filename).unwrap(),
            _ => println!("Unknown command")
        };
}
