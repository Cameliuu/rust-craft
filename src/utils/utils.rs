use std::{collections::HashMap, fs, path::Path};
use serde::Deserialize;
use serde_json;

use vintor::{encode,EncodeError};
pub fn extend_with_string(vec: &mut Vec<u8>, string: String) -> Result<(),EncodeError> {
    let str_len = encode(string.len() as i32)?;
    let str_bytes = string.as_bytes();

    vec.extend(str_len);
    vec.extend(str_bytes);

    Ok(())
}
#[derive(Debug, Deserialize)]
pub struct RegistryFile(pub HashMap<String, HashMap<String, serde_json::Value>>);

pub fn load_registries_from_file<P: AsRef<Path>>(path: P) -> Result<RegistryFile, Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string(path)?;
    let registry_file: RegistryFile = serde_json::from_str(&json_data)?;
    Ok(registry_file)
}

