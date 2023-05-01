mod cli;
mod io;
mod utils;

use crossterm::style::{style, Color, Stylize};
use dirs::home_dir;
use once_cell::sync::OnceCell;

fn main() {
    static HOME_DIR: OnceCell<String> = OnceCell::new();

    if let Some(path) = home_dir() {
        let home_dir_str = path.to_string_lossy().to_string();
        HOME_DIR.set(home_dir_str).unwrap();
    } else {
        panic!("Could not determine the HOME_DIR")
    }

    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("version", _)) => {
            println!(
                "{} version: {}",
                style("Sentinel").with(Color::Cyan),
                style(env!("CARGO_PKG_VERSION").to_string()).with(Color::Green),
            )
        }

        Some(("config", _)) => {
            println!(
                "Current ${}: {}",
                style("HOME").with(Color::DarkCyan),
                style(format!("{}", *HOME_DIR.get().unwrap())).with(Color::Green)
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
                    let params = io::Params {
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
