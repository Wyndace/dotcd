use crate::controllers::command::Commands;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "dotcd")]
#[command(about = "A Tool for manipulating the ControlD DNSOverTLS connection", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommands,
}

#[derive(Debug, Subcommand)]
pub enum CliCommands {
    #[command(arg_required_else_help = true)]
    #[command(about = "Run dotcd as tray icon")]
    Tray {
        #[command(subcommand)]
        command: TrayCommands,
    },
    #[command(arg_required_else_help = true)]
    #[command(about = "Use dotcd as cli icon")]
    Cli {
        #[command(subcommand)]
        command: Commands,
    },
    #[command(about = "Run dotcd as stdio-cli tool")]
    Stdio,
}

#[derive(Subcommand, Copy, Clone, Debug)]
pub enum TrayCommands {
    On,
    Off,
}
