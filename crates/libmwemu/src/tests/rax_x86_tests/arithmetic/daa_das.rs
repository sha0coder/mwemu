use crate::*;

// DAA — Decimal Adjust AL After Addition
// DAS — Decimal Adjust AL After Subtraction
//
// Opcodes:
// - 27       DAA       Decimal adjust AL after addition
// - 2F       DAS       Decimal adjust AL after subtraction
//
// DAA Operation:
//   IF ((AL AND 0FH) > 9) OR (AF = 1) THEN
//     AL := AL + 6;
//     CF := CF OR (AL > 0xFF);
//     AF := 1;
//   ELSE
//     AF := 0;
//   FI;
//   IF (AL > 0x9F) OR (CF = 1) THEN
//     AL := AL + 0x60;
//     CF := 1;
//   ELSE
//     CF := 0;
//   FI;
//
// DAS Operation:
//   IF ((AL AND 0FH) > 9) OR (AF = 1) THEN
//     AL := AL - 6;
//     CF := CF OR (borrow occurred);
//     AF := 1;
//   ELSE
//     AF := 0;
//   FI;
//   IF (AL > 0x9F) OR (CF = 1) THEN
//     AL := AL - 0x60;
//     CF := 1;
//   ELSE
//     CF := 0;
//   FI;
//
// Flags: SF, ZF, PF, CF, AF are modified. OF is undefined.

// ============================================================================
// DAA (Decimal Adjust After Addition) Tests
// ============================================================================

#[test]
fn test_daa_no_adjustment_needed() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x25 (valid BCD), no flags set
    let code = [
        0x27, // DAA
        0xf4, // HLT
    ];
    emu.regs_mut().rax = 0x0025;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x25, "AL should remain 0x25");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_af, "AF should be clear");
}

#[test]
fn test_daa_low_nibble_adjustment() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x1F (low nibble > 9, needs adjustment)
    let code = [0x27, 0xf4]; // DAA, HLT
    emu.regs_mut().rax = 0x001F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL = 0x1F + 6 = 0x25
    assert_eq!(emu.regs().rax & 0xFF, 0x25, "AL should be 0x25 after adjustment");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_daa_high_nibble_adjustment() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0xA5 (high nibble > 9, needs adjustment)
    let code = [0x27, 0xf4]; // DAA, HLT
    emu.regs_mut().rax = 0x00A5;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL = 0xA5 + 0x60 = 0x05 (with carry)
    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should be 0x05 after adjustment");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_daa_both_nibbles_adjustment() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0xAF (both nibbles > 9)
    let code = [0x27, 0xf4]; // DAA, HLT
    emu.regs_mut().rax = 0x00AF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL = 0xAF + 6 + 0x60 = 0x15 (with carry)
    assert_eq!(emu.regs().rax & 0xFF, 0x15, "AL should be 0x15 after adjustment");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_daa_af_set_triggers_adjustment() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x05 (valid), but AF is set from previous operation
    let code = [0x27, 0xf4]; // DAA, HLT
    emu.regs_mut().rax = 0x0005;
    emu.flags_mut().load(0x10); // Set AF flag
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL = 0x05 + 6 = 0x0B
    assert_eq!(emu.regs().rax & 0xFF, 0x0B, "AL should be adjusted to 0x0B");
    assert!(emu.flags().f_af, "AF should remain set");
}

#[test]
fn test_daa_cf_set_triggers_adjustment() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x25 (valid), but CF is set from previous operation
    let code = [0x27, 0xf4]; // DAA, HLT
    emu.regs_mut().rax = 0x0025;
    emu.flags_mut().load(0x01); // Set CF flag
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL = 0x25 + 0x60 = 0x85
    assert_eq!(emu.regs().rax & 0xFF, 0x85, "AL should be adjusted to 0x85");
    assert!(emu.flags().f_cf, "CF should remain set");
}

#[test]
fn test_daa_bcd_addition_no_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x25 + 0x13 = 0x38 (valid BCD, no adjustment needed)
    let code = [0x27, 0xf4]; // DAA, HLT
    emu.regs_mut().rax = 0x0038;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x38, "Result should remain 0x38");
}

#[test]
fn test_daa_bcd_addition_with_adjustment() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x28 + 0x34 = 0x5C, needs adjustment -> 0x62
    let code = [0x27, 0xf4]; // DAA, HLT
    emu.regs_mut().rax = 0x005C;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x62, "Result should be 0x62 (62 in BCD)");
}

#[test]
fn test_daa_bcd_addition_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x89 + 0x25 = 0xAE, needs adjustment -> 0x14 with carry
    let code = [0x27, 0xf4]; // DAA, HLT
    emu.regs_mut().rax = 0x00AE;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x14, "Result should be 0x14");
    assert!(emu.flags().f_cf, "CF should be set (carry to next digit)");
}

#[test]
fn test_daa_zero_result() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x27, 0xf4]; // DAA, HLT
    emu.regs_mut().rax = 0x0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should remain 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_daa_preserves_high_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x27, 0xf4]; // DAA, HLT
    emu.regs_mut().rax = 0xDEADBEEF_12345625;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax >> 8, 0xDEADBEEF_123456, "High bits should be preserved");
}

