use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use byteorder::{LittleEndian, ReadBytesExt};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HiveError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid hive signature")]
    InvalidSignature,
    #[error("File too small")]
    FileTooSmall,
    #[error("Invalid block type")]
    InvalidBlockType,
    #[error("Key not found")]
    KeyNotFound,
    #[error("Value not found")]
    ValueNotFound,
    #[error("Invalid value type")]
    InvalidValueType,
    #[error("Name buffer overflow")]
    NameBufferOverflow,
}

#[derive(Debug, Clone, Copy)]
struct Offsets {
    block_size: i32,
    block_type: [u8; 2],
    count: i16,
    first: i32,
    hash: i32,
}

#[derive(Debug, Clone, Copy)]
struct KeyBlock {
    block_size: i32,
    block_type: [u8; 2],
    subkey_count: i32,
    subkeys_offset: i32,
    value_count: i32,
    offsets_offset: i32,
    name_len: i16,
    name: [u8; 255],
}

#[derive(Debug, Clone, Copy)]
struct ValueBlock {
    block_size: i32,
    block_type: [u8; 2],
    name_len: i16,
    size: i32,
    data_offset: i32,
    value_type: i32,
    name: [u8; 255],
}

pub enum RegistryValue {
    String(String),
    Dword(u32),
    Binary(Vec<u8>),
    MultiString(Vec<String>),
}

pub struct HiveKey<'a> {
    key_block: KeyBlock,
    base_offset: u64,
    file: &'a mut File,
}

#[derive(Debug, Clone)]
struct HiveSubpath {
    path: String,
    key_offset: u64,
}

#[derive(Debug, Clone)]
struct HiveCache {
    main_key_offset: u64,
    subpaths: Vec<HiveSubpath>,
}

pub struct HiveParser {
    file: File,  // Own the file instead of referencing it
    base_offset: u64,
    subkey_cache: HashMap<String, HiveCache>,
}

impl Offsets {
    fn read_from_file(file: &mut File, offset: u64) -> Result<Self, HiveError> {
        file.seek(SeekFrom::Start(offset))?;

        let block_size = file.read_i32::<LittleEndian>()?;
        let block_type = [file.read_u8()?, file.read_u8()?];
        let count = file.read_i16::<LittleEndian>()?;
        let first = file.read_i32::<LittleEndian>()?;
        let hash = file.read_i32::<LittleEndian>()?;

        Ok(Self {
            block_size,
            block_type,
            count,
            first,
            hash,
        })
    }
}

impl KeyBlock {
    fn read_from_file(file: &mut File, offset: u64) -> Result<Self, HiveError> {
        file.seek(SeekFrom::Start(offset))?;

        let block_size = file.read_i32::<LittleEndian>()?;
        let block_type = [file.read_u8()?, file.read_u8()?];

        // Skip dummy data (18 bytes)
        let mut dummy = [0u8; 18];
        file.read_exact(&mut dummy)?;

        let subkey_count = file.read_i32::<LittleEndian>()?;

        // Skip dummy data (4 bytes)
        let mut dummy = [0u8; 4];
        file.read_exact(&mut dummy)?;

        let subkeys_offset = file.read_i32::<LittleEndian>()?;

        // Skip dummy data (4 bytes)
        let mut dummy = [0u8; 4];
        file.read_exact(&mut dummy)?;

        let value_count = file.read_i32::<LittleEndian>()?;
        let offsets_offset = file.read_i32::<LittleEndian>()?;

        // Skip dummy data (28 bytes)
        let mut dummy = [0u8; 28];
        file.read_exact(&mut dummy)?;

        let name_len = file.read_i16::<LittleEndian>()?;

        // Skip dummy data (2 bytes)
        let mut dummy = [0u8; 2];
        file.read_exact(&mut dummy)?;

        let mut name = [0u8; 255];
        file.read_exact(&mut name)?;

        Ok(Self {
            block_size,
            block_type,
            subkey_count,
            subkeys_offset,
            value_count,
            offsets_offset,
            name_len,
            name,
        })
    }

