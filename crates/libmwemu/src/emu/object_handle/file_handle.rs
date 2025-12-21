use crate::emu::disassemble::InstructionCache;
use crate::{
    banzai::Banzai,
    breakpoint::Breakpoints,
    colors::Colors,
    config::Config,
    definitions::{Definition, StoredContext},
    global_locks::GlobalLocks,
    hooks::Hooks,
    maps::Maps,
    pe::{pe32::PE32, pe64::PE64},
    structures::MemoryOperation,
    thread_context::ThreadContext,
};
use crate::maps::heap_allocation::O1Heap;
use std::fs::File;
use std::fs::ReadDir;
use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use ahash::HashMap;
use crate::emu::object_handle::windows_to_emulate_path;

// --- WinAPI Constants (Commonly Used) ---
pub const INVALID_HANDLE_VALUE: usize = !0; // -1 as usize
pub const GENERIC_READ: usize = 0x80000000;
pub const GENERIC_WRITE: usize = 0x40000000;
pub const GENERIC_EXECUTE: usize = 0x20000000;
pub const GENERIC_ALL: usize = 0x10000000;

pub const CREATE_NEW: usize = 1;
pub const CREATE_ALWAYS: usize = 2;
pub const OPEN_EXISTING: usize = 3;
pub const OPEN_ALWAYS: usize = 4;
pub const TRUNCATE_EXISTING: usize = 5;

pub const FILE_ATTRIBUTE_READONLY: usize = 0x1;
pub const FILE_ATTRIBUTE_HIDDEN: usize = 0x2;
pub const FILE_ATTRIBUTE_SYSTEM: usize = 0x4;
pub const FILE_ATTRIBUTE_DIRECTORY: usize = 0x10;
pub const FILE_ATTRIBUTE_ARCHIVE: usize = 0x20;
pub const FILE_ATTRIBUTE_NORMAL: usize = 0x80;
pub const FILE_ATTRIBUTE_TEMPORARY: usize = 0x100;
pub const FILE_ATTRIBUTE_COMPRESSED: usize = 0x800;
pub const FILE_ATTRIBUTE_OFFLINE: usize = 0x1000;
pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: usize = 0x2000;
pub const FILE_ATTRIBUTE_ENCRYPTED: usize = 0x4000;

pub const FILE_FLAG_WRITE_THROUGH: usize = 0x80000000;
pub const FILE_FLAG_OVERLAPPED: usize = 0x40000000;
pub const FILE_FLAG_NO_BUFFERING: usize = 0x20000000;
pub const FILE_FLAG_RANDOM_ACCESS: usize = 0x10000000;
pub const FILE_FLAG_SEQUENTIAL_SCAN: usize = 0x8000000;
pub const FILE_FLAG_DELETE_ON_CLOSE: usize = 0x4000000;
pub const FILE_FLAG_BACKUP_SEMANTICS: usize = 0x2000000;
pub const FILE_FLAG_POSIX_SEMANTICS: usize = 0x1000000;
pub const FILE_FLAG_OPEN_REPARSE_POINT: usize = 0x200000;
pub const FILE_FLAG_OPEN_NO_RECALL: usize = 0x100000;
pub const FILE_FLAG_FIRST_PIPE_INSTANCE: usize = 0x80000;
struct FileHandleManagement {
    handle_management: HashMap<u32, FileHandle>, // Fixed typo: handleMagemement -> handle_management
}

// Represents the state and metadata associated with a Windows file handle
pub struct FileHandle {
    // --- Core File/Directory Info ---
    name: String,              // Original name used to open the handle
    path: PathBuf,             // Resolved path
    is_dir: bool,              // Whether this handle represents a directory
    // --- Rust File/ReadDir Objects ---
    file: Option<File>,        // Actual Rust file handle (for files)
    dir: Option<ReadDir>,      // Actual Rust directory iterator handle (for directories opened for enumeration)
    // --- WinAPI Handle State ---
    access_mode: u32,          // Access flags (e.g., GENERIC_READ, GENERIC_WRITE)
    creation_disposition: u32, // How the file was opened (CREATE_ALWAYS, OPEN_EXISTING, etc.)
    flags_and_attributes: u32, // File attributes and flags (FILE_ATTRIBUTE_HIDDEN, FILE_FLAG_SEQUENTIAL_SCAN, etc.)
    // --- File Position and State ---
    file_position: u64,        // Current position in the file (for seeking/reading/writing)
    // --- Sharing and Security (Simplified for Emulation) ---
    sharing_mode: u32,         // Sharing flags (FILE_SHARE_READ, FILE_SHARE_WRITE, etc.)
    // --- Additional State ---
    is_valid: bool,            // Whether the handle is considered valid (e.g., not closed)
    is_eof: bool,              // End-of-file flag
    // --- Example: Potential for caching or other emulation-specific data ---
    // cache: Option<SomeCacheType>,
}

