use clap::Parser;
use std::io::BufReader;
use std::fs::File;

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    _pattern: String,
    

    /// The path of the file to read
    _path: std::path::PathBuf,
}


fn main() {
    let _pattern = std::env::args().nth(1).expect("no pattern given");
    let _path = std::env::args().nth(2).expect("no path given");
    
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args._path).expect("could not read the file provided!");
    
    for line in content.lines() {
        if line.contains(&args._pattern) {
            println!("{}", line);
        }
    }
    println!("pattern {:?}, {:?}", args._pattern, args._path);
}
