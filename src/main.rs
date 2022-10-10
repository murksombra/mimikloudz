use std::env;
use std::fs::File;
use std::io::prelude::*;


fn main()-> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let cloud_provider = &args[1];
    println!("[@] MiMiKlOuDz - Intelligence gathering tool [@]");
    
    let mut file = File::open("aws.txt")?;
    let mut paths = String::new();
    file.read_to_string(&mut paths)?;

    Ok(())
}

