use serde_json::to_string_pretty;
use std::env;
use std::error::Error;
use std::fs::{read_to_string, write};
use string_error::into_err;
use toml;

pub fn covert_toml_to_json(toml_file: &str, json_file: &str) -> Result<(), String> {
    let toml = read_to_string(toml_file)
        .map_err(|e| format!("failed to read toml file: {:?}", e))?
        .parse::<toml::Value>()
        .map_err(|e| format!("failed to parse toml file: {:?}", e))?;
    let json = to_string_pretty(&toml)
        .map_err(|e| format!("failed to convert to json string: {:?}", e))?;
    write(&json_file, &json).map_err(|e| format!("failed to write to json file: {:?}", e))?;
    println!("Done");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let (toml_path, json_path) = match args.len() {
        3 => (args[1].clone(), args[2].clone()),
        2 => (args[1].clone(), args[1].clone() + ".json"),
        _ => {
            return Err(into_err(format!(
                "Incorrect argument numbers {:?}",
                args.len() - 1
            )))
        }
    };

    covert_toml_to_json(&toml_path, &json_path).map_err(|err| into_err(err))
}
