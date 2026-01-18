use crate::*;

// BSR - Bit Scan Reverse
// Searches the source operand (second operand) for the most significant set bit (1 bit).
// If a most significant 1 bit is found, its bit index is stored in the destination operand.
// The source operand can be a register or a memory location; the destination operand is a register.
// The bit index is an unsigned offset from bit 0 of the source operand.
// If the source operand is 0, the ZF flag is set, and the destination operand is undefined.
// Otherwise, the ZF flag is cleared.
//
// Opcodes:
// 0F BD /r    BSR r16, r/m16    - Bit scan reverse on r/m16
// 0F BD /r    BSR r32, r/m32    - Bit scan reverse on r/m32
// REX.W 0F BD /r BSR r64, r/m64 - Bit scan reverse on r/m64

#[test]
fn test_bsr_ax_bx_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR AX, BX - find most significant bit (only bit 0)
    let code = [
        0x66, 0x0f, 0xbd, 0xc3, // BSR AX, BX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b0000_0000_0000_0001; // bit 0 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0, "AX should contain 0 (bit 0 is MSB)");
    assert!(!emu.flags().f_zf, "ZF should be clear (source is non-zero)");
}

#[test]
fn test_bsr_ax_bx_bit_15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR AX, BX - find most significant bit (bit 15)
    let code = [
        0x66, 0x0f, 0xbd, 0xc3, // BSR AX, BX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000; // bit 15 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 15, "AX should contain 15 (bit 15 is MSB)");
    assert!(!emu.flags().f_zf, "ZF should be clear (source is non-zero)");
}

#[test]
fn test_bsr_eax_ebx_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR EAX, EBX - find most significant bit (bit 0, 32-bit)
    let code = [
        0x0f, 0xbd, 0xc3, // BSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b0000_0001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should contain 0 (bit 0 is MSB)");
    assert!(!emu.flags().f_zf, "ZF should be clear (source is non-zero)");
}

#[test]
fn test_bsr_eax_ebx_bit_31() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR EAX, EBX - find most significant bit (bit 31)
    let code = [
        0x0f, 0xbd, 0xc3, // BSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000; // bit 31 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 31, "EAX should contain 31 (bit 31 is MSB)");
    assert!(!emu.flags().f_zf, "ZF should be clear (source is non-zero)");
}

#[test]
fn test_bsr_rax_rbx_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR RAX, RBX - find most significant bit (bit 0, 64-bit)
    let code = [
        0x48, 0x0f, 0xbd, 0xc3, // BSR RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b0000_0001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "RAX should contain 0 (bit 0 is MSB)");
    assert!(!emu.flags().f_zf, "ZF should be clear (source is non-zero)");
}

#[test]
fn test_bsr_rax_rbx_bit_63() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR RAX, RBX - find most significant bit (bit 63)
    let code = [
        0x48, 0x0f, 0xbd, 0xc3, // BSR RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000_0000_0000_0000; // bit 63 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 63, "RAX should contain 63 (bit 63 is MSB)");
    assert!(!emu.flags().f_zf, "ZF should be clear (source is non-zero)");
}

#[test]
fn test_bsr_zero_source() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR with zero source sets ZF
    let code = [
        0x0f, 0xbd, 0xc3, // BSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0; // zero source
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_zf, "ZF should be set (source is zero)");
}

