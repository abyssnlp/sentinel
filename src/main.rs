mod cli;
mod io;
mod utils;

use crossterm::style::{style, Color, Stylize};

#[derive(Debug)]
struct Params {
    path: String,
    pyexec: String,
    name: String,
}

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
                    let path = sub_matches
                        .get_one::<String>("path")
                        .map(|s| s.as_str())
                        .expect("Path to the python program is required!");
                    let pyexec = sub_matches
                        .get_one::<String>("pyexec")
                        .map(|s| s.as_str())
                        .unwrap_or("python");
                    let name = sub_matches
                        .get_one::<String>("name")
                        .map(|s| s.as_str())
                        .unwrap();

                    if utils::validate_py(path) {
                        println!("Received path: {:?}", path)
                    } else {
                        println!(
                            "{} : Couldn't validate path to Python program",
                            style("Error").with(Color::Red)
                        )
                    }
                    let params = Params {
                        path: String::from(path),
                        pyexec: String::from(pyexec),
                        name: String::from(name),
                    };
                    println!("{:?}", params)
                }

                (name, _) => {
                    unreachable!("Unsupported command: {}", name)
                }
            }
        }

        _ => unreachable!(),
    }
}
