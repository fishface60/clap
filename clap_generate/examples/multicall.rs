use clap::{App, AppSettings};
use clap_generate::{generate, generators::Bash};
use std::io;

fn main() {
    let mut app = App::new(env!("CARGO_CRATE_NAME"))
        .setting(AppSettings::Multicall)
        .subcommand(
            App::new("test")
                .subcommand(App::new("config"))
                .subcommand(App::new("generate-completions")),
        )
        .subcommand(App::new("hello"));
    for applet in app.get_subcommands_mut() {
        let name = applet.get_name().to_owned();
        generate(Bash, applet, &name, &mut io::stdout());
    }
}
