#![allow(unused)]

use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use base64::prelude::*;
use reqwest;
use gjson;
use tokio;
use rand::Rng;
use mail_send::*;
use mail_builder::headers::address::Address;
use crate::mail_builder::MessageBuilder;
//use lettre;


//const Queryv1API : &str = "aHR0cHM6Ly9jc2MuZHJpdmVlem1kLmNvbS9hcGkvUGF5VG9sbHMvUGF5bWVudC9QZW5kaW5nLw==";
const QueryAPI : &'static str = "aHR0cHM6Ly9jc2MuZHJpdmVlem1kLmNvbS9hcGkvUGF5VG9sbHMvUGVuZGluZ1BheW1lbnRzVG90YWwv";

static mut TestDebug: bool = false;
static mut NoMail: bool = false;

type MessageResult = std::result::Result<String, std::io::Error>;

//#[derive(Serialize, Deserialize, Debug)]
struct Record {

    Type: String,
    Data: String,
    State: String,
    Zipcode: String,
    Email: Vec<String>,

}

fn main() {

    // See how I can make this static/global
    
    unsafe 
    {
        let args: Vec<String> = env::args().collect();
        TestDebug = args.contains(&String::from("-testdebug"));
        NoMail = args.contains(&String::from("-nomail"));
        println!("The TestDebug value is {}.\nNoMail value is {}.", TestDebug), NoMail;
    }


    let rec = CreateRecordFromConfig(std::string::String::from("LIC || 7EH7532 || MD || 77040 || firkille@@hotbot.net"));
    unsafe 
    { 
        QueryNotice(rec)
    };

}

fn CreateRecordFromConfig(cfgdata : String) -> Record
{

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

    Record{Type: chunks[0].to_string(), Data: chunks[1].to_string(), State: chunks[2].to_string(), Zipcode: chunks[3].to_string(), Email: chunks[4].split(",").map(|v| v.to_string()).collect::<Vec<_>>()}


}

unsafe fn QueryNotice(r : Record)
{

    // Currently built only to to use the License plate + zip
    let baseURL = String::from_utf8(BASE64_STANDARD.decode(QueryAPI).unwrap()).unwrap();

    if TestDebug == true 
    {
        println!("Base URL => {}\n", baseURL);
        println!("Zipcode is => {}\n", r.Zipcode);
        println!("License Plate is => {}\n", r.Data);
    }

    // Explicitfy the separators for ease
    //let QueryURL = baseURL + "0/" + r.Zipcode + "/" + r.Data + "/1/25/"                    // API V1 (deprecated)
    let  QueryURL = baseURL + "0/" + &r.Zipcode + "/" + &r.Data + "/1/25/" + "0/"  ;           // API V2 requirement
    println!("Target URL {}", QueryURL);

    // mutable data holder for successful responses or mocked sample.json file
    let mut json_resp_data = String::new();

    // If we're not in TestDebug mode then dont look for a sample.json file
    if TestDebug == false {
        // Should be a Result<reqwest:Response> type
        let resp_Result = reqwest::blocking::get(QueryURL);//.expect("FAILURE TO REACH BASEURL").text();

        json_resp_data = match resp_Result {
            Ok(ref Response) => {  
                  
                if resp_Result.as_ref().expect("Error retrieving an HTTP Status Number.").status().as_u16() > 299 {
                    // Will need to convert this to a try! or Err(invariant) match block.
                    println!("Response had a StatusCode: {}\n Body: <not in code due to bug>\n\n", resp_Result.as_ref().unwrap().status().as_str() /*, resp_Result.unwrap().text()*/);
                    //log.Println(err.Error())
                    //SendErrorMail(err.Error(), r.Email)
                    panic!("Status Code Response Error: {}", resp_Result.as_ref().unwrap().status().as_str());
                }
                else {
                    println!("Response data seem to be successful");
                    resp_Result.unwrap().text();
                }

            },
            Err(error) => { 
                //SendErrorMail(err.Error(), r.Email)
                panic!("Problem making the reqwest: {error:?}");
            },
        };

	} else {

            println!("Utilizing local sample.json file");

            // Read in sample.json since no current tolls exist
        	let sample_file_result = File::open("sample.json");

    		// I still dont understand this dumb match pattern and how each passes to the other
    		// sample_file when it successfully acquired and checked with print, printed "OK(21694)". So it was an OK with the amount of bytes. Actually the exact size in bytes of the sample.json file
    		// LL: I had to borrow the stupid file as a mutable in order to read the mutable string?
        	let sample_file = match sample_file_result {
        	        // read the whole file or error out
            		Ok(mut file) => file.read_to_string(&mut json_resp_data),
            		Err(error) => panic!("Error opening sample.json: {error:?}"),
            };
    
            // print the sample data to stdout
            println!("Data JSON read:\t {:?}",json_resp_data)

	}

	// Check length of bytes here.
	// Check number of records
	// Return nil + send email as response length mayve changed
	{
		use std::thread::sleep;
		use std::time::Duration;
		std::thread::sleep(Duration::new(rand::thread_rng().gen_range(2..4), 0));
	}


	let message = SearchJSONResponse(json_resp_data);

    match message 
    {
         Some(msg) => tokio::runtime::Runtime::new()
                         .unwrap()
                         .block_on( SendMail(msg, r.Email) ) ,
                         
         None => println!("No matches found in SearchJSON."),
    }

	
	return

}