    fn get_name(&self) -> Result<String, HiveError> {
        if self.name_len <= 0 || self.name_len as usize > self.name.len() {
            return Err(HiveError::NameBufferOverflow);
        }

        let name_str = String::from_utf8_lossy(&self.name[..self.name_len as usize]).into_owned();
        Ok(name_str)
    }
}

impl ValueBlock {
    fn read_from_file(file: &mut File, offset: u64) -> Result<Self, HiveError> {
        file.seek(SeekFrom::Start(offset))?;

        let block_size = file.read_i32::<LittleEndian>()?;
        let block_type = [file.read_u8()?, file.read_u8()?];
        let name_len = file.read_i16::<LittleEndian>()?;
        let size = file.read_i32::<LittleEndian>()?;
        let data_offset = file.read_i32::<LittleEndian>()?;
        let value_type = file.read_i32::<LittleEndian>()?;

        // Skip flags and dummy (4 bytes)
        let mut dummy = [0u8; 4];
        file.read_exact(&mut dummy)?;

        let mut name = [0u8; 255];
        file.read_exact(&mut name)?;

        Ok(Self {
            block_size,
            block_type,
            name_len,
            size,
            data_offset,
            value_type,
            name,
        })
    }

    fn get_name(&self) -> Result<String, HiveError> {
        if self.name_len <= 0 || self.name_len as usize > self.name.len() {
            return Err(HiveError::NameBufferOverflow);
        }

        let name_str = String::from_utf8_lossy(&self.name[..self.name_len as usize]).into_owned();
        Ok(name_str)
    }
}

impl<'a> HiveKey<'a> {
    pub fn new(key_block: KeyBlock, base_offset: u64, file: &'a mut File) -> Self {
        Self {
            key_block,
            base_offset,
            file,
        }
    }

    pub fn subkeys_list(&mut self) -> Result<Vec<String>, HiveError> {
        let offsets_offset = self.base_offset + self.key_block.subkeys_offset as u64;
        let offsets = Offsets::read_from_file(self.file, offsets_offset)?;

        // Check block type ('f' or 'h')
        if offsets.block_type[1] != b'f' && offsets.block_type[1] != b'h' {
            return Err(HiveError::InvalidBlockType);
        }

        let mut result = Vec::with_capacity(self.key_block.subkey_count as usize);

        for i in 0..self.key_block.subkey_count {
            // Read the offset from the offsets table
            let offset_entry_offset = offsets_offset + 16 + (i as u64 * 8); // 16 = size of Offsets struct before 'first'
            self.file.seek(SeekFrom::Start(offset_entry_offset))?;
            let subkey_offset = self.file.read_i32::<LittleEndian>()? as u64;

            if subkey_offset == 0 {
                continue;
            }

            let subkey_abs_offset = self.base_offset + subkey_offset as u64;
            let subkey = KeyBlock::read_from_file(self.file, subkey_abs_offset)?;

            let name = subkey.get_name()?;
            result.push(name);
        }

        Ok(result)
    }

    pub fn keys_list(&mut self) -> Result<Vec<String>, HiveError> {
        if self.key_block.value_count == 0 {
            return Ok(Vec::new());
        }

        let mut result = Vec::with_capacity(self.key_block.value_count as usize);

        // Read value offsets
        let offsets_base = self.base_offset + self.key_block.offsets_offset as u64 + 4;

        for i in 0..self.key_block.value_count {
            let offset_entry_offset = offsets_base + (i as u64 * 4);
            self.file.seek(SeekFrom::Start(offset_entry_offset))?;
            let value_offset = self.file.read_i32::<LittleEndian>()? as u64;

            if value_offset == 0 {
                continue;
            }

            let value_abs_offset = self.base_offset + value_offset as u64;
            let value = ValueBlock::read_from_file(self.file, value_abs_offset)?;

            let name = value.get_name()?;
            result.push(name);
        }

        Ok(result)
    }

