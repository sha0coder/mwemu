use crate::*;

// AAD — ASCII Adjust AX Before Division
//
// Opcode: D5 0A
// Instruction: AAD
// Op/En: ZO (no operands)
// 64-bit Mode: Invalid
// Compat/Leg Mode: Valid
//
// Also: D5 ib - AAD imm8 (adjust to number base imm8)
//
// Description:
// Adjusts two unpacked BCD digits (the least-significant digit in the AL register
// and the most-significant digit in the AH register) so that a division operation
// performed on the result will yield a correct unpacked BCD value. AAD is only
// useful when it precedes a DIV instruction that divides (binary division) the
// adjusted value in the AX register by an unpacked BCD value.
//
// Operation:
// tempAL := AL;
// tempAH := AH;
// AL := (tempAL + (tempAH * imm8)) AND FFH;  (* imm8 is set to 0AH for AAD mnemonic *)
// AH := 0;
//
// Flags Affected:
// SF, ZF, and PF are set according to the resulting binary value in the AL register.
// OF, AF, and CF are undefined.

// ============================================================================
// AAD - Basic Cases
// ============================================================================

#[test]
fn test_aad_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0000, result: AL = 0, AH = 0
    let code = [
        0xd5, 0x0a, // AAD
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0x0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should be 0");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
    assert!(emu.flags().f_zf, "ZF should be set");
    assert!(!emu.flags().f_sf, "SF should be clear");
}

#[test]
fn test_aad_single_digit_in_al() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0005 (AH=0, AL=5), result: AL = 5, AH = 0
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0005;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x05, "AL should be 5");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_single_digit_in_ah() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0300 (AH=3, AL=0), result: AL = 30, AH = 0
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0300;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 30, "AL should be 30 (3 * 10)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

// ============================================================================
// AAD - Two Digit BCD Numbers
// ============================================================================

#[test]
fn test_aad_12() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0102 (BCD 12), result: AL = 12, AH = 0
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0102;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 12, "AL should be 12");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_25() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0205 (BCD 25), result: AL = 25, AH = 0
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0205;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 25, "AL should be 25");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_37() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0307 (BCD 37), result: AL = 37, AH = 0
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0307;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 37, "AL should be 37");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_58() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0508 (BCD 58), result: AL = 58, AH = 0
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0508;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 58, "AL should be 58");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_99() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0909 (BCD 99), result: AL = 99, AH = 0
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0909;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 99, "AL should be 99");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

// ============================================================================
// AAD - Multiples of 10
// ============================================================================

#[test]
fn test_aad_10() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0100 (BCD 10), result: AL = 10, AH = 0
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0100;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 10, "AL should be 10");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_20() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0200 (BCD 20), result: AL = 20, AH = 0
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0200;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 20, "AL should be 20");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_50() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0500 (BCD 50), result: AL = 50, AH = 0
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0500;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 50, "AL should be 50");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_90() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0900 (BCD 90), result: AL = 90, AH = 0
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0900;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 90, "AL should be 90");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

// ============================================================================
// AAD - Realistic Division Setup Examples
// ============================================================================

#[test]
fn test_aad_before_div_58_by_4() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb8, 0x08, 0x05, // MOV AX, 0x0508 (BCD 58)
        0xd5, 0x0a,       // AAD (convert to binary 58)
        0xb3, 0x04,       // MOV BL, 4
        0xf6, 0xf3,       // DIV BL (AL = quotient, AH = remainder)
        0xf4,             // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 14, "Quotient should be 14");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 2, "Remainder should be 2");
}

#[test]
fn test_aad_before_div_72_by_8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb8, 0x02, 0x07, // MOV AX, 0x0702 (BCD 72)
        0xd5, 0x0a,       // AAD (convert to binary 72)
        0xb3, 0x08,       // MOV BL, 8
        0xf6, 0xf3,       // DIV BL
        0xf4,             // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 9, "Quotient should be 9");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0, "Remainder should be 0");
}

#[test]
fn test_aad_before_div_37_by_5() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb8, 0x07, 0x03, // MOV AX, 0x0307 (BCD 37)
        0xd5, 0x0a,       // AAD (convert to binary 37)
        0xb3, 0x05,       // MOV BL, 5
        0xf6, 0xf3,       // DIV BL
        0xf4,             // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 7, "Quotient should be 7");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 2, "Remainder should be 2");
}

#[test]
fn test_aad_before_div_99_by_9() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xb8, 0x09, 0x09, // MOV AX, 0x0909 (BCD 99)
        0xd5, 0x0a,       // AAD (convert to binary 99)
        0xb3, 0x09,       // MOV BL, 9
        0xf6, 0xf3,       // DIV BL
        0xf4,             // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 11, "Quotient should be 11");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0, "Remainder should be 0");
}

// ============================================================================
// AAD - Custom Bases (imm8)
// ============================================================================

