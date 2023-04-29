pub fn validate_py(path: &str) -> bool {
    if path.ends_with("py") {
        true
    } else {
        false
    }
}
