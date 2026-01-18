use crate::*;

// BLSR - Reset Lowest Set Bit (BMI1)
// Resets the lowest set bit in the source operand and writes the result to the destination.
// All other bits are unchanged.
// This is equivalent to: dest = src & (src - 1)
// ZF is set if result is zero, CF is set if source is zero, SF is updated based on result, OF is cleared.
//
// Opcodes:
// VEX.NDD.LZ.0F38.W0 F3 /1   BLSR r32, r/m32   - Reset lowest set bit
// VEX.NDD.LZ.0F38.W1 F3 /1   BLSR r64, r/m64   - Reset lowest set bit

#[test]
fn test_blsr_eax_ebx_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR EAX, EBX - reset bit 0 (only bit set)
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b0000_0001; // bit 0 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0b0000_0000, "EAX should be zero (bit 0 reset)");
    assert!(emu.flags().f_zf, "ZF should be set (result is zero)");
    assert!(!emu.flags().f_cf, "CF should be clear (source was non-zero)");
}

#[test]
fn test_blsr_eax_ebx_bit_3() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR EAX, EBX - reset bit 3
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b0000_1000; // only bit 3 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0b0000_0000, "EAX should be zero");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_blsr_eax_ebx_multiple_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR EAX, EBX - reset lowest of multiple bits
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b1010_1000; // bits 3, 5, 7 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0b1010_0000, "EAX should have bit 3 reset (bits 5,7 remain)");
    assert!(!emu.flags().f_zf, "ZF should be clear (result is non-zero)");
}

#[test]
fn test_blsr_eax_ebx_bit_31() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR EAX, EBX - reset bit 31
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000; // only bit 31 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000000, "EAX should be zero");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_blsr_rax_rbx_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR RAX, RBX - 64-bit version
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xcb, // BLSR RAX, RBX (W1)
        0xf4,
    ];
    emu.regs_mut().rbx = 0b0000_0001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0b0000_0000, "RAX should be zero");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_blsr_rax_rbx_bit_63() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR RAX, RBX - reset bit 63
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xcb, // BLSR RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000_0000_0000_0000; // only bit 63 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000_0000_0000_0000, "RAX should be zero");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_blsr_zero_source() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR with zero source
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0; // zero source
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should be zero (0 & -1 = 0)");
    assert!(emu.flags().f_zf, "ZF should be set (result is zero)");
    assert!(emu.flags().f_cf, "CF should be set (source was zero)");
}

#[test]
fn test_blsr_all_bits_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR with all bits set should reset bit 0
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFE, "EAX should have bit 0 reset");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsr_alternating_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR with alternating pattern 1010...1010
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xAAAAAAAA; // 1010...1010 (bit 1 is lowest set bit)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xAAAAAAA8, "EAX should have bit 1 reset");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsr_alternating_pattern_inverted() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR with alternating pattern 0101...0101
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x55555555; // 0101...0101 (bit 0 is lowest set bit)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x55555554, "EAX should have bit 0 reset");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsr_single_bit_positions() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for bit_pos in 0..32 {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
            0xf4,
        ];
        emu.regs_mut().rbx = 1u64 << bit_pos;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should be zero for single bit {}", bit_pos);
        assert!(emu.flags().f_zf, "ZF should be set for bit {}", bit_pos);
    }
}

#[test]
fn test_blsr_with_extended_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR R8D, R9D
    let code = [
        0xc4, 0xc2, 0x38, 0xf3, 0xc9, // BLSR R8D, R9D
        0xf4,
    ];
    emu.regs_mut().r9 = 0b0001_1000; // bits 3 and 4 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFFFFFFFF, 0b0001_0000, "R8D should have bit 3 reset (bit 4 remains)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsr_r15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR R15, R15
    let code = [
        0xc4, 0xc2, 0x80, 0xf3, 0xcf, // BLSR R15, R15
        0xf4,
    ];
    emu.regs_mut().r15 = 0x1_0000_0001; // bits 0 and 32 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 0x1_0000_0000, "R15 should have bit 0 reset (bit 32 remains)");
}

#[test]
fn test_blsr_mem32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR EAX, [mem]
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0x0c, 0x25, 0x00, 0x20, 0x00, 0x00, // BLSR EAX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0xFFFFF000); // bits 12-31 set
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFE000, "EAX should have bit 12 reset");
}

