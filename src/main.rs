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
    println!("[@] MiMiKlOuDz - Intelligence gathering tool [@]");
    
    if let Ok(lines) = read_lines("./aws.txt"){
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
