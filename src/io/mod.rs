use std::path::PathBuf;

/// HashMap: Name -> Service Info (serialized to disk)

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

pub fn save_service(path: String, pyexec: String, name: String) {
    let params = Params {
        path: String::from(path),
        pyexec: String::from(pyexec),
        name: String::from(name),
    };
}
