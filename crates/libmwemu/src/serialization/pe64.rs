use serde::{Deserialize, Serialize};

use crate::loaders::pe::lief::LiefPe;
use crate::loaders::pe::lief::error::LiefError;
use crate::loaders::pe::runtime_pe64::RuntimePe64;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SerializablePe64Backend {
    /// Legacy backend is no longer used at runtime but kept in this enum
    /// for backwards-compatible deserialization of old dump files. New
    /// dumps are always tagged `Lief`. The `Legacy` variant is parsed
    /// for compatibility but is otherwise treated identically to `Lief`
    /// at the runtime layer — LIEF is the only backend.
    Legacy,
    Lief,
}

impl Default for SerializablePe64Backend {
    fn default() -> Self {
        SerializablePe64Backend::Lief
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializablePE64 {
    pub filename: String,
    pub raw: Vec<u8>,
    #[serde(default)]
    pub backend: Option<SerializablePe64Backend>,
}

impl From<&RuntimePe64> for SerializablePE64 {
    fn from(runtime_pe: &RuntimePe64) -> Self {
        let lief_pe = runtime_pe.as_lief();
        SerializablePE64 {
            filename: lief_pe.file_path().to_string(),
            raw: lief_pe.persistent_raw().to_vec(),
            backend: Some(SerializablePe64Backend::Lief),
        }
    }
}

impl From<RuntimePe64> for SerializablePE64 {
    fn from(runtime_pe: RuntimePe64) -> Self {
        SerializablePE64::from(&runtime_pe)
    }
}

/// Deserialize a `SerializablePE64` back into a `RuntimePe64`.
///
/// The conversion is fallible: it requires LIEF to be able to parse the
/// embedded raw bytes and to confirm the parsed binary is a 64-bit PE.
/// Bytes that LIEF rejects (synthetic, truncated, or 32-bit mislabeled
/// payloads) surface as `Err(LiefError)`. Callers that cannot tolerate a
/// parse failure (e.g. the minidump restore path) must handle the error
/// explicitly rather than silently substituting a fabricated runtime.
impl TryFrom<SerializablePE64> for RuntimePe64 {
    type Error = LiefError;

    fn try_from(serialized: SerializablePE64) -> Result<Self, Self::Error> {
        let lief_pe = LiefPe::load_from_raw(&serialized.filename, &serialized.raw)?;
        if !lief_pe.is_pe64() {
            return Err(LiefError::ParseFailed(format!(
                "Serialized bytes are not PE64: {}",
                serialized.filename
            )));
        }
        Ok(RuntimePe64::from_inner(lief_pe))
    }
}
