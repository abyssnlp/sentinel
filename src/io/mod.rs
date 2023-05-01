/// HashMap: Name -> Service Info (serialized to disk)
use std::path::{Path, PathBuf};

const SERVICES_FILE: &str = ".sentinel/state";

#[derive(Debug)]
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

pub fn get_state_location(home_dir: String) -> PathBuf {
    Path::new(home_dir.as_str()).join(Path::new(SERVICES_FILE))
}

pub fn save_service(home_dir: String, path: String, pyexec: String, name: String) -> Params {
    let params = Params {
        path: String::from(path),
        pyexec: String::from(pyexec),
        name: String::from(name),
    };
    println!("{}", home_dir);
    println!("Location of state: {:?}", get_state_location(home_dir));
    params
}

pub fn load_services(home_dir: String) {}
