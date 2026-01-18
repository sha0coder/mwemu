use crate::*;

// AAS — ASCII Adjust AL After Subtraction
//
// Opcode: 3F
// Instruction: AAS
// Op/En: ZO (no operands)
// 64-bit Mode: Invalid
// Compat/Leg Mode: Valid
//
// Description:
// Adjusts the result of the subtraction of two unpacked BCD values to create
// an unpacked BCD result. The AL register is the implied source and destination.
// AAS is only useful when it follows a SUB instruction that subtracts (binary
// subtraction) one unpacked BCD value from another and stores a byte result in AL.
//
// Operation:
// IF ((AL AND 0FH) > 9) or (AF = 1)
// THEN
//     AX := AX - 6;
//     AH := AH - 1;
//     AF := 1;
//     CF := 1;
//     AL := AL AND 0FH;
// ELSE
//     CF := 0;
//     AF := 0;
//     AL := AL AND 0FH;
// FI;
//
// Flags Affected:
// AF and CF are set to 1 if there is a decimal borrow; otherwise cleared to 0.
// OF, SF, ZF, and PF are undefined.

// ============================================================================
// AAS - Basic Cases (No Adjustment)
// ============================================================================

#[test]
fn test_aas_no_adjustment_needed() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x05 (valid BCD), no adjustment needed
    let code = [
        0x3f, // AAS
        0xf4, // HLT
    ];
    emu.regs_mut().rax = 0x05;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should remain 0x05");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should remain 0x00");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_af, "AF should be clear");
}

#[test]
fn test_aas_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x00, no adjustment needed
    let code = [0x3f, 0xf4]; // AAS, HLT
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should remain 0x00");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should remain 0x00");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_af, "AF should be clear");
}

#[test]
fn test_aas_max_valid_bcd() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x09 (max valid BCD digit), no adjustment needed
    let code = [0x3f, 0xf4];
    emu.regs_mut().rax = 0x09;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x09, "AL should remain 0x09");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should remain 0x00");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_af, "AF should be clear");
}

// ============================================================================
// AAS - Adjustment Required (Lower Nibble > 9)
// ============================================================================

#[test]
fn test_aas_lower_nibble_0a() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x0A, requires adjustment
    let code = [0x3f, 0xf4];
    emu.regs_mut().rax = 0x010A;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x04, "AL should be 0x04 (0x0A - 0x06 = 0x04)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be decremented to 0x00");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_aas_lower_nibble_0b() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x0B
    let code = [0x3f, 0xf4];
    emu.regs_mut().rax = 0x010B;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should be 0x05");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be decremented to 0x00");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_aas_lower_nibble_0f() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x0F (max value for lower nibble)
    let code = [0x3f, 0xf4];
    emu.regs_mut().rax = 0x010F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x09, "AL should be 0x09");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be decremented to 0x00");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

// ============================================================================
// AAS - With Upper Nibble Set
// ============================================================================

#[test]
fn test_aas_with_upper_nibble_1x() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x1A, upper nibble should be cleared
    let code = [0x3f, 0xf4];
    emu.regs_mut().rax = 0x011A;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x04, "AL should be 0x04 (upper nibble cleared)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be decremented to 0x00");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_aas_with_upper_nibble_2x() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x2B
    let code = [0x3f, 0xf4];
    emu.regs_mut().rax = 0x012B;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should be 0x05");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be decremented to 0x00");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_aas_with_upper_nibble_fx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0xFC
    let code = [0x3f, 0xf4];
    emu.regs_mut().rax = 0x01FC;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x06, "AL should be 0x06");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be decremented to 0x00");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

// ============================================================================
// AAS - With Non-Zero AH
// ============================================================================

#[test]
fn test_aas_ah_nonzero_no_adjust() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AH = 0x05, AL = 0x03, no adjustment needed
    let code = [0x3f, 0xf4];
    emu.regs_mut().rax = 0x0503;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03, "AL should remain 0x03");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x05, "AH should remain 0x05");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_af, "AF should be clear");
}

#[test]
fn test_aas_ah_nonzero_adjust() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AH = 0x07, AL = 0x0D, adjustment needed
    let code = [0x3f, 0xf4];
    emu.regs_mut().rax = 0x070D;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x07, "AL should be 0x07");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x06, "AH should be decremented to 0x06");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_aas_ah_underflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AH = 0x00, AL = 0x0E, test AH underflow
    let code = [0x3f, 0xf4];
    emu.regs_mut().rax = 0x000E;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x08, "AL should be 0x08");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0xFF, "AH should wrap to 0xFF");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

// ============================================================================
// AAS - With AF Flag Set
// ============================================================================

#[test]
fn test_aas_af_set_valid_bcd() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x05 (valid BCD), but AF is set - should still adjust
    let code = [0x3f, 0xf4];
    emu.regs_mut().rax = 0x0105;
    emu.flags_mut().load(0x10); // Set AF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x0F, "AL should be 0x0F (0x05 - 0x06 = 0xFF, masked = 0x0F)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0xFF, "AH should be decremented with borrow");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_aas_af_set_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x00, AF is set - should adjust
    let code = [0x3f, 0xf4];
    emu.regs_mut().rax = 0x0100;
    emu.flags_mut().load(0x10); // Set AF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x0A, "AL should be 0x0A");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0xFF, "AH should be decremented with borrow");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_aas_af_set_nine() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x09, AF is set - should adjust
    let code = [0x3f, 0xf4];
    emu.regs_mut().rax = 0x0109;
    emu.flags_mut().load(0x10); // Set AF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03, "AL should be 0x03 (0x09 - 0x06 = 0x03)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be decremented to 0x00");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

