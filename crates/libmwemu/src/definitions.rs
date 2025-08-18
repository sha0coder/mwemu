use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use crate::emu::Emu;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub source: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Definition {
    #[serde(deserialize_with = "deserialize_address")]
    pub address: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_context: Option<String>,
    pub parameters: Vec<Parameter>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Definitions {
    events: Vec<Definition>,
}

#[derive(Clone, Debug)]
pub struct StoredContext {
    pub values: HashMap<String, u64>,
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
    for def in definitions.events {
        map.insert(def.address, def);
    }
    map
}

impl Emu {
    pub fn show_definition(&mut self) {
        let rip = self.regs().rip;
        let definitions = &self.cfg.definitions;
        if let Some(definition) = definitions.get(&rip) {
            log::info!("Function definition: {} (0x{:x})", definition.name, rip);
                
            for (i, param) in definition.parameters.iter().enumerate() {
                let value = match i {
                    0 => self.regs().rcx,
                    1 => self.regs().rdx,
                    2 => self.regs().r8,
                    3 => self.regs().r9,
                    _ => {
                        // Stack parameters (5th and beyond) at rsp + 0x20 + (i-4)*8
                        let addr = self.regs().rsp + 0x20 + ((i - 4) * 8) as u64;
                        self.maps.read_qword(addr).unwrap_or(0)
                    }
                };
                
                let display_value = match param.param_type.as_str() {
                    "pointer" => format!("0x{:x}", value),
                    "wide_string" => {
                        if value != 0 {
                            let s = self.maps.read_wide_string(value);
                            format!("L\"{}\" (0x{:x})", s, value)
                        } else {
                            "NULL".to_string()
                        }
                    }
                    _ => format!("0x{:x}", value),
                };
                
                log::info!("    {}: {}", param.name, display_value);
            }
        }
    }
}