    pub fn get_key_value<T>(&mut self, name: &str) -> Result<Option<T>, HiveError>
    where
        T: TryFrom<RegistryValue, Error = HiveError>,
    {
        if self.key_block.value_count == 0 {
            return Ok(None);
        }

        let offsets_base = self.base_offset + self.key_block.offsets_offset as u64 + 4;

        for i in 0..self.key_block.value_count {
            let offset_entry_offset = offsets_base + (i as u64 * 4);
            self.file.seek(SeekFrom::Start(offset_entry_offset))?;
            let value_offset = self.file.read_i32::<LittleEndian>()? as u64;

            if value_offset == 0 {
                continue;
            }

            let value_abs_offset = self.base_offset + value_offset as u64;
            let value = ValueBlock::read_from_file(self.file, value_abs_offset)?;

            let value_name = value.get_name()?;
            if value_name != name {
                continue;
            }

            // Read the actual value data
            let registry_value = self.read_value_data(&value)?;

            return match T::try_from(registry_value) {
                Ok(val) => Ok(Some(val)),
                Err(_) => Err(HiveError::InvalidValueType),
            };
        }

        Ok(None)
    }

    fn read_value_data(&mut self, value: &ValueBlock) -> Result<RegistryValue, HiveError> {
        let data_size = value.size & 0xFFFF;
        let is_inline = (value.size & (1 << 31)) != 0;

        if is_inline {
            // Inline data - stored directly in the value block
            let mut buffer = vec![0u8; data_size as usize];
            // The inline data is stored in the data_offset field itself
            let inline_data = value.data_offset.to_le_bytes();
            let copy_len = std::cmp::min(inline_data.len(), buffer.len());
            buffer[..copy_len].copy_from_slice(&inline_data[..copy_len]);
            return Ok(RegistryValue::Binary(buffer));
        }

        // External data - stored at the data offset
        let data_offset = self.base_offset + value.data_offset as u64 + 4;
        self.file.seek(SeekFrom::Start(data_offset))?;

        match value.value_type {
            1 | 2 => { // REG_SZ, REG_EXPAND_SZ
                let mut buffer = vec![0u8; data_size as usize];
                self.file.read_exact(&mut buffer)?;
                let text = String::from_utf8_lossy(&buffer).into_owned();
                Ok(RegistryValue::String(text))
            }
            3 => { // REG_BINARY
                let mut buffer = vec![0u8; data_size as usize];
                self.file.read_exact(&mut buffer)?;
                Ok(RegistryValue::Binary(buffer))
            }
            4 => { // REG_DWORD
                let dword = self.file.read_u32::<LittleEndian>()?;
                Ok(RegistryValue::Dword(dword))
            }
            7 => { // REG_MULTI_SZ
                let mut buffer = vec![0u8; data_size as usize];
                self.file.read_exact(&mut buffer)?;

                let mut strings = Vec::new();
                let mut current = String::new();

                for &byte in &buffer {
                    if byte == 0 {
                        if !current.is_empty() {
                            strings.push(current.clone());
                            current.clear();
                        }
                    } else {
                        current.push(byte as char);
                    }
                }

                if !current.is_empty() {
                    strings.push(current);
                }

                Ok(RegistryValue::MultiString(strings))
            }
            _ => Err(HiveError::InvalidValueType),
        }
    }
}

impl HiveParser {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, HiveError> {
        let mut file = File::open(path)?;

        // Check signature
        let mut signature = [0u8; 4];
        file.read_exact(&mut signature)?;
        if &signature != b"regf" {
            return Err(HiveError::InvalidSignature);
        }

        // Base offset is 0x1000 (4096) for the hive data
        let base_offset = 0x1000;

        // Main key block is at 0x1020 from the start of the file
        let main_key_offset = base_offset + 0x20;
        let main_key = KeyBlock::read_from_file(&mut file, main_key_offset)?;

        let mut parser = Self {
            file,  // Now we move the file into the struct
            base_offset,
            subkey_cache: HashMap::new(),
        };

