use std::path::Path;
use std::io::BufReader;
use std::io::Lines;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use error_chain::error_chain;
use tokio;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let cloud_provider = &args[1];
    let base_cloud_addr = &args[2];
    let mut wordlist = String::new();

    println!("[@] MiMiKlOuDz - Intelligence gathering tool [@]");
    
    match cloud_provider.as_str() {
        "aws" => (wordlist = "aws.txt".to_string()),
        "digitalocean" => (wordlist = "digitalocean.txt".to_string()),
        "oracle" => (wordlist = "oracle.txt".to_string()),
        _ => println!("{} not supported yet.", cloud_provider.as_str()),
    }

    send_requests(wordlist, base_cloud_addr.to_string()).await;
    Ok(())

}

fn read_lines<P>(filename: P) -> std::io::Result<Lines<BufReader<File>>>
    where P: AsRef<Path>,{

    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

async fn send_requests(wordlist: String, base_addr: String){
    if let Ok(lines) = read_lines(wordlist){
        for line in lines {
            if let Ok(path) = line {
                tokio::task::spawn(request(path, base_addr.clone()));     
            }
        }
    }

}

async fn request (path: String, base_addr: String) -> Result<()> {
    println!("[*] Requesting {}/{}", base_addr, path);
    let target = format!("http://{base_addr}/{path}");
    let content = reqwest::get(target)
    .await.expect("BAzinga")
    .text()
    .await;
    println!("[+] Full Response:\n{:?}",content);
    Ok(())
}
