use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Definition {
    #[serde(deserialize_with = "deserialize_address")]
    pub address: u64,
    pub parameters: Vec<Parameter>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Definitions {
    functions: Vec<Definition>,
}

fn deserialize_address<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    if s.starts_with("0x") {
        u64::from_str_radix(&s[2..], 16)
            .map_err(|e| serde::de::Error::custom(e))
    } else {
        s.parse::<u64>()
            .map_err(|e| serde::de::Error::custom(e))
    }
}

pub fn load_definitions(filename: &str) -> HashMap<u64, Definition> {
    let contents = fs::read_to_string(filename)
        .expect("Failed to read definitions file");
    
    let definitions: Definitions = serde_yaml::from_str(&contents)
        .expect("Failed to parse YAML");
    
    let mut map = HashMap::new();
    for def in definitions.functions {
        map.insert(def.address, def);
    }
    map
}