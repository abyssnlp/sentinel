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
                "{} version: {:?}",
                style("Sentinel").with(Color::Cyan),
                env!("CARGO_PKG_VERSION"),
            )
        }

        _ => unreachable!(),
    }
}
