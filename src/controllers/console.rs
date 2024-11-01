use std::process::Command;


fn check_notify() -> bool {
    Command::new("which")
        .arg("notify-send")
        .output()
        .expect("failed to execute process").status.success()
}

pub fn send_notify(title: &str, text: &str) {
    if check_notify() {
    Command::new("notify-send")
        .arg("-a")
        .arg("dotcd notify")
        .arg(title)
        .arg(text)
        .spawn()
        .expect("failed to execute process");
    }
}

pub fn check_pkexec() -> bool {
    Command::new("which")
        .arg("pkexec")
        .output()
        .expect("").status.success()
}

pub fn set_chmod(chmod: u16, file_path: &str) {
    let _ = Command::new("chmod")
        .arg(chmod.to_string())
        .arg(file_path)
        .spawn();
}

fn pkexec_restart_resolved_daemon() {
    let _ = Command::new("pkexec")
        .arg("systemctl")
        .arg("restart")
        .arg("systemd-resolved.service")
        .status()
        .unwrap()
        .success();
}

fn pkexec_move_resolved_daemon() {
    let _ = Command::new("pkexec")
        .arg("mv")
        .arg("/tmp/dotcd-resolved.conf")
        .arg("/etc/systemd/resolved.conf")
        .status()
        .unwrap()
        .success();
}

fn sudo_restart_resolved_daemon() {
    let _ = Command::new("sudo")
        .arg("systemctl")
        .arg("restart")
        .arg("systemd-resolved.service")
        .status()
        .unwrap()
        .success();
}

fn sudo_move_resolved_daemon() {
    let _ = Command::new("sudo")
        .arg("mv")
        .arg("/tmp/dotcd-resolved.conf")
        .arg("/etc/systemd/resolved.conf")
        .status()
        .unwrap()
        .success();
}

pub fn restart_resolved_daemon() {
    if check_pkexec() {
        pkexec_restart_resolved_daemon();
    } else {
        sudo_restart_resolved_daemon();
    }
}

pub fn move_resolved_daemon() {
    if check_pkexec() {
        pkexec_move_resolved_daemon();
    } else {
        sudo_move_resolved_daemon();
    }
}