
use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use base64;
use reqwest;
use lettre;
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
struct Record {

    Type: String,
    Data: String,
    State: String,
    Zipcode: String,
    Email: String,

}

fn main() {

    // See how I can make this static/global
    let mut client = reqwest::Client::new();


    let args: Vec<String> = env::args().collect();


    let res = client.post("http://httpbin.org/post")
        .json("{'name':'disco bob', 'subject': 'Math', 'grade' : 'fifth'")
        .send();

    
    
}

fn CreateRecordFromConfig(cfgdata : String)
{

    let mut rec : Record ;
    let mut chunks: Vec<&str>;
     
    let splitter = String::from("||");
    chunks = cfgdata.split(&splitter).collect();


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

