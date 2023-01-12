use std::env;
use std::path::PathBuf;

use folder_size::analysis::*;

use crate::real_proxies::RealFileOperations;
use crate::tui::view;

mod real_proxies;
mod tui;

fn main() {
    let args: Vec<String> = env::args().collect();
    let buf = args.get(1).map(PathBuf::from).unwrap_or(env::current_dir().expect("error getting `current_dir`"));
    println!("working on {}...", buf.to_str().unwrap());
    let result = run(buf, &RealFileOperations);
    view(result);
}
