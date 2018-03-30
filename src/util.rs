use std::path::PathBuf;
use std::env;

pub fn get_dir() -> PathBuf{
    let user_path = env::var("USERPROFILE").unwrap_or_else(|e|{
        panic!("Could not get home directory: {}", e);
    });

    let mut path_buf = PathBuf::from(user_path);
    path_buf.push("eventw");

    path_buf
}

pub fn get_file(name: String) -> PathBuf{
    let mut path_buf = get_dir();
    path_buf.push(name);

    path_buf.with_extension("txt")
}