        parser.build_cache("", main_key_offset)?;
        Ok(parser)
    }

    fn build_cache(&mut self, current_path: &str, key_offset: u64) -> Result<(), HiveError> {
        let key = KeyBlock::read_from_file(&mut self.file, key_offset)?;

        if key.subkey_count == 0 {
            return Ok(());
        }

        let offsets_offset = self.base_offset + key.subkeys_offset as u64;
        let offsets = Offsets::read_from_file(&mut self.file, offsets_offset)?;

        if offsets.block_type[1] != b'f' && offsets.block_type[1] != b'h' {
            return Err(HiveError::InvalidBlockType);
        }

        let mut subpaths = Vec::new();

        for i in 0..key.subkey_count {
            let offset_entry_offset = offsets_offset + 16 + (i as u64 * 8);
            self.file.seek(SeekFrom::Start(offset_entry_offset))?;
            let subkey_offset = self.file.read_i32::<LittleEndian>()? as u64;

            if subkey_offset == 0 {
                continue;
            }

            let subkey_abs_offset = self.base_offset + subkey_offset as u64;
            let subkey = KeyBlock::read_from_file(&mut self.file, subkey_abs_offset)?;

            let subkey_name = subkey.get_name()?;
            let full_path = if current_path.is_empty() {
                subkey_name.clone()
            } else {
                format!("{}/{}", current_path, subkey_name)
            };

            // For first-level keys, create cache entries
            if current_path.is_empty() {
                self.subkey_cache.entry(subkey_name.clone()).or_insert_with(|| HiveCache {
                    main_key_offset: subkey_abs_offset,
                    subpaths: Vec::new(),
                });
            }

            subpaths.push(HiveSubpath {
                path: full_path.clone(),
                key_offset: subkey_abs_offset,
            });

            // Recursively process subkeys
            if subkey.subkey_count > 0 {
                self.build_cache(&full_path, subkey_abs_offset)?;
            }
        }

        // Add subpaths to parent cache if this is not the root
        if !current_path.is_empty() {
            if let Some(cache) = self.subkey_cache.get_mut(current_path) {
                cache.subpaths.extend(subpaths);
            }
        }

        Ok(())
    }

    pub fn get_subkey(&mut self, key_name: &str, path: &str) -> Result<Option<HiveKey>, HiveError> {
        if let Some(cache) = self.subkey_cache.get(key_name) {
            for subpath in &cache.subpaths {
                if subpath.path == path {
                    let key_block = KeyBlock::read_from_file(&mut self.file, subpath.key_offset)?;
                    return Ok(Some(HiveKey::new(key_block, self.base_offset, &mut self.file)));
                }
            }
        }
        Ok(None)
    }

    pub fn success(&self) -> bool {
        !self.subkey_cache.is_empty()
    }
}

// Conversion traits for convenience
impl TryFrom<RegistryValue> for String {
    type Error = HiveError;

    fn try_from(value: RegistryValue) -> Result<Self, Self::Error> {
        match value {
            RegistryValue::String(s) => Ok(s),
            _ => Err(HiveError::InvalidValueType),
        }
    }
}

impl TryFrom<RegistryValue> for u32 {
    type Error = HiveError;

    fn try_from(value: RegistryValue) -> Result<Self, Self::Error> {
        match value {
            RegistryValue::Dword(d) => Ok(d),
            _ => Err(HiveError::InvalidValueType),
        }
    }
}

impl TryFrom<RegistryValue> for Vec<u8> {
    type Error = HiveError;

    fn try_from(value: RegistryValue) -> Result<Self, Self::Error> {
        match value {
            RegistryValue::Binary(b) => Ok(b),
            _ => Err(HiveError::InvalidValueType),
        }
    }
}

impl TryFrom<RegistryValue> for Vec<String> {
    type Error = HiveError;

    fn try_from(value: RegistryValue) -> Result<Self, Self::Error> {
        match value {
            RegistryValue::MultiString(s) => Ok(s),
            _ => Err(HiveError::InvalidValueType),
        }
    }
}

