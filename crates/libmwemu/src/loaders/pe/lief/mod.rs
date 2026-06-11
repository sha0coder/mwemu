//! LIEF-based PE parser module
//!
//! This module provides a memory-efficient PE parser using LIEF library
//! with the following features:
//! - Header-only parsing (~4KB initial read)
//! - Lazy section loading on demand
//! - Zero-copy file access via memory mapping
//! - Configurable cache policies

pub mod error;
pub mod lief_header_parser;
pub mod lief_pe;
pub mod lief_section_manager;
pub mod traits;

#[cfg(test)]
mod tests;

pub use error::{
    ExportInfo, ImportFunction, ImportInfo, LiefError, RelocationInfo, ResourceEntryInfo,
    ResourceInfo,
};
pub use lief_header_parser::LiefHeaderParser;
pub use lief_pe::{CacheStats, DelayLoadDescriptor, LiefPe};
pub use lief_section_manager::{CachePolicy, LiefSectionManager};
pub use traits::LiefPeReader;
