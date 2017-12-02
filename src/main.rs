use std::env;

fn scanner(file_name: &String)
{
	println!("{}", file_name);
}

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
