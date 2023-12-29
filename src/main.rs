use clap::Parser;
use std::collections::HashMap;


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
    let mut check_pattern = false; 
    let mut high_freq_wordmap: HashMap<String, i32> = HashMap::new();
    // This can be dynamically done
    //let high_freq_words = vec!["dog", "fox", "nice", "good"];
    for line in content.lines() {
        if line.contains(&args._pattern) {
            check_pattern = true;
        }
        for word in line.split_whitespace() {
            {
                let mut temp_word = word.to_string(); 
                temp_word.retain(|temp_word| !r#"()[]{},.*":'"#.contains(temp_word));
                //println!("temp word {}", temp_word);
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
