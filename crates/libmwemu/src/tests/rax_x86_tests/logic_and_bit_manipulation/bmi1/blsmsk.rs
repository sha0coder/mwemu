use crate::*;

// BLSMSK - Get Mask Up to Lowest Set Bit (BMI1)
// Sets all the lower bits of the destination operand to 1 up to and including the lowest set bit
// in the source operand. All other bits are cleared.
// This is equivalent to: dest = src ^ (src - 1)
// ZF is cleared, CF is set if source is zero, SF is updated based on result, OF is cleared.
//
// Opcodes:
// VEX.NDD.LZ.0F38.W0 F3 /2   BLSMSK r32, r/m32   - Create mask from lowest set bit
// VEX.NDD.LZ.0F38.W1 F3 /2   BLSMSK r64, r/m64   - Create mask from lowest set bit

#[test]
fn test_blsmsk_eax_ebx_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK EAX, EBX - mask up to bit 0
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b0000_0001; // bit 0 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0b0000_0001, "EAX should contain mask up to bit 0");
    assert!(!emu.flags().f_zf, "ZF should be clear (source is non-zero)");
    assert!(!emu.flags().f_cf, "CF should be clear (source is non-zero)");
}

#[test]
fn test_blsmsk_eax_ebx_bit_3() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK EAX, EBX - mask up to bit 3
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b0000_1000; // bit 3 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0b0000_1111, "EAX should contain mask up to bit 3 (bits 0-3)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_blsmsk_eax_ebx_bit_31() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK EAX, EBX - mask up to bit 31
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000; // only bit 31 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF, "EAX should contain mask up to bit 31 (all bits)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsmsk_rax_rbx_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK RAX, RBX - 64-bit version
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xd3, // BLSMSK RAX, RBX (W1)
        0xf4,
    ];
    emu.regs_mut().rbx = 0b0000_0001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0b0000_0001, "RAX should contain mask up to bit 0");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsmsk_rax_rbx_bit_63() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK RAX, RBX - mask up to bit 63
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xd3, // BLSMSK RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000_0000_0000_0000; // only bit 63 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFF_FFFF_FFFF_FFFF, "RAX should contain mask up to bit 63 (all bits)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsmsk_zero_source() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK with zero source
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0; // zero source
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF, "EAX should be all ones (src ^ (src-1) = 0 ^ -1)");
    assert!(!emu.flags().f_zf, "ZF should be clear (BLSMSK clears ZF)");
    assert!(emu.flags().f_cf, "CF should be set (source is zero)");
}

#[test]
fn test_blsmsk_multiple_bits_uses_lowest() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK should use only the lowest set bit
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b1010_1000; // bits 3, 5, 7 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0b0000_1111, "EAX should contain mask up to bit 3 (lowest)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsmsk_all_bits_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK with all bits set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1, "EAX should contain mask up to bit 0");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsmsk_alternating_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK with alternating pattern 1010...1010
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xAAAAAAAA; // 1010...1010 (bit 1 is lowest set bit)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0b11, "EAX should contain mask up to bit 1");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsmsk_alternating_pattern_inverted() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK with alternating pattern 0101...0101
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x55555555; // 0101...0101 (bit 0 is lowest set bit)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1, "EAX should contain mask up to bit 0");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsmsk_single_bit_positions() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for bit_pos in 0..32 {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
            0xf4,
        ];
        emu.regs_mut().rbx = 1u64 << bit_pos;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        let expected = (1u64 << (bit_pos + 1)) - 1;
        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "EAX should contain mask for bit {}", bit_pos);
        assert!(!emu.flags().f_zf, "ZF should be clear for bit {}", bit_pos);
    }
}

#[test]
fn test_blsmsk_with_extended_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK R8D, R9D
    let code = [
        0xc4, 0xc2, 0x38, 0xf3, 0xd1, // BLSMSK R8D, R9D
        0xf4,
    ];
    emu.regs_mut().r9 = 0b0001_0000; // bit 4 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFFFFFFFF, 0b0001_1111, "R8D should contain mask up to bit 4");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_blsmsk_r15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK R15, R15
    let code = [
        0xc4, 0xc2, 0x80, 0xf3, 0xd7, // BLSMSK R15, R15
        0xf4,
    ];
    emu.regs_mut().r15 = 0x1_0000_0000; // bit 32 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let expected = (1u64 << 33) - 1;
    assert_eq!(emu.regs().r15, expected, "R15 should contain mask up to bit 32");
}

