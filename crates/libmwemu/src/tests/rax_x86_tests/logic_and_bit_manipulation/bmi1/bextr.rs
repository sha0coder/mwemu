use crate::*;

// BEXTR - Bit Field Extract (BMI1)
// Extracts contiguous bits from the first source operand using an index and length specified
// in the second source operand. Bits[7:0] of the second source specify the starting bit position.
// Bits[15:8] specify the length in bits to extract.
// The extracted bits are written to the destination register with zero extension.
// ZF is set if the extracted field is all zeros, CF is cleared, OF/SF/AF/PF are undefined.
//
// Opcodes:
// VEX.NDS.LZ.0F38.W0 F7 /r   BEXTR r32, r/m32, r32   - Extract bits from r/m32 using r32
// VEX.NDS.LZ.0F38.W1 F7 /r   BEXTR r64, r/m64, r64   - Extract bits from r/m64 using r64

#[test]
fn test_bextr_eax_ebx_ecx_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BEXTR EAX, EBX, ECX - extract 8 bits starting at bit 4
    // VEX.NDS.LZ.0F38.W0 F7 /r
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.regs_mut().rcx = (8 << 8) | 4; // length=8, start=4
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x12345678 >> 4 = 0x01234567, mask 8 bits = 0x67
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x67, "EAX should contain extracted bits");
    assert!(!emu.flags().f_zf, "ZF should be clear (result is non-zero)");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_bextr_eax_ebx_ecx_zero_result() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BEXTR that extracts all zeros
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00000000;
    emu.regs_mut().rcx = (8 << 8) | 4; // length=8, start=4
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should be zero");
    assert!(emu.flags().f_zf, "ZF should be set (result is zero)");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_bextr_eax_ebx_ecx_start_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BEXTR starting at bit 0
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.regs_mut().rcx = (16 << 8) | 0; // length=16, start=0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFF, "EAX should contain lower 16 bits");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bextr_eax_ebx_ecx_length_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BEXTR with length=1 (extract single bit)
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00000010; // bit 4 set
    emu.regs_mut().rcx = (1 << 8) | 4; // length=1, start=4
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1, "EAX should contain 1");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bextr_eax_ebx_ecx_length_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BEXTR with length=0 should return 0
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.regs_mut().rcx = (0 << 8) | 4; // length=0, start=4
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should be zero (length=0)");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_bextr_eax_ebx_ecx_full_32bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BEXTR extracting all 32 bits
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.regs_mut().rcx = (32 << 8) | 0; // length=32, start=0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x12345678, "EAX should contain all bits");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bextr_rax_rbx_rcx_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BEXTR RAX, RBX, RCX - 64-bit version
    let code = [
        0xc4, 0xe2, 0xf0, 0xf7, 0xc3, // BEXTR RAX, RBX, RCX (W1)
        0xf4,
    ];
    emu.regs_mut().rbx = 0x123456789ABCDEF0;
    emu.regs_mut().rcx = (16 << 8) | 8; // length=16, start=8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x123456789ABCDEF0 >> 8 = 0x00123456789ABCDE, mask 16 bits = 0xBCDE
    assert_eq!(emu.regs().rax, 0xBCDE, "RAX should contain extracted bits");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bextr_rax_rbx_rcx_high_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BEXTR extracting from high bits of 64-bit value
    let code = [
        0xc4, 0xe2, 0xf0, 0xf7, 0xc3, // BEXTR RAX, RBX, RCX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFF00_0000_0000_0000;
    emu.regs_mut().rcx = (8 << 8) | 56; // length=8, start=56
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFF, "RAX should contain top 8 bits");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bextr_start_beyond_operand_size() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BEXTR with start position beyond operand size
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.regs_mut().rcx = (8 << 8) | 32; // length=8, start=32 (beyond 32-bit operand)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should be zero (start beyond size)");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_bextr_length_exceeds_remaining() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BEXTR with length that would exceed operand size
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.regs_mut().rcx = (20 << 8) | 20; // length=20, start=20 (would go to bit 40)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFF, "EAX should contain remaining bits");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_bextr_with_extended_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BEXTR R8D, R9D, R10D
    let code = [
        0xc4, 0x42, 0x28, 0xf7, 0xc1, // BEXTR R8D, R9D, R10D
        0xf4,
    ];
    emu.regs_mut().r9 = 0xABCDEF01;
    emu.regs_mut().r10 = (12 << 8) | 4; // length=12, start=4
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let expected = (0xABCDEF01 >> 4) & 0xFFF;
    assert_eq!(emu.regs().r8 & 0xFFFFFFFF, expected, "R8D should contain extracted bits");
}