#[test]
fn test_daa_all_valid_bcd_pairs() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for high in 0..=9 {
        for low in 0..=9 {
            let val = (high << 4) | low;
            let code = [0x27, 0xf4]; // DAA, HLT
            emu.regs_mut().rax = val;
            emu.load_code_bytes(&code);
    emu.run(None).unwrap();

            // Valid BCD values should not be adjusted
            assert_eq!(emu.regs().rax & 0xFF, val,
                "Valid BCD 0x{:02X} should not be adjusted", val);
        }
    }
}

#[test]
fn test_daa_boundary_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = vec![
        (0x09, 0x09, false), // Boundary: low nibble = 9
        (0x0A, 0x10, true),  // Boundary: low nibble = 10 (A)
        (0x90, 0x90, false), // Boundary: high nibble = 9
        (0x9A, 0x00, true),  // Boundary: 0x9A -> 0xA0 -> 0x100 (0x00 with CF)
    ];

    for (input, expected, expect_cf_or_af) in test_cases {
        let code = [0x27, 0xf4]; // DAA, HLT
        emu.regs_mut().rax = input;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFF, expected,
            "Wrong result for input 0x{:02X}", input);
        if expect_cf_or_af {
            assert!(emu.flags().f_cf || emu.flags().f_af,
                "CF or AF should be set for input 0x{:02X}", input);
        }
    }
}

// ============================================================================
// DAS (Decimal Adjust After Subtraction) Tests
// ============================================================================

#[test]
fn test_das_no_adjustment_needed() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x25 (valid BCD), no flags set
    let code = [
        0x2F, // DAS
        0xf4, // HLT
    ];
    emu.regs_mut().rax = 0x0025;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x25, "AL should remain 0x25");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_af, "AF should be clear");
}

#[test]
fn test_das_low_nibble_adjustment() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x1F (low nibble > 9, needs adjustment)
    let code = [0x2F, 0xf4]; // DAS, HLT
    emu.regs_mut().rax = 0x001F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL = 0x1F - 6 = 0x19
    assert_eq!(emu.regs().rax & 0xFF, 0x19, "AL should be 0x19 after adjustment");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_das_high_nibble_adjustment() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0xA5 (high nibble > 9, needs adjustment)
    let code = [0x2F, 0xf4]; // DAS, HLT
    emu.regs_mut().rax = 0x00A5;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL = 0xA5 - 0x60 = 0x45
    assert_eq!(emu.regs().rax & 0xFF, 0x45, "AL should be 0x45 after adjustment");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_das_both_nibbles_adjustment() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0xAF (both nibbles > 9)
    let code = [0x2F, 0xf4]; // DAS, HLT
    emu.regs_mut().rax = 0x00AF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL = 0xAF - 6 - 0x60 = 0x49
    assert_eq!(emu.regs().rax & 0xFF, 0x49, "AL should be 0x49 after adjustment");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_das_af_set_triggers_adjustment() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x05 (valid), but AF is set from previous operation
    let code = [0x2F, 0xf4]; // DAS, HLT
    emu.regs_mut().rax = 0x0005;
    emu.flags_mut().load(0x10); // Set AF flag
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL = 0x05 - 6 = 0xFF (wraps around)
    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "AL should wrap to 0xFF");
    assert!(emu.flags().f_af, "AF should remain set");
}

#[test]
fn test_das_cf_set_triggers_adjustment() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x45 (valid), but CF is set from previous operation
    let code = [0x2F, 0xf4]; // DAS, HLT
    emu.regs_mut().rax = 0x0045;
    emu.flags_mut().load(0x01); // Set CF flag
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL = 0x45 - 0x60 = 0xE5 (wraps around)
    assert_eq!(emu.regs().rax & 0xFF, 0xE5, "AL should be 0xE5");
    assert!(emu.flags().f_cf, "CF should remain set");
}

#[test]
fn test_das_bcd_subtraction_no_borrow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x58 - 0x23 = 0x35 (valid BCD, no adjustment needed)
    let code = [0x2F, 0xf4]; // DAS, HLT
    emu.regs_mut().rax = 0x0035;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x35, "Result should remain 0x35");
}

#[test]
fn test_das_bcd_subtraction_with_adjustment() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2F, 0xf4]; // DAS, HLT
    emu.regs_mut().rax = 0x005C; // Invalid BCD result from subtraction
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL = 0x5C - 6 = 0x56
    assert_eq!(emu.regs().rax & 0xFF, 0x56, "Result should be 0x56");
}

#[test]
fn test_das_zero_result() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2F, 0xf4]; // DAS, HLT
    emu.regs_mut().rax = 0x0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should remain 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_das_preserves_high_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2F, 0xf4]; // DAS, HLT
    emu.regs_mut().rax = 0xDEADBEEF_12345625;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax >> 8, 0xDEADBEEF_123456, "High bits should be preserved");
}

