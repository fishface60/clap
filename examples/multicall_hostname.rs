use std::env::consts::EXE_SUFFIX;

use clap::{App, AppSettings};

fn match_subcmd_name(name: &str, s: &str) -> bool {
    match name.strip_suffix(EXE_SUFFIX) {
        Some(stem) => stem == s,
        None => name == s,
    }
}

fn main() {
    let app = App::new(env!("CARGO_CRATE_NAME"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand_value_name("APPLET")
        .subcommand_help_heading("APPLETS")
        .subcommand(App::new(format!("hostname{}", EXE_SUFFIX)).about("show hostname part of FQDN"))
        .subcommand(
            App::new(format!("dnsdomainname{}", EXE_SUFFIX)).about("show domain name part of FQDN"),
        );

    let app = app.setting(AppSettings::Multicall);

    match app.get_matches().subcommand_name() {
        Some(name) if match_subcmd_name(name, "hostname") => println!("www"),
        Some(name) if match_subcmd_name(name, "dnsdomainname") => println!("example.com"),
        _ => unreachable!("parser should ensure only valid subcommand names are used"),
    }
}
