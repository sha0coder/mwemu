use std::fmt;

#[derive(Debug)]
pub enum LiefElfError {
    FileNotFound(String),
    ReadFailed(String),
    ParseFailed(String),
    UnsupportedClass(String),
    UnsupportedMachine {
        machine: String,
    },
    InvalidSegmentBounds {
        index: usize,
        offset: u64,
        size: u64,
        file_len: usize,
    },
    InvalidSectionBounds {
        index: usize,
        offset: u64,
        size: u64,
        file_len: usize,
    },
    TempFileFailed(String),
}

impl fmt::Display for LiefElfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiefElfError::FileNotFound(path) => write!(f, "File not found: {}", path),
            LiefElfError::ReadFailed(msg) => write!(f, "Read failed: {}", msg),
            LiefElfError::ParseFailed(msg) => write!(f, "ELF parsing failed: {}", msg),
            LiefElfError::UnsupportedClass(cls) => write!(f, "Unsupported ELF class: {}", cls),
            LiefElfError::UnsupportedMachine { machine } => {
                write!(f, "Unsupported ELF machine: {}", machine)
            }
            LiefElfError::InvalidSegmentBounds {
                index,
                offset,
                size,
                file_len,
            } => {
                write!(
                    f,
                    "Invalid segment {} bounds: offset=0x{:x} size=0x{:x} file_len=0x{:x}",
                    index, offset, size, file_len
                )
            }
            LiefElfError::InvalidSectionBounds {
                index,
                offset,
                size,
                file_len,
            } => {
                write!(
                    f,
                    "Invalid section {} bounds: offset=0x{:x} size=0x{:x} file_len=0x{:x}",
                    index, offset, size, file_len
                )
            }
            LiefElfError::TempFileFailed(msg) => write!(f, "Temp file error: {}", msg),
        }
    }
}

impl std::error::Error for LiefElfError {}
