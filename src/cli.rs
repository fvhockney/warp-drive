use clap::{App, AppSettings, Arg, SubCommand};

pub fn cli() -> App<'static, 'static> {
    App::new("warp-drive-bin")
        .bin_name("warp")
        .version("1.0.0")
        .arg(
            Arg::with_name("waypoint")
                .index(1)
                .required(true)
                .help("waypoint to warp to"),
        )
        .subcommand(
            SubCommand::with_name("add")
                .arg(Arg::with_name("name").index(1).required(true))
                .about("add a waypoint for the current path"),
        )
        .subcommand(
            SubCommand::with_name("to")
                .arg(Arg::with_name("name").index(1).required(true))
                .about("go to a waypoint"),
        )
        .subcommand(
            SubCommand::with_name("eject")
                .arg(Arg::with_name("name").index(1).required(true))
                .about("remove a waypoint"),
        )
        .subcommand(
            SubCommand::with_name("update")
                .arg(Arg::with_name("name").index(1).required(true))
                .about("update a waypoint to use the current path"),
        )
        .subcommand(SubCommand::with_name("dump").about("dump the config to stdout"))
        .subcommand(SubCommand::with_name("edit").about("edit the config with $EDITOR"))
        .subcommand(SubCommand::with_name("init").about("print a shell alias for warp to stdout"))
        .subcommand(
            SubCommand::with_name("ls")
                .arg(
                    Arg::with_name("long")
                        .required(false)
                        .short("l")
                        .long("long"),
                )
                .alias("list")
                .about("list wapoints"),
        )
        .setting(AppSettings::SubcommandsNegateReqs)
}
