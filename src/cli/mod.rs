use clap::Command;

pub fn cli() -> Command {
    Command::new("sentinel")
        .about("Systemd runner utility")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .subcommand(Command::new("version").about("Get current sentinel version"))
}
