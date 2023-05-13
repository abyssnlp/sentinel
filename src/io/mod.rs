/// HashMap: Name -> Service Info (serialized to disk)
mod compress_serde;

use crate::service;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

const SERVICES_FILE: &str = "state";
const SERVICES_DIR: &str = "services";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Params {
    pub path: String,
    pub pyexec: String,
    pub name: String,
    pub unit_file_path: String,
    pub systemd_file_path: String,
}

/*
    Send path, pyexec and name -> Create IO
    Construct Params
    Read hashmap from disk and deserialize
    Add to the hashmap and serialize to disk
*/

pub fn get_state_location(home_dir: &str) -> PathBuf {
    Path::new(home_dir).join(Path::new(SERVICES_FILE))
}

pub fn get_services_dir(home_dir: &str) -> Result<PathBuf, Error> {
    let path = Path::new(home_dir).join(Path::new(SERVICES_DIR));
    fs::create_dir_all(&path)?;
    Ok(path.clone())
}

pub fn load_services(home_dir: &str) -> Result<HashMap<String, Params>, Error> {
    let state_file = get_state_location(home_dir);
    let map = compress_serde::decompress_from_file(state_file)?;
    Ok(map)
}

pub fn save_service<'a>(
    home_dir: &'a str,
    path: &'a str,
    pyexec: &'a str,
    name: &'a str,
) -> Result<Params, Error> {
    let (unit_file_path, systemd_file_path) = service::create_service(
        name,
        format!("{} {}", pyexec, path).as_str(),
        get_services_dir(home_dir)?,
    )?;
    let params = Params {
        path: String::from(path),
        pyexec: String::from(pyexec),
        name: String::from(name),
        systemd_file_path: String::from(systemd_file_path),
        unit_file_path: String::from(unit_file_path),
    };
    println!("{}", home_dir);
    println!("Location of state: {:?}", get_state_location(home_dir));
    let mut recovered_map = load_services(home_dir)?;
    println!("{:?}", recovered_map);
    recovered_map.insert(params.name.clone(), params.clone());
    compress_serde::compress_to_file(get_state_location(home_dir), &recovered_map)?;
    let mut recovered_map2 = compress_serde::decompress_from_file(get_state_location(home_dir))?;
    println!("{:?}", recovered_map2);

    Ok(params.clone())
}
