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
#[command(about, long_about = None)]
struct Args {
    /// Optional. A valid directory to start analysing from. Use [drive-letter]:\ to indicate drive root on Windows.
    /// If none is supplied, or supplied value is not valid, will analyse from the current directory.
    root_directory: Option<String>,
    /// How many results to show per page load
    #[arg(short, long, default_value_t = 25)]
    page_size: u8
}

fn main() {
    let (valid_root_directory, page_size) = get_arguments();
    println!("working on {}...", valid_root_directory.to_str().unwrap());
    let result = read_fs(valid_root_directory, &RealFileOperations);
    display_result(result, page_size);
}

fn get_arguments() -> (PathBuf, u8) {
    let args = Args::parse();
    let root_directory = args
        .root_directory
        .clone()
        .map(|arg| {
            // println!("argument is: {}", arg);
            arg.replace('"', "")
        })
        .map(PathBuf::from)
        .filter(|path| path.is_dir())
        .or_else(|| {
            eprintln!("supplied argument is not a directory, or no argument supplied. reverting to current directory");
            None
        })
        .unwrap_or_else(|| env::current_dir().expect("error getting `current_dir`"));
    (root_directory, args.page_size)
}
