use crate::*;

// TZCNT - Count Trailing Zero Bits
// Counts the number of trailing zero bits in the source operand.
// The count is written to the destination register.
// If the source is zero, the count equals the operand size in bits, and CF is set.
// If the source is non-zero, CF is cleared and ZF reflects whether the count is zero.
//
// Opcodes:
// F3 0F BC /r    TZCNT r16, r/m16    - Count trailing zeros in r/m16
// F3 0F BC /r    TZCNT r32, r/m32    - Count trailing zeros in r/m32
// F3 REX.W 0F BC /r TZCNT r64, r/m64 - Count trailing zeros in r/m64

#[test]
fn test_tzcnt_ax_bx_all_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT AX, BX - all zeros
    let code = [
        0x66, 0xf3, 0x0f, 0xbc, 0xc3, // TZCNT AX, BX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 16, "AX should contain 16 (all bits are zero)");
    assert!(emu.flags().f_cf, "CF should be set (source is zero)");
    assert!(!emu.flags().f_zf, "ZF should be clear (count is non-zero)");
}

#[test]
fn test_tzcnt_ax_bx_lsb_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT AX, BX - LSB set (no trailing zeros)
    let code = [
        0x66, 0xf3, 0x0f, 0xbc, 0xc3, // TZCNT AX, BX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0001; // bit 0 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0, "AX should contain 0 (no trailing zeros)");
    assert!(!emu.flags().f_cf, "CF should be clear (source is non-zero)");
    assert!(emu.flags().f_zf, "ZF should be set (count is zero)");
}

#[test]
fn test_tzcnt_eax_ebx_all_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT EAX, EBX - all zeros (32-bit)
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 32, "EAX should contain 32 (all bits are zero)");
    assert!(emu.flags().f_cf, "CF should be set (source is zero)");
}

#[test]
fn test_tzcnt_eax_ebx_lsb_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT EAX, EBX - LSB set (32-bit)
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00000001; // bit 0 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should contain 0 (no trailing zeros)");
    assert!(!emu.flags().f_cf, "CF should be clear (source is non-zero)");
    assert!(emu.flags().f_zf, "ZF should be set (count is zero)");
}

#[test]
fn test_tzcnt_rax_rbx_all_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT RAX, RBX - all zeros (64-bit)
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 64, "RAX should contain 64 (all bits are zero)");
    assert!(emu.flags().f_cf, "CF should be set (source is zero)");
}

#[test]
fn test_tzcnt_rax_rbx_lsb_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT RAX, RBX - LSB set (64-bit)
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0000000000000001; // bit 0 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "RAX should contain 0 (no trailing zeros)");
    assert!(!emu.flags().f_cf, "CF should be clear (source is non-zero)");
    assert!(emu.flags().f_zf, "ZF should be set (count is zero)");
}

#[test]
fn test_tzcnt_eax_ebx_one_trailing_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT with 1 trailing zero
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00000002; // bit 1 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1, "EAX should contain 1 (one trailing zero)");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_tzcnt_eax_ebx_multiple_trailing_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT with multiple trailing zeros
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFF0000; // bits 16-31 set (16 trailing zeros)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 16, "EAX should contain 16 (sixteen trailing zeros)");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_tzcnt_power_of_two() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for bit_pos in 0..32 {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        emu.regs_mut().rbx = 1u64 << bit_pos;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, bit_pos as u64, "TZCNT(2^{}) should be {}", bit_pos, bit_pos);
    }
}

#[test]
fn test_tzcnt_with_extended_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT R8D, R9D
    let code = [
        0xf3, 0x45, 0x0f, 0xbc, 0xc1, // TZCNT R8D, R9D
        0xf4,
    ];
    emu.regs_mut().r9 = 0x00001000; // bit 12 set (12 trailing zeros)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFFFFFFFF, 12, "R8D should contain 12");
}

#[test]
fn test_tzcnt_r15_64bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT R15, R15
    let code = [
        0xf3, 0x4d, 0x0f, 0xbc, 0xff, // TZCNT R15, R15
        0xf4,
    ];
    emu.regs_mut().r15 = 0x0100000000000000; // bit 56 set (56 trailing zeros)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 56, "R15 should contain 56");
}

#[test]
fn test_tzcnt_mem16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT AX, [mem]
    let code = [
        0x66, 0xf3, 0x0f, 0xbc, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // TZCNT AX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0x0100); // bit 8 set (8 trailing zeros)
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 8, "AX should contain 8");
}

