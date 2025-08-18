use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Definition {
    pub address: u64,
    pub parameters: Vec<Parameter>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Definitions {
    functions: Vec<Definition>,
}

pub fn load_definitions(filename: &str) -> Vec<Definition> {
    let contents = fs::read_to_string(filename)
        .expect("Failed to read definitions file");
    
    let definitions: Definitions = serde_yaml::from_str(&contents)
        .expect("Failed to parse YAML");
    
    definitions.functions
}