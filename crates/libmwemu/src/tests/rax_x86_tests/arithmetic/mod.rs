// Comprehensive tests for x86_64 arithmetic instructions.
//
// This module contains tests for:
// - NEG (Two's Complement Negation)
// - INC/DEC (Increment/Decrement)
// - IMUL (Signed Multiply)
// - MUL (Unsigned Multiply)
// - DIV (Unsigned Divide)
// - IDIV (Signed Divide)
// - ADCX/ADOX (Multi-precision Arithmetic)
// - BCD (Binary Coded Decimal) arithmetic instructions
// - AAA/AAS (ASCII Adjust After Addition/Subtraction)
// - AAM/AAD (ASCII Adjust After Multiply/Divide)
// - DAA/DAS (Decimal Adjust After Addition/Subtraction)
// - ADC (Add with Carry)
// - SBB (Subtract with Borrow)
// - ADD (Integer Addition)
// - SUB (Integer Subtraction)
// - CMP (Compare Two Operands)

pub mod neg;
pub mod inc_dec;
pub mod imul;
pub mod mul;
pub mod div;
pub mod idiv;
pub mod adcx_adox;
pub mod bcd;
pub mod aaa_aas;
pub mod aam_aad;
pub mod daa_das;
pub mod adc_extended;
pub mod sbb_extended;
pub mod add_extended;
pub mod sub_extended;
pub mod cmp_extended;
