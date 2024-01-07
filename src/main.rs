use clap::Parser;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use anyhow::{Context, Result};


/// Search and logically process patterns on the Command line
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    /// The pattern to look for
    #[arg(short='r', long, default_value="")]
    _pattern: String,
    

    /// The path of the file to read
    #[arg(short, long, default_value="")]
    _path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let args = Args::parse();
    let f = File::open(&args._path).expect("Unable to open the file!");
    let mut br = BufReader::new(f);
    let mut line = String::new();
    let mut check_pattern = false; 
    let mut high_freq_wordmap: HashMap<String, i32> = HashMap::new();

    loop {
        let content = br.read_line(&mut line).with_context(|| format!("could not read line"))?;
        if content == 0 {
            break;
        }
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
        line.clear();
    }
    println!("pattern {:?} in {:?} {:?}", args._pattern, args._path, check_pattern);
    println!("######################################");
    for (key, val) in high_freq_wordmap.iter() {
        println!("{key}:{val}");
    }
    println!("######################################");

    Ok(())
}
