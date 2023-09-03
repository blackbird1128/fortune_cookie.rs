mod cli;
mod file_utils;
mod fortune;
mod percentage;
mod pick;

use clap::Parser;
use regex::Regex;
use std::process::exit;

#[derive(Parser)]
#[command(about = "Yet another fortune clone")]
pub struct Args {
    #[arg(short, long)]
    all: bool,

    #[arg(short, long)]
    cookie: bool,

    #[arg(short, long)]
    equal: bool,

    #[arg(short, long)]
    file: bool,

    #[arg(short, long)]
    long: bool,

    #[arg(short = 'm', long = "pattern")]
    pattern: Option<String>,

    #[arg(short = 'n', long = "length")]
    length: Option<i32>,

    #[arg(short, long)]
    offensive: bool,

    #[arg(short, long)]
    short: bool,

    #[arg(short, long)]
    ignore: bool,

    #[arg(short, long)]
    wait: bool,

    #[arg()]
    files: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();

    cli::handle_file_arg(&args);
    cli::handle_pattern_arg(&args);

    let files = args.files.clone().unwrap_or(vec!["".to_owned()]).join(" ");

    if files.trim().len() == 0 {
        cli::handle_zero_file_arg(&args);
        // no files specified
    } else {
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
}
