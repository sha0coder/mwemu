use serde::{Deserialize, Serialize};

use crate::maps::Maps;

#[derive(Serialize, Deserialize)]
pub struct SerializableMaps {
    maps: Maps,
}

impl From<Maps> for SerializableMaps {
    fn from(maps: Maps) -> Self {
        SerializableMaps { maps }
    }
}

impl From<&Maps> for SerializableMaps {
    fn from(maps: &Maps) -> Self {
        SerializableMaps { maps: maps.clone() }
    }
}

impl From<SerializableMaps> for Maps {
    fn from(serialized: SerializableMaps) -> Self {
        serialized.maps
    }
}

impl Default for SerializableMaps {
    fn default() -> Self {
        Self {
            maps: Default::default(),
        }
    }
}

impl SerializableMaps {
    pub fn new(maps: Maps) -> Self {
        Self { maps }
    }
}
