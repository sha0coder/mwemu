// Variable Shift Instructions (BMI2)
//
// This module contains tests for BMI2 variable shift instructions that perform
// shift operations without affecting flags. These instructions are part of the
// Bit Manipulation Instruction Set 2 (BMI2) extension.
//
// Instructions:
// - SHLX: Logical Left Shift Without Affecting Flags
// - SHRX: Logical Right Shift Without Affecting Flags
// - SARX: Arithmetic Right Shift Without Affecting Flags
//
// Key characteristics:
// - 3-operand form: dest = src << count (count from register)
// - No flags modified (unlike traditional SHL/SHR/SAR)
// - Count masked to 5 bits (32-bit) or 6 bits (64-bit)
// - Requires BMI2 CPU feature
//
// Each test file contains:
// - All shift count tests (0 to max)
// - Flag preservation tests
// - Pattern tests
// - Memory operand tests
// - Extended register tests (R8-R15)
// - Edge cases

#[cfg(test)]
mod shlx;

#[cfg(test)]
mod shrx;

#[cfg(test)]
mod sarx;
