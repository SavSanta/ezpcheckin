use std::fs::File;
use std::io::{Read, BufReader, BufRead, Error};

fn main() {
    let mut recs : Vec<String>;
    let file: Result<File, std::io::Error> = File::open("ezpstore_test.txt");
    match file {
        Ok(file) => {

            let buffered = BufReader::new(file);
            
            for line in buffered
                         .lines()
                         .filter(|x| !x.as_ref().unwrap().trim().is_empty())
                         .filter(|x| !x.as_ref().unwrap().trim().starts_with(&['#'])) 
            {
                
                println!("{:?}", line);
                recs.Add(line);
            }
        }
        Err(err) => {
            println!("The ezpstore.txt file could not be opened: {err}");
        }
    }
}