#[test]
fn test_bextr_mem32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BEXTR EAX, [mem], ECX
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BEXTR EAX, [DATA_ADDR], ECX
        0xf4,
    ];
    emu.regs_mut().rcx = (8 << 8) | 8; // length=8, start=8
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0xAABBCCDD);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xCC, "EAX should contain extracted bits from memory");
}

#[test]
fn test_bextr_mem64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BEXTR RAX, [mem], RCX
    let code = [
        0xc4, 0xe2, 0xf0, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BEXTR RAX, [DATA_ADDR], RCX
        0xf4,
    ];
    emu.regs_mut().rcx = (16 << 8) | 16; // length=16, start=16
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x0123456789ABCDEF);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x89AB, "RAX should contain extracted bits from memory");
}

#[test]
fn test_bextr_nibble_extraction() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];

    let value = 0x12345678u64;
    for nibble_idx in 0..8 {
        emu.regs_mut().rbx = value;
        emu.regs_mut().rcx = (4 << 8) | (nibble_idx * 4); // length=4, start=nibble_idx*4
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        let expected = (value >> (nibble_idx * 4)) & 0xF;
        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "Should extract nibble {}", nibble_idx);
    }
}

#[test]
fn test_bextr_byte_extraction() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];

    let value = 0x12345678u64;
    for byte_idx in 0..4 {
        emu.regs_mut().rbx = value;
        emu.regs_mut().rcx = (8 << 8) | (byte_idx * 8); // length=8, start=byte_idx*8
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        let expected = (value >> (byte_idx * 8)) & 0xFF;
        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "Should extract byte {}", byte_idx);
    }
}

#[test]
fn test_bextr_alternating_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xAAAAAAAA; // 1010...1010
    emu.regs_mut().rcx = (8 << 8) | 0; // length=8, start=0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xAA, "EAX should contain extracted pattern");
}

#[test]
fn test_bextr_single_bit_scan() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];

    let value = 0x80000001u64; // bits 0 and 31 set
    for bit_idx in 0..32 {
        emu.regs_mut().rbx = value;
        emu.regs_mut().rcx = (1 << 8) | bit_idx; // length=1, start=bit_idx
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        let expected = ((value >> bit_idx) & 1) as u64;
        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "Should extract bit {}", bit_idx);
    }
}

#[test]
fn test_bextr_max_length_255() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BEXTR with maximum length value (255)
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.regs_mut().rcx = (255 << 8) | 0; // length=255, start=0 (will be clamped to 32 bits)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF, "EAX should contain all 32 bits");
}

#[test]
fn test_bextr_preserves_source() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BEXTR should not modify source operand
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.regs_mut().rcx = (8 << 8) | 4;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
    assert_eq!(emu.regs().rcx & 0xFFFFFFFF, (8 << 8) | 4, "ECX should be unchanged");
}

#[test]
fn test_bextr_zero_extension() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF; // Set all bits in RAX
    emu.regs_mut().rbx = 0x000000FF;
    emu.regs_mut().rcx = (8 << 8) | 0; // length=8, start=0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 32-bit operation should zero upper 32 bits of RAX
    assert_eq!(emu.regs().rax, 0xFF, "RAX should be zero-extended (upper bits cleared)");
}

#[test]
fn test_bextr_mask_creation() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BEXTR can be used to create bit masks
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.regs_mut().rcx = (16 << 8) | 0; // length=16, start=0 - creates 16-bit mask
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFF, "EAX should contain 16-bit mask");
}

#[test]
fn test_bextr_field_alignment() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xDEADBEEF;
    emu.regs_mut().rcx = (16 << 8) | 16; // length=16, start=16 - extract upper 16 bits
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xDEAD, "EAX should contain upper 16 bits");
}

#[test]
fn test_bextr_unaligned_field() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.regs_mut().rcx = (10 << 8) | 7; // length=10, start=7 - extract bits 7-16
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let expected = (0xFFFFFFFF >> 7) & 0x3FF;
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "EAX should contain 10 bits");
}

#[test]
fn test_bextr_64bit_full_extraction() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf0, 0xf7, 0xc3, // BEXTR RAX, RBX, RCX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x123456789ABCDEF0;
    emu.regs_mut().rcx = (64 << 8) | 0; // length=64, start=0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "RAX should contain all 64 bits");
}
