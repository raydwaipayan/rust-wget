extern crate clap;
extern crate reqwest;
extern crate colored;
extern crate indicatif;

use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use std::process;
use std::error::Error;
use std::fmt;
use clap::{ Arg,App };
use indicatif::{ProgressBar,ProgressStyle};
use reqwest::Client;
use colored::*;

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"{}", self.0.red())
    }
}
impl Error for MyError{}

fn create_bar(x: Option<u64>) -> ProgressBar {
   match x{
       Some(s) =>{
            let pb=ProgressBar::new(s);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .progress_chars("#>-"));
            pb
        },
        None => {
            let pb=ProgressBar::new_spinner();
            pb.set_style(ProgressStyle::default_spinner());
            pb
        }
    }
}

fn download(url: &str) -> Result<(), Box<dyn Error>>{
    let client=Client::new();
    let mut res = client.get(url).send()?;

    if res.status().is_success() {
        let clt=res.content_length();

        let csize=match clt{
            Some(s) => s as usize/99,
            None => 1024usize,
        };
        let name=url.split("/").last().unwrap();

        let mut buf: Vec<u8> =Vec::new();
        println!("Downloading {}",name.cyan());
        let pb=create_bar(clt);
        loop{
            let mut buf2: Vec<u8> = vec![0;csize];
            let c=res.read(&mut buf2).unwrap();
            buf2.truncate(c);
            if buf2.is_empty(){
                break;
            }
            buf.append(&mut buf2);
            pb.inc(c as u64);
        }
        let mut file=File::create(name)?;
        file.write_all(&mut buf)?;
        pb.finish_and_clear();
        println!("{}","Success!".green());
    }
    else{
        return Result::Err(Box::new(MyError("Error fetching file!".into())));
    }
    Ok(())
}
fn main() {
    let matches=App::new("rust-wget")
        .version("0.1")
        .author("Dwaipayan Ray")
        .about("simple wget in rust")
        .arg(Arg::with_name("URL")
            .help("URL for the file")
            .required(true)
            .takes_value(true))
        .get_matches();
    let url=matches.value_of("URL").unwrap();
    
    if let Err(e)=download(url){
        eprintln!("{}",e.to_string().red());
        process::exit(1);
    }
}
