use crate::conf::DEFAULT_FOLDERS;
use crate::file_utils;
use crate::file_utils::FortuneResult;
use crate::fortune;
use crate::pick;
use crate::pick::FilterFile;
use crate::pick::FilterLen;
use crate::pick::FortuneFilter;
use crate::Args;
use regex::Regex;
use std::path::Path;
use std::process::exit;

fn check_fortunes_folders_exist(paths: &[&str]) {
    for path in paths {
        if !Path::new(path).exists() {
            if path == &DEFAULT_FOLDERS[0] {
                eprintln!("Error: folder {} does not exist", path);
                eprintln!("Did you install with make install ?");
                exit(1);
            }
        }
    }
}

fn check_health() {
    let mut error_count = 0;
    for path in &DEFAULT_FOLDERS {
        if !Path::new(path).exists() {
            if path == &DEFAULT_FOLDERS[0] {
                eprintln!("Warning: folder {} does not exist", path);
                eprintln!("Did you install with make install ?");
                error_count += 1;
            }
            if path == &DEFAULT_FOLDERS[1] {
                eprintln!(
                    "Warning: folder {} does not exist (fortune default fortunes not available)",
                    path
                );
                eprintln!("You can install them with: sudo apt install fortune-mod");
                error_count += 1;
            }
        }
    }
    if error_count == 0 {
        println!("Check health: OK");
    }
}

pub fn handle_check_health(args: &Args) {
    if args.checkhealth {
        check_health();
        exit(0);
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

pub fn produce_filter_from_args(args: &Args) -> FortuneFilter {
    let mut filter = FortuneFilter {
        file: FilterFile::Default,
        len: FilterLen::Short,
        len_value: 160,
    };
    if args.all {
        filter.file = FilterFile::All;
    }
    if args.offensive {
        filter.file = FilterFile::Offensive;
    }
    if args.long {
        filter.len = FilterLen::Long;
    }
    if args.short {
        filter.len = FilterLen::Short;
    }
    if args.length.is_some() {
        filter.len_value = args.length.unwrap();
    }
    filter
}

pub fn handle_pattern_arg(args: &Args) -> Option<Vec<FortuneResult>> {
    if args.ignore && !args.pattern.is_some() {
        // -i without -m
        eprintln!("The -i option can only be used with -m");
        exit(1);
    }
    if args.pattern.is_some() {
        let fortune_files = file_utils::get_fortune_files(&DEFAULT_FOLDERS);
        let mut pattern = args.pattern.as_ref().unwrap().to_string();
        if args.ignore {
            pattern = format!("(?i){}", pattern);
        }
        let lines = pick::pick_all_from_files(fortune_files).unwrap_or_else(|e| {
            eprintln!("{}", e);
            exit(1);
        });
        let re = Regex::new(&pattern).unwrap_or_else(|_| {
            eprintln!("Error: invalid regex");
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
    let filter = produce_filter_from_args(args);
    let fortune_files = file_utils::get_fortune_files(&DEFAULT_FOLDERS);
    match pick::pick_line_from_files_uniform(fortune_files, filter) {
        Ok(fortune_result) => {
            if args.cookie {
                println!("({})\n%", fortune_result.file_path);
            }
            fortune::print_and_delay_size(&fortune_result.fortune, args.wait);
            exit(0);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            exit(1);
        }
    }
}

pub fn handle_multiplie_files_arg(args: &Args, files: &str) {
    let re = Regex::new(r"(\d\d?%?\s)?(\S+)+").expect("Error: invalid regex");
    if !re.is_match(&files) {
        eprintln!("Error: files path must respect this format: [[n%] file/dir/all]");
        exit(1);
    }

    let fortune_files = match file_utils::file_args_to_file_contribution(&files) {
        Ok(x) => x,
        Err(x) => {
            eprintln!("Error: {}", x);
            exit(1);
        }
    };
    let filter = produce_filter_from_args(args);
    let fortune_result = pick::pick_line_from_file_contributions(fortune_files, filter); // Add a way to handle len
    if args.cookie {
        println!("({})\n%", fortune_result.file_path);
    }
    fortune::print_and_delay_size(&fortune_result.fortune, args.wait);
    exit(0);
}
