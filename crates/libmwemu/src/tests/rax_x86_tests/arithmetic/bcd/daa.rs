use crate::*;

// DAA — Decimal Adjust AL After Addition
//
// Opcode: 27
// Instruction: DAA
// Op/En: ZO (no operands)
// 64-bit Mode: Invalid
// Compat/Leg Mode: Valid
//
// Description:
// Adjusts the sum of two packed BCD values to create a packed BCD result.
// The AL register is the implied source and destination operand. DAA is only
// useful when it follows an ADD instruction that adds (binary addition) two
// 2-digit, packed BCD values and stores a byte result in the AL register.
//
// Operation:
// old_AL := AL;
// old_CF := CF;
// CF := 0;
// IF (((AL AND 0FH) > 9) or AF = 1) THEN
//     AL := AL + 6;
//     CF := old_CF or (Carry from AL := AL + 6);
//     AF := 1;
// ELSE
//     AF := 0;
// FI;
// IF ((old_AL > 99H) or (old_CF = 1)) THEN
//     AL := AL + 60H;
//     CF := 1;
// ELSE
//     CF := 0;
// FI;
//
// Flags Affected:
// CF and AF are set if adjustment results in decimal carry in either digit.
// SF, ZF, and PF are set according to result. OF is undefined.

// ============================================================================
// DAA - Basic Cases (No Adjustment)
// ============================================================================

#[test]
fn test_daa_no_adjustment() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x25 (valid packed BCD), no adjustment needed
    let code = [
        0x27, // DAA
        0xf4, // HLT
    ];
    emu.regs_mut().rax = 0x25;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x25, "AL should remain 0x25");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_af, "AF should be clear");
}

#[test]
fn test_daa_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x00
    let code = [0x27, 0xf4];
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should remain 0x00");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_af, "AF should be clear");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_daa_valid_bcd_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let valid_bcd = [0x00, 0x09, 0x10, 0x19, 0x25, 0x33, 0x44, 0x58, 0x67, 0x99];

    for val in valid_bcd.iter() {
        let code = [0x27, 0xf4];
        emu.regs_mut().rax = *val as u64;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFF, *val as u64, "AL should remain {:#04x}", val);
        assert!(!emu.flags().f_cf, "CF should be clear for {:#04x}", val);
        assert!(!emu.flags().f_af, "AF should be clear for {:#04x}", val);
    }
}

// ============================================================================
// DAA - Lower Nibble Adjustment
// ============================================================================

#[test]
fn test_daa_lower_nibble_0a() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x0A (lower nibble > 9), needs adjustment
    let code = [0x27, 0xf4];
    emu.regs_mut().rax = 0x0A;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x10, "AL should be 0x10 (0x0A + 0x06)");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_daa_lower_nibble_0f() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x0F (lower nibble = 15), needs adjustment
    let code = [0x27, 0xf4];
    emu.regs_mut().rax = 0x0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x15, "AL should be 0x15 (0x0F + 0x06)");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_daa_lower_nibble_1c() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x1C (lower nibble = 12), needs adjustment
    let code = [0x27, 0xf4];
    emu.regs_mut().rax = 0x1C;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x22, "AL should be 0x22 (0x1C + 0x06)");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(emu.flags().f_af, "AF should be set");
}

// ============================================================================
// DAA - Upper Nibble Adjustment
// ============================================================================

#[test]
fn test_daa_upper_nibble_a0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0xA0 (upper nibble > 9), needs adjustment
    let code = [0x27, 0xf4];
    emu.regs_mut().rax = 0xA0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should be 0x00 (0xA0 + 0x60 = 0x100, wrapped)");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_daa_upper_nibble_f0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0xF0 (upper nibble = 15), needs adjustment
    let code = [0x27, 0xf4];
    emu.regs_mut().rax = 0xF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x50, "AL should be 0x50 (0xF0 + 0x60 = 0x150, wrapped)");
    assert!(emu.flags().f_cf, "CF should be set");
}

// ============================================================================
// DAA - Both Nibbles Require Adjustment
// ============================================================================

