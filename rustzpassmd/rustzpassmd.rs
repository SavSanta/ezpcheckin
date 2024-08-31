#![allow(unused)]

use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use base64;
use reqwest;
//use lettre;
use rand;       //random library support
use serde_json; // Dunno why this even exists opposed to regular serde
use serde::{Serialize,Deserialize};


// Deletable
//static myrando : u8 = rand::random::<u8>();


//const Queryv1API : &str = "aHR0cHM6Ly9jc2MuZHJpdmVlem1kLmNvbS9hcGkvUGF5VG9sbHMvUGF5bWVudC9QZW5kaW5nLw==";
const QueryAPI : &'static str = "aHR0cHM6Ly9jc2MuZHJpdmVlem1kLmNvbS9hcGkvUGF5VG9sbHMvUGVuZGluZ1BheW1lbnRzVG90YWwv";

static mut TestDebug: bool = false;
static mut NoMail: bool = false;

#[derive(Serialize, Deserialize, Debug)]
struct Record<'a> {

    Type: &'a str,
    Data: &'a str,
    State: &'a str,
    Zipcode: &'a str,
    Email:Vec<&'a str>,

}

fn main() {

    // See how I can make this static/global
    let mut client = reqwest::Client::new();


    let args: Vec<String> = env::args().collect();


    let res = client.post("http://httpbin.org/post")
        .json("{'name':'disco bob', 'subject': 'Math', 'grade' : 'fifth'")
        .send();

}

fn CreateRecordFromConfig<'a>(cfgdata : String) -> Record<'a>
{

    //let mut rec : Record ;
    let mut chunks: Vec<&str>;
     
    chunks = cfgdata.split("||").collect();

    if (chunks.len() == 5)
    {
        for chunk in chunks.iter_mut()
        {
            *chunk = chunk.trim();
        }
    }
    else 
    {
        panic!("config file has insufficient chunks.");
    }

    Record{Type: chunks[0], Data: chunks[1], State: chunks[2], Zipcode: chunks[3], Email: chunks[4].split(",").collect::<Vec<_>>()}


}

fn QueryNotice()
{

}

fn SearchJSONResponse()
{

}

fn MakePayment()
{

}

fn SendMail()
{

}

fn SendErrorMail()
{

}

