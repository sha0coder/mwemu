use crate::*;

// LZCNT - Count Leading Zero Bits
// Counts the number of leading zero bits in the source operand.
// The count is written to the destination register.
// If the source is zero, the count equals the operand size in bits, and CF is set.
// If the source is non-zero, CF is cleared and ZF reflects whether the count is zero.
//
// Opcodes:
// F3 0F BD /r    LZCNT r16, r/m16    - Count leading zeros in r/m16
// F3 0F BD /r    LZCNT r32, r/m32    - Count leading zeros in r/m32
// F3 REX.W 0F BD /r LZCNT r64, r/m64 - Count leading zeros in r/m64

#[test]
fn test_lzcnt_ax_bx_all_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT AX, BX - all zeros
    let code = [
        0x66, 0xf3, 0x0f, 0xbd, 0xc3, // LZCNT AX, BX
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
fn test_lzcnt_ax_bx_msb_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT AX, BX - MSB set (no leading zeros)
    let code = [
        0x66, 0xf3, 0x0f, 0xbd, 0xc3, // LZCNT AX, BX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000; // bit 15 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0, "AX should contain 0 (no leading zeros)");
    assert!(!emu.flags().f_cf, "CF should be clear (source is non-zero)");
    assert!(emu.flags().f_zf, "ZF should be set (count is zero)");
}

#[test]
fn test_lzcnt_eax_ebx_all_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT EAX, EBX - all zeros (32-bit)
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 32, "EAX should contain 32 (all bits are zero)");
    assert!(emu.flags().f_cf, "CF should be set (source is zero)");
}

#[test]
fn test_lzcnt_eax_ebx_msb_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT EAX, EBX - MSB set (32-bit)
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000; // bit 31 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should contain 0 (no leading zeros)");
    assert!(!emu.flags().f_cf, "CF should be clear (source is non-zero)");
    assert!(emu.flags().f_zf, "ZF should be set (count is zero)");
}

#[test]
fn test_lzcnt_rax_rbx_all_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT RAX, RBX - all zeros (64-bit)
    let code = [
        0xf3, 0x48, 0x0f, 0xbd, 0xc3, // LZCNT RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 64, "RAX should contain 64 (all bits are zero)");
    assert!(emu.flags().f_cf, "CF should be set (source is zero)");
}

#[test]
fn test_lzcnt_rax_rbx_msb_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT RAX, RBX - MSB set (64-bit)
    let code = [
        0xf3, 0x48, 0x0f, 0xbd, 0xc3, // LZCNT RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000000000000000; // bit 63 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "RAX should contain 0 (no leading zeros)");
    assert!(!emu.flags().f_cf, "CF should be clear (source is non-zero)");
    assert!(emu.flags().f_zf, "ZF should be set (count is zero)");
}

#[test]
fn test_lzcnt_eax_ebx_one_leading_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT with 1 leading zero
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x40000000; // bit 30 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1, "EAX should contain 1 (one leading zero)");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_lzcnt_eax_ebx_multiple_leading_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT with multiple leading zeros
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00FF0000; // bits 16-23 set (8 leading zeros)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 8, "EAX should contain 8 (eight leading zeros)");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_lzcnt_power_of_two() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for bit_pos in 0..32 {
        let code = [
            0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
            0xf4,
        ];
        emu.regs_mut().rbx = 1u64 << bit_pos;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        let expected = 31 - bit_pos;
        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected as u64, "LZCNT(2^{}) should be {}", bit_pos, expected);
    }
}

#[test]
fn test_lzcnt_with_extended_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT R8D, R9D
    let code = [
        0xf3, 0x45, 0x0f, 0xbd, 0xc1, // LZCNT R8D, R9D
        0xf4,
    ];
    emu.regs_mut().r9 = 0x00001000; // bit 12 set (19 leading zeros)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFFFFFFFF, 19, "R8D should contain 19");
}

