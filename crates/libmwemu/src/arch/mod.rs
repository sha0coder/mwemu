pub mod aarch64;
pub mod x86;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Arch {
    X86,
    X86_64,
    Aarch64,
}

impl Arch {
    /// True for any 64-bit address space (X86_64 or Aarch64).
    /// Use for pointer width, address space decisions (e.g., Maps).
    pub fn is_64bits(self) -> bool {
        matches!(self, Arch::X86_64 | Arch::Aarch64)
    }

    /// True for x86-64 specifically. Use for x86 32-vs-64 branching
    /// (decoder, registers, operand sizes, etc.).
    pub fn is_x64(self) -> bool {
        matches!(self, Arch::X86_64)
    }

    /// True for any x86 variant (32 or 64).
    pub fn is_x86(self) -> bool {
        matches!(self, Arch::X86 | Arch::X86_64)
    }

    /// True for AArch64 / ARM64.
    pub fn is_aarch64(self) -> bool {
        matches!(self, Arch::Aarch64)
    }
}
