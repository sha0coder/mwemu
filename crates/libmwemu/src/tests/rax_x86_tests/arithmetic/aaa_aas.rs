use crate::*;

// AAA — ASCII Adjust After Addition
// AAS — ASCII Adjust After Subtraction
//
// Opcodes:
// - 37       AAA       ASCII adjust AL after addition
// - 3F       AAS       ASCII adjust AL after subtraction
//
// AAA Operation:
//   IF ((AL AND 0FH) > 9) OR (AF = 1) THEN
//     AX := AX + 106H;
//     AF := 1;
//     CF := 1;
//   ELSE
//     AF := 0;
//     CF := 0;
//   FI;
//   AL := AL AND 0FH;
//
// AAS Operation:
//   IF ((AL AND 0FH) > 9) OR (AF = 1) THEN
//     AX := AX - 6;
//     AH := AH - 1;
//     AF := 1;
//     CF := 1;
//   ELSE
//     AF := 0;
//     CF := 0;
//   FI;
//   AL := AL AND 0FH;
//
// Flags: AF and CF are modified. OF, SF, ZF, PF are undefined.

// ============================================================================
// AAA (ASCII Adjust After Addition) Tests
// ============================================================================

#[test]
fn test_aaa_no_adjustment_needed() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 5 (0x05), low nibble <= 9 and AF = 0
    let code = [
        0x37, // AAA
        0xf4, // HLT
    ];
    emu.regs_mut().rax = 0x0005;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should remain 5");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should remain 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_af, "AF should be clear");
}

#[test]
fn test_aaa_adjustment_needed_low_nibble() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x0A (low nibble > 9)
    let code = [0x37, 0xf4]; // AAA, HLT
    emu.regs_mut().rax = 0x000A;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AX = 0x000A + 0x0106 = 0x0110, then AL masked to 0x0F -> 0x00
    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should be 0 after masking");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x01, "AH should be incremented to 1");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_aaa_adjustment_needed_af_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 5, but AF is set (previous operation had auxiliary carry)
    let code = [0x37, 0xf4]; // AAA, HLT
    emu.regs_mut().rax = 0x0005;
    emu.flags_mut().load(0x10); // Set AF flag
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AX = 0x0005 + 0x0106 = 0x010B, then AL masked to 0x0F -> 0x0B
    assert_eq!(emu.regs().rax & 0xFF, 0x0B, "AL should be 0x0B after masking");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x01, "AH should be incremented");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_aaa_all_digits_0_through_9() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for digit in 0..=9 {
        let code = [0x37, 0xf4]; // AAA, HLT
        emu.regs_mut().rax = digit;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFF, digit, "AL should remain {}", digit);
        assert!(!emu.flags().f_cf, "CF should be clear for digit {}", digit);
        assert!(!emu.flags().f_af, "AF should be clear for digit {}", digit);
    }
}

#[test]
fn test_aaa_values_0a_through_0f() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for val in 0x0A..=0x0F {
        let code = [0x37, 0xf4]; // AAA, HLT
        emu.regs_mut().rax = val;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        let expected_al = val.wrapping_add(6) & 0x0F;
        assert_eq!(emu.regs().rax & 0xFF, expected_al, "AL should be masked for value 0x{:02X}", val);
        assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x01, "AH should be 1 for value 0x{:02X}", val);
        assert!(emu.flags().f_cf, "CF should be set for value 0x{:02X}", val);
        assert!(emu.flags().f_af, "AF should be set for value 0x{:02X}", val);
    }
}

#[test]
fn test_aaa_with_initial_ah() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AH = 5, AL = 0x0A
    let code = [0x37, 0xf4]; // AAA, HLT
    emu.regs_mut().rax = 0x050A; // AH=5, AL=0x0A
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AX = 0x050A + 0x0106 = 0x0610, then AL masked to 0x0F -> 0x00
    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should be 0");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x06, "AH should be 6");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_aaa_bcd_addition_example() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x37, 0xf4]; // AAA, HLT
    emu.regs_mut().rax = 0x000F; // Result of 8 + 7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should be 5 (ones digit)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x01, "AH should be 1 (tens digit)");
}

#[test]
fn test_aaa_preserves_high_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x37, 0xf4]; // AAA, HLT
    emu.regs_mut().rax = 0xDEADBEEF_12340A0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax >> 16, 0xDEADBEEF_1234, "High bits should be preserved");
}

#[test]
fn test_aaa_max_al_value() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0xFF (both nibbles high)
    let code = [0x37, 0xf4]; // AAA, HLT
    emu.regs_mut().rax = 0x00FF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x00FF + 0x0106 = 0x0205, AL masked to 0x0F -> 0x05
    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should be 5 after masking");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x02, "AH should be 2");
}

