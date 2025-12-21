use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::emu::object_handle::hive_parser::{HiveParser, RegistryValue, HiveError};

// --- WinAPI Constants for Registry ---
pub const HKEY_CLASSES_ROOT: u32 = 0x80000000;
pub const HKEY_CURRENT_USER: u32 = 0x80000001;
pub const HKEY_LOCAL_MACHINE: u32 = 0x80000002;
pub const HKEY_USERS: u32 = 0x80000003;
pub const HKEY_PERFORMANCE_DATA: u32 = 0x80000004;
pub const HKEY_CURRENT_CONFIG: u32 = 0x80000005;
pub const HKEY_DYN_DATA: u32 = 0x80000006;

pub const REG_NONE: u32 = 0;
pub const REG_SZ: u32 = 1;
pub const REG_EXPAND_SZ: u32 = 2;
pub const REG_BINARY: u32 = 3;
pub const REG_DWORD: u32 = 4;
pub const REG_DWORD_LITTLE_ENDIAN: u32 = 4;
pub const REG_DWORD_BIG_ENDIAN: u32 = 5;
pub const REG_LINK: u32 = 6;
pub const REG_MULTI_SZ: u32 = 7;
pub const REG_RESOURCE_LIST: u32 = 8;
pub const REG_FULL_RESOURCE_DESCRIPTOR: u32 = 9;
pub const REG_RESOURCE_REQUIREMENTS_LIST: u32 = 10;
pub const REG_QWORD: u32 = 11;
pub const REG_QWORD_LITTLE_ENDIAN: u32 = 11;

// Registry access rights
pub const KEY_QUERY_VALUE: u32 = 0x0001;
pub const KEY_SET_VALUE: u32 = 0x0002;
pub const KEY_CREATE_SUB_KEY: u32 = 0x0004;
pub const KEY_ENUMERATE_SUB_KEYS: u32 = 0x0008;
pub const KEY_NOTIFY: u32 = 0x0010;
pub const KEY_CREATE_LINK: u32 = 0x0020;
pub const KEY_WOW64_64KEY: u32 = 0x0100;
pub const KEY_WOW64_32KEY: u32 = 0x0200;
pub const KEY_READ: u32 = 0x20019;
pub const KEY_WRITE: u32 = 0x20006;
pub const KEY_EXECUTE: u32 = 0x20019;
pub const KEY_ALL_ACCESS: u32 = 0xF003F;

// Registry open/create options
pub const REG_OPTION_NON_VOLATILE: u32 = 0x00000000;
pub const REG_OPTION_VOLATILE: u32 = 0x00000001;
pub const REG_OPTION_CREATE_LINK: u32 = 0x00000002;
pub const REG_OPTION_BACKUP_RESTORE: u32 = 0x00000004;
pub const REG_OPTION_OPEN_LINK: u32 = 0x00000008;

// Registry hive file paths
pub const SYSTEM_HIVE_PATH: &str = "C:\\Windows\\System32\\config\\SYSTEM";
pub const SOFTWARE_HIVE_PATH: &str = "C:\\Windows\\System32\\config\\SOFTWARE";
pub const SAM_HIVE_PATH: &str = "C:\\Windows\\System32\\config\\SAM";
pub const SECURITY_HIVE_PATH: &str = "C:\\Windows\\System32\\config\\SECURITY";

// Static cache for loaded registry hives
lazy_static::lazy_static! {
    static ref REGISTRY_HIVES: Arc<Mutex<HashMap<String, HiveParser>>> = {
        let mut map = HashMap::new();
        // Pre-load common hives (could be made lazy)
        Arc::new(Mutex::new(map))
    };
}

/// Represents the state and metadata associated with a Windows registry key handle
pub struct RegistryHandle {
    // --- Core Registry Key Info ---
    root_key: u32,                 // HKEY_* constant
    sub_key_path: String,          // Path to the subkey
    original_path: String,         // Original path used to open the key
    hive_path: Option<PathBuf>,    // Path to the hive file if using a hive file directly
    // --- Access Rights ---
    access_rights: u32,            // Access flags (KEY_READ, KEY_WRITE, etc.)
    // --- Handle State ---
    is_valid: bool,                // Whether the handle is valid
    is_root: bool,                 // Whether this is a root key handle
    // --- Cached Data (for performance) ---
    cached_values: Option<HashMap<String, RegistryValue>>,
    cached_subkeys: Option<Vec<String>>,
}

impl RegistryHandle {
    /// Creates a new RegistryHandle for a predefined root key
    pub fn new_root_key(root_key: u32, access_rights: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let path = match root_key {
            HKEY_LOCAL_MACHINE => "HKEY_LOCAL_MACHINE".to_string(),
            HKEY_CURRENT_USER => "HKEY_CURRENT_USER".to_string(),
            HKEY_CLASSES_ROOT => "HKEY_CLASSES_ROOT".to_string(),
            HKEY_USERS => "HKEY_USERS".to_string(),
            HKEY_CURRENT_CONFIG => "HKEY_CURRENT_CONFIG".to_string(),
            _ => format!("HKEY_0x{:08X}", root_key),
        };

        let hive_path = Self::get_hive_path_for_root(root_key);

        Ok(RegistryHandle {
            root_key,
            sub_key_path: String::new(),
            original_path: path.clone(),
            hive_path,
            access_rights,
            is_valid: true,
            is_root: true,
            cached_values: None,
            cached_subkeys: None,
        })
    }

