use mail_send::*;
use crate::mail_builder::MessageBuilder;
use std::io::prelude::*;
use std::fs::File;
use tokio;

fn main() 
{

    println!("Smtp Tests Began -- Begin Build 09-18 1140hrs");
    println!("Calling function");
    
    let loremipsum = String::from("The test string message begins here!! ");
    
    let recipients = vec![
            ("Test1", "noreply@zixzixyahoo.com"),
            ("Test2", "noreply@nixwiixgmail.com"),
        ];
    
    let retopt = 

    tokio::runtime::Runtime::new().unwrap().block_on(
	SendMail(loremipsum, recipients)  
    );

/*
    match retopt 
    {
         Some(mesg) => println!("Some received with message: {mesg}"),
         None => println!(""),
    }
*/

//    println!("Future check type {}", retopt);
    println!("Hopefully the message was sent...");


}

async fn SendMail(msgdata : String, emailto :Vec<(&str, &str)>)
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
        //.credentials(("john", "p4ssw0rd"))
        .connect_plain()
        .await
        .unwrap()
        .send(message)
        .await
        .unwrap();


     println!("Email sent out hopefully to rcpts");

}
