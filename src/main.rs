use std::io;
use std::fs::File;



fn main() {
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Reading error");

	let file = File::open(input.trim());
	match file {
		Ok(_) => println!("success"),
		Err(_) => println!("failure")
	}
}
