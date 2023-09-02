mod file_utils;
mod fortune;
mod percentage;
mod pick;

use clap::Parser;
use regex::Regex;
use std::process::exit;

#[derive(Parser)]
#[command(about = "Yet another fortune clone")]
struct Args {
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
    let cli = Args::parse();
    const DEFAULT_FOLDERS: [&str; 1] = ["./fortunes/"];
    if cli.file {
        for file in file_utils::get_fortune_files(&DEFAULT_FOLDERS) {
            println!("{file}");
        }
        exit(0);
    }
    let files = cli.files.unwrap_or(vec!["".to_owned()]).join(" ");

    let re = Regex::new(r"(\d\d?%?\s)?(\S+)+").unwrap();
    if !re.is_match(&files) {
        println!("Error: files path must respect this format: [[n%] file/dir/all]");
    }

    if files.trim().len() == 0 {
        // no files specified
        let fortune_files = file_utils::get_fortune_files(&DEFAULT_FOLDERS);
        println!("{}", pick::pick_line_from_files_uniform(fortune_files));
        exit(0);
    } else {
        let fortune_files = file_utils::file_args_to_file_contribution(&files);
        println!("{}", pick::pick_line_from_file_contributions(fortune_files));
        exit(0);
    }
}
