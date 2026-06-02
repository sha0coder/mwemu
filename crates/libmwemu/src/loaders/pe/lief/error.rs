//! Error types for LIEF PE parsing

use std::fmt;

// ============================================================================
// Supporting Types for PE Information
// ============================================================================

/// Information about an imported DLL and its functions
#[derive(Debug, Clone)]
pub struct ImportInfo {
    /// Name of the imported DLL
    pub dll_name: String,
    /// List of imported functions from this DLL
    pub functions: Vec<ImportFunction>,
}

/// Information about a single imported function
#[derive(Debug, Clone)]
pub struct ImportFunction {
    /// Name of the imported function
    pub name: String,
    /// Ordinal value (if available)
    pub ordinal: Option<u16>,
    /// Relative Virtual Address of the import
    pub rva: u64,
}

/// Information about an exported function
#[derive(Debug, Clone)]
pub struct ExportInfo {
    /// Name of the exported function
    pub name: String,
    /// Ordinal value
    pub ordinal: u16,
    /// Relative Virtual Address
    pub rva: u64,
}

/// Information about a resource entry (flat projection)
#[derive(Debug, Clone)]
pub struct ResourceInfo {
    pub resource_type: u32,
    pub name: String,
    pub rva: u64,
    pub size: u64,
}

/// Hierarchy-preserving resource entry descriptor.
///
/// Preserves the PE resource directory tree structure:
/// type -> name/id -> language.
///
/// `data_rva` is the RVA of the resource data entry content (from
/// `IMAGE_RESOURCE_DATA_ENTRY.OffsetToData`). `file_offset` is the
/// corresponding file offset, if resolvable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceEntryInfo {
    pub type_id: Option<u32>,
    pub type_name: Option<String>,
    pub name_id: Option<u32>,
    pub name: Option<String>,
    pub language_id: Option<u32>,
    pub data_rva: u64,
    pub file_offset: Option<u64>,
    pub size: u64,
}

/// Information about a relocation entry
#[derive(Debug, Clone)]
pub struct RelocationInfo {
    /// Relative Virtual Address of the relocation
    pub rva: u64,
    /// Relocation type
    pub reloc_type: u16,
}

// ============================================================================
// Error Types
// ============================================================================

/// Error types that can occur during LIEF PE operations
#[derive(Debug)]
pub enum LiefError {
    /// File was not found at the specified path
    FileNotFound(String),
    /// Memory mapping failed
    MmapFailed(String),
    /// PE parsing failed
    ParseFailed(String),
    /// Header extraction failed
    HeaderExtractionFailed,
    /// Section not found
    SectionNotFound(String),
    /// Invalid offset specified
    InvalidOffset(String),
    /// Cache operation error
    CacheError(String),
    /// Relocation parsing error
    RelocationError(String),
    /// Unsupported relocation type
    UnsupportedRelocationType { reloc_type: u16, rva: u64 },
}

impl fmt::Display for LiefError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiefError::FileNotFound(path) => write!(f, "File not found: {}", path),
            LiefError::MmapFailed(msg) => write!(f, "Memory mapping failed: {}", msg),
            LiefError::ParseFailed(msg) => write!(f, "PE parsing failed: {}", msg),
            LiefError::HeaderExtractionFailed => write!(f, "Header extraction failed"),
            LiefError::SectionNotFound(name) => write!(f, "Section not found: {}", name),
            LiefError::InvalidOffset(offset) => write!(f, "Invalid offset: {}", offset),
            LiefError::CacheError(msg) => write!(f, "Cache error: {}", msg),
            LiefError::RelocationError(msg) => write!(f, "Relocation error: {}", msg),
            LiefError::UnsupportedRelocationType { reloc_type, rva } => {
                write!(
                    f,
                    "Unsupported relocation type {} at RVA 0x{:x}",
                    reloc_type, rva
                )
            }
        }
    }
}

impl std::error::Error for LiefError {}
