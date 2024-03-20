use std::path::PathBuf;

pub fn join_path(base_path: &str, file: &str) -> String {
    let mut path = PathBuf::from(base_path);
    path.push(file);
    path.to_str().unwrap().to_string().replace("\\", "/")
}