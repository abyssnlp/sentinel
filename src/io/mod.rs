pub fn io_test() -> String { String::from("Hello") }

#[derive(Debug)]
pub struct Params {
    pub path: String,
    pub pyexec: String,
    pub name: String,
}
