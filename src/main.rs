use clap::Parser;
use regex::Regex;

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

    #[arg(short='m', long="pattern")]
    pattern: Option<String>,

    #[arg(short='n', long="length")]
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
    files: String

}

fn main() {

    let cli = Args::parse();

    let files: String = cli.files;

    let re = Regex::new(r"((?:\d\d%)?(?:\S+))+").unwrap();

    if !re.is_match(&files) {
        println!("Error: files path must respect this format: [[n%] file/dir/all]");
    }

    for elem in re.captures_iter(&files)    {
        println!("elem: {:?}",elem);
    }


    println!("files: {}", files);
    println!("cookie:  {:?}",cli.cookie); 
}
