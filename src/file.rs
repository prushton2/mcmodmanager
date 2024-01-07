use serde_json;
use serde::{Serialize, Deserialize};
use std::fs;
use std::io::Write;


#[derive(Serialize, Deserialize)]
pub struct Config {
	pub os: String
}


pub fn load_config(file_name: &str) -> Result<Config, &str> {
	let file_contents = fs::read_to_string(file_name);


	if file_contents.is_err() {
		return Err("File could not be read")
	}

	let config: Config = serde_json::from_str(&file_contents.unwrap()).unwrap();

	return Ok(config)
}


pub fn write_config(file_name: &str, config: Config) -> Result<&str, &str> {
	
	let file_result = fs::File::create(file_name);
	let serialized = serde_json::to_string(&config);

	if serialized.is_err() {
		return Err("Serialization Failed")
	}

	if file_result.is_err() {
		return Err("Error opening file")
	}

	let mut file = file_result.unwrap();

	let _ = file.write_all(serialized.unwrap().as_bytes());
	return Ok("File Written")
}



pub fn _delete_file(file_name: &str) -> Result<&str, &str> {
	let result = fs::remove_file(file_name);

	if result.is_ok() {
		return Ok("File Removed");
	} else {
		return Err("Error occurred during file removal");
	}
}