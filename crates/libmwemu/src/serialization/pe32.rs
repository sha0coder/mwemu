use serde::{Deserialize, Serialize};

use crate::pe::pe32::PE32;

#[derive(Serialize, Deserialize)]
pub struct SerializablePE32 {
    pub filename: String,
    pub raw: Vec<u8>,
}

impl From<PE32> for SerializablePE32 {
    fn from(pe32: PE32) -> Self {
        SerializablePE32 {
            filename: pe32.filename,
            raw: pe32.raw,
        }
    }
}

impl From<&PE32> for SerializablePE32 {
    fn from(pe32: &PE32) -> Self {
        SerializablePE32 {
            filename: pe32.filename.clone(),
            raw: pe32.raw.clone(),
        }
    }
}

impl From<SerializablePE32> for PE32 {
    fn from(serialized: SerializablePE32) -> Self {
        PE32::load_from_raw(&serialized.filename, &serialized.raw)
    }
}
