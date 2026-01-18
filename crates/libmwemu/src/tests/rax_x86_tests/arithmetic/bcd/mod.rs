// Comprehensive tests for x86_64 BCD (Binary Coded Decimal) arithmetic instructions.
//
// This module contains tests for:
// - AAA (ASCII Adjust After Addition)
// - AAS (ASCII Adjust After Subtraction)
// - AAM (ASCII Adjust After Multiplication)
// - AAD (ASCII Adjust Before Division)
// - DAA (Decimal Adjust After Addition)
// - DAS (Decimal Adjust After Subtraction)
//
// These instructions support BCD arithmetic for both unpacked (ASCII) and
// packed BCD formats, primarily used in legacy applications that require
// decimal arithmetic.

pub mod aaa;
pub mod aas;
pub mod aam;
pub mod aad;
pub mod daa;
pub mod das;
