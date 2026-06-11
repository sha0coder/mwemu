use std::fmt;

#[derive(Debug)]
pub enum LiefMachoError {
    FileNotFound(String),
    ReadFailed(String),
    ParseFailed(String),
    NoMatchingSlice {
        wanted: String,
    },
    UnsupportedCpu(String),
    Not64Bit,
    InvalidSegmentBounds {
        name: String,
        file_offset: u64,
        file_size: u64,
        file_len: usize,
    },
    MissingEntrypoint,
    TempFileFailed(String),
}

impl fmt::Display for LiefMachoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiefMachoError::FileNotFound(path) => write!(f, "File not found: {}", path),
            LiefMachoError::ReadFailed(msg) => write!(f, "Read failed: {}", msg),
            LiefMachoError::ParseFailed(msg) => write!(f, "Mach-O parsing failed: {}", msg),
            LiefMachoError::NoMatchingSlice { wanted } => {
                write!(f, "No matching slice found: {}", wanted)
            }
            LiefMachoError::UnsupportedCpu(cpu) => write!(f, "Unsupported CPU type: {}", cpu),
            LiefMachoError::Not64Bit => write!(f, "Not a 64-bit Mach-O binary"),
            LiefMachoError::InvalidSegmentBounds {
                name,
                file_offset,
                file_size,
                file_len,
            } => {
                write!(
                    f,
                    "Invalid segment '{}' bounds: file_offset=0x{:x} file_size=0x{:x} file_len=0x{:x}",
                    name, file_offset, file_size, file_len
                )
            }
            LiefMachoError::MissingEntrypoint => write!(f, "No valid entrypoint found"),
            LiefMachoError::TempFileFailed(msg) => write!(f, "Temp file error: {}", msg),
        }
    }
}

impl std::error::Error for LiefMachoError {}
