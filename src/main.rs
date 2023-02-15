use std::env;
use std::path::PathBuf;

use clap::*;

use crate::file_analysis::read_fs;
use crate::real_proxies::RealFileOperations;
use crate::tui::display_result;

mod file_analysis;
mod real_proxies;
mod tui;

#[derive(Parser)]
struct Args {
    root_directory: PathBuf
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let root_directory = args
        .get(1)
        .map(|arg| {
            println!("argument is: {}", arg);
            arg.replace("\"", "")
        })
        .map(PathBuf::from)
        .filter(|path| path.is_dir())
        .or_else(|| {
            eprintln!("supplied argument is not a directory, or no argument supplied. reverting to current directory");
            None
        })
        .unwrap_or(env::current_dir().expect("error getting `current_dir`"));
    // let root_directory = Args::parse().root_directory;
    println!("working on {}...", root_directory.to_str().unwrap());
    let result = read_fs(root_directory, &RealFileOperations);
    display_result(result);
}
