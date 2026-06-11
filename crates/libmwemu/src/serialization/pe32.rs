use serde::{Deserialize, Serialize};

use crate::loaders::pe::lief::LiefPe;
use crate::loaders::pe::lief::error::LiefError;
use crate::loaders::pe::runtime_pe32::RuntimePe32;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SerializablePe32Backend {
    /// Legacy backend is no longer used at runtime but kept in this enum
    /// for backwards-compatible deserialization of old dump files. New
    /// dumps are always tagged `Lief`. The `Legacy` variant is parsed
    /// for compatibility but is otherwise treated identically to `Lief`
    /// at the runtime layer — LIEF is the only backend.
    Legacy,
    Lief,
}

impl Default for SerializablePe32Backend {
    fn default() -> Self {
        SerializablePe32Backend::Lief
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializablePE32 {
    pub filename: String,
    pub raw: Vec<u8>,
    #[serde(default)]
    pub backend: Option<SerializablePe32Backend>,
}

impl From<&RuntimePe32> for SerializablePE32 {
    fn from(runtime_pe: &RuntimePe32) -> Self {
        let lief_pe = runtime_pe.as_lief();
        SerializablePE32 {
            filename: lief_pe.file_path().to_string(),
            raw: lief_pe.persistent_raw().to_vec(),
            backend: Some(SerializablePe32Backend::Lief),
        }
    }
}

impl From<RuntimePe32> for SerializablePE32 {
    fn from(runtime_pe: RuntimePe32) -> Self {
        SerializablePE32::from(&runtime_pe)
    }
}

/// Deserialize a `SerializablePE32` back into a `RuntimePe32`.
///
/// The conversion is fallible: it requires LIEF to be able to parse the
/// embedded raw bytes and to confirm the parsed binary is a 32-bit PE.
/// Bytes that LIEF rejects (synthetic, truncated, or 64-bit mislabeled
/// payloads) surface as `Err(LiefError)`. Callers that cannot tolerate a
/// parse failure (e.g. the minidump restore path) must handle the error
/// explicitly rather than silently substituting a fabricated runtime.
impl TryFrom<SerializablePE32> for RuntimePe32 {
    type Error = LiefError;

    fn try_from(serialized: SerializablePE32) -> Result<Self, Self::Error> {
        let lief_pe = LiefPe::load_from_raw(&serialized.filename, &serialized.raw)?;
        if !lief_pe.is_pe32() {
            return Err(LiefError::ParseFailed(format!(
                "Serialized bytes are not PE32: {}",
                serialized.filename
            )));
        }
        Ok(RuntimePe32::from_inner(lief_pe))
    }
}
