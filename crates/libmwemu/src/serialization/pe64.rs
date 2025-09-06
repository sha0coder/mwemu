use serde::{Deserialize, Serialize};

use crate::pe::pe64::PE64;

#[derive(Serialize, Deserialize)]
pub struct SerializablePE64 {
    pub filename: String,
    pub raw: Vec<u8>,
}

impl From<PE64> for SerializablePE64 {
    fn from(pe64: PE64) -> Self {
        SerializablePE64 {
            filename: pe64.filename,
            raw: pe64.raw,
        }
    }
}

impl From<&PE64> for SerializablePE64 {
    fn from(pe64: &PE64) -> Self {
        SerializablePE64 {
            filename: pe64.filename.clone(),
            raw: pe64.raw.clone(),
        }
    }
}

impl From<SerializablePE64> for PE64 {
    fn from(serialized: SerializablePE64) -> Self {
        PE64::load_from_raw(&serialized.filename, &serialized.raw)
    }
}