#[test]
fn test_aaa_ah_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AH = 0xFF, AL = 0x0A (causes AH to wrap)
    let code = [0x37, 0xf4]; // AAA, HLT
    emu.regs_mut().rax = 0xFF0A;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0xFF0A + 0x0106 = 0x10010, AX wraps to 0x0010, then AL masked to 0x00
    assert_eq!(emu.regs().rax & 0xFFFF, 0x0000, "AX should wrap and mask to 0");
}

// ============================================================================
// AAS (ASCII Adjust After Subtraction) Tests
// ============================================================================

#[test]
fn test_aas_no_adjustment_needed() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 5 (0x05), low nibble <= 9 and AF = 0
    let code = [
        0x3F, // AAS
        0xf4, // HLT
    ];
    emu.regs_mut().rax = 0x0005;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should remain 5");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should remain 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_af, "AF should be clear");
}

#[test]
fn test_aas_adjustment_needed_low_nibble() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0x0F (low nibble > 9)
    let code = [0x3F, 0xf4]; // AAS, HLT
    emu.regs_mut().rax = 0x000F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL = AL - 6 = 0x0F - 6 = 0x09, then masked to 0x0F -> 0x09
    // AH = AH - 1 = 0x00 - 1 = 0xFF
    assert_eq!(emu.regs().rax & 0xFF, 0x09, "AL should be 9");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0xFF, "AH should be 0xFF (decremented)");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_aas_adjustment_needed_af_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 5, but AF is set
    let code = [0x3F, 0xf4]; // AAS, HLT
    emu.regs_mut().rax = 0x0005;
    emu.flags_mut().load(0x10); // Set AF flag
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL = AL - 6 = 5 - 6 = -1 = 0xFF, masked to 0x0F -> 0x0F
    // AH borrows from AL subtraction, then decrements by 1
    assert_eq!(emu.regs().rax & 0xFF, 0x0F, "AL should be 0x0F");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0xFE, "AH should be decremented with borrow");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_aas_all_digits_0_through_9() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for digit in 0..=9 {
        let code = [0x3F, 0xf4]; // AAS, HLT
        emu.regs_mut().rax = digit;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFF, digit, "AL should remain {}", digit);
        assert!(!emu.flags().f_cf, "CF should be clear for digit {}", digit);
        assert!(!emu.flags().f_af, "AF should be clear for digit {}", digit);
    }
}

#[test]
fn test_aas_values_0a_through_0f() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for val in 0x0A..=0x0F {
        let code = [0x3F, 0xf4]; // AAS, HLT
        emu.regs_mut().rax = val;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        let expected_al = ((val as i8 - 6) as u8) & 0x0F;
        assert_eq!(emu.regs().rax & 0xFF, expected_al as u64, "AL should be adjusted for value 0x{:02X}", val);
        assert_eq!((emu.regs().rax >> 8) & 0xFF, 0xFF, "AH should be 0xFF for value 0x{:02X}", val);
        assert!(emu.flags().f_cf, "CF should be set for value 0x{:02X}", val);
        assert!(emu.flags().f_af, "AF should be set for value 0x{:02X}", val);
    }
}

#[test]
fn test_aas_with_initial_ah() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AH = 5, AL = 0x0A
    let code = [0x3F, 0xf4]; // AAS, HLT
    emu.regs_mut().rax = 0x050A;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL = 0x0A - 6 = 4, masked to 0x0F -> 0x04
    // AH = 5 - 1 = 4
    assert_eq!(emu.regs().rax & 0xFF, 0x04, "AL should be 4");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x04, "AH should be 4");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_aas_bcd_subtraction_example() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3F, 0xf4]; // AAS, HLT
    emu.regs_mut().rax = 0x020C; // Simulating 12 - 6 with intermediate result
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL = 0x0C - 6 = 6, masked to 0x0F -> 0x06
    // AH = 2 - 1 = 1
    assert_eq!(emu.regs().rax & 0xFF, 0x06, "AL should be 6");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x01, "AH should be 1");
}

#[test]
fn test_aas_preserves_high_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3F, 0xf4]; // AAS, HLT
    emu.regs_mut().rax = 0xDEADBEEF_12340F0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax >> 16, 0xDEADBEEF_1234, "High bits should be preserved");
}

#[test]
fn test_aas_zero_ah() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AH = 0, AL = 0x0E (requires decrement of AH)
    let code = [0x3F, 0xf4]; // AAS, HLT
    emu.regs_mut().rax = 0x000E;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL = 0x0E - 6 = 8, masked to 0x0F -> 0x08
    // AH = 0 - 1 = 0xFF (wraps)
    assert_eq!(emu.regs().rax & 0xFF, 0x08, "AL should be 8");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0xFF, "AH should wrap to 0xFF");
}

