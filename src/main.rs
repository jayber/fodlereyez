use std::env;
use std::path::PathBuf;

use crate::file_analysis::run;
use crate::real_proxies::RealFileOperations;
use crate::tui::view;

mod color;
mod file_analysis;
mod real_proxies;
mod tui;

fn main() {
    let args: Vec<String> = env::args().collect();
    let root_directory = args
        .get(1)
        .map(PathBuf::from)
        .unwrap_or(env::current_dir().expect("error getting `current_dir`"));
    println!("working on {}...", root_directory.to_str().unwrap());
    let result = run(root_directory, &RealFileOperations);
    view(result);
}
