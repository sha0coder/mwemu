//! PE module surface — LIEF-only at runtime.
//!
//! `RuntimePe32` / `RuntimePe64` are LIEF-backed wrappers; they are the
//! only PE runtime types used by the loader, binding, and serialization
//! paths. The `lief` submodule holds the actual LIEF parser glue.
//!
//! The legacy hand-rolled `pe32` / `pe64` modules (and the parity tests
//! that compare them against LIEF) are gated behind the
//! `legacy-pe-parity` cargo feature so that a default build does not
//! expose or instantiate the legacy parser types. Enable the feature
//! only if you specifically need to exercise the parity tests.

pub mod lief;
pub mod runtime_pe32;
pub mod runtime_pe64;

#[cfg(feature = "legacy-pe-parity")]
pub mod pe32;
#[cfg(feature = "legacy-pe-parity")]
pub mod pe64;
#[cfg(feature = "legacy-pe-parity")]
pub(crate) mod readers;
#[cfg(feature = "legacy-pe-parity")]
pub(crate) mod shared;

pub use machine::{
    IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_ARM64, IMAGE_FILE_MACHINE_I386, pe_machine_type,
};

mod machine {
    //! Tiny always-available helper module: PE file-format constants
    //! and a fast header sniff (`pe_machine_type`) used by the loader
    //! to pick the right path before committing to LIEF. The full
    //! `shared` module (with the legacy `Image*` structs) is gated
    //! behind the `legacy-pe-parity` feature.

    pub const IMAGE_FILE_MACHINE_I386: u16 = 0x014c;
    pub const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;
    pub const IMAGE_FILE_MACHINE_ARM64: u16 = 0xAA64;

    /// Read the COFF Machine field from a PE file without fully parsing it.
    /// Returns `None` if the file is not a valid PE (no MZ signature, bad e_lfanew, etc.).
    pub fn pe_machine_type(filename: &str) -> Option<u16> {
        use std::fs::File;
        use std::io::Read as _;

        const IMAGE_DOS_SIGNATURE: u16 = 0x5A4D;
        const IMAGE_NT_SIGNATURE: u32 = 0x00004550;
        const DOS_HEADER_SIZE: usize = 64;

        let mut fd = File::open(filename).ok()?;
        let file_size = fd.metadata().ok()?.len();

        if file_size < DOS_HEADER_SIZE as u64 {
            return None;
        }

        let mut buf = vec![0u8; DOS_HEADER_SIZE];
        fd.read_exact(&mut buf).ok()?;

        // Read e_lfanew (offset 0x3c) without depending on the legacy
        // ImageDosHeader struct.
        let e_lfanew = u32::from_le_bytes([buf[0x3c], buf[0x3d], buf[0x3e], buf[0x3f]]);
        let e_magic = u16::from_le_bytes([buf[0], buf[1]]);
        if e_magic != IMAGE_DOS_SIGNATURE {
            return None;
        }

        let need = (e_lfanew as usize) + 4 + 2; // signature(4) + machine(2)
        if need > file_size as usize {
            return None;
        }

        let mut full_buf = vec![0u8; need];
        let mut fd2 = File::open(filename).ok()?;
        fd2.read_exact(&mut full_buf).ok()?;

        let sig = u32::from_le_bytes([
            full_buf[e_lfanew as usize],
            full_buf[e_lfanew as usize + 1],
            full_buf[e_lfanew as usize + 2],
            full_buf[e_lfanew as usize + 3],
        ]);
        if sig != IMAGE_NT_SIGNATURE {
            return None;
        }

        let machine = u16::from_le_bytes([
            full_buf[e_lfanew as usize + 4],
            full_buf[e_lfanew as usize + 5],
        ]);
        Some(machine)
    }
}