    /// Creates a new RegistryHandle for a subkey
    pub fn new_subkey(
        root_key: u32,
        sub_key: &str,
        access_rights: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let full_path = match root_key {
            HKEY_LOCAL_MACHINE => format!("HKEY_LOCAL_MACHINE\\{}", sub_key),
            HKEY_CURRENT_USER => format!("HKEY_CURRENT_USER\\{}", sub_key),
            HKEY_CLASSES_ROOT => format!("HKEY_CLASSES_ROOT\\{}", sub_key),
            HKEY_USERS => format!("HKEY_USERS\\{}", sub_key),
            HKEY_CURRENT_CONFIG => format!("HKEY_CURRENT_CONFIG\\{}", sub_key),
            _ => format!("HKEY_0x{:08X}\\{}", root_key, sub_key),
        };

        let hive_path = Self::get_hive_path_for_root(root_key);

        Ok(RegistryHandle {
            root_key,
            sub_key_path: sub_key.to_string(),
            original_path: full_path,
            hive_path,
            access_rights,
            is_valid: true,
            is_root: false,
            cached_values: None,
            cached_subkeys: None,
        })
    }

    pub fn new_from_directory(
        hive_file_path: &str,
        key_path: &str,
        access_rights: u32
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(RegistryHandle {
            root_key: 0,
            sub_key_path: key_path.to_string(),
            original_path: format!("{}:{}", hive_file_path, key_path),
            hive_path: Some(resolved_path),
            access_rights,
            is_valid: true,
            is_root: key_path.is_empty(),
            cached_values: None,
            cached_subkeys: None,
        })
    }

    /// Creates a RegistryHandle directly from a hive file
    pub fn new_from_hive_file(
        hive_file_path: &str,
        key_path: &str,
        access_rights: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let resolved_path = crate::emu::object_handle::windows_to_emulate_path(hive_file_path);

        Ok(RegistryHandle {
            root_key: 0,
            sub_key_path: key_path.to_string(),
            original_path: format!("{}:{}", hive_file_path, key_path),
            hive_path: Some(resolved_path),
            access_rights,
            is_valid: true,
            is_root: key_path.is_empty(),
            cached_values: None,
            cached_subkeys: None,
        })
    }

    /// Gets the hive file path for a given root key
    fn get_hive_path_for_root(root_key: u32) -> Option<PathBuf> {
        match root_key {
            HKEY_LOCAL_MACHINE => {
                let path = crate::emu::object_handle::windows_to_emulate_path(SYSTEM_HIVE_PATH);
                Some(path)
            }
            HKEY_USERS => {
                // For now, use a default path. Could be more sophisticated
                let path = crate::emu::object_handle::windows_to_emulate_path("C:\\Windows\\System32\\config\\DEFAULT");
                Some(path)
            }
            _ => None, // Other keys might not map directly to hive files
        }
    }

    /// Gets or loads the hive parser for this handle
    fn get_hive_parser(&self) -> Result<HiveParser, HiveError> {
        if let Some(ref hive_path) = self.hive_path {
            let mut hives = REGISTRY_HIVES.lock().unwrap();
            let hive_key = hive_path.to_string_lossy().to_string();

            if !hives.contains_key(&hive_key) {
                let parser = HiveParser::from_file(hive_path)?;
                hives.insert(hive_key.clone(), parser);
            }

            // Clone the parser (this clones the file handle, which might be expensive)
            Ok(hives.get(&hive_key).unwrap().clone())
        } else {
            Err(HiveError::InvalidSignature) // No hive file associated
        }
    }

    /// Enumerates subkeys of the current key
    pub fn enumerate_subkeys(&mut self) -> Result<Vec<String>, HiveError> {
        if let Some(ref cached) = self.cached_subkeys {
            return Ok(cached.clone());
        }

        let mut parser = self.get_hive_parser()?;

        // Split the subkey path into components
        let path_parts: Vec<&str> = self.sub_key_path.split('\\').collect();

        if path_parts.is_empty() || self.is_root {
            // For root keys, enumerate the top-level keys in the hive
            let mut result = Vec::new();
            // This depends on how the hive parser is structured
            // For simplicity, let's assume the hive parser has a method to get root keys
            // Since the hive parser doesn't have this, we'll return empty for now
            self.cached_subkeys = Some(result.clone());
            return Ok(result);
        } else {
            // Navigate to the specific subkey
            let mut current_key = None;

            for (i, part) in path_parts.iter().enumerate() {
                let parent_path = path_parts[..i].join("\\");
                current_key = parser.get_subkey(part, &parent_path)?;
            }

            if let Some(mut key) = current_key {
                let subkeys = key.subkeys_list()?;
                self.cached_subkeys = Some(subkeys.clone());
                Ok(subkeys)
            } else {
                Err(HiveError::KeyNotFound)
            }
        }
    }