fn SearchJSONResponse(json_resp_data : String) -> Option<String>
{

    let entries = gjson::get(&json_resp_data, "#.itemDescription");
    let total_amt = gjson::get(&json_resp_data, "#.formattedTotal");
    let last = entries.array().len() - 1;

    // Test sample.json Output
    //["Invoiced Toll Transaction","Invoiced Toll Transaction","Invoiced Toll Transaction","Invoiced Toll Transaction","Invoiced Toll Transaction","Invoiced Toll Transaction","Invoiced Toll Transaction","Total Amount Due"] 
    //["$6.00","$6.00","$6.00","$6.00","$6.00","$6.00","$12.00","$48.00"]

    // Different from python and go implementations which check explicitly against -1 for out-of-bounds but this check is essentially the same since less-than/ge4ater-than arent inclusive.
    if last > 0
	{
        let fstring = format!("The {} is {} from {} tolls", entries.array()[last-1], total_amt.array()[last-1], last);

        //TODO: Add support for "-nomail" flag to print to standard output. 
        println!(fstring);
        Some(fstring)
	}
    else 
	{
	    // Return Option(None). The failure message is printed in the calling function match braces.
	    None
	}

}

fn MakePayment()
{

}


/// Need to strengthen my understanding of lifetime annotations here. 
/// Though, based on previous knowledge I realized struggling with unwrapping the Vec(&str, &str> or casting Vec<(Vec<String>, Vec<String>)> was not necessary.
/// Here we simply use the Address type from https://docs.rs/mail-builder/0.3.1/mail_builder/struct.MessageBuilder.html#method.write_to to pass in either a convertible String, &str, or Vec. (hopefully)

async fn SendMail(msgdata : String, emailto : impl for <'x> Into<Address<'x>>)
{


    // Build a simple multipart message
    let message = MessageBuilder::new()
        .from(("Banana Rama", "banana@example.net"))
        .to(
            emailto
          )
        .subject("Rusty Tolls Alert!")
        //.html_body("<h1>Hello, world!</h1>")
        .text_body(msgdata);

    // Connect to the SMTP submissions port, upgrade to TLS and
    // authenticate using the provided credentials.
    SmtpClientBuilder::new("127.0.0.1", 25)
        .implicit_tls(false)
        .connect_plain()
        .await
        .unwrap()
        .send(message)
        .await
        .unwrap();


     println!("Email dispatched to rcpts");

}

fn SendErrorMail()
{

}

