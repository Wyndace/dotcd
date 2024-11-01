use appindicator3::{prelude::*, IndicatorStatus};
use appindicator3::{Indicator, IndicatorCategory};
use daemonize::Daemonize;
use gtk::prelude::*;
use gtk::{Menu, MenuItem, SeparatorMenuItem};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use crate::controllers::{command, console};
use crate::controllers::command::Commands;

fn tray() {
    gtk::init().expect("Failed to initialize GTK.");


    let menu = Menu::new();
    let status_item = Arc::new(MenuItem::with_label("ControlD Status: false"));
    status_item.set_sensitive(false);
    menu.add(&*status_item);
    status_item.show_all();

    let indicator = Arc::new(Mutex::new(Indicator::builder("dotcd")
        .category(IndicatorCategory::ApplicationStatus)
        .menu(&menu)
        .status(IndicatorStatus::Active)
        .label("dotcd | tray tool")
        .build()));

    let update_status = {
        let status_item = Arc::clone(&status_item);
        let indicator = Arc::clone(&indicator);

        move |command: Commands| {
            command::run(command);

            // Обновляем текст статуса
            let status = command::run(Commands::CheckStatus);
            let status_text = format!("ControlD Status: {}", &status);
            status_item.set_label(&status_text);
            console::send_notify("ControlD Status", status.to_string().as_str());

            // Обновляем иконку в зависимости от статуса
            let new_icon = if status {
                "network-vpn-symbolic"
            } else {
                "network-vpn-disconnected-symbolic"
            };

            if let Ok(ind) = indicator.lock() {
                ind.set_icon_full(new_icon, "icon");
            }
        }
    };

    let update_status = update_status.clone();
    update_status(Commands::CheckStatus);

    let delimiter_item = SeparatorMenuItem::new();
    menu.add(&delimiter_item);
    delimiter_item.show_all();

    let controld_check = MenuItem::with_label("Check ControlD Status");
    controld_check.connect_activate({
        let update_status = update_status.clone();
        move |_| {
            update_status(Commands::CheckStatus);
        }
    });
    menu.add(&controld_check);
    controld_check.show_all();

    menu.add(&controld_check);
    controld_check.show_all();

    let controld_up = MenuItem::with_label("ControlD Up");
    controld_up.connect_activate({
        let update_status = update_status.clone();
        move |_| {
            update_status(Commands::UpControlD);
        }
    });
    menu.add(&controld_up);
    controld_up.show_all();

    let controld_down = MenuItem::with_label("ControlD Down");
    controld_down.connect_activate({
        let update_status = update_status.clone();
        move |_| {
            update_status(Commands::DownControlD);
        }
    });
    menu.add(&controld_down);
    controld_down.show_all();

    let delimiter_item = SeparatorMenuItem::new();
    menu.add(&delimiter_item);
    delimiter_item.show_all();

    let exit = MenuItem::with_label("Turn off tray");
    exit.connect_activate(|_| {
        off();
    });
    menu.add(&exit);
    exit.show_all();

    gtk::main();
}


pub fn run() {
    let daemonize = Daemonize::new()
        .pid_file("/tmp/dotcd-daemon-tray")
        .working_directory("/tmp")
        .stdout(File::create("/tmp/dotcd-daemon-tray.out").unwrap())
        .stderr(File::create("/tmp/dotcd-daemon-tray.error").unwrap())
        .user("wyndace");

    match daemonize.start() {
        Ok(_) => {
                tray();
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn off() {
    if Path::new("/tmp/dotcd-daemon-tray").exists() {
        let mut pid_string = String::new();
        File::open("/tmp/dotcd-daemon-tray")
            .unwrap()
            .read_to_string(&mut pid_string)
            .unwrap();
        let pid = pid_string.trim().parse::<i32>().unwrap();
        Command::new("kill")
            .arg(format!("{}", pid))
            .status()
            .expect("failed to execute process");
        std::fs::remove_file("/tmp/dotcd-daemon-tray.out").unwrap()
    }
}
