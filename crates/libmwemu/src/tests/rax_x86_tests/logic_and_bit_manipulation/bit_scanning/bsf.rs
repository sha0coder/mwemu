use crate::*;

// BSF - Bit Scan Forward
// Searches the source operand (second operand) for the least significant set bit (1 bit).
// If a least significant 1 bit is found, its bit index is stored in the destination operand.
// The source operand can be a register or a memory location; the destination operand is a register.
// The bit index is an unsigned offset from bit 0 of the source operand.
// If the source operand is 0, the ZF flag is set, and the destination operand is undefined.
// Otherwise, the ZF flag is cleared.
//
// Opcodes:
// 0F BC /r    BSF r16, r/m16    - Bit scan forward on r/m16
// 0F BC /r    BSF r32, r/m32    - Bit scan forward on r/m32
// REX.W 0F BC /r BSF r64, r/m64 - Bit scan forward on r/m64

#[test]
fn test_bsf_ax_bx_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF AX, BX - find least significant bit (bit 0)
    let code = [
        0x66, 0x0f, 0xbc, 0xc3, // BSF AX, BX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b0000_0000_0000_0001; // bit 0 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0, "AX should contain 0 (bit 0 is LSB)");
    assert!(!emu.flags().f_zf, "ZF should be clear (source is non-zero)");
}

#[test]
fn test_bsf_ax_bx_bit_15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF AX, BX - find least significant bit (bit 15 only)
    let code = [
        0x66, 0x0f, 0xbc, 0xc3, // BSF AX, BX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000; // only bit 15 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 15, "AX should contain 15 (bit 15 is LSB)");
    assert!(!emu.flags().f_zf, "ZF should be clear (source is non-zero)");
}

#[test]
fn test_bsf_eax_ebx_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF EAX, EBX - find least significant bit (bit 0, 32-bit)
    let code = [
        0x0f, 0xbc, 0xc3, // BSF EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b0000_0001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should contain 0 (bit 0 is LSB)");
    assert!(!emu.flags().f_zf, "ZF should be clear (source is non-zero)");
}

#[test]
fn test_bsf_eax_ebx_bit_31() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF EAX, EBX - find least significant bit (bit 31 only)
    let code = [
        0x0f, 0xbc, 0xc3, // BSF EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000; // only bit 31 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 31, "EAX should contain 31 (bit 31 is LSB)");
    assert!(!emu.flags().f_zf, "ZF should be clear (source is non-zero)");
}

#[test]
fn test_bsf_rax_rbx_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF RAX, RBX - find least significant bit (bit 0, 64-bit)
    let code = [
        0x48, 0x0f, 0xbc, 0xc3, // BSF RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b0000_0001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "RAX should contain 0 (bit 0 is LSB)");
    assert!(!emu.flags().f_zf, "ZF should be clear (source is non-zero)");
}

#[test]
fn test_bsf_rax_rbx_bit_63() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF RAX, RBX - find least significant bit (bit 63 only)
    let code = [
        0x48, 0x0f, 0xbc, 0xc3, // BSF RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000_0000_0000_0000; // only bit 63 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 63, "RAX should contain 63 (bit 63 is LSB)");
    assert!(!emu.flags().f_zf, "ZF should be clear (source is non-zero)");
}

#[test]
fn test_bsf_zero_source() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF with zero source sets ZF
    let code = [
        0x0f, 0xbc, 0xc3, // BSF EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0; // zero source
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_zf, "ZF should be set (source is zero)");
}

