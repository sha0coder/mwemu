use serde::{Deserialize, Serialize};

use crate::loaders::pe::pe64::PE64;
use crate::loaders::pe::runtime_pe64::RuntimePe64;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SerializablePe64Backend {
    Legacy,
    Lief,
}

impl Default for SerializablePe64Backend {
    fn default() -> Self {
        SerializablePe64Backend::Legacy
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializablePE64 {
    pub filename: String,
    pub raw: Vec<u8>,
    #[serde(default)]
    pub backend: Option<SerializablePe64Backend>,
}

impl From<PE64> for SerializablePE64 {
    fn from(pe64: PE64) -> Self {
        SerializablePE64 {
            filename: pe64.filename,
            raw: pe64.raw,
            backend: Some(SerializablePe64Backend::Legacy),
        }
    }
}

impl From<&PE64> for SerializablePE64 {
    fn from(pe64: &PE64) -> Self {
        SerializablePE64 {
            filename: pe64.filename.clone(),
            raw: pe64.raw.clone(),
            backend: Some(SerializablePe64Backend::Legacy),
        }
    }
}

impl From<SerializablePE64> for PE64 {
    fn from(serialized: SerializablePE64) -> Self {
        PE64::load_from_raw(&serialized.filename, &serialized.raw)
    }
}

impl From<&RuntimePe64> for SerializablePE64 {
    fn from(runtime_pe: &RuntimePe64) -> Self {
        match runtime_pe {
            RuntimePe64::Legacy(pe) => pe.into(),
            RuntimePe64::Lief(lief_pe) => {
                let raw = lief_pe.mapped_file_data().to_vec();
                SerializablePE64 {
                    filename: lief_pe.file_path().to_string(),
                    raw,
                    backend: Some(SerializablePe64Backend::Lief),
                }
            }
        }
    }
}

impl From<RuntimePe64> for SerializablePE64 {
    fn from(runtime_pe: RuntimePe64) -> Self {
        match runtime_pe {
            RuntimePe64::Legacy(pe) => pe.into(),
            RuntimePe64::Lief(lief_pe) => {
                let raw = lief_pe.mapped_file_data().to_vec();
                SerializablePE64 {
                    filename: lief_pe.file_path().to_string(),
                    raw,
                    backend: Some(SerializablePe64Backend::Lief),
                }
            }
        }
    }
}

impl From<SerializablePE64> for RuntimePe64 {
    fn from(serialized: SerializablePE64) -> Self {
        match serialized.backend.unwrap_or_default() {
            SerializablePe64Backend::Lief => {
                match crate::loaders::pe::lief::LiefPe::load_from_raw(
                    &serialized.filename,
                    &serialized.raw,
                ) {
                    Ok(lief_pe) => RuntimePe64::Lief(lief_pe),
                    Err(e) => {
                        log::warn!(
                            "LIEF deserialization failed for {}, falling back to legacy: {}",
                            serialized.filename,
                            e
                        );
                        RuntimePe64::Legacy(PE64::load_from_raw(
                            &serialized.filename,
                            &serialized.raw,
                        ))
                    }
                }
            }
            SerializablePe64Backend::Legacy => {
                RuntimePe64::Legacy(PE64::load_from_raw(&serialized.filename, &serialized.raw))
            }
        }
    }
}
