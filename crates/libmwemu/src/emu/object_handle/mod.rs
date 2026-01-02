pub(crate) use file_handle::FileHandle;
pub(crate) use mapping_handle::MappingHandle;
use slab::Slab;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub mod file_handle;
mod hive_parser;
pub mod mapping_handle;
mod registry_handle;
// TODO: support more handle: registry, thread, heap, etc
/*
Here the handle management is control by Slab and return a number, that number can be used as
handle id to get the right handle. In the document, it doesn't specific that the handle need to be divided by 4.

*/

pub static HANDLE_MANGEMENT: std::sync::LazyLock<
    Arc<Mutex<crate::emu::object_handle::HandleManagement>>,
> = std::sync::LazyLock::new(|| {
    Arc::new(Mutex::new(
        crate::emu::object_handle::HandleManagement::new(),
    ))
});
enum HandleType {
    FileHandle(FileHandle),
    MappingHandle(MappingHandle),
}

pub(crate) struct HandleManagement {
    number_of_handle: usize,
    handle_types: Slab<HandleType>,
}

impl HandleManagement {
    pub fn new() -> Self {
        Self {
            handle_types: Slab::with_capacity(200),
            number_of_handle: 0,
        }
    }

    pub fn insert_file_handle(&mut self, file_handle: FileHandle) -> u32 {
        let key = self
            .handle_types
            .insert(HandleType::FileHandle(file_handle));
        self.number_of_handle += 1;
        key as u32 // Assuming u32 is sufficient for slab keys in your context
    }

    pub fn insert_mapping_handle(&mut self, mapping_handle: MappingHandle) -> u32 {
        let key = self
            .handle_types
            .insert(HandleType::MappingHandle(mapping_handle));
        self.number_of_handle += 1;
        key as u32
    }

    // Method to get a mutable reference to a FileHandle by its key
    pub fn get_mut_file_handle(&mut self, key: u32) -> Option<&mut FileHandle> {
        if let Some(handle_type) = self.handle_types.get_mut(key as usize) {
            match handle_type {
                HandleType::FileHandle(fh) => Some(fh),
                // Add other handle type matches if/when they are implemented
                _ => None, // Handle exists but is not a FileHandle
            }
        } else {
            None // Handle key does not exist
        }
    }

    pub fn get_mut_mapping_handle(&mut self, key: u32) -> Option<&mut MappingHandle> {
        if let Some(handle_type) = self.handle_types.get_mut(key as usize) {
            match handle_type {
                HandleType::MappingHandle(mh) => Some(mh),
                _ => None,
            }
        } else {
            None
        }
    }

    // Method to remove a FileHandle (useful for CloseHandle)
    pub fn remove_file_handle(&mut self, key: u32) -> Option<FileHandle> {
        if let Some(handle_type) = self.handle_types.try_remove(key as usize) {
            match handle_type {
                HandleType::FileHandle(fh) => {
                    self.number_of_handle -= 1;
                    Some(fh)
                }
                _ => {
                    // Put it back if it wasn't a FileHandle, though this indicates a logic error
                    self.handle_types.insert(handle_type);
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn remove_mapping_handle(&mut self, key: u32) -> Option<MappingHandle> {
        if let Some(handle_type) = self.handle_types.try_remove(key as usize) {
            match handle_type {
                HandleType::MappingHandle(mh) => {
                    self.number_of_handle -= 1;
                    Some(mh)
                }
                _ => {
                    self.handle_types.insert(handle_type);
                    None
                }
            }
        } else {
            None
        }
    }
}

const DEFAULT_PATH: &str = "D:\\malware\\temp";

pub fn windows_to_emulate_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let input_path = path.as_ref();
    let base_path = Path::new(DEFAULT_PATH);

    if input_path.is_absolute() {
        if let Some(first_component) = input_path.components().next() {
            let mut mapped_path = base_path.to_path_buf();
            if let std::path::Component::Prefix(prefix_component) = first_component {
                let drive_part = prefix_component.as_os_str().to_string_lossy();
                if drive_part.len() >= 2 && drive_part.ends_with(':') {
                    let drive_letter = &drive_part[..2];
                    mapped_path.push(drive_letter);
                } else {
                    mapped_path.push("DEFAULT_DRIVE");
                }
            }
            for component in input_path.components().skip(1) {
                mapped_path.push(component);
            }
            return mapped_path;
        }
    } else {
        return base_path.join(input_path);
    }

    base_path.to_path_buf()
}

// Converts an emulated path back to its original Windows path representation.
// This function assumes the emulated path strictly follows the format:
// DEFAULT_PATH + DriveLetter + RestOfPath (e.g., "D:\malware\temp\C\some\path" -> "C:\some\path").
pub fn emulate_path_to_windows_path<P: AsRef<Path>>(emulated_path: P) -> Option<PathBuf> {
    let emulated_path_buf = emulated_path.as_ref().to_path_buf();
    let default_path_buf = Path::new(DEFAULT_PATH).to_path_buf();
    let relative_to_default = emulated_path_buf.strip_prefix(&default_path_buf).ok()?;
    let mut components = relative_to_default.components();
    let drive_letter_component = components.next()?;
    let drive_letter_str = drive_letter_component.as_os_str().to_string_lossy();

    if drive_letter_str.len() != 2 || !drive_letter_str.ends_with(':') {
        return None;
    }

    let mut windows_path = PathBuf::new();
    windows_path.push(format!("{}\\", drive_letter_str));
    windows_path.push(components.as_path());

    Some(windows_path)
}
