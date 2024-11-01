use clap::Subcommand;
use tokio::runtime::Runtime;
use crate::controllers::{console, controld, resolved_conf};

#[derive(Debug, Subcommand, Clone, Copy)]
pub enum Commands {
    #[command(about = "Check status of Controld Connection")]
    CheckStatus,
    #[command(about = "Up Controld Connection")]
    UpControlD,
    #[command(about = "Down Controld Connection")]
    DownControlD,
}

pub fn run(command: Commands) -> bool {
    match command {
        Commands::CheckStatus => {
            let status = Runtime::new()
                .unwrap()
                .block_on(controld::get_status_of_controld());
            println!("ControlD status: {}", status);
            status
        }
        Commands::UpControlD => {
            resolved_conf::up_controld_in_systemd_resolved_conf();
            console::restart_resolved_daemon();
            let mut status = Runtime::new()
                .unwrap()
                .block_on(controld::get_status_of_controld());
            let mut count: u8 = 0;
            while !status {
                status = Runtime::new()
                    .unwrap()
                    .block_on(controld::get_status_of_controld());
                count += 1;
                if count == 255 {
                    println!("Failed to check status of ControlD");
                    return status;
                }
            }
            println!("ControlD status: {}", status);
            status
        }
        Commands::DownControlD => {
            resolved_conf::down_controld_in_systemd_resolved_conf();
            console::restart_resolved_daemon();
            let mut status = Runtime::new()
                .unwrap()
                .block_on(controld::get_status_of_controld());
            let mut count: u8 = 0;
            while status {
                status = Runtime::new()
                    .unwrap()
                    .block_on(controld::get_status_of_controld());
                count += 1;
                if count == 255 {
                    println!("Failed to check status of ControlD");
                    return status;
                }
            }
            println!("ControlD status: {}", status);
            status
        }
    }
}