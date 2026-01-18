use crate::*;

// PEXT - Parallel Bits Extract (BMI2)
// Extracts bits from the source operand at positions specified by the mask (second source).
// For each set bit in the mask, the corresponding bit from the source is extracted and
// packed contiguously into the destination, starting from the LSB.
// This is the inverse operation of PDEP.
//
// Opcodes:
// VEX.NDS.LZ.F3.0F38.W0 F5 /r   PEXT r32, r32, r/m32   - Parallel extract of bits
// VEX.NDS.LZ.F3.0F38.W1 F5 /r   PEXT r64, r64, r/m64   - Parallel extract of bits

#[test]
fn test_pext_eax_ebx_ecx_all_mask() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PEXT EAX, EBX, ECX - mask all ones (identity)
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.regs_mut().rcx = 0xFFFFFFFF; // all mask bits set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x12345678, "EAX should equal source (identity with full mask)");
}

#[test]
fn test_pext_eax_ebx_ecx_zero_mask() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PEXT EAX, EBX, ECX - mask all zeros
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.regs_mut().rcx = 0x00000000; // no mask bits set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should be zero (no bits extracted)");
}

#[test]
fn test_pext_eax_ebx_ecx_single_bit_mask() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PEXT with single bit mask
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0b0000_1000; // bit 3 set
    emu.regs_mut().rcx = 0b0000_1000; // extract bit 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0b0000_0001, "EAX should have extracted bit at position 0");
}

#[test]
fn test_pext_eax_ebx_ecx_alternating_mask() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PEXT with alternating mask 0101...0101
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x55555555; // alternating bits
    emu.regs_mut().rcx = 0x55555555; // extract every other bit
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x0000FFFF, "EAX should have 16 bits packed");
}

#[test]
fn test_pext_eax_ebx_ecx_low_nibble() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.regs_mut().rcx = 0x0000000F; // mask for bits 0-3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x8, "EAX should contain extracted nibble");
}

#[test]
fn test_pext_rax_rbx_rcx_64bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PEXT RAX, RBX, RCX - 64-bit version
    let code = [
        0xc4, 0xe2, 0xe2, 0xf5, 0xc1, // PEXT RAX, RBX, RCX (W1)
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFF00000000000000;
    emu.regs_mut().rcx = 0xFF00000000000000; // extract high byte
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFF, "RAX should have extracted high byte at low position");
}

#[test]
fn test_pext_sparse_mask() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PEXT with sparse mask
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80001001; // bits 0, 12, 31 set
    emu.regs_mut().rcx = 0x80001001; // extract those bits
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0b111, "EAX should have 3 bits packed at low positions");
}

#[test]
fn test_pext_with_extended_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PEXT R8D, R9D, R10D
    let code = [
        0xc4, 0x42, 0x32, 0xf5, 0xc2, // PEXT R8D, R9D, R10D
        0xf4,
    ];
    emu.regs_mut().r9 = 0x00FF0000; // bits 16-23 set
    emu.regs_mut().r10 = 0x00FF0000; // extract bits 16-23
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFFFFFFFF, 0xFF, "R8D should have extracted byte");
}

#[test]
fn test_pext_mem32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PEXT EAX, EBX, [mem]
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // PEXT EAX, EBX, [DATA_ADDR]
        0xf4,
    ];
    emu.regs_mut().rbx = 0x000F0000;
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x000F0000); // mask bits 16-19
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xF, "EAX should have extracted nibble from memory mask");
}

#[test]
fn test_pext_mem64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PEXT RAX, RBX, [mem]
    let code = [
        0xc4, 0xe2, 0xe2, 0xf5, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // PEXT RAX, RBX, [DATA_ADDR]
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00FF000000000000;
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x00FF000000000000); // mask bits 48-55
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFF, "RAX should have extracted byte from memory mask");
}

#[test]
fn test_pext_preserves_sources() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PEXT should not modify source operands
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
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
fn test_pext_sequential_extracts() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = vec![
        (0x0000000F, 0x0000000F, 0x0000000F),
        (0x000000FF, 0x000000FF, 0x000000FF),
        (0x00000FFF, 0x00000FFF, 0x00000FFF),
        (0x0000FFFF, 0x0000FFFF, 0x0000FFFF),
    ];

    for (src, mask, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
            0xf4,
        ];
        emu.regs_mut().rbx = src;
        emu.regs_mut().rcx = mask;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "PEXT(0x{:X}, 0x{:X}) should be 0x{:X}", src, mask, expected);
    }
}

