use std::{env::consts::EXE_SUFFIX, process::exit};

use clap::{App, AppSettings, Arg, ArgMatches};

fn applet_commands(suffix: &str) -> [App<'static>; 2] {
    [
        App::new(format!("true{}", suffix)).about("does nothing successfully"),
        App::new(format!("false{}", suffix)).about("does nothing unsuccessfully"),
    ]
}

fn match_subcmd_name<'a>(
    subcommand: &Option<(&'a str, &'a ArgMatches)>,
    s: &str,
) -> Option<&'a ArgMatches> {
    let (name, cmd) = subcommand.as_ref()?;
    match name.strip_suffix(EXE_SUFFIX) {
        Some(stem) if stem == s => Some(cmd),
        None if *name == s => Some(cmd),
        _ => None,
    }
}

fn main() {
    let app = App::new(env!("CARGO_CRATE_NAME"))
        .setting(AppSettings::Multicall)
        .subcommand(
            App::new(format!("busybox{}", EXE_SUFFIX))
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand_value_name("APPLET")
                .subcommand_help_heading("APPLETS")
                .arg(
                    Arg::new("install")
                        .long("install")
                        .help("Install hardlinks for all subcommands in path")
                        .exclusive(true)
                        .takes_value(true)
                        .default_missing_value("/usr/local/bin")
                        .use_delimiter(false),
                )
                .subcommands(applet_commands("")),
        )
        .subcommands(applet_commands(std::env::consts::EXE_SUFFIX));

    let matches = app.get_matches();
    let mut subcommand = matches.subcommand();
    if let Some(cmd) = match_subcmd_name(&subcommand, "busybox") {
        if cmd.occurrences_of("install") > 0 {
            unimplemented!("Make hardlinks to the executable here");
        }
        subcommand = cmd.subcommand();
    }
    if match_subcmd_name(&subcommand, "false").is_some() {
        exit(1)
    } else if match_subcmd_name(&subcommand, "true").is_some() {
        exit(0)
    } else {
        unreachable!("parser should ensure only valid subcommand names are used")
    }
}
