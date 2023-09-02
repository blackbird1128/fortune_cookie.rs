mod fortune;

use clap::Parser;
use regex::Regex;
use std::process::exit;
use walkdir::WalkDir;

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

fn get_fortune_files(vec_folders: &[&str]) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for folder in vec_folders {
        for file in WalkDir::new(folder)
            .into_iter()
            .filter_map(|file| file.ok())
        {
            if file.metadata().unwrap().is_file() {
                files.push(file.path().to_str().unwrap().to_owned());
            }
        }
    }
    files
}

fn main() {
    let cli = Args::parse();
    const DEFAULT_FOLDERS: [&str; 1] = ["./fortunes/"];

    if cli.file {
        for file in get_fortune_files(&DEFAULT_FOLDERS) {
            println!("{file}");
        }
        exit(0);
    }

    let files = cli.files.unwrap_or(vec!["".to_owned()]).join(" ");

    println!("files:{files}");

    let re = Regex::new(r"(\d\d?%?\s)?(\S+)+").unwrap();
    if !re.is_match(&files) {
        println!("Error: files path must respect this format: [[n%] file/dir/all]");
    }

    re.captures_iter(&files).for_each(|cap| {
        cap.get(1).map_or_else(
            || println!("No percentage"),
            |m| {
                println!("Percentage: {}", m.as_str());
            },
        );
    });
    println!();

    println!("all: {}", cli.all);
    println!("cookie:  {:?}", cli.cookie);
}