#[test]
fn test_pext_extract_nibbles() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0D0C0B0A;
    emu.regs_mut().rcx = 0x0F0F0F0F; // every other nibble
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xDCBA, "Should extract alternating nibbles packed");
}

#[test]
fn test_pext_bit_gather() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x01010101; // one bit per byte
    emu.regs_mut().rcx = 0x01010101; // extract those bits
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xF, "Should gather 4 scattered bits");
}

#[test]
fn test_pext_power_of_two_masks() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for bit_pos in 0..32 {
        let code = [
            0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
            0xf4,
        ];
        emu.regs_mut().rbx = 1u64 << bit_pos;
        emu.regs_mut().rcx = 1u64 << bit_pos;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1, "Should extract single bit to position 0");
    }
}

#[test]
fn test_pext_inverse_of_pdep() {
    let DATA_ADDR = 0x7000;
    // PEXT is the inverse of PDEP with the same mask
    let code_pdep = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let code_pext = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];

    let value = 0x12345678u64;
    let mask = 0x0F0F0F0Fu64;

    let mut emu = emu64();
    emu.regs_mut().rbx = value;
    emu.regs_mut().rcx = mask;
    emu.load_code_bytes(&code_pdep);
    emu.run(None).unwrap();
    let deposited = emu.regs().rax & 0xFFFFFFFF;

    let mut emu = emu64();
    emu.regs_mut().rbx = deposited;
    emu.regs_mut().rcx = mask;
    emu.load_code_bytes(&code_pext);
    emu.run(None).unwrap();

    let mask_bits = mask & 0xFFFF_FFFF;
    let bit_count = mask_bits.count_ones();
    let expected = if bit_count == 64 {
        value
    } else {
        value & ((1u64 << bit_count) - 1)
    };
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "PEXT(PDEP(x, mask), mask) should recover deposited bits");
}

#[test]
fn test_pext_byte_compaction() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0A0B0C0D;
    emu.regs_mut().rcx = 0x00FF00FF; // two bytes separated
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x0B0D, "Should compact separated bytes");
}

#[test]
fn test_pext_zero_source() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0;
    emu.regs_mut().rcx = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "Zero source should produce zero");
}

#[test]
fn test_pext_64bit_high_positions() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xe2, 0xf5, 0xc1, // PEXT RAX, RBX, RCX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFF000000000000;
    emu.regs_mut().rcx = 0xFFFF000000000000; // extract high 16 bits
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFF, "Should extract high bits to low positions");
}

#[test]
fn test_pext_pattern_extraction() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = vec![
        (0x11111111, 0x11111111, 0x000000FF),  // extract every 4th bit
        (0x33333333, 0x33333333, 0x0000FFFF),  // extract two bits per nibble
        (0x0F0F0F0F, 0x0F0F0F0F, 0x0000FFFF),  // extract nibbles
    ];

    for (src, mask, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
            0xf4,
        ];
        emu.regs_mut().rbx = src;
        emu.regs_mut().rcx = mask;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "PEXT(0x{:X}, 0x{:X}) should be 0x{:X}", src, mask, expected);
    }
}

#[test]
fn test_pext_consecutive_mask_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.regs_mut().rcx = 0x0000FFFF; // lower 16 bits consecutive
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00005678, "Consecutive mask extracts lower bits");
}

#[test]
fn test_pext_field_unpacking() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0F0000F0; // two 4-bit fields
    emu.regs_mut().rcx = 0x0F0000F0; // extract both
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFF, "Should unpack fields contiguously");
}

#[test]
fn test_pext_mask_population_count() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF; // all source bits set
    emu.regs_mut().rcx = 0x000000FF; // 8 mask bits
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFF, "Result limited by mask popcount");
}

#[test]
fn test_pext_interleaved_bytes() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xAABBCCDD;
    emu.regs_mut().rcx = 0x00FF00FF; // bytes at positions 0 and 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xBBDD, "Should extract interleaved bytes");
}

#[test]
fn test_pext_bit_reversal_aid() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PEXT can help with bit manipulation patterns
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xF0F0F0F0;
    emu.regs_mut().rcx = 0xF0F0F0F0; // extract high nibbles of each byte
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFF, "Should extract all high nibbles");
}

#[test]
fn test_pext_single_byte_from_dword() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for byte_idx in 0..4 {
        let code = [
            0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
            0xf4,
        ];
        emu.regs_mut().rbx = 0x03020100;
        emu.regs_mut().rcx = 0xFFu64 << (byte_idx * 8);
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax & 0xFFFFFFFF, byte_idx as u64, "Should extract byte {}", byte_idx);
    }
}
