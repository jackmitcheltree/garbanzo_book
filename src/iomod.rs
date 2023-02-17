use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

pub fn load_file_txt(path : &PathBuf) -> Result<Vec<String>, std::io::Error> {
	//create vec we will return
	let mut result : Vec<String> = Vec::new();

	//string to hold file contents upon read
	let mut text = String::new();

	//open file using path provided
	let mut file = match File::open(&path) {
		Ok(file) => file,
		Err(e) => return Err(e)
	};

	//read file as string to text variable
	match file.read_to_string(&mut text) {
		Ok(bytes) => bytes,
		Err(e) => return Err(e)
	};
	
	//convert text to vector of string slices splitting on \n char
	let mut text : Vec<&str> = text.split("\r\n").collect();

	//for every entry in text write as String to result vec
	for line in text {
		let mut _line = String::from(line); 
		_line.push('\n'); //design choice: consider not putting a \n on last line
		result.push( _line )
	};

	//implicitly returns Ok(result)
	Ok(result)
}

//untested
pub fn save_file_txt(path : &PathBuf, text : &Vec<String>) {
	//assume that the strings in text already have \n appended to the end

	//string to write all the lines of text to so we can save
	let mut result = String::new();

	//create makes a new file if none exists or destories old one and replaces with new
	//create file
	let mut file = match File::create(&path) {
		Ok(file) => file,
		Err(_) => panic!("Save failed.")
	};

	//write to result string
	for line in text {
		result.push_str(&line)
	};

	//save the sting to the file that was created
	match file.write_all(result.as_bytes()) {
		Ok(_) => println!("{:?} saved successfully.", *path),
		Err(e) => panic!("Save failed.")
	};
}