#[test]
fn test_bsr_multiple_bits_finds_highest() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR should find the highest set bit when multiple are set
    let code = [
        0x0f, 0xbd, 0xc3, // BSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b1010_1000; // bits 3, 5, 7 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 7, "EAX should contain 7 (highest bit set)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsr_all_bits_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR with all bits set should find highest bit
    let code = [
        0x0f, 0xbd, 0xc3, // BSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 31, "EAX should contain 31 (bit 31 is highest)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsr_alternating_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR with alternating pattern 1010...1010
    let code = [
        0x0f, 0xbd, 0xc3, // BSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xAAAAAAAA; // 1010...1010 (bits 1,3,5,7,... set)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 31, "EAX should contain 31 (highest bit set)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsr_alternating_bits_inverted() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR with alternating pattern 0101...0101
    let code = [
        0x0f, 0xbd, 0xc3, // BSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x55555555; // 0101...0101 (bits 0,2,4,6,... set)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 30, "EAX should contain 30 (highest bit set)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsr_single_bit_positions() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for bit_pos in 0..32 {
        let code = [
            0x0f, 0xbd, 0xc3, // BSR EAX, EBX
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
fn test_bsr_with_extended_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR R8D, R9D - test with extended registers
    let code = [
        0x45, 0x0f, 0xbd, 0xc1, // BSR R8D, R9D
        0xf4,
    ];
    emu.regs_mut().r9 = 0b0000_1111; // bits 0-3 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFFFFFFFF, 3, "R8D should contain 3");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsr_r15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR with R15
    let code = [
        0x4d, 0x0f, 0xbd, 0xff, // BSR R15, R15
        0xf4,
    ];
    emu.regs_mut().r15 = 0x1_0000_0000; // bit 32 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 32, "R15 should contain 32");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsr_mem16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR AX, [mem]
    let code = [
        0x66, 0x0f, 0xbd, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BSR AX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0x0100); // bit 8 set
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 8, "AX should contain 8");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsr_mem32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR EAX, [mem]
    let code = [
        0x0f, 0xbd, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BSR EAX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x00010000); // bit 16 set
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 16, "EAX should contain 16");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsr_mem64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR RAX, [mem]
    let code = [
        0x48, 0x0f, 0xbd, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BSR RAX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x100_0000_0000); // bit 40 set
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 40, "RAX should contain 40");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsr_mem_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR with zero in memory
    let code = [
        0x0f, 0xbd, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BSR EAX, [DATA_ADDR]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0); // zero
    emu.run(None).unwrap();

    assert!(emu.flags().f_zf, "ZF should be set (memory is zero)");
}

#[test]
fn test_bsr_high_bits_64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR with high bits in 64-bit operand
    let code = [
        0x48, 0x0f, 0xbd, 0xc3, // BSR RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0800_0000_0000_0000; // bit 59 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 59, "RAX should contain 59");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsr_mixed_high_low() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR with both high and low bits, should find high
    let code = [
        0x48, 0x0f, 0xbd, 0xc3, // BSR RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000_0000_0000_0100; // bits 8 and 63 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 63, "RAX should contain 63 (higher bit)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsr_sparse_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR with sparse bit pattern
    let code = [
        0x0f, 0xbd, 0xc3, // BSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80001000; // bits 12 and 31 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 31, "EAX should contain 31 (higher bit)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsr_consecutive_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR with consecutive bits set
    let code = [
        0x0f, 0xbd, 0xc3, // BSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00FF0000; // bits 16-23 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 23, "EAX should contain 23 (highest of consecutive bits)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsr_preserves_source() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR should not modify source register
    let code = [
        0x0f, 0xbd, 0xc3, // BSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
}

#[test]
fn test_bsr_dest_equals_source() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR where destination equals source
    let code = [
        0x0f, 0xbd, 0xc0, // BSR EAX, EAX
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_1111; // bits 0-3 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 3, "EAX should contain 3");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsr_power_of_two() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR with powers of two
    for i in 0..32 {
        let code = [
            0x0f, 0xbd, 0xc3, // BSR EAX, EBX
            0xf4,
        ];
        emu.regs_mut().rbx = 1u64 << i;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, i as u64, "EAX should contain {} for 2^{}", i, i);
    }
}

#[test]
fn test_bsr_leading_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR effectively finds highest set bit (inverse of leading zeros)
    let code = [
        0x0f, 0xbd, 0xc3, // BSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00000FFF; // bits 0-11 set, 20 leading zeros
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 11, "EAX should contain 11 (highest bit set)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsr_sign_bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR with sign bit set (treated as unsigned)
    let code = [
        0x0f, 0xbd, 0xc3, // BSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000; // sign bit set (bit 31)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 31, "EAX should contain 31");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsr_vs_bsf_comparison() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR should find highest, BSF should find lowest
    let code_bsr = [
        0x0f, 0xbd, 0xc3, // BSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000001; // bits 0 and 31 set
    emu.load_code_bytes(&code_bsr);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 31, "BSR should find bit 31");

    let code_bsf = [
        0x0f, 0xbc, 0xc3, // BSF EAX, EBX
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rbx = 0x80000001;
    emu.load_code_bytes(&code_bsf);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "BSF should find bit 0");
}

#[test]
fn test_bsr_low_byte() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR with only low byte set
    let code = [
        0x0f, 0xbd, 0xc3, // BSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x000000FF; // bits 0-7 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 7, "EAX should contain 7");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bsr_high_byte() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BSR with only high byte set (32-bit)
    let code = [
        0x0f, 0xbd, 0xc3, // BSR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFF000000; // bits 24-31 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 31, "EAX should contain 31");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}