#[test]
fn test_blsmsk_mem32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK EAX, [mem]
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // BLSMSK EAX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x00001000); // bit 12 set
    emu.run(None).unwrap();

    let expected = (1u32 << 13) - 1;
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected as u64, "EAX should contain mask up to bit 12");
}

#[test]
fn test_blsmsk_mem64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK RAX, [mem]
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // BLSMSK RAX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x100_0000_0000); // bit 40 set
    emu.run(None).unwrap();

    let expected = (1u64 << 41) - 1;
    assert_eq!(emu.regs().rax, expected, "RAX should contain mask up to bit 40");
}

#[test]
fn test_blsmsk_trailing_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK creates mask for trailing zeros + 1
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFF000; // 12 trailing zeros, bit 12 is lowest set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let expected = (1u32 << 13) - 1;
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected as u64, "EAX should contain 13-bit mask");
}

#[test]
fn test_blsmsk_sparse_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK with sparse bit pattern
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80001000; // bits 12 and 31 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let expected = (1u32 << 13) - 1;
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected as u64, "EAX should contain mask up to bit 12 (lowest)");
}

#[test]
fn test_blsmsk_preserves_source() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK should not modify source operand
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
}

#[test]
fn test_blsmsk_vs_xor_sub1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK is equivalent to src ^ (src - 1)
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let value = 0x12345678u32;
    emu.regs_mut().rbx = value as u64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let expected = value ^ (value.wrapping_sub(1));
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected as u64, "BLSMSK should equal src ^ (src-1)");
}

#[test]
fn test_blsmsk_power_of_two() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK of power of two creates mask with that many bits
    for i in 0..32 {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
            0xf4,
        ];
        emu.regs_mut().rbx = 1u64 << i;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        let expected = (1u64 << (i + 1)) - 1;
        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "BLSMSK(2^{}) should create {}-bit mask", i, i + 1);
    }
}

#[test]
fn test_blsmsk_consecutive_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK with consecutive bits set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00FF0000; // bits 16-23 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let expected = (1u32 << 17) - 1;
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected as u64, "EAX should contain mask up to bit 16");
}

#[test]
fn test_blsmsk_sign_bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK with sign bit set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000; // sign bit set (bit 31)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF, "EAX should contain all bits");
}

#[test]
fn test_blsmsk_creates_bit_masks() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK can create various bit masks
    let test_cases = vec![
        (0x00000001, 0x00000001), // 1-bit mask
        (0x00000002, 0x00000003), // 2-bit mask
        (0x00000004, 0x00000007), // 3-bit mask
        (0x00000008, 0x0000000F), // 4-bit mask
        (0x00000010, 0x0000001F), // 5-bit mask
        (0x00000100, 0x000001FF), // 9-bit mask
        (0x00010000, 0x0001FFFF), // 17-bit mask
    ];

    for (input, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
            0xf4,
        ];
        emu.regs_mut().rbx = input;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "BLSMSK(0x{:08X}) should be 0x{:08X}", input, expected);
    }
}

#[test]
fn test_blsmsk_high_bits_64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK with high bits in 64-bit operand
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xd3, // BLSMSK RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0800_0000_0000_0000; // bit 59 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let expected = (1u64 << 60) - 1;
    assert_eq!(emu.regs().rax, expected, "RAX should contain 60-bit mask");
}

#[test]
fn test_blsmsk_mixed_high_low() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK with both high and low bits, should use lowest
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xd3, // BLSMSK RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000_0000_0000_0100; // bits 8 and 63 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let expected = (1u64 << 9) - 1;
    assert_eq!(emu.regs().rax, expected, "RAX should contain 9-bit mask (up to bit 8)");
}

#[test]
fn test_blsmsk_clear_sf_of() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BLSMSK clears OF and updates SF based on result
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00001000;
    emu.flags_mut().load(0x2 | (1 << 7) | (1 << 11)); // Set SF and OF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_sf, "SF should be clear");
    assert!(!emu.flags().f_of, "OF should be clear");
}