#[test]
fn test_aas_max_ah() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AH = 0xFF, AL = 0x0B
    let code = [0x3F, 0xf4]; // AAS, HLT
    emu.regs_mut().rax = 0xFF0B;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL = 0x0B - 6 = 5, masked to 0x0F -> 0x05
    // AH = 0xFF - 1 = 0xFE
    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should be 5");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0xFE, "AH should be 0xFE");
}

// ============================================================================
// Sequential AAA/AAS Tests
// ============================================================================

#[test]
fn test_aaa_then_aas() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x37, // AAA
        0x3F, // AAS
        0xf4, // HLT
    ];
    emu.regs_mut().rax = 0x000E;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xFF0E, "Result should be 0xFF0E");
}

#[test]
fn test_multiple_aaa_operations() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x37, // AAA
        0x37, // AAA
        0x37, // AAA
        0xf4, // HLT
    ];
    emu.regs_mut().rax = 0x000F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x03, "AH should be 3");
}

// ============================================================================
// Edge Cases and Corner Cases
// ============================================================================

#[test]
fn test_aaa_with_all_flags_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x37, 0xf4]; // AAA, HLT
    emu.regs_mut().rax = 0x0003;
    emu.flags_mut().load(0xFFF); // Set all flags
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AF was set, so adjustment occurs: 0x0003 + 0x0106 = 0x0109, masked -> 0x0109
    assert_eq!(emu.regs().rax & 0xFF, 0x09, "AL should be 9");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x01, "AH should be 1");
}

#[test]
fn test_aas_with_all_flags_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3F, 0xf4]; // AAS, HLT
    emu.regs_mut().rax = 0x0508;
    emu.flags_mut().load(0xFFF); // Set all flags
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AF was set, so adjustment occurs: AL = 8 - 6 = 2, AH = 5 - 1 = 4
    assert_eq!(emu.regs().rax & 0xFF, 0x02, "AL should be 2");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x04, "AH should be 4");
}

#[test]
fn test_aaa_zero_value() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x37, 0xf4]; // AAA, HLT
    emu.regs_mut().rax = 0x0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0000, "AX should remain 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_af, "AF should be clear");
}

#[test]
fn test_aas_zero_value() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3F, 0xf4]; // AAS, HLT
    emu.regs_mut().rax = 0x0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0000, "AX should remain 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_af, "AF should be clear");
}

#[test]
fn test_aaa_boundary_9_to_10() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x37, 0xf4]; // AAA, HLT

    emu.regs_mut().rax = 0x0009;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0x09, "AL=9 needs no adjustment");
    assert!(!emu.flags().f_af, "AF should be clear for 9");

    emu.regs_mut().rax = 0x000A;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0x0100, "AL=0x0A should adjust to 0x0100");
    assert!(emu.flags().f_af, "AF should be set for 0x0A");
}

#[test]
fn test_aas_boundary_9_to_10() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3F, 0xf4]; // AAS, HLT

    emu.regs_mut().rax = 0x0509;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0x0509, "AL=9 needs no adjustment");
    assert!(!emu.flags().f_af, "AF should be clear for 9");

    emu.regs_mut().rax = 0x050A;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0x0404, "AL=0x0A should adjust");
    assert!(emu.flags().f_af, "AF should be set for 0x0A");
}

#[test]
fn test_aaa_unpacked_bcd_chain() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x37, // AAA on first result
        0xf4, // HLT
    ];
    emu.regs_mut().rax = 0x0011; // 9 + 8 = 17 (0x11 in hex, adjust needed)
    emu.flags_mut().load(0x10); // Set AF to reflect carry from prior addition
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x0011 + 0x0106 = 0x0117, masked -> 0x0107
    assert_eq!(emu.regs().rax & 0xFF, 0x07, "AL should be 7 (ones digit)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x01, "AH should be 1 (tens digit)");
}

#[test]
fn test_aas_unpacked_bcd_chain() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x3F, // AAS
        0xf4, // HLT
    ];
    emu.regs_mut().rax = 0x030C; // Intermediate result needing adjustment
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL = 0x0C - 6 = 6, AH = 3 - 1 = 2
    assert_eq!(emu.regs().rax & 0xFF, 0x06, "AL should be 6");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x02, "AH should be 2");
}

#[test]
fn test_aaa_masking_high_nibble() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x37, 0xf4]; // AAA, HLT
    emu.regs_mut().rax = 0x00F5; // High nibble = F, low nibble = 5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should be 5 (high nibble masked)");
}

#[test]
fn test_aas_masking_high_nibble() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3F, 0xf4]; // AAS, HLT
    emu.regs_mut().rax = 0x05F8; // High nibble = F, low nibble = 8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x08, "AL should be 8 (high nibble masked)");
}