    /// Enumerates values of the current key
    pub fn enumerate_values(&mut self) -> Result<Vec<String>, HiveError> {
        let mut parser = self.get_hive_parser()?;

        if self.is_root || self.sub_key_path.is_empty() {
            // Root keys typically have no values
            return Ok(Vec::new());
        }

        let path_parts: Vec<&str> = self.sub_key_path.split('\\').collect();
        let mut current_key = None;

        for (i, part) in path_parts.iter().enumerate() {
            let parent_path = path_parts[..i].join("\\");
            current_key = parser.get_subkey(part, &parent_path)?;
        }

        if let Some(mut key) = current_key {
            key.keys_list()
        } else {
            Err(HiveError::KeyNotFound)
        }
    }

    /// Queries a value from the registry key
    pub fn query_value(&mut self, value_name: &str) -> Result<Option<RegistryValue>, HiveError> {
        let mut parser = self.get_hive_parser()?;

        if self.is_root || self.sub_key_path.is_empty() {
            return Ok(None);
        }

        let path_parts: Vec<&str> = self.sub_key_path.split('\\').collect();
        let mut current_key = None;

        for (i, part) in path_parts.iter().enumerate() {
            let parent_path = path_parts[..i].join("\\");
            current_key = parser.get_subkey(part, &parent_path)?;
        }

        if let Some(mut key) = current_key {
            // Try different value types
            if let Ok(Some(value)) = key.get_key_value::<String>(value_name) {
                return Ok(Some(RegistryValue::String(value)));
            }

            if let Ok(Some(value)) = key.get_key_value::<u32>(value_name) {
                return Ok(Some(RegistryValue::Dword(value)));
            }

            if let Ok(Some(value)) = key.get_key_value::<Vec<u8>>(value_name) {
                return Ok(Some(RegistryValue::Binary(value)));
            }

            if let Ok(Some(value)) = key.get_key_value::<Vec<String>>(value_name) {
                return Ok(Some(RegistryValue::MultiString(value)));
            }

            Ok(None)
        } else {
            Err(HiveError::KeyNotFound)
        }
    }

    /// Sets a value in the registry key (simplified - doesn't actually write to hive)
    pub fn set_value(&mut self, value_name: &str, value: RegistryValue) -> Result<(), HiveError> {
        // Note: This is a simplified implementation that doesn't actually write to the hive file.
        // Writing to registry hives is complex and requires understanding the binary format.
        // For emulation purposes, we might want to track changes in memory without modifying
        // the actual hive files.

        // For now, we'll just update the cache
        if self.cached_values.is_none() {
            self.cached_values = Some(HashMap::new());
        }

        if let Some(ref mut cache) = self.cached_values {
            cache.insert(value_name.to_string(), value);
        }

        Ok(())
    }

    /// Creates a subkey (simplified)
    pub fn create_subkey(&mut self, subkey_name: &str) -> Result<RegistryHandle, Box<dyn std::error::Error>> {
        let new_path = if self.sub_key_path.is_empty() {
            subkey_name.to_string()
        } else {
            format!("{}\\{}", self.sub_key_path, subkey_name)
        };

        RegistryHandle::new_subkey(self.root_key, &new_path, self.access_rights)
    }

    /// Closes the registry handle
    pub fn close(&mut self) {
        self.is_valid = false;
        // Clear cache to free memory
        self.cached_values = None;
        self.cached_subkeys = None;
    }

    /// Getter methods
    pub fn is_valid(&self) -> bool { self.is_valid }
    pub fn is_root(&self) -> bool { self.is_root }
    pub fn get_path(&self) -> &str { &self.original_path }
    pub fn get_root_key(&self) -> u32 { self.root_key }
    pub fn get_subkey_path(&self) -> &str { &self.sub_key_path }
    pub fn get_access_rights(&self) -> u32 { self.access_rights }
}

// Implement Clone for HiveParser if needed (or use Arc internally)
// For now, we'll add a Clone implementation to HiveParser in hive_parser.rs
// If you can't modify hive_parser.rs, you'll need to use Arc<Mutex<HiveParser>> instead

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_key_creation() {
        let handle = RegistryHandle::new_root_key(HKEY_LOCAL_MACHINE, KEY_READ).unwrap();
        assert!(handle.is_valid());
        assert!(handle.is_root());
        assert_eq!(handle.get_root_key(), HKEY_LOCAL_MACHINE);
        assert_eq!(handle.get_path(), "HKEY_LOCAL_MACHINE");
    }

    #[test]
    fn test_subkey_creation() {
        let handle = RegistryHandle::new_subkey(
            HKEY_LOCAL_MACHINE,
            "Software\\Microsoft\\Windows",
            KEY_READ,
        ).unwrap();

        assert!(handle.is_valid());
        assert!(!handle.is_root());
        assert_eq!(handle.get_subkey_path(), "Software\\Microsoft\\Windows");
    }
}