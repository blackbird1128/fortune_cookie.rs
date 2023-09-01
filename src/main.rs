use clap::Parser;


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

    #[arg(short, long)]
    pattern: Option<String> 
        
}

fn main() {

    let cli = Args::parse();
    

    println!("all : {:?}", cli.all);
    println!("cookie:  {:?}",cli.cookie); 
}