#[test]
fn test_aad_base_2() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0301 (3*2 + 1 = 7 in base 2), result: AL = 7
    let code = [
        0xd5, 0x02, // AAD 2
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0x0301;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 7, "AL should be 7 (3*2 + 1)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_base_8_octal() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0305 (3*8 + 5 = 29 in base 8), result: AL = 29
    let code = [
        0xd5, 0x08, // AAD 8 (octal)
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0x0305;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 29, "AL should be 29 (3*8 + 5)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_base_12() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0407 (4*12 + 7 = 55 in base 12), result: AL = 55
    let code = [
        0xd5, 0x0c, // AAD 12
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0x0407;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 55, "AL should be 55 (4*12 + 7)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_base_16_hex() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0A0B (10*16 + 11 = 171 in base 16), result: AL = 171
    let code = [
        0xd5, 0x10, // AAD 16 (hex)
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0x0A0B;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 171, "AL should be 171 (10*16 + 11)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_base_7() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0506 (5*7 + 6 = 41 in base 7), result: AL = 41
    let code = [
        0xd5, 0x07, // AAD 7
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0x0506;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 41, "AL should be 41 (5*7 + 6)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

// ============================================================================
// AAD - Overflow Cases (AL > 255)
// ============================================================================

#[test]
fn test_aad_overflow_truncated() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x1905 (25*10 + 5 = 255), should fit exactly
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x1905;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 255, "AL should be 255");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_overflow_wraps() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x1A00 (26*10 = 260), should wrap: 260 & 0xFF = 4
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x1A00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 4, "AL should be 4 (260 & 0xFF)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_large_value() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0xFF09 (255*10 + 9 = 2559), should wrap: 2559 & 0xFF = 255
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0xFF09;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 255, "AL should be 255 (2559 & 0xFF)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

// ============================================================================
// AAD - Flag Testing
// ============================================================================

#[test]
fn test_aad_flags_zero_result() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0000, result AL = 0 (should set ZF)
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should be 0");
    assert!(emu.flags().f_zf, "ZF should be set");
    assert!(!emu.flags().f_sf, "SF should be clear");
}

#[test]
fn test_aad_flags_nonzero_result() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0205, result AL = 25 (should clear ZF)
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0205;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 25, "AL should be 25");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_aad_parity_flag_even() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0102, result AL = 12 (even parity)
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0102;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 12, "AL should be 12");
    assert!(emu.flags().f_pf, "PF should be set for even parity");
}

#[test]
fn test_aad_parity_flag_odd() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0101, result AL = 11 (odd parity)
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0101;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 11, "AL should be 11");
    assert!(!emu.flags().f_pf, "PF should be clear for odd parity");
}

// ============================================================================
// AAD - Edge Cases
// ============================================================================

#[test]
fn test_aad_preserves_high_rax() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x1234_5678_DEAD_0205;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax >> 16, 0x1234_5678_DEAD, "High bits of RAX should be preserved");
    assert_eq!(emu.regs().rax & 0xFF, 25, "AL should be 25");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_all_two_digit_bcd() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for tens in 0..=9 {
        for ones in 0..=9 {
            let bcd = (tens << 8) | ones;
            let expected = tens * 10 + ones;

            let code = [0xd5, 0x0a, 0xf4];
            emu.regs_mut().rax = bcd;
            emu.load_code_bytes(&code);
    emu.run(None).unwrap();

            assert_eq!(emu.regs().rax & 0xFF, expected, "AL should be {} for BCD {}{}", expected, tens, ones);
            assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0 for BCD {}{}", tens, ones);
        }
    }
}

#[test]
fn test_aad_al_only() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for al_val in 0..=9 {
        let code = [0xd5, 0x0a, 0xf4];
        emu.regs_mut().rax = al_val;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFF, al_val, "AL should remain {}", al_val);
        assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
    }
}

#[test]
fn test_aad_ah_only() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for ah_val in 0..=9 {
        let code = [0xd5, 0x0a, 0xf4];
        emu.regs_mut().rax = ah_val << 8;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFF, ah_val * 10, "AL should be {}", ah_val * 10);
        assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
    }
}

#[test]
fn test_aad_with_different_bases() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = [
        (2, 3 * 2 + 4),     // base 2: 3*2 + 4 = 10
        (5, 3 * 5 + 4),     // base 5: 3*5 + 4 = 19
        (10, 3 * 10 + 4),   // base 10: 3*10 + 4 = 34
        (12, 3 * 12 + 4),   // base 12: 3*12 + 4 = 40
        (16, 3 * 16 + 4),   // base 16: 3*16 + 4 = 52
    ];

    for (base, expected) in test_cases.iter() {
        let code = [0xd5, *base, 0xf4];
        emu.regs_mut().rax = 0x0304;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFF, *expected as u64, "AL incorrect for base {}", base);
        assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0 for base {}", base);
    }
}

#[test]
fn test_aad_max_ah_max_al() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX = 0x0909 (9*10 + 9 = 99)
    let code = [0xd5, 0x0a, 0xf4];
    emu.regs_mut().rax = 0x0909;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 99, "AL should be 99");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "AH should be 0");
}