#[test]
fn test_blsr_mem64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR RAX, [mem]
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0x0c, 0x25, 0x00, 0x20, 0x00, 0x00, // BLSR RAX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x100_0000_0001); // bits 0 and 40 set
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x100_0000_0000, "RAX should have bit 0 reset (bit 40 remains)");
}

#[test]
fn test_blsr_trailing_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR resets bit at position of trailing zeros count
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFF000; // 12 trailing zeros, bit 12 is lowest set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFE000, "EAX should have bit 12 reset");
}

#[test]
fn test_blsr_sparse_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR with sparse bit pattern
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80001000; // bits 12 and 31 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x80000000, "EAX should have bit 12 reset (bit 31 remains)");
}

#[test]
fn test_blsr_preserves_source() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR should not modify source operand
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
}

#[test]
fn test_blsr_vs_and_sub1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR is equivalent to src & (src - 1)
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let value = 0x12345678u32;
    emu.regs_mut().rbx = value as u64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let expected = value & (value.wrapping_sub(1));
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected as u64, "BLSR should equal src & (src-1)");
}

#[test]
fn test_blsr_power_of_two() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR of power of two returns zero
    for i in 0..32 {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
            0xf4,
        ];
        emu.regs_mut().rbx = 1u64 << i;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "BLSR(2^{}) should be zero", i);
        assert!(emu.flags().f_zf, "ZF should be set for 2^{}", i);
    }
}

#[test]
fn test_blsr_consecutive_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR with consecutive bits set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00FF0000; // bits 16-23 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00FE0000, "EAX should have bit 16 reset (bits 17-23 remain)");
}

#[test]
fn test_blsr_sign_bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR with sign bit set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000; // sign bit set (bit 31)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should be zero");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_blsr_iterative_clearing() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];

    let mut value = 0b1111u64; // 4 bits set
    let expected_values = vec![0b1110, 0b1100, 0b1000, 0b0000];

    for &expected in &expected_values {
        emu.regs_mut().rbx = value;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "Should progressively clear bits");
        value = emu.regs().rax;
    }
}

#[test]
fn test_blsr_high_bits_64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR with high bits in 64-bit operand
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xcb, // BLSR RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0800_0000_0000_0000; // bit 59 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "RAX should be zero");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_blsr_mixed_high_low() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR with both high and low bits, should reset lowest
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xcb, // BLSR RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000_0000_0000_0100; // bits 8 and 63 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x8000_0000_0000_0000, "RAX should have bit 8 reset (bit 63 remains)");
}

#[test]
fn test_blsr_count_set_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR can be used to count set bits by iterating until zero
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];

    let mut value = 0x12345678u64;
    let mut count = 0;

    while value != 0 {
        count += 1;
        emu.regs_mut().rbx = value;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();
        value = emu.regs().rax & 0xFFFFFFFF;
    }

    assert_eq!(count, 0x12345678u32.count_ones(), "Should count all set bits");
}

#[test]
fn test_blsr_clear_sf_of() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR clears OF and updates SF based on result
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00001000;
    emu.flags_mut().load(0x2 | (1 << 7) | (1 << 11)); // Set SF and OF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_sf, "SF should be clear");
    assert!(!emu.flags().f_of, "OF should be clear");
}

#[test]
fn test_blsr_complement_of_blsi() {
    let DATA_ADDR = 0x7000;
    // BLSR removes lowest bit, BLSI isolates it - they're complementary
    let code_blsr = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let code_blsi = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];

    let value = 0xAAAAAAAAu64;

    let mut emu_blsr = emu64();
    emu_blsr.regs_mut().rbx = value;
    emu_blsr.load_code_bytes(&code_blsr);
    emu_blsr.run(None).unwrap();

    let mut emu_blsi = emu64();
    emu_blsi.regs_mut().rbx = value;
    emu_blsi.load_code_bytes(&code_blsi);
    emu_blsi.run(None).unwrap();

    // BLSR | BLSI should equal original value
    let combined = (emu_blsr.regs().rax | emu_blsi.regs().rax) & 0xFFFFFFFF;
    assert_eq!(combined, value, "BLSR | BLSI should reconstruct original value");
}

#[test]
fn test_blsr_two_bits_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSR with exactly two bits set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b0000_0101; // bits 0 and 2 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0b0000_0100, "EAX should have bit 0 reset (bit 2 remains)");
    assert!(!emu.flags().f_zf, "ZF should be clear (result is non-zero)");
}