// Corrected function signature and implementation

impl FileHandle {
    // Creates a new FileHandle instance based on a Windows path and parameters
    // This would typically be called from an emulated CreateFile function
    pub fn new(
        name: String,
        access_mode: u32,
        creation_disposition: u32,
        flags_and_attributes: u32,
        sharing_mode: u32,
    ) -> Result<FileHandle, Box<dyn std::error::Error>> { // Return Result for better error handling

        let resolved_path = windows_to_emulate_path(&name);
        println!("Attempting to resolve path: {} -> {:?}", name, resolved_path);

        // Check if the target is a directory first
        let metadata_result = fs::metadata(&resolved_path);
        let is_dir = match metadata_result {
            Ok(metadata) => metadata.is_dir(),
            Err(e) => {
                // Handle case where path doesn't exist based on creation_disposition
                match creation_disposition {
                    // Example: CREATE_NEW, CREATE_ALWAYS, OPEN_ALWAYS might allow creation
                    // OPEN_EXISTING, TRUNCATE_EXISTING require it to exist
                    3 | 4 => false, // OPEN_EXISTING (3) or TRUNCATE_EXISTING (4) -> error if doesn't exist
                    _ => {
                        // For other dispositions, assume it might be a file to create
                        // This is a simplification; real WinAPI logic is more complex.
                        // Let the File::open/create handle the error if necessary.
                        false
                    }
                }
            }
        };

        // Determine if it's a directory *after* potential creation logic (simplified here)
        let is_dir = fs::metadata(&resolved_path).map(|m| m.is_dir()).unwrap_or(false);

        let (file, dir) = if is_dir {
            // If it's a directory, we might open it for enumeration (ReadDir) depending on access_mode
            // For simplicity, let's assume opening a directory handle (not enumeration) might just store the path
            // and set file=None, dir=None initially, or use ReadDir if specific flags are set.
            // Let's assume basic directory handle (not enumeration) for now.
            (None, None) // Or potentially Some(fs::read_dir(&resolved_path)?) if enumeration is intended by access_mode
        } else {
            // It's a file, attempt to open/create based on creation_disposition and access_mode
            let file_result = match creation_disposition {
                1 => File::create(&resolved_path),        // CREATE_NEW
                2 => File::create(&resolved_path),        // CREATE_ALWAYS
                3 => File::open(&resolved_path),          // OPEN_EXISTING
                4 => File::options().read(true).write(true).open(&resolved_path), // TRUNCATE_EXISTING
                5 => File::options().read(true).open(&resolved_path), // OPEN_ALWAYS
                _ => File::options().read(true).write(true).create(true).open(&resolved_path), // Default or unknown, try open with create
            };
            (Some(file_result?), None)
        };

        Ok(FileHandle {
            name,
            path: resolved_path,
            is_dir,
            file,
            dir,
            access_mode,
            creation_disposition,
            flags_and_attributes,
            file_position: 0,
            sharing_mode,
            is_valid: true, // Assume valid upon creation
            is_eof: false,  // Assume not at EOF initially
        })
    }

    // Example methods that might be called by emulated WinAPI functions
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, std::io::Error> {
        if !self.is_valid || self.is_dir {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid handle or operation for directory"));
        }
        if let Some(ref mut f) = self.file {
            let bytes_read = f.read(buffer)?;
            self.file_position += bytes_read as u64;
            // Update EOF flag if necessary
            if bytes_read < buffer.len() && bytes_read == 0 {
                // This might indicate EOF, but depends on read behavior
                // A more robust check might involve seeking to end and comparing position
                // For now, assume if read returns 0, we are at EOF
                self.is_eof = true;
            } else {
                self.is_eof = false;
            }
            Ok(bytes_read)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No file object associated"))
        }
    }