#[test]
fn test_das_all_valid_bcd_pairs() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for high in 0..=9 {
        for low in 0..=9 {
            let val = (high << 4) | low;
            let code = [0x2F, 0xf4]; // DAS, HLT
            emu.regs_mut().rax = val;
            emu.load_code_bytes(&code);
    emu.run(None).unwrap();

            // Valid BCD values should not be adjusted
            assert_eq!(emu.regs().rax & 0xFF, val,
                "Valid BCD 0x{:02X} should not be adjusted", val);
        }
    }
}

// ============================================================================
// DAA/DAS Combined Tests
// ============================================================================

#[test]
fn test_daa_das_sequence() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x27, // DAA
        0x2F, // DAS
        0xf4, // HLT
    ];
    emu.regs_mut().rax = 0x0025; // Valid BCD
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x25, "Result should remain 0x25");
}

#[test]
fn test_multiple_daa_operations() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x27, // DAA
        0x27, // DAA
        0x27, // DAA
        0xf4, // HLT
    ];
    emu.regs_mut().rax = 0x0045; // Valid BCD
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x45, "Result should remain 0x45");
}

#[test]
fn test_multiple_das_operations() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x2F, // DAS
        0x2F, // DAS
        0x2F, // DAS
        0xf4, // HLT
    ];
    emu.regs_mut().rax = 0x0067; // Valid BCD
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x67, "Result should remain 0x67");
}

// ============================================================================
// Edge Cases and Special Scenarios
// ============================================================================

#[test]
fn test_daa_max_value() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x27, 0xf4]; // DAA, HLT
    emu.regs_mut().rax = 0x00FF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0xFF + 6 + 0x60 = 0x165, AL = 0x65 with CF
    assert_eq!(emu.regs().rax & 0xFF, 0x65, "AL should be 0x65");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_das_max_value() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2F, 0xf4]; // DAS, HLT
    emu.regs_mut().rax = 0x00FF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0xFF - 6 - 0x60 = 0x99
    assert_eq!(emu.regs().rax & 0xFF, 0x99, "AL should be 0x99");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_daa_sign_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x27, 0xf4]; // DAA, HLT
    emu.regs_mut().rax = 0x0080; // Bit 7 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_sf, "SF should be set when bit 7 is set");
}

#[test]
fn test_das_sign_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2F, 0xf4]; // DAS, HLT
    emu.regs_mut().rax = 0x0090;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_sf, "SF should be set when result has bit 7 set");
}

#[test]
fn test_daa_parity_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x27, 0xf4]; // DAA, HLT
    emu.regs_mut().rax = 0x0003; // Will become 0x03 (even parity)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03, "AL should be 0x03");
    assert!(emu.flags().f_pf, "PF should be set for even parity");
}

#[test]
fn test_das_parity_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2F, 0xf4]; // DAS, HLT
    emu.regs_mut().rax = 0x0003; // Will become 0x03 (even parity)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03, "AL should be 0x03");
    assert!(emu.flags().f_pf, "PF should be set for even parity");
}

#[test]
fn test_daa_with_both_flags_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x27, 0xf4]; // DAA, HLT
    emu.regs_mut().rax = 0x0023;
    emu.flags_mut().load(0x11); // Set both AF and CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AF set: +6 -> 0x29, CF set: +0x60 -> 0x89
    assert_eq!(emu.regs().rax & 0xFF, 0x89, "AL should be 0x89");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_das_with_both_flags_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2F, 0xf4]; // DAS, HLT
    emu.regs_mut().rax = 0x0089;
    emu.flags_mut().load(0x11); // Set both AF and CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AF set: -6 -> 0x83, CF set: -0x60 -> 0x23
    assert_eq!(emu.regs().rax & 0xFF, 0x23, "AL should be 0x23");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_daa_bcd_chain_99_plus_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x99 + 0x01 = 0x9A, DAA -> 0x00 with CF
    let code = [0x27, 0xf4]; // DAA, HLT
    emu.regs_mut().rax = 0x009A;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should be 0x00 (overflow to 100)");
    assert!(emu.flags().f_cf, "CF should be set (carry to next byte)");
}

#[test]
fn test_daa_sequential_additions() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = vec![
        (0x15, 0x15), // 15 (valid BCD)
        (0x19, 0x19), // 19 (valid BCD)
        (0x1E, 0x24), // 1E -> 24 (adjustment needed)
        (0x2A, 0x30), // 2A -> 30 (adjustment needed)
    ];

    for (input, expected) in test_cases {
        let code = [0x27, 0xf4]; // DAA, HLT
        emu.regs_mut().rax = input;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFF, expected,
            "DAA(0x{:02X}) should be 0x{:02X}", input, expected);
    }
}

#[test]
fn test_das_sequential_subtractions() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = vec![
        (0x35, 0x35), // 35 (valid BCD)
        (0x42, 0x42), // 42 (valid BCD)
        (0x1F, 0x19), // 1F -> 19 (adjustment needed)
        (0xA0, 0x40), // A0 -> 40 (adjustment needed)
    ];

    for (input, expected) in test_cases {
        let code = [0x2F, 0xf4]; // DAS, HLT
        emu.regs_mut().rax = input;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFF, expected,
            "DAS(0x{:02X}) should be 0x{:02X}", input, expected);
    }
}
