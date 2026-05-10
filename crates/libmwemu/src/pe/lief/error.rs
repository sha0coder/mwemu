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

/// Information about a resource entry
#[derive(Debug, Clone)]
pub struct ResourceInfo {
    /// Resource type (e.g., RT_BITMAP, RT_ICON, etc.)
    pub resource_type: u32,
    /// Name of the resource
    pub name: String,
    /// Relative Virtual Address
    pub rva: u64,
    /// Size of the resource
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
        }
    }
}

impl std::error::Error for LiefError {}