    pub fn write(&mut self, buffer: &[u8]) -> Result<usize, std::io::Error> {
        if !self.is_valid || self.is_dir {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid handle or operation for directory"));
        }
        if let Some(ref mut f) = self.file {
            let bytes_written = f.write(buffer)?;
            self.file_position += bytes_written as u64;
            self.is_eof = false; // Writing typically means not at EOF
            Ok(bytes_written)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No file object associated"))
        }
    }

    pub fn seek(&mut self, pos: SeekFrom) -> Result<u64, std::io::Error> {
        if !self.is_valid || self.is_dir {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid handle or operation for directory"));
        }
        if let Some(ref mut f) = self.file {
            let new_pos = f.seek(pos)?;
            self.file_position = new_pos;
            self.is_eof = false; // Seeking usually means not at EOF
            Ok(new_pos)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No file object associated"))
        }
    }

    pub fn close(&mut self) {
        // Release resources
        self.file.take(); // Drops the File, closing it
        self.dir.take();  // Drops the ReadDir
        self.is_valid = false;
        // Other cleanup if necessary
    }

    // Getter methods
    pub fn is_valid(&self) -> bool { self.is_valid }
    pub fn is_dir(&self) -> bool { self.is_dir }
    pub fn get_path(&self) -> &Path { &self.path }
    pub fn get_position(&self) -> u64 { self.file_position }
    pub fn is_eof(&self) -> bool { self.is_eof }

    // Setter for file_position (e.g., from SetFilePointer)
    pub fn set_position(&mut self, pos: u64) {
        if let Some(ref mut f) = self.file {
            if f.seek(SeekFrom::Start(pos)).is_ok() {
                self.file_position = pos;
                self.is_eof = false; // Seeking usually means not at EOF
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::emu::object_handle::{emulate_path_to_windows_path, DEFAULT_PATH};
    use super::*;

    #[test]
    fn test_windows_to_emulate_path_absolute() {
        let result = windows_to_emulate_path("C:\\Windows\\System32");
        let expected = Path::new(DEFAULT_PATH).join("C:").join("Windows").join("System32");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_windows_to_emulate_path_relative() {
        let result = windows_to_emulate_path("some\\relative\\path");
        let expected = Path::new(DEFAULT_PATH).join("some").join("relative").join("path");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_windows_to_emulate_path_current_dir() {
        let result = windows_to_emulate_path(".\\current\\file.txt");
        let expected = Path::new(DEFAULT_PATH).join("current").join("file.txt");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_windows_to_emulate_path_parent_dir() {
        let result = windows_to_emulate_path("..\\parent");
        // This joins the parent component to the base path, so result is DEFAULT_PATH\parent
        let expected = Path::new(DEFAULT_PATH).join("parent");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_emulate_to_windows_path_drive_c() {
        let emulated = Path::new(DEFAULT_PATH).join("C:").join("some").join("path");
        let result = emulate_path_to_windows_path(emulated).unwrap();
        let expected = Path::new("C:\\some\\path");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_emulate_to_windows_path_drive_d() {
        let emulated = Path::new(DEFAULT_PATH).join("D:").join("another").join("dir");
        let result = emulate_path_to_windows_path(emulated).unwrap();
        let expected = Path::new("D:\\another\\dir");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_emulate_to_windows_path_invalid_no_drive() {
        // Path under DEFAULT_PATH that doesn't start with a drive letter (e.g., from a relative path input)
        let emulated = Path::new(DEFAULT_PATH).join("some").join("relative").join("path");
        let result = emulate_path_to_windows_path(emulated);
        // Expect None because the first component after DEFAULT_PATH is not a drive letter
        assert!(result.is_none());
    }

    #[test]
    fn test_emulate_to_windows_path_invalid_not_under_default() {
        // Path that doesn't start with DEFAULT_PATH
        let emulated = Path::new("C:\\some\\other\\path");
        let result = emulate_path_to_windows_path(emulated);
        // Expect None because it doesn't have the DEFAULT_PATH prefix
        assert!(result.is_none());
    }
}