use configparser::ini::{Ini, WriteOptions};
use std::path::Path;
use crate::controllers::console;
const COMSS_URL: &str = "76.76.2.22#comss.dns.controld.com";

fn read_systemd_resolved_conf() -> Ini {
    let mut ini = Ini::new_cs();
    ini.load(Path::new("/etc/systemd/resolved.conf")).unwrap();
    ini
}
fn write_systemd_resolved_conf(ini: Ini) {
    ini.pretty_write(
        Path::new("/tmp/dotcd-resolved.conf"),
        &WriteOptions::new_with_params(false, 2, 1),
    )
    .unwrap();
    console::set_chmod(644, "/tmp/dotcd-resolved.conf");
    console::move_resolved_daemon();
}

fn remove_comss_settings_from_conf(ini: &mut Ini) {
    if ini.get("Resolve", "DNS").is_some() {
        ini.remove_key("Resolve", "DNS").unwrap();
    }
    if ini.get("Resolve", "DNSOverTLS").is_some() {
        ini.remove_key("Resolve", "DNSOverTLS").unwrap();
    }
}

fn add_comss_settings_to_conf(ini: &mut Ini) {
    ini.setstr("Resolve", "DNS", Some(COMSS_URL));
    ini.setstr("Resolve", "DNSOverTLS", Some("yes"));
}

pub fn up_controld_in_systemd_resolved_conf() {
    let mut ini = read_systemd_resolved_conf();
    add_comss_settings_to_conf(&mut ini);
    write_systemd_resolved_conf(ini);
}

pub fn down_controld_in_systemd_resolved_conf() {
    let mut ini = read_systemd_resolved_conf();
    remove_comss_settings_from_conf(&mut ini);
    write_systemd_resolved_conf(ini);
}