#[test]
fn test_daa_both_nibbles_ae() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0xAE (both nibbles > 9), needs both adjustments
    let code = [0x27, 0xf4];
    emu.regs_mut().rax = 0xAE;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0xAE + 0x06 = 0xB4, then 0xB4 + 0x60 = 0x14 (wrapped)
    assert_eq!(emu.regs().rax & 0xFF, 0x14, "AL should be 0x14");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_daa_both_nibbles_ff() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0xFF (maximum value)
    let code = [0x27, 0xf4];
    emu.regs_mut().rax = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0xFF + 0x06 = 0x105 (wrapped to 0x05), then 0x05 + 0x60 = 0x65
    assert_eq!(emu.regs().rax & 0xFF, 0x65, "AL should be 0x65");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

// ============================================================================
// DAA - Realistic Packed BCD Addition Examples
// ============================================================================

#[test]
fn test_daa_after_add_25_plus_34() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 25 + 34 = 59 (no carry)
    let code = [
        0xb0, 0x25, // MOV AL, 0x25
        0x04, 0x34, // ADD AL, 0x34
        0x27,       // DAA
        0xf4,       // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x59, "Result should be 0x59 (BCD 59)");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_daa_after_add_79_plus_35() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb0, 0x79, // MOV AL, 0x79
        0x04, 0x35, // ADD AL, 0x35 (result: 0xAE)
        0x27,       // DAA (should produce 0x14 with CF=1)
        0xf4,       // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x14, "Result should be 0x14 (ones place of 114)");
    assert!(emu.flags().f_cf, "CF should be set (carry to next digit)");
}

#[test]
fn test_daa_after_add_58_plus_46() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 58 + 46 = 104
    let code = [
        0xb0, 0x58, // MOV AL, 0x58
        0x04, 0x46, // ADD AL, 0x46 (result: 0x9E)
        0x27,       // DAA
        0xf4,       // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x04, "Result should be 0x04 (ones place of 104)");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_daa_after_add_99_plus_99() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 99 + 99 = 198
    let code = [
        0xb0, 0x99, // MOV AL, 0x99
        0x04, 0x99, // ADD AL, 0x99 (result: 0x132, wrapped to 0x32, CF=1)
        0x27,       // DAA
        0xf4,       // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x98, "Result should be 0x98 (ones place of 198)");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_daa_after_add_15_plus_27() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 15 + 27 = 42
    let code = [
        0xb0, 0x15, // MOV AL, 0x15
        0x04, 0x27, // ADD AL, 0x27 (result: 0x3C)
        0x27,       // DAA
        0xf4,       // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "Result should be 0x42 (BCD 42)");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

// ============================================================================
// DAA - With AF Flag Set
// ============================================================================

#[test]
fn test_daa_af_set_valid_lower_nibble() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x25 (valid), but AF is set - should still adjust
    let code = [0x27, 0xf4];
    emu.regs_mut().rax = 0x25;
    emu.flags_mut().load(0x10); // Set AF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x2B, "AL should be 0x2B (0x25 + 0x06)");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_daa_af_set_causes_upper_adjust() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x99, AF set - causes cascade to upper nibble
    let code = [0x27, 0xf4];
    emu.regs_mut().rax = 0x99;
    emu.flags_mut().load(0x10); // Set AF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x99 + 0x06 = 0x9F; upper adjust only applies when old AL > 0x99 or CF=1
    assert_eq!(emu.regs().rax & 0xFF, 0x9F, "AL should be 0x9F");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(emu.flags().f_af, "AF should be set");
}

// ============================================================================
// DAA - With CF Flag Set
// ============================================================================

#[test]
fn test_daa_cf_set_causes_upper_adjust() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x25, CF is set - should adjust upper nibble
    let code = [0x27, 0xf4];
    emu.regs_mut().rax = 0x25;
    emu.flags_mut().load(0x01); // Set CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x85, "AL should be 0x85 (0x25 + 0x60)");
    assert!(emu.flags().f_cf, "CF should remain set");
}

#[test]
fn test_daa_cf_set_with_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0xA5, CF is set
    let code = [0x27, 0xf4];
    emu.regs_mut().rax = 0xA5;
    emu.flags_mut().load(0x01); // Set CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0xA5 + 0x60 = 0x105, wrapped to 0x05
    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should be 0x05");
    assert!(emu.flags().f_cf, "CF should be set");
}

