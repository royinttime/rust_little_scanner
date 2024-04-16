mod util;
use std::env;

use util::scanner;

fn main() 
{
	let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        scanner(&args[1]);
    }
    else {
    	println!("Please pass the file name as parameter.");
    }
}
