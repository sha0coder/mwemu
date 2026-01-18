use crate::*;

// AAM — ASCII Adjust AX After Multiply
//
// Opcode: D4 0A
// Instruction: AAM
// Op/En: ZO (no operands)
// 64-bit Mode: Invalid
// Compat/Leg Mode: Valid
//
// Also: D4 ib - AAM imm8 (adjust to number base imm8)
//
// Description:
// Adjusts the result of the multiplication of two unpacked BCD values to create
// a pair of unpacked (base 10) BCD values. The AX register is the implied source
// and destination operand. AAM is only useful when it follows a MUL instruction
// that multiplies (binary multiplication) two unpacked BCD values and stores a
// word result in the AX register.
//
// Operation:
// tempAL := AL;
// AH := tempAL / imm8;  (* imm8 is set to 0AH for the AAM mnemonic *)
// AL := tempAL MOD imm8;
//
// Flags Affected:
// SF, ZF, and PF are set according to the resulting binary value in the AL register.
// OF, AF, and CF are undefined.

// ============================================================================
// AAM - Basic Multiplication Results
// ============================================================================

#[test]
fn test_aam_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0, result: AH = 0, AL = 0
    let code = [
        0xd4, 0x0a, // AAM
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should be 0");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
    assert!(emu.flags().f_zf, "ZF should be set");
    assert!(!emu.flags().f_sf, "SF should be clear");
}

#[test]
fn test_aam_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 1, result: AH = 0, AL = 1
    let code = [0xd4, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x01, "AL should be 1");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
    assert!(!emu.flags().f_zf, "ZF should be clear");
    assert!(!emu.flags().f_sf, "SF should be clear");
}

#[test]
fn test_aam_nine() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 9, result: AH = 0, AL = 9
    let code = [0xd4, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x09;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x09, "AL should be 9");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
    assert!(!emu.flags().f_zf, "ZF should be clear");
    assert!(!emu.flags().f_sf, "SF should be clear");
}

#[test]
fn test_aam_ten() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 10 (0x0A), result: AH = 1, AL = 0
    let code = [0xd4, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0A;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should be 0 (10 MOD 10)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x01, "AH should be 1 (10 / 10)");
    assert!(emu.flags().f_zf, "ZF should be set (AL = 0)");
}

// ============================================================================
// AAM - Two Digit Results
// ============================================================================

#[test]
fn test_aam_12() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 12 (0x0C), result: AH = 1, AL = 2
    let code = [0xd4, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0C;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x02, "AL should be 2");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x01, "AH should be 1");
}

#[test]
fn test_aam_25() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 25 (0x19), result: AH = 2, AL = 5
    let code = [0xd4, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x19;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should be 5");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x02, "AH should be 2");
}

#[test]
fn test_aam_45() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 45 (0x2D), result: AH = 4, AL = 5
    let code = [0xd4, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x2D;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should be 5");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x04, "AH should be 4");
}

#[test]
fn test_aam_81() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 81 (0x51 = 9 * 9), result: AH = 8, AL = 1
    let code = [0xd4, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x51;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x01, "AL should be 1");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x08, "AH should be 8");
}

#[test]
fn test_aam_99() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 99 (0x63), result: AH = 9, AL = 9
    let code = [0xd4, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x63;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x09, "AL should be 9");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x09, "AH should be 9");
}

// ============================================================================
// AAM - Realistic Multiplication Examples
// ============================================================================

#[test]
fn test_aam_after_mul_3_times_4() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb0, 0x03,       // MOV AL, 3
        0xb3, 0x04,       // MOV BL, 4
        0xf6, 0xe3,       // MUL BL
        0xd4, 0x0a,       // AAM
        0xf4,             // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x02, "Ones digit should be 2");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x01, "Tens digit should be 1");
}

#[test]
fn test_aam_after_mul_5_times_6() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb0, 0x05,       // MOV AL, 5
        0xb3, 0x06,       // MOV BL, 6
        0xf6, 0xe3,       // MUL BL
        0xd4, 0x0a,       // AAM
        0xf4,             // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "Ones digit should be 0");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x03, "Tens digit should be 3");
}

#[test]
fn test_aam_after_mul_7_times_8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb0, 0x07,       // MOV AL, 7
        0xb3, 0x08,       // MOV BL, 8
        0xf6, 0xe3,       // MUL BL
        0xd4, 0x0a,       // AAM
        0xf4,             // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x06, "Ones digit should be 6");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x05, "Tens digit should be 5");
}

#[test]
fn test_aam_after_mul_9_times_9() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb0, 0x09,       // MOV AL, 9
        0xb3, 0x09,       // MOV BL, 9
        0xf6, 0xe3,       // MUL BL
        0xd4, 0x0a,       // AAM
        0xf4,             // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x01, "Ones digit should be 1");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x08, "Tens digit should be 8");
}

#[test]
fn test_aam_after_mul_8_times_7() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb0, 0x08,       // MOV AL, 8
        0xb3, 0x07,       // MOV BL, 7
        0xf6, 0xe3,       // MUL BL
        0xd4, 0x0a,       // AAM
        0xf4,             // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x06, "Ones digit should be 6");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x05, "Tens digit should be 5");
}

// ============================================================================
// AAM - Large Values
// ============================================================================

#[test]
fn test_aam_100() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 100 (0x64), result: AH = 10, AL = 0
    let code = [0xd4, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should be 0");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x0A, "AH should be 10");
}

#[test]
fn test_aam_127() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 127 (0x7F), result: AH = 12, AL = 7
    let code = [0xd4, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x7F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x07, "AL should be 7");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x0C, "AH should be 12");
}

