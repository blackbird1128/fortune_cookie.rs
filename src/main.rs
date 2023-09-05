mod cli;
mod conf;
mod file_utils;
mod fortune;
mod percentage;
mod pick;

use std::process::exit;

use clap::Parser;

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
    length: Option<u32>,

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
    let pattern_result = cli::handle_pattern_arg(&args);
    if pattern_result.is_some() {
        let mut pattern_result = pattern_result.unwrap();
        let pattern_str = pattern_result
            .iter_mut()
            .map(|x| x.fortune.clone())
            .collect::<Vec<String>>()
            .join("\n%\n");
        println!("{}", pattern_str);
        exit(0);
    }

    let files = args.files.clone().unwrap_or(vec!["".to_owned()]).join(" ");

    if files.trim().len() == 0 {
        cli::handle_zero_file_arg(&args);
        // no files specified
    } else {
        cli::handle_multiplie_files_arg(&args, &files);
    }
}
