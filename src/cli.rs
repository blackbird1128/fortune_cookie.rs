use crate::file_utils;
use crate::fortune;
use crate::pick;
use crate::Args;
use std::path::Path;
use std::process::exit;

pub const DEFAULT_FOLDERS: [&str; 1] = ["./fortunes/"];

fn check_fortunes_folders_exist(paths: &[&str]) {
    for path in paths {
        if !Path::new(path).exists() {
            println!("Error: folder {} does not exist (default folder)", path);
            exit(1);
        }
    }
}

pub fn handle_file_arg(args: &Args) {
    if args.file {
        // -f
        for file in file_utils::get_fortune_files(&DEFAULT_FOLDERS) {
            println!("{file}");
        }
        exit(0);
    }
}

pub fn handle_pattern_arg(args: &Args) {
    if args.ignore && !args.pattern.is_some() {
        // -i without -m
        println!("The -i option can only be used with -m");
        exit(1);
    }
}

pub fn handle_zero_file_arg(args: &Args) {
    check_fortunes_folders_exist(&DEFAULT_FOLDERS);
    let fortune_files = file_utils::get_fortune_files(&DEFAULT_FOLDERS);
    match pick::pick_line_from_files_uniform(fortune_files) {
        Ok(fortune_result) => {
            if args.cookie {
                println!("({})\n%", fortune_result.file_path);
            }
            fortune::print_and_delay_size(&fortune_result.fortune, args.wait);
            exit(0);
        }
        Err(error) => {
            println!("Error: {}", error);
            exit(1);
        }
    }
}
