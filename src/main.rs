use clap::Parser;
use std::collections::HashMap;


/// Search and logically process patterns on the Command line
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Cli {
    /// The pattern to look for
    #[arg(short='r', long, default_value="")]
    _pattern: String,
    

    /// The path of the file to read
    #[arg(short, long, default_value="")]
    _path: std::path::PathBuf,
}

fn main() {
    let _pattern = std::env::args().nth(1);
    let _path = std::env::args().nth(2);
    
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args._path).expect("could not read the file provided!");
    let mut check_pattern = false; 
    let mut high_freq_wordmap: HashMap<String, i32> = HashMap::new(); 
    for line in content.lines() {
        if line.contains(&args._pattern) {
            check_pattern = true;
        }
        for word in line.split_whitespace() {
            {
                let mut temp_word = word.to_string(); 
                temp_word.retain(|temp_word| !r#"()[]{},.*":'"#.contains(temp_word));
                high_freq_wordmap.entry(temp_word).and_modify(|counter| *counter += 1).or_insert(1);
            }
        }
        }
    println!("pattern {:?} in {:?} {:?}", args._pattern, args._path, check_pattern);
    println!("######################################");
    for (key, val) in high_freq_wordmap.iter() {
        println!("{key}:{val}");
    }
    println!("######################################");
}
