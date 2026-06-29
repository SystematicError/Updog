mod position;
mod uci;

use crate::uci::Uci;
use std::io::{BufRead, stdin};
use std::process::exit;

fn main() {
    for line in stdin().lock().lines() {
        if let Some(command) = Uci::parse(&line.unwrap()) {
            match command {
                Uci::Uci => {
                    println!("id name Updog");
                    println!("id author SystematicError");
                    println!("option name Threads type spin default 1 min 1 max 1");
                    println!("option name Hash type spin default 0 min 0 max 0");
                    println!("uciok");
                }

                Uci::IsReady => println!("readyok"),

                Uci::SetOption(_name, _value) => todo!(),

                Uci::Position(_position) => todo!(),

                Uci::Go => todo!(),

                Uci::Quit => exit(0),
            }
        }
    }
}
