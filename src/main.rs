mod cli;
mod io;
mod service;
mod utils;

use crossterm::style::{style, Color, Stylize};
use lazy_static::lazy_static;
use std::fs;
use std::io::{Error, ErrorKind};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

const HOME_DIR: &str = "/var/sentinel";

lazy_static! {
    static ref HOME_PATH: PathBuf = {
        let path = Path::new(HOME_DIR);
        if !path.exists() {
            fs::create_dir_all(path).expect("Failed to create home directory");
            fs::set_permissions(path, fs::Permissions::from_mode(0o777))
                .expect("Failed to set required permission on the home directory");
        }
        path.to_path_buf()
    };
}

fn get_or_create_dir() -> Result<&'static str, Error> {
    let path_str = HOME_PATH
        .to_str()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Failed to convert path to string"))?;
    Ok(path_str)
}

fn main() {
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
                style(format!("{}", get_or_create_dir().unwrap_or(""))).with(Color::Green)
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

                    println!(
                        "{:?}",
                        io::save_service(
                            get_or_create_dir().unwrap(), // TODO: Fix ownership
                            path,
                            pyexec,
                            name
                        )
                    )
                }

                (name, _) => {
                    unreachable!("Unsupported command: {}", name)
                }
            }
        }

        Some(("status", sub_matches)) => {
            let service = sub_matches
                .get_one::<String>("name")
                .map(|s| s.as_str())
                .unwrap_or("all");

            println!("{:?}", service);
        }

        _ => unreachable!(),
    }
}