// ============================================================================
// AAS - Realistic BCD Subtraction Examples
// ============================================================================

#[test]
fn test_aas_after_sub_8_minus_3() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb0, 0x08, // MOV AL, 8
        0x2c, 0x03, // SUB AL, 3
        0x3f,       // AAS
        0xf4,       // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x05, "Result should be 5");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_af, "AF should be clear");
}

#[test]
fn test_aas_after_sub_3_minus_5() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb0, 0x03, // MOV AL, 3
        0x2c, 0x05, // SUB AL, 5 (result: 0xFE, AF set)
        0x3f,       // AAS
        0xf4,       // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x08, "AL should be 0x08 (0xFE - 0x06 = 0xF8, masked = 0x08)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0xFF, "AH should be 0xFF (borrow)");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_aas_after_sub_9_minus_9() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb0, 0x09, // MOV AL, 9
        0x2c, 0x09, // SUB AL, 9
        0x3f,       // AAS
        0xf4,       // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "Result should be 0");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_af, "AF should be clear");
}

#[test]
fn test_aas_after_sub_2_minus_7() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb0, 0x02, // MOV AL, 2
        0x2c, 0x07, // SUB AL, 7 (result: 0xFB, AF set)
        0x3f,       // AAS
        0xf4,       // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should be 0x05 (0xFB - 0x06 = 0xF5, masked = 0x05)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0xFF, "AH should be 0xFF (borrow)");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

// ============================================================================
// AAS - Multi-Digit BCD Subtraction
// ============================================================================

#[test]
fn test_aas_multidigit_78_minus_34() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb0, 0x08, // MOV AL, 8
        0x2c, 0x04, // SUB AL, 4
        0x3f,       // AAS
        0xf4,       // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x04, "Ones digit should be 4");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "No borrow to tens");
}

#[test]
fn test_aas_multidigit_52_minus_37() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb0, 0x02, // MOV AL, 2
        0x2c, 0x07, // SUB AL, 7 (result: 0xFB, AF set)
        0x3f,       // AAS
        0xf4,       // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x05, "Ones digit should be 5");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0xFF, "Borrow from tens");
}

// ============================================================================
// AAS - Edge Cases and Boundary Conditions
// ============================================================================

#[test]
fn test_aas_all_lower_nibbles() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for val in 0x00..=0x0F {
        let code = [0x3f, 0xf4];
        emu.regs_mut().rax = 0x0100 | val;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        if val <= 9 {
            assert_eq!(emu.regs().rax & 0xFF, val, "AL should remain {:#04x}", val);
            assert!(!emu.flags().f_cf, "CF should be clear for {:#04x}", val);
        } else {
            let expected = (val.wrapping_sub(6)) & 0x0F;
            assert_eq!(emu.regs().rax & 0xFF, expected, "AL should be {:#04x} for input {:#04x}", expected, val);
            assert!(emu.flags().f_cf, "CF should be set for {:#04x}", val);
        }
    }
}

#[test]
fn test_aas_upper_bits_cleared() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3f, 0xf4];
    emu.regs_mut().rax = 0x01AB;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xF0, 0x00, "Upper 4 bits of AL should be cleared");
}

#[test]
fn test_aas_preserves_high_rax() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3f, 0xf4];
    emu.regs_mut().rax = 0x1234_5678_DEAD_BE0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax >> 16, 0x1234_5678_DEAD, "High bits of RAX should be preserved");
}

#[test]
fn test_aas_sequential_operations() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb8, 0x03, 0x00, // MOV AX, 0x0003 (AH=0, AL=3)
        0x2c, 0x05,       // SUB AL, 5 (result: 0xFE, borrow)
        0x3f,             // AAS (result: AL=8, AH=FF)
        0x80, 0xec, 0x09, // SUB AH, 9 (manually adjust tens)
        0xf4,             // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x08, "Final AL should be 8");
}

#[test]
fn test_aas_max_value_ff() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0xFF (all bits set)
    let code = [0x3f, 0xf4];
    emu.regs_mut().rax = 0x01FF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x09, "AL should be 0x09");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0x00");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_aas_double_borrow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb8, 0x00, 0x00, // MOV AX, 0x0000
        0x2c, 0x05,       // SUB AL, 5 (result: 0xFB)
        0x3f,             // AAS
        0xf4,             // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should be 5");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0xFF, "AH should wrap to 0xFF");
}

#[test]
fn test_aas_with_high_ah() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AH = 0x99, AL = 0x0C (test with high AH value)
    let code = [0x3f, 0xf4];
    emu.regs_mut().rax = 0x990C;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x06, "AL should be 0x06");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x98, "AH should be decremented to 0x98");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}