#[test]
fn test_lzcnt_r15_64bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT R15, R15
    let code = [
        0xf3, 0x4d, 0x0f, 0xbd, 0xff, // LZCNT R15, R15
        0xf4,
    ];
    emu.regs_mut().r15 = 0x0000010000000000; // bit 40 set (23 leading zeros)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 23, "R15 should contain 23");
}

#[test]
fn test_lzcnt_mem16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT AX, [mem]
    let code = [
        0x66, 0xf3, 0x0f, 0xbd, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // LZCNT AX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0x0100); // bit 8 set (7 leading zeros)
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 7, "AX should contain 7");
}

#[test]
fn test_lzcnt_mem32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT EAX, [mem]
    let code = [
        0xf3, 0x0f, 0xbd, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // LZCNT EAX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x00010000); // bit 16 set (15 leading zeros)
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 15, "EAX should contain 15");
}

#[test]
fn test_lzcnt_mem64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT RAX, [mem]
    let code = [
        0xf3, 0x48, 0x0f, 0xbd, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // LZCNT RAX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x0000100000000000); // bit 44 set (19 leading zeros)
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 19, "RAX should contain 19");
}

#[test]
fn test_lzcnt_preserves_source() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT should not modify source operand
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
}

#[test]
fn test_lzcnt_all_ones() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT with all bits set
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should contain 0 (no leading zeros)");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_lzcnt_single_bit_patterns() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = vec![
        (0x00000001, 31), // bit 0
        (0x00000002, 30), // bit 1
        (0x00000004, 29), // bit 2
        (0x00000008, 28), // bit 3
        (0x00000010, 27), // bit 4
        (0x00000100, 23), // bit 8
        (0x00010000, 15), // bit 16
        (0x01000000, 7),  // bit 24
    ];

    for (value, expected) in test_cases {
        let code = [
            0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
            0xf4,
        ];
        emu.regs_mut().rbx = value;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "LZCNT(0x{:08X}) should be {}", value, expected);
    }
}

#[test]
fn test_lzcnt_alternating_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT with alternating pattern
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xAAAAAAAA; // 1010...1010 (MSB set)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should contain 0 (MSB is set)");
}

#[test]
fn test_lzcnt_consecutive_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT with consecutive bits set
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x000FFFFF; // bits 0-19 set (12 leading zeros)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 12, "EAX should contain 12");
}

#[test]
fn test_lzcnt_64bit_high_bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT in 64-bit with high bit set
    let code = [
        0xf3, 0x48, 0x0f, 0xbd, 0xc3, // LZCNT RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0800000000000000; // bit 59 set (4 leading zeros)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 4, "RAX should contain 4");
}

#[test]
fn test_lzcnt_sparse_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT with sparse bits
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00100001; // bits 0 and 20 set (11 leading zeros)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 11, "EAX should contain 11");
}

#[test]
fn test_lzcnt_byte_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = vec![
        (0x000000FF, 24), // lower byte
        (0x0000FF00, 16), // second byte
        (0x00FF0000, 8),  // third byte
        (0xFF000000, 0),  // upper byte
    ];

    for (value, expected) in test_cases {
        let code = [
            0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
            0xf4,
        ];
        emu.regs_mut().rbx = value;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "LZCNT(0x{:08X}) should be {}", value, expected);
    }
}

#[test]
fn test_lzcnt_complement_of_tzcnt_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x08000000; // bit 27 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 4, "EAX should contain 4 (31 - 27 = 4)");
}

#[test]
fn test_lzcnt_64bit_lower_half() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT 64-bit with only lower 32 bits
    let code = [
        0xf3, 0x48, 0x0f, 0xbd, 0xc3, // LZCNT RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0000000080000000; // bit 31 set (32 leading zeros)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 32, "RAX should contain 32");
}

#[test]
fn test_lzcnt_64bit_upper_half() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // LZCNT 64-bit with only upper 32 bits
    let code = [
        0xf3, 0x48, 0x0f, 0xbd, 0xc3, // LZCNT RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0100000000000000; // bit 56 set (7 leading zeros)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 7, "RAX should contain 7");
}
