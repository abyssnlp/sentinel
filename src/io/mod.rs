/// HashMap: Name -> Service Info (serialized to disk)
mod compress_serde;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

const SERVICES_FILE: &str = ".sentinel/state";
const SERVICES_DIR: &str = ".sentinel/services";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Params {
    pub path: String,
    pub pyexec: String,
    pub name: String,
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
    fs::create_dir_all(path)?;
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
    let params = Params {
        path: String::from(path),
        pyexec: String::from(pyexec),
        name: String::from(name),
    };
    println!("{}", home_dir);
    println!("Location of state: {:?}", get_state_location(home_dir));
    println!("Create the systemd service");
    println!("Enable and start the service");
    println!("Fetch hashmap from disk and deserialize into hashmap<T>");
    println!("Create a new entry for the hashmap and serialize to disk");
    // hashmap -> bytes -> zstd compress
    let mut recovered_map = compress_serde::decompress_from_file(get_state_location(home_dir))?;
    println!("{:?}", recovered_map);
    // let mut map = HashMap::<String, Params>::new();
    recovered_map.insert(params.name.clone(), params.clone());
    compress_serde::compress_to_file(get_state_location(home_dir), &recovered_map)?;
    let mut recovered_map2 = compress_serde::decompress_from_file(get_state_location(home_dir))?;
    println!("{:?}", recovered_map2);

    Ok(params.clone())
}
