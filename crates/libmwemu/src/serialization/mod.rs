use std::{fs::File, io::Write as _};

use crate::emu::Emu;
use crate::serialization::emu::SerializableEmu;
use crate::serialization::minidump_converter::MinidumpConverter;

mod emu;
mod fpu;
mod instant;
mod maps;
mod minidump_converter;
mod pe32;
mod pe64;
mod thread_context;

pub struct Serialization {}

impl Serialization {
    pub fn serialize(emu: &Emu) -> Vec<u8> {
        let serialized = SerializableEmu::from(emu);
        bitcode::serialize(&serialized).unwrap()
    }

    pub fn deserialize(data: &[u8]) -> Emu {
        let deserialized: SerializableEmu = bitcode::deserialize(data).unwrap();
        deserialized.into()
    }

    pub fn dump_to_file(emu: &Emu, filename: &str) {
        std::fs::create_dir_all("./dumps/").unwrap();

        let serialized = SerializableEmu::from(emu);
        let data = bitcode::serialize(&serialized).unwrap();
        let mut file = File::create(filename).unwrap();
        file.write_all(&data).unwrap();
        file.flush().unwrap();
        drop(file);

        // for binary analysis
        emu.maps.save_all("./dumps".to_string());
    }

    pub fn load_from_file(filename: &str) -> Emu {
        let data = std::fs::read(filename).unwrap();
        Self::deserialize(&data)
    }

    pub fn load_from_minidump(filename: &str) -> Emu {
        let serializable = MinidumpConverter::from_minidump_file(filename).unwrap();
        serializable.into()
    }
}
