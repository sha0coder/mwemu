use crate::*;

// BLSI - Extract Lowest Set Isolated Bit (BMI1)
// Extracts the lowest set bit from the source operand and sets that bit in the destination.
// All other bits in the destination are cleared.
// This is equivalent to: dest = src & -src
// ZF is set if the result is zero, CF is set if source is non-zero, SF is updated based on result, OF is cleared.
//
// Opcodes:
// VEX.NDD.LZ.0F38.W0 F3 /3   BLSI r32, r/m32   - Extract lowest set bit
// VEX.NDD.LZ.0F38.W1 F3 /3   BLSI r64, r/m64   - Extract lowest set bit

#[test]
fn test_blsi_eax_ebx_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI EAX, EBX - extract lowest bit (bit 0)
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b0000_0001; // bit 0 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0b0000_0001, "EAX should contain isolated bit 0");
    assert!(!emu.flags().f_zf, "ZF should be clear (source is non-zero)");
    assert!(emu.flags().f_cf, "CF should be set (source is non-zero)");
}

#[test]
fn test_blsi_eax_ebx_bit_31() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI EAX, EBX - extract lowest bit (only bit 31 set)
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000; // only bit 31 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x80000000, "EAX should contain isolated bit 31");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsi_rax_rbx_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI RAX, RBX - 64-bit version
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xdb, // BLSI RAX, RBX (W1)
        0xf4,
    ];
    emu.regs_mut().rbx = 0b0000_0001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0b0000_0001, "RAX should contain isolated bit 0");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsi_rax_rbx_bit_63() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI RAX, RBX - extract bit 63
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xdb, // BLSI RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000_0000_0000_0000; // only bit 63 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x8000_0000_0000_0000, "RAX should contain isolated bit 63");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsi_zero_source() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI with zero source
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0; // zero source
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should be zero");
    assert!(emu.flags().f_zf, "ZF should be set (source is zero)");
    assert!(!emu.flags().f_cf, "CF should be clear (source is zero)");
}

#[test]
fn test_blsi_multiple_bits_isolates_lowest() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI should isolate only the lowest set bit
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b1010_1000; // bits 3, 5, 7 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0b0000_1000, "EAX should contain only bit 3 (lowest)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsi_all_bits_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI with all bits set should isolate bit 0
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1, "EAX should contain bit 0");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsi_alternating_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI with alternating pattern 1010...1010
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xAAAAAAAA; // 1010...1010 (bit 1 is lowest set bit)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0b10, "EAX should contain bit 1");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsi_alternating_pattern_inverted() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI with alternating pattern 0101...0101
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x55555555; // 0101...0101 (bit 0 is lowest set bit)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1, "EAX should contain bit 0");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsi_single_bit_positions() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for bit_pos in 0..32 {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
            0xf4,
        ];
        emu.regs_mut().rbx = 1u64 << bit_pos;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1u64 << bit_pos, "EAX should contain isolated bit {}", bit_pos);
        assert!(!emu.flags().f_zf, "ZF should be clear for bit {}", bit_pos);
    }
}

#[test]
fn test_blsi_with_extended_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI R8D, R9D
    let code = [
        0xc4, 0xc2, 0x38, 0xf3, 0xd9, // BLSI R8D, R9D
        0xf4,
    ];
    emu.regs_mut().r9 = 0b0001_1000; // bits 3 and 4 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFFFFFFFF, 0b0000_1000, "R8D should contain bit 3");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsi_r15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI R15, R15
    let code = [
        0xc4, 0xc2, 0x80, 0xf3, 0xdf, // BLSI R15, R15
        0xf4,
    ];
    emu.regs_mut().r15 = 0x1_0000_0000; // bit 32 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 0x1_0000_0000, "R15 should contain isolated bit 32");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsi_mem32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI EAX, [mem]
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BLSI EAX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0xFFFFF000); // bits 12-31 set
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x1000, "EAX should contain isolated bit 12");
}

#[test]
fn test_blsi_mem64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI RAX, [mem]
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BLSI RAX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x100_0000_0000); // bit 40 set
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x100_0000_0000, "RAX should contain isolated bit 40");
}

#[test]
fn test_blsi_trailing_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI isolates bit at position of trailing zeros count
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFF000; // 12 trailing zeros
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x1000, "EAX should contain bit 12");
}

#[test]
fn test_blsi_sparse_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI with sparse bit pattern
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80001000; // bits 12 and 31 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x1000, "EAX should contain bit 12 (lowest)");
}

#[test]
fn test_blsi_preserves_source() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI should not modify source operand
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
}

#[test]
fn test_blsi_vs_and_neg() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI is equivalent to src & -src
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    let value = 0x12345678u32;
    emu.regs_mut().rbx = value as u64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let expected = value & value.wrapping_neg();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected as u64, "BLSI should equal src & -src");
}

#[test]
fn test_blsi_power_of_two() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI of power of two returns itself
    for i in 0..32 {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
            0xf4,
        ];
        emu.regs_mut().rbx = 1u64 << i;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1u64 << i, "BLSI(2^{}) should equal 2^{}", i, i);
    }
}

#[test]
fn test_blsi_consecutive_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI with consecutive bits set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00FF0000; // bits 16-23 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00010000, "EAX should contain bit 16 (lowest of consecutive)");
}

#[test]
fn test_blsi_sign_bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI with sign bit set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000; // sign bit set (bit 31)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x80000000, "EAX should contain bit 31");
}

#[test]
fn test_blsi_iterative_isolation() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];

    let mut value = 0b1010_1010u64;
    let expected_bits = vec![1, 3, 5, 7];

    for &expected_bit in &expected_bits {
        emu.regs_mut().rbx = value;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1u64 << expected_bit, "Should isolate bit {}", expected_bit);

        // Remove lowest bit for next iteration
        value = value & (value - 1);
    }
}

#[test]
fn test_blsi_high_bits_64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI with high bits in 64-bit operand
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xdb, // BLSI RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0800_0000_0000_0000; // bit 59 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0800_0000_0000_0000, "RAX should contain isolated bit 59");
}

#[test]
fn test_blsi_mixed_high_low() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI with both high and low bits, should isolate lowest
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xdb, // BLSI RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000_0000_0000_0100; // bits 8 and 63 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x100, "RAX should contain bit 8 (lowest)");
}

#[test]
fn test_blsi_max_value() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI with maximum value
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xdb, // BLSI RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFFFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 1, "RAX should contain bit 0");
}

#[test]
fn test_blsi_clear_sf_of() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSI clears OF and updates SF based on result
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000; // Would have SF set if treated as signed
    emu.flags_mut().load(0x2 | (1 << 7) | (1 << 11)); // Set SF and OF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_sf, "SF should reflect sign of result");
    assert!(!emu.flags().f_of, "OF should be clear");
}
