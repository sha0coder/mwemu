use crate::*;

// PDEP - Parallel Bits Deposit (BMI2)
// Deposits bits from the source operand into positions specified by the mask (second source).
// For each set bit in the mask, the corresponding bit position receives the next bit from the source.
// Bits in positions corresponding to clear mask bits are zeroed.
// This is the inverse operation of PEXT.
//
// Opcodes:
// VEX.NDS.LZ.F2.0F38.W0 F5 /r   PDEP r32, r32, r/m32   - Parallel deposit of bits
// VEX.NDS.LZ.F2.0F38.W1 F5 /r   PDEP r64, r64, r/m64   - Parallel deposit of bits

#[test]
fn test_pdep_eax_ebx_ecx_all_mask() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PDEP EAX, EBX, ECX - mask all ones (identity)
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.regs_mut().rcx = 0xFFFFFFFF; // all mask bits set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x12345678, "EAX should equal source (identity with full mask)");
}

#[test]
fn test_pdep_eax_ebx_ecx_zero_mask() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PDEP EAX, EBX, ECX - mask all zeros
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.regs_mut().rcx = 0x00000000; // no mask bits set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should be zero (no deposit positions)");
}

#[test]
fn test_pdep_eax_ebx_ecx_single_bit_mask() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PDEP with single bit mask
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b0000_0001; // bit 0 from source
    emu.regs_mut().rcx = 0b0000_1000; // deposit to bit 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0b0000_1000, "EAX should have bit 3 set");
}

#[test]
fn test_pdep_eax_ebx_ecx_alternating_mask() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PDEP with alternating mask 0101...0101
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFF; // lower 16 bits set
    emu.regs_mut().rcx = 0x55555555; // alternating bits
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x55555555, "EAX should have alternating pattern");
}

#[test]
fn test_pdep_eax_ebx_ecx_low_nibble() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b1111; // 4 bits
    emu.regs_mut().rcx = 0x0F00; // mask for bits 8-11
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x0F00, "EAX should have bits 8-11 set");
}

#[test]
fn test_pdep_rax_rbx_rcx_64bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PDEP RAX, RBX, RCX - 64-bit version
    let code = [
        0xc4, 0xe2, 0xe3, 0xf5, 0xc1, // PDEP RAX, RBX, RCX (W1)
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFF;
    emu.regs_mut().rcx = 0xFF00000000000000; // deposit to high byte
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFF00000000000000, "RAX should have high byte set");
}

#[test]
fn test_pdep_sparse_mask() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PDEP with sparse mask
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b111; // 3 bits
    emu.regs_mut().rcx = 0x80001001; // bits 0, 12, 31
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x80001001, "EAX should have bits at mask positions");
}

#[test]
fn test_pdep_with_extended_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PDEP R8D, R9D, R10D
    let code = [
        0xc4, 0x42, 0x33, 0xf5, 0xc2, // PDEP R8D, R9D, R10D
        0xf4,
    ];
    emu.regs_mut().r9 = 0xFF;
    emu.regs_mut().r10 = 0x00FF0000; // deposit to bits 16-23
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFFFFFFFF, 0x00FF0000, "R8D should have bits 16-23 set");
}

#[test]
fn test_pdep_mem32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PDEP EAX, EBX, [mem]
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // PDEP EAX, EBX, [DATA_ADDR]
        0xf4,
    ];
    emu.regs_mut().rbx = 0xF;
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x000F0000); // mask bits 16-19
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x000F0000, "EAX should have bits deposited from memory mask");
}

#[test]
fn test_pdep_mem64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PDEP RAX, RBX, [mem]
    let code = [
        0xc4, 0xe2, 0xe3, 0xf5, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // PDEP RAX, RBX, [DATA_ADDR]
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFF;
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x00FF000000000000); // mask bits 48-55
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x00FF000000000000, "RAX should have bits deposited from memory mask");
}

#[test]
fn test_pdep_preserves_sources() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PDEP should not modify source operands
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.regs_mut().rcx = 0xAAAAAAAA;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
    assert_eq!(emu.regs().rcx & 0xFFFFFFFF, 0xAAAAAAAA, "ECX should be unchanged");
}