// ============================================================================
// DAA - Multi-Digit Addition Simulation
// ============================================================================

#[test]
fn test_daa_multidigit_12_plus_34() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb0, 0x02, // MOV AL, 2
        0x04, 0x04, // ADD AL, 4
        0x27,       // DAA
        0xf4,       // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x06, "Ones digit should be 6");
    assert!(!emu.flags().f_cf, "No carry");
}

#[test]
fn test_daa_multidigit_28_plus_37() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb0, 0x08, // MOV AL, 8
        0x04, 0x07, // ADD AL, 7 (result: 0x0F)
        0x27,       // DAA
        0xf4,       // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x15, "Result should be 0x15 (BCD 15)");
    assert!(!emu.flags().f_cf, "CF should be clear (< 100)");
}

// ============================================================================
// DAA - Edge Cases
// ============================================================================

#[test]
fn test_daa_all_lower_nibbles() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for lower in 0..=0xF {
        let code = [0x27, 0xf4];
        emu.regs_mut().rax = lower;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        if lower <= 9 {
            assert_eq!(emu.regs().rax & 0xFF, lower, "AL should remain {:#04x}", lower);
            assert!(!emu.flags().f_af, "AF should be clear for {:#04x}", lower);
        } else {
            let expected = lower + 6;
            assert_eq!(emu.regs().rax & 0xFF, expected, "AL should be {:#04x} for input {:#04x}", expected, lower);
            assert!(emu.flags().f_af, "AF should be set for {:#04x}", lower);
        }
    }
}

#[test]
fn test_daa_preserves_high_rax() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x27, 0xf4];
    emu.regs_mut().rax = 0x1234_5678_DEAD_BE0A;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax >> 8, 0x1234_5678_DEAD_BE, "High bits of RAX should be preserved");
}

#[test]
fn test_daa_flag_combinations() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x27, 0xf4];
    emu.regs_mut().rax = 0x88;
    emu.flags_mut().load(0x11); // Set both AF and CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x88 + 0x06 = 0x8E, then 0x8E + 0x60 = 0xEE
    assert_eq!(emu.regs().rax & 0xFF, 0xEE, "AL should be 0xEE");
    assert!(emu.flags().f_cf, "CF should remain set");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_daa_boundary_09() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x09 (boundary of lower nibble)
    let code = [0x27, 0xf4];
    emu.regs_mut().rax = 0x09;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x09, "AL should remain 0x09");
    assert!(!emu.flags().f_af, "AF should be clear");
}

#[test]
fn test_daa_boundary_90() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x90 (boundary of upper nibble)
    let code = [0x27, 0xf4];
    emu.regs_mut().rax = 0x90;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x90, "AL should remain 0x90");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_daa_sequential_additions() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb0, 0x15, // MOV AL, 0x15
        0x04, 0x27, // ADD AL, 0x27
        0x27,       // DAA (result: 0x42)
        0x04, 0x38, // ADD AL, 0x38
        0x27,       // DAA (result: 0x80)
        0xf4,       // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x80, "Final result should be 0x80 (BCD 80)");
}

#[test]
fn test_daa_with_previous_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb0, 0x99, // MOV AL, 0x99
        0x04, 0x01, // ADD AL, 0x01 (result: 0x9A)
        0x27,       // DAA (should produce 0x00 with CF=1)
        0xf4,       // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "Result should be 0x00");
    assert!(emu.flags().f_cf, "CF should be set (carry to next byte)");
}

#[test]
fn test_daa_comprehensive_packed_bcd() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = [
        (0x12, 0x34, 0x46),  // 12 + 34 = 46
        (0x45, 0x23, 0x68),  // 45 + 23 = 68
        (0x50, 0x49, 0x99),  // 50 + 49 = 99
        (0x33, 0x44, 0x77),  // 33 + 44 = 77
    ];

    for (a, b, expected) in test_cases.iter() {
        let code = [
            0xb0, *a,   // MOV AL, a
            0x04, *b,   // ADD AL, b
            0x27,       // DAA
            0xf4,       // HLT
        ];
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFF, *expected as u64,
            "Result of {:#04x} + {:#04x} should be {:#04x}", a, b, expected);
    }
}
