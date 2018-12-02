use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    return App::new("Advent of Code cli")
        .version("1.2")
        .author("Heiko Carrasco <heiko.carrasco@yahoo.com>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("1")
                .about("First day")
                .arg(Arg::with_name("INPUT").required(true))
                .arg(
                    Arg::with_name("two")
                        .short("t")
                        .help("Compute the value of the second part"),
                ),
        ).subcommand(
            SubCommand::with_name("2")
                .about("Second day")
                .arg(Arg::with_name("INPUT").required(true))
                .arg(
                    Arg::with_name("two")
                        .short("t")
                        .help("Compute the value of the second part"),
                ),
        );
}
