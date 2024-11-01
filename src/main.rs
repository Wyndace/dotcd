mod runners;
mod controllers;

use std::process::{ExitCode};
use runners::cli::{CliCommands, TrayCommands};
use controllers::command;
use clap::Parser;

fn main() -> ExitCode {
    let args = runners::cli::Cli::parse();
    match args.command {
        CliCommands::Tray { command } => {
            match command {
                TrayCommands::On => {
                    runners::tray::run();
                    println!("dotls run in tray successfully!");
                }
                TrayCommands::Off => {
                    runners::tray::off();
                }
            }
        }
        CliCommands::Cli { command } => {
            command::run(command);
        }
        CliCommands::Stdio => {
            runners::stdio::run();
        }
    }
    ExitCode::SUCCESS
}
