use std::path::Path;
use std::io::BufReader;
use std::io::Lines;
use std::io::Result;
use std::env;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let args: Vec<String> = env::args().collect();
    let cloud_provider = &args[1];
    let mut wordlist = String::new();

    println!("[@] MiMiKlOuDz - Intelligence gathering tool [@]");
    
    match cloud_provider.as_str() {
        "aws" => (wordlist = "aws.txt".to_string()),
        "digitalocean" => (wordlist = "digitalocean.txt".to_string()),
        "oracle" => (wordlist = "oracle.txt".to_string()),
        _ => println!("{} not supported yet.", cloud_provider.as_str()),
    }

    if let Ok(lines) = read_lines(wordlist){
        for line in lines {
            if let Ok(path) = line {
                println!("{}",path);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>,{

    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
