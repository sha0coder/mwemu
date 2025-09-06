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
        u64::from_str_radix(&s[2..], 16).map_err(|e| serde::de::Error::custom(e))
    } else {
        s.parse::<u64>().map_err(|e| serde::de::Error::custom(e))
    }
}

pub fn load_definitions(filename: &str) -> HashMap<u64, Definition> {
    let contents = fs::read_to_string(filename).expect("Failed to read definitions file");

    let definitions: Definitions = serde_yaml::from_str(&contents).expect("Failed to parse YAML");

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
            log::info!(
                "Event: {} (0x{:x}) - {}",
                definition.name,
                rip,
                definition.event_type
            );

            // Store context if needed
            if let Some(context_name) = &definition.store_context {
                let mut context_values = HashMap::new();
                for param in &definition.parameters {
                    let value = self.resolve_source(&param.source);
                    context_values.insert(param.name.clone(), value);
                }
                self.stored_contexts.insert(
                    context_name.clone(),
                    StoredContext {
                        values: context_values,
                    },
                );
            }

            // Display parameters
            for param in &definition.parameters {
                let value = self.resolve_source(&param.source);
                let display_value = self.format_parameter_value(value, &param.param_type);
                log::info!("    {}: {}", param.name, display_value);
            }
        }
    }

    fn resolve_source(&self, source: &str) -> u64 {
        let parts: Vec<&str> = source.split(':').collect();

        match parts[0] {
            "deref" => {
                // deref:context:context_name:param_name or deref:register
                if parts.len() >= 2 {
                    let inner_source = &source[6..]; // Skip "deref:"
                    let ptr_value = self.resolve_source(inner_source);
                    if ptr_value != 0 {
                        self.maps.read_qword(ptr_value).unwrap_or(0)
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            "context" => {
                // context:context_name:param_name
                if parts.len() == 3 {
                    let context_name = parts[1];
                    let param_name = parts[2];
                    if let Some(context) = self.stored_contexts.get(context_name) {
                        *context.values.get(param_name).unwrap_or(&0)
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            _ => {
                // Direct register or other source
                self.get_parameter_value(source)
            }
        }
    }

    fn get_parameter_value(&self, source: &str) -> u64 {
        match source {
            "rcx" => self.regs().rcx,
            "rdx" => self.regs().rdx,
            "r8" => self.regs().r8,
            "r9" => self.regs().r9,
            "rax" => self.regs().rax,
            "rbx" => self.regs().rbx,
            "rsi" => self.regs().rsi,
            "rdi" => self.regs().rdi,
            "rsp" => self.regs().rsp,
            "rbp" => self.regs().rbp,
            _ => {
                // Try to parse as stack offset like "rsp+0x20"
                if source.starts_with("rsp+") {
                    if let Ok(offset) = u64::from_str_radix(&source[6..], 16) {
                        let addr = self.regs().rsp + offset;
                        self.maps.read_qword(addr).unwrap_or(0)
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
        }
    }

    fn format_parameter_value(&self, value: u64, param_type: &str) -> String {
        match param_type {
            "pointer" => format!("0x{:x}", value),
            "wide_string" => {
                if value != 0 {
                    let s = self.maps.read_wide_string(value);
                    if !s.is_empty() {
                        format!("L\"{}\" (0x{:x})", s, value)
                    } else {
                        format!("0x{:x}", value)
                    }
                } else {
                    "NULL".to_string()
                }
            }
            "int32" => format!("{} (0x{:x})", value as i32, value),
            _ => format!("0x{:x}", value),
        }
    }
}
