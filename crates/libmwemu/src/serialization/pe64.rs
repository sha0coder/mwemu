use serde::{Deserialize, Serialize};

use crate::pe::lief::lief_pe::LiefPe;
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

impl From<&LiefPe> for SerializablePE64 {
    fn from(lief_pe: &LiefPe) -> Self {
        SerializablePE64 {
            filename: lief_pe.file_path().to_string(),
            raw: lief_pe.mapped_file_data().to_vec(),
        }
    }
}

impl From<SerializablePE64> for LiefPe {
    fn from(_serialized: SerializablePE64) -> Self {
        // Deserialization from SerializablePE64 to LiefPe is not directly supported
        // because LiefPe requires a file path to load from disk.
        // This would require significant architectural changes to support.
        unimplemented!(
            "Deserialization of LiefPe from SerializablePE64 is not supported. \
            LiefPe requires a file path to load from disk."
        );
    }
}
