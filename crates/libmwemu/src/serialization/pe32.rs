use serde::{Deserialize, Serialize};

use crate::loaders::pe::lief::LiefPe;
use crate::loaders::pe::pe32::PE32;
use crate::loaders::pe::runtime_pe32::RuntimePe32;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SerializablePe32Backend {
    Legacy,
    Lief,
}

impl Default for SerializablePe32Backend {
    fn default() -> Self {
        SerializablePe32Backend::Legacy
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializablePE32 {
    pub filename: String,
    pub raw: Vec<u8>,
    #[serde(default)]
    pub backend: Option<SerializablePe32Backend>,
}

impl From<PE32> for SerializablePE32 {
    fn from(pe32: PE32) -> Self {
        SerializablePE32 {
            filename: pe32.filename,
            raw: pe32.raw,
            backend: Some(SerializablePe32Backend::Legacy),
        }
    }
}

impl From<&PE32> for SerializablePE32 {
    fn from(pe32: &PE32) -> Self {
        SerializablePE32 {
            filename: pe32.filename.clone(),
            raw: pe32.raw.clone(),
            backend: Some(SerializablePe32Backend::Legacy),
        }
    }
}

impl From<SerializablePE32> for PE32 {
    fn from(serialized: SerializablePE32) -> Self {
        PE32::load_from_raw(&serialized.filename, &serialized.raw)
    }
}

impl From<&RuntimePe32> for SerializablePE32 {
    fn from(runtime_pe: &RuntimePe32) -> Self {
        match runtime_pe {
            RuntimePe32::Legacy(pe) => pe.into(),
            RuntimePe32::Lief(lief_pe) => {
                let raw = lief_pe.mapped_file_data().to_vec();
                SerializablePE32 {
                    filename: lief_pe.file_path().to_string(),
                    raw,
                    backend: Some(SerializablePe32Backend::Lief),
                }
            }
        }
    }
}

impl From<RuntimePe32> for SerializablePE32 {
    fn from(runtime_pe: RuntimePe32) -> Self {
        match runtime_pe {
            RuntimePe32::Legacy(pe) => pe.into(),
            RuntimePe32::Lief(lief_pe) => {
                let raw = lief_pe.mapped_file_data().to_vec();
                SerializablePE32 {
                    filename: lief_pe.file_path().to_string(),
                    raw,
                    backend: Some(SerializablePe32Backend::Lief),
                }
            }
        }
    }
}

impl From<SerializablePE32> for RuntimePe32 {
    fn from(serialized: SerializablePE32) -> Self {
        match serialized.backend.unwrap_or_default() {
            SerializablePe32Backend::Lief => {
                match LiefPe::load_from_raw(&serialized.filename, &serialized.raw) {
                    Ok(lief_pe) => RuntimePe32::Lief(lief_pe),
                    Err(e) => {
                        log::warn!(
                            "LIEF deserialization failed for {}, falling back to legacy: {}",
                            serialized.filename,
                            e
                        );
                        RuntimePe32::Legacy(PE32::load_from_raw(
                            &serialized.filename,
                            &serialized.raw,
                        ))
                    }
                }
            }
            SerializablePe32Backend::Legacy => {
                RuntimePe32::Legacy(PE32::load_from_raw(&serialized.filename, &serialized.raw))
            }
        }
    }
}
