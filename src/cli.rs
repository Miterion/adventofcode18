use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    return App::new("Advent of Code cli")
        .version("1.3")
        .author("Heiko Carrasco <heiko.carrasco@yahoo.com>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("two")
                .short("t")
                .help("Compute the value of the second part"),
        ).subcommand(
            SubCommand::with_name("1")
                .about("First day")
                .arg(Arg::with_name("INPUT").required(true))
        ).subcommand(
            SubCommand::with_name("2")
                .about("Second day")
                .arg(Arg::with_name("INPUT").required(true))
        ).subcommand(
            SubCommand::with_name("3")
                .about("Third day")
                .arg(Arg::with_name("INPUT").required(true))
        );
}