#[test]
fn test_pdep_sequential_deposits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = vec![
        (0b1, 0x00000001, 0x00000001),
        (0b11, 0x00000003, 0x00000003),
        (0b111, 0x00000007, 0x00000007),
        (0b1111, 0x0000000F, 0x0000000F),
    ];

    for (src, mask, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
            0xf4,
        ];
        emu.regs_mut().rbx = src;
        emu.regs_mut().rcx = mask;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "PDEP(0x{:X}, 0x{:X}) should be 0x{:X}", src, mask, expected);
    }
}

#[test]
fn test_pdep_extract_nibbles() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xABCD; // 4 nibbles
    emu.regs_mut().rcx = 0x0F0F0F0F; // every other nibble position
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x0A0B0C0D, "Should deposit nibbles to alternating positions");
}

#[test]
fn test_pdep_bit_scatter() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFF; // 8 bits
    emu.regs_mut().rcx = 0x01010101; // scatter to bytes
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x01010101, "Should scatter bits across byte positions");
}

#[test]
fn test_pdep_power_of_two_masks() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for bit_pos in 0..32 {
        let code = [
            0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
            0xf4,
        ];
        emu.regs_mut().rbx = 1; // single bit
        emu.regs_mut().rcx = 1u64 << bit_pos;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1u64 << bit_pos, "Should deposit to bit {}", bit_pos);
    }
}

#[test]
fn test_pdep_inverse_of_pext() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PDEP is the inverse of PEXT with the same mask
    let code_pext = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let code_pdep = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];

    let value = 0x12345678u64;
    let mask = 0x0F0F0F0Fu64;

    emu.regs_mut().rbx = value;
    emu.regs_mut().rcx = mask;
    emu.load_code_bytes(&code_pext);
    emu.run(None).unwrap();
    let extracted = emu.regs().rax & 0xFFFFFFFF;

    let mut emu2 = emu64();
    emu2.regs_mut().rbx = extracted;
    emu2.regs_mut().rcx = mask;
    emu2.load_code_bytes(&code_pdep);
    emu2.run(None).unwrap();

    let masked_original = value & mask;
    assert_eq!(emu2.regs().rax & 0xFFFFFFFF, masked_original, "PDEP(PEXT(x, mask), mask) should equal x & mask");
}

#[test]
fn test_pdep_excess_source_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF; // all bits
    emu.regs_mut().rcx = 0x0000000F; // only 4 mask bits
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x0000000F, "Should use only first 4 source bits");
}

#[test]
fn test_pdep_byte_expansion() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFF;
    emu.regs_mut().rcx = 0x00FF00FF; // two bytes separated
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x000000FF, "Should expand byte to separated positions");
}

#[test]
fn test_pdep_zero_source() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0;
    emu.regs_mut().rcx = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "Zero source should produce zero");
}

#[test]
fn test_pdep_64bit_high_positions() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xe3, 0xf5, 0xc1, // PDEP RAX, RBX, RCX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFF;
    emu.regs_mut().rcx = 0xFFFF000000000000; // high 16 bits
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFF000000000000, "Should deposit to high positions");
}

#[test]
fn test_pdep_pattern_generation() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = vec![
        (0x1, 0x11111111, 0x00000001),     // deposit single bit
        (0x3, 0x33333333, 0x00000003),     // deposit two bits
        (0xF, 0x0F0F0F0F, 0x0000000F),     // deposit nibble
    ];

    for (src, mask, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
            0xf4,
        ];
        emu.regs_mut().rbx = src;
        emu.regs_mut().rcx = mask;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "PDEP(0x{:X}, 0x{:X}) should be 0x{:X}", src, mask, expected);
    }
}

#[test]
fn test_pdep_consecutive_mask_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.regs_mut().rcx = 0x0000FFFF; // lower 16 bits consecutive
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00005678, "Consecutive mask preserves lower bits");
}

#[test]
fn test_pdep_field_packing() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b11111111; // 8 bits to pack
    emu.regs_mut().rcx = 0x0F0000F0; // two 4-bit fields
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x0F0000F0, "Should pack bits into fields");
}