#[test]
fn test_bsf_multiple_bits_finds_lowest() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF should find the lowest set bit when multiple are set
    let code = [
        0x0f, 0xbc, 0xc3, // BSF EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b1010_1000; // bits 3, 5, 7 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 3, "EAX should contain 3 (lowest bit set)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsf_all_bits_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF with all bits set should find bit 0
    let code = [
        0x0f, 0xbc, 0xc3, // BSF EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should contain 0 (bit 0 is lowest)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsf_alternating_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF with alternating pattern 1010...1010
    let code = [
        0x0f, 0xbc, 0xc3, // BSF EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xAAAAAAAA; // 1010...1010 (bits 1,3,5,7,... set)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1, "EAX should contain 1 (lowest bit set)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsf_alternating_bits_inverted() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF with alternating pattern 0101...0101
    let code = [
        0x0f, 0xbc, 0xc3, // BSF EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x55555555; // 0101...0101 (bits 0,2,4,6,... set)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should contain 0 (lowest bit set)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsf_single_bit_positions() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for bit_pos in 0..32 {
        let code = [
            0x0f, 0xbc, 0xc3, // BSF EAX, EBX
            0xf4,
        ];
        emu.regs_mut().rbx = 1u64 << bit_pos;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, bit_pos as u64, "EAX should contain {} for bit {}", bit_pos, bit_pos);
        assert!(!emu.flags().f_zf, "ZF should be clear for bit {}", bit_pos);
    }
}

#[test]
fn test_bsf_with_extended_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF R8D, R9D - test with extended registers
    let code = [
        0x45, 0x0f, 0xbc, 0xc1, // BSF R8D, R9D
        0xf4,
    ];
    emu.regs_mut().r9 = 0b0000_1000; // bit 3 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFFFFFFFF, 3, "R8D should contain 3");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsf_r15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF with R15
    let code = [
        0x4d, 0x0f, 0xbc, 0xff, // BSF R15, R15
        0xf4,
    ];
    emu.regs_mut().r15 = 0x1_0000_0000; // bit 32 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 32, "R15 should contain 32");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsf_mem16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF AX, [mem]
    let code = [
        0x66, 0x0f, 0xbc, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BSF AX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0x0100); // bit 8 set
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 8, "AX should contain 8");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsf_mem32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF EAX, [mem]
    let code = [
        0x0f, 0xbc, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BSF EAX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x00010000); // bit 16 set
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 16, "EAX should contain 16");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsf_mem64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF RAX, [mem]
    let code = [
        0x48, 0x0f, 0xbc, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BSF RAX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x100_0000_0000); // bit 40 set
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 40, "RAX should contain 40");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsf_mem_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF with zero in memory
    let code = [
        0x0f, 0xbc, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BSF EAX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0); // zero
    emu.run(None).unwrap();

    assert!(emu.flags().f_zf, "ZF should be set (memory is zero)");
}

#[test]
fn test_bsf_high_bits_64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF with high bits in 64-bit operand
    let code = [
        0x48, 0x0f, 0xbc, 0xc3, // BSF RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0800_0000_0000_0000; // bit 59 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 59, "RAX should contain 59");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsf_mixed_high_low() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF with both high and low bits, should find low
    let code = [
        0x48, 0x0f, 0xbc, 0xc3, // BSF RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000_0000_0000_0100; // bits 8 and 63 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 8, "RAX should contain 8 (lower bit)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsf_sparse_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF with sparse bit pattern
    let code = [
        0x0f, 0xbc, 0xc3, // BSF EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80001000; // bits 12 and 31 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 12, "EAX should contain 12 (lower bit)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsf_consecutive_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF with consecutive bits set
    let code = [
        0x0f, 0xbc, 0xc3, // BSF EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00FF0000; // bits 16-23 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 16, "EAX should contain 16 (lowest of consecutive bits)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsf_preserves_source() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF should not modify source register
    let code = [
        0x0f, 0xbc, 0xc3, // BSF EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
}

#[test]
fn test_bsf_dest_equals_source() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF where destination equals source
    let code = [
        0x0f, 0xbc, 0xc0, // BSF EAX, EAX
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_1000; // bit 3 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 3, "EAX should contain 3");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsf_power_of_two() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF with powers of two
    for i in 0..32 {
        let code = [
            0x0f, 0xbc, 0xc3, // BSF EAX, EBX
            0xf4,
        ];
        emu.regs_mut().rbx = 1u64 << i;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, i as u64, "EAX should contain {} for 2^{}", i, i);
    }
}

#[test]
fn test_bsf_trailing_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF effectively counts trailing zeros + finds first set bit
    let code = [
        0x0f, 0xbc, 0xc3, // BSF EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFF000; // 12 trailing zeros
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 12, "EAX should contain 12 (12 trailing zeros)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsf_sign_bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSF with sign bit set (treated as unsigned)
    let code = [
        0x0f, 0xbc, 0xc3, // BSF EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000; // sign bit set (bit 31)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 31, "EAX should contain 31");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}