#[test]
fn test_tzcnt_mem32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT EAX, [mem]
    let code = [
        0xf3, 0x0f, 0xbc, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // TZCNT EAX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x00010000); // bit 16 set (16 trailing zeros)
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 16, "EAX should contain 16");
}

#[test]
fn test_tzcnt_mem64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT RAX, [mem]
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // TZCNT RAX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x0100000000000000); // bit 56 set (56 trailing zeros)
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 56, "RAX should contain 56");
}

#[test]
fn test_tzcnt_preserves_source() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT should not modify source operand
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
}

#[test]
fn test_tzcnt_all_ones() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT with all bits set
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should contain 0 (no trailing zeros)");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_tzcnt_single_bit_patterns() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = vec![
        (0x00000001, 0),  // bit 0
        (0x00000002, 1),  // bit 1
        (0x00000004, 2),  // bit 2
        (0x00000008, 3),  // bit 3
        (0x00000010, 4),  // bit 4
        (0x00000100, 8),  // bit 8
        (0x00010000, 16), // bit 16
        (0x01000000, 24), // bit 24
    ];

    for (value, expected) in test_cases {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        emu.regs_mut().rbx = value;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "TZCNT(0x{:08X}) should be {}", value, expected);
    }
}

#[test]
fn test_tzcnt_alternating_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT with alternating pattern
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xAAAAAAAA; // 1010...1010 (bit 1 is LSB set bit)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1, "EAX should contain 1 (one trailing zero)");
}

#[test]
fn test_tzcnt_consecutive_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT with consecutive bits set
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFF00000; // bits 20-31 set (20 trailing zeros)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 20, "EAX should contain 20");
}

#[test]
fn test_tzcnt_64bit_low_bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT in 64-bit with low bit set
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0000000000000020; // bit 5 set (5 trailing zeros)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 5, "RAX should contain 5");
}

#[test]
fn test_tzcnt_sparse_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT with sparse bits
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80001000; // bits 12 and 31 set (12 trailing zeros)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 12, "EAX should contain 12");
}

#[test]
fn test_tzcnt_byte_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = vec![
        (0x000000FF, 0),  // lower byte (LSB set)
        (0x0000FF00, 8),  // second byte
        (0x00FF0000, 16), // third byte
        (0xFF000000, 24), // upper byte
    ];

    for (value, expected) in test_cases {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        emu.regs_mut().rbx = value;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "TZCNT(0x{:08X}) should be {}", value, expected);
    }
}

#[test]
fn test_tzcnt_vs_bsf_similarity() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT and BSF give same result for non-zero values
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00FF0000; // bits 16-23 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 16, "TZCNT should find first set bit at position 16");
}

#[test]
fn test_tzcnt_64bit_lower_half() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT 64-bit with bit in lower 32 bits
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0000000000008000; // bit 15 set (15 trailing zeros)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 15, "RAX should contain 15");
}

#[test]
fn test_tzcnt_64bit_upper_half() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT 64-bit with bit only in upper 32 bits
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0100000000000000; // bit 56 set (56 trailing zeros)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 56, "RAX should contain 56");
}

#[test]
fn test_tzcnt_alignment_detection() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TZCNT can detect alignment (power of 2 divisibility)
    let test_cases = vec![
        (0x00000001, 0), // 2^0 aligned
        (0x00000002, 1), // 2^1 aligned
        (0x00000004, 2), // 2^2 aligned
        (0x00000008, 3), // 2^3 aligned
        (0x00001000, 12), // 2^12 aligned (4KB)
    ];

    for (value, expected) in test_cases {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        emu.regs_mut().rbx = value;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "Value 0x{:08X} is 2^{} aligned", value, expected);
    }
}

#[test]
fn test_tzcnt_odd_numbers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let odd_values = vec![1, 3, 5, 7, 9, 11, 13, 15];

    for value in odd_values {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        emu.regs_mut().rbx = value;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "Odd number {} should have 0 trailing zeros", value);
    }
}

#[test]
fn test_tzcnt_even_numbers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = vec![
        (2, 1),   // one trailing zero
        (4, 2),   // two trailing zeros
        (6, 1),   // one trailing zero
        (8, 3),   // three trailing zeros
        (12, 2),  // two trailing zeros
        (16, 4),  // four trailing zeros
    ];

    for (value, expected) in test_cases {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        emu.regs_mut().rbx = value;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "Even number {} should have {} trailing zeros", value, expected);
    }
}
