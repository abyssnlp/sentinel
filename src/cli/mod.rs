use clap::{Arg, ArgAction, Command};

pub fn cli() -> Command {
    Command::new("sentinel")
        .about("Systemd runner utility")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .subcommand(Command::new("version").about("Get current sentinel version"))
        .subcommand(Command::new("config").about("Get current Sentinel configuration"))
        .subcommand(
            Command::new("run")
                .about("Run a systemd service")
                .subcommand_required(true)
                .subcommand(
                    Command::new("py")
                        .about("Runs a python program as a service")
                        .arg(
                            Arg::new("path")
                                .long("path")
                                .short('p')
                                .required(true)
                                .help("Path to the python program")
                                .action(ArgAction::Set),
                        )
                        .arg(
                            Arg::new("pyexec")
                                .long("pyexec")
                                .required(false)
                                .default_value("python")
                                .help("Path to the Python executable to use")
                                .action(ArgAction::Set),
                        )
                        .arg(
                            Arg::new("name")
                                .long("name")
                                .short('n')
                                .required(true)
                                .help(
                                    "Name of the service, this will be used to reference it later",
                                )
                                .action(ArgAction::Set),
                        ),
                ),
        )
}
