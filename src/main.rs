mod cli;
mod io;

use crossterm::style::{style, Color, Stylize};

fn main() {
    println!("Hello, world!");
    println!("{}", io::io_test());

    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("version", _)) => {
            println!(
                "{} version: {}",
                style("Sentinel").with(Color::Cyan),
                style(env!("CARGO_PKG_VERSION").to_string()).with(Color::Green),
            )
        }

        Some(("run", sub_matches)) => {
            let process_command = sub_matches.subcommand().unwrap_or(("help", sub_matches));
            match process_command {
                ("py", sub_matches) => {
                    let path = sub_matches.get_one::<String>("path").map(|s| s.as_str());
                    println!("Received path: {:?}", path.unwrap())
                }

                (name, _) => {
                    unreachable!("Unsupported command: {}", name)
                }
            }
        }

        _ => unreachable!(),
    }
}
