use std::fs::read_to_string;

use clap::Parser;
/*  Invocation of grrs :

    grrs will act as grep for a file

    Usage: 
    
    grrs {text_file} {word to search}

*/ 

#[derive(Parser)]
struct Cli{
    pattern : String,
    path : std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();

    let content = read_to_string(&args.path).expect("[x] Couldn't read file !");

    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("{}",line);
        }
    }
}