#[test]
fn test_aam_200() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 200 (0xC8), result: AH = 20, AL = 0
    let code = [0xd4, 0x0a, 0xf4];
    emu.regs_mut().rax = 0xC8;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should be 0");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x14, "AH should be 20");
}

#[test]
fn test_aam_255() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 255 (0xFF), result: AH = 25, AL = 5
    let code = [0xd4, 0x0a, 0xf4];
    emu.regs_mut().rax = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should be 5");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x19, "AH should be 25");
}

// ============================================================================
// AAM - Custom Bases (imm8)
// ============================================================================

#[test]
fn test_aam_base_2() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 5, base 2: AH = 2, AL = 1 (5 = 2*2 + 1)
    let code = [
        0xd4, 0x02, // AAM 2
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0x05;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x01, "AL should be 1");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x02, "AH should be 2");
}

#[test]
fn test_aam_base_8_octal() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 25 (0x19), base 8: AH = 3, AL = 1 (25 = 3*8 + 1)
    let code = [
        0xd4, 0x08, // AAM 8 (octal)
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0x19;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x01, "AL should be 1");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x03, "AH should be 3");
}

#[test]
fn test_aam_base_12() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 37 (0x25), base 12: AH = 3, AL = 1 (37 = 3*12 + 1)
    let code = [
        0xd4, 0x0c, // AAM 12
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0x25;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x01, "AL should be 1");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x03, "AH should be 3");
}

#[test]
fn test_aam_base_16_hex() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 0xAB (171), base 16: AH = 10, AL = 11
    let code = [
        0xd4, 0x10, // AAM 16 (hex)
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0xAB;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x0B, "AL should be 11");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x0A, "AH should be 10");
}

#[test]
fn test_aam_base_7() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 50 (0x32), base 7: AH = 7, AL = 1 (50 = 7*7 + 1)
    let code = [
        0xd4, 0x07, // AAM 7
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0x32;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x01, "AL should be 1");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x07, "AH should be 7");
}

// ============================================================================
// AAM - Flag Testing
// ============================================================================

#[test]
fn test_aam_flags_zero_result() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 10, result AL = 0 (should set ZF)
    let code = [0xd4, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0A;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should be 0");
    assert!(emu.flags().f_zf, "ZF should be set");
    assert!(!emu.flags().f_sf, "SF should be clear");
}

#[test]
fn test_aam_flags_nonzero_result() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 25, result AL = 5 (should clear ZF)
    let code = [0xd4, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x19;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should be 5");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_aam_parity_flag_even() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 13, result AL = 3 (even parity)
    let code = [0xd4, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0D;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03, "AL should be 3");
    assert!(emu.flags().f_pf, "PF should be set for even parity");
}

#[test]
fn test_aam_parity_flag_odd() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL = 11, result AL = 1 (odd parity)
    let code = [0xd4, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0B;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x01, "AL should be 1");
    assert!(!emu.flags().f_pf, "PF should be clear for odd parity");
}

// ============================================================================
// AAM - Edge Cases
// ============================================================================

#[test]
fn test_aam_preserves_high_rax() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xd4, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x1234_5678_DEAD_BE19;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax >> 16, 0x1234_5678_DEAD, "High bits of RAX should be preserved");
    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should be 5");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x02, "AH should be 2");
}

#[test]
fn test_aam_all_single_digits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for val in 0..=9 {
        let code = [0xd4, 0x0a, 0xf4];
        emu.regs_mut().rax = val;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFF, val, "AL should remain {} for single digit", val);
        assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0 for single digit");
    }
}

#[test]
fn test_aam_multiples_of_10() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for mult in 1..=25 {
        let val = mult * 10;
        let code = [0xd4, 0x0a, 0xf4];
        emu.regs_mut().rax = val;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should be 0 for {}", val);
        assert_eq!((emu.regs().rax >> 8) & 0xFF, mult as u64, "AH should be {} for {}", mult, val);
    }
}

#[test]
fn test_aam_sequential_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for val in 0..=99 {
        let code = [0xd4, 0x0a, 0xf4];
        emu.regs_mut().rax = val;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        let expected_ah = val / 10;
        let expected_al = val % 10;
        assert_eq!(emu.regs().rax & 0xFF, expected_al, "AL should be {} for input {}", expected_al, val);
        assert_eq!((emu.regs().rax >> 8) & 0xFF, expected_ah, "AH should be {} for input {}", expected_ah, val);
    }
}

#[test]
fn test_aam_ignores_initial_ah() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AAM should only use AL, not AH
    let code = [0xd4, 0x0a, 0xf4];
    emu.regs_mut().rax = 0xFF19; // AH = 0xFF, AL = 25
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should be 5 (25 MOD 10)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x02, "AH should be 2 (25 / 10)");
}

#[test]
fn test_aam_with_different_bases_comprehensive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = [
        (2, 0, 100 / 2),    // base 2
        (3, 1, 100 / 3),    // base 3
        (5, 0, 100 / 5),    // base 5
        (10, 0, 10),        // base 10
        (11, 1, 100 / 11),  // base 11
        (20, 0, 5),         // base 20
    ];

    for (base, expected_al, expected_ah) in test_cases.iter() {
        let code = [0xd4, *base, 0xf4];
        emu.regs_mut().rax = 100;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFF, *expected_al as u64, "AL incorrect for base {}", base);
        assert_eq!((emu.regs().rax >> 8) & 0xFF, *expected_ah as u64, "AH incorrect for base {}", base);
    }
}
