use crate::file_utils;
use crate::file_utils::FortuneResult;
use crate::fortune;
use crate::pick;
use crate::Args;
use regex::Regex;
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

pub fn handle_pattern_arg(args: &Args) -> Option<Vec<FortuneResult>> {
    if args.ignore && !args.pattern.is_some() {
        // -i without -m
        println!("The -i option can only be used with -m");
        exit(1);
    }
    if args.pattern.is_some() {
        let fortune_files = file_utils::get_fortune_files(&DEFAULT_FOLDERS);
        let pattern: String = args.pattern.as_ref().unwrap().to_string();
        let lines = pick::pick_all_from_files(fortune_files).unwrap_or_else(|_| {
            println!("Error: no fortune files found");
            exit(1);
        });
        let re = Regex::new(&pattern).unwrap_or_else(|_| {
            println!("Error: invalid regex");
            exit(1);
        });
        let mut fortunes = Vec::new();
        for line in lines {
            if re.is_match(&line.fortune) {
                fortunes.push(line);
            }
        }
        return Some(fortunes);
    }
    None
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

pub fn handle_multiplie_files_arg(args: &Args, files: &str) {
    let re = Regex::new(r"(\d\d?%?\s)?(\S+)+").expect("Error: invalid regex");
    if !re.is_match(&files) {
        println!("Error: files path must respect this format: [[n%] file/dir/all]");
        exit(1);
    }

    let fortune_files = match file_utils::file_args_to_file_contribution(&files) {
        Ok(x) => x,
        Err(x) => {
            println!("Error: {}", x);
            exit(1);
        }
    };

    let fortune_result = pick::pick_line_from_file_contributions(fortune_files);
    if args.cookie {
        println!("({})\n%", fortune_result.file_path);
    }
    fortune::print_and_delay_size(&fortune_result.fortune, args.wait);
    exit(0);
}
