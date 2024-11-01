use std::io::stdin;
use crate::controllers::command;
use crate::controllers::command::Commands;

pub fn run() {
    loop {
        println!("Please, write a command...");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "up" => {
                command::run(Commands::UpControlD);
            }
            "down" => {
                command::run(Commands::DownControlD);
            }
            "check" => {
                command::run(Commands::CheckStatus);
            }
            "exit" => break,
            _ => {
                println!("Not found command!")
            }
        }
    }
}
