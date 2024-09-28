
use std::io::prelude::*;
use std::fs::File;
use gjson;
use gjson::Kind;

fn main() 
{

    println!("Program Began");
    println!("Calling function");
    let retopt = guppy();

    match retopt 
    {
         Some(mesg) => println!("Some received with message: {mesg}"),
         None => println!(""),
    }


}


fn guppy() -> Option<String> 
{
    let file = File::open("sample.json");
    let mut read_data: String = Default::default();

    // read file contnets. Using OK because this returns a Result<T,E> but we dont give a damn due to expect.
    file.expect("Failed to open handle to 'sample.json'").read_to_string(&mut read_data).ok()?;
    //let n = f.read(&mut buffer[..])?;

    println!("Size of file in bytes: {:?}", read_data.len());
    println!("GJSON debug testing begins");

    let entries = gjson::get(&read_data, "#.itemDescription");
    let total_amt = gjson::get(&read_data, "#.formattedTotal");
    let num_entries = entries.array().len();
    /* obsoleted
    //let ilast = isize::try_from(entries.array().len() - 1).unwrap();
    //let last = usize::try_from(ilast).unwrap();
    */


    // Different from python and go implementations which use some form of last/last-1 for bounds check. 
    // This proves difficult here because Rust requires a usize type for indexing but usize is unsigned and needs to be mangled into a isize.
    // Much easier to discover that modifying it using built-in method of Last() on array got us the same and stopped fatal panics.
    if num_entries > 0
	{

	    println!("The entries value total of {:#?}", last);
	    //println!("The entries value as dictated from isize last is {:#?}", ilast);
	    println!("The data had SOME entries pp {} \n and totals {}", entries, total_amt);

let fstring = format!("The {} is {} from {} tolls", entries.array().last().unwrap(), total_amt.array().last().unwrap(), num_entries);
println!("{fstring}");

            // Add support for "-nomail" flag to print to standard output
            //println!(msg);
//	    Some(Default::default())     // dbghelp - to prevent error on func signature mismatch with compile
	    Some(fstring)
	}

    else 
	{
	    println!("No matches found in JSON.");
	    None

	}
    
}
