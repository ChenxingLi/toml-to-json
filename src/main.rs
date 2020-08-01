use serde_json::to_string_pretty;
use std::env;
use std::error::Error;
use std::fs::{read_to_string, write};
use string_error::{into_err, new_err};
use toml::Value as tomlVal;

pub fn covert_toml_to_json(toml_file: &str, json_file: &str) -> Result<(), String> {
    let toml = read_to_string(toml_file)
        .map_err(|e| format!("failed to read toml file: {:?}", e))?
        .parse::<tomlVal>()
        .map_err(|e| format!("failed to parse toml file: {:?}", e))?;
    let json = to_string_pretty(&toml)
        .map_err(|e| format!("failed to convert to json string: {:?}", e))?;
    write(&json_file, &json).map_err(|e| format!("failed to write to json file: {:?}", e))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!();
        return Err(new_err("Incorrect argument numbers"));
    }

    covert_toml_to_json(&args[0], &args[1]).map_err(|err| into_err(err))
}
