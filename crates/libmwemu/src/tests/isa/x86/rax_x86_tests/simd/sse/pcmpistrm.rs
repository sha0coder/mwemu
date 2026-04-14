use crate::*;

// PCMPISTRM - Packed Compare Implicit Length Strings, Return Mask
//
// SSE4.2 instruction that performs a string comparison similar to PCMPISTRI but
// returns a mask in XMM0 instead of an index in ECX.
//
// Returns a mask in XMM0 based on the comparison result:
//   - Bit mask (default): Each bit corresponds to a byte/word match
//   - Index mask (bit 6 set): Bits set for matched indices
//
// Sets flags based on the comparison result:
//   CF = 1 if any match found (IntRes2 != 0)
//   ZF = 1 if end of string reached in second operand
//   SF = 1 if end of string reached in first operand
//   OF = 1 if IntRes2 is non-zero at bit position 0
//
// Control byte format (imm8) - same as PCMPISTRI:
//   Bits 0-1: Source data format (00=ubytes, 01=uwords, 10=sbytes, 11=swords)
//   Bits 2-3: Aggregation (00=equal any, 01=ranges, 10=equal each, 11=equal ordered)
//   Bit 4: Polarity (0=positive, 1=negative)
//   Bit 5: Output selection (0=LSB, 1=MSB)
//   Bit 6: Mask type (0=bit mask, 1=byte/word mask)
//
// Opcode:
//   66 0F 3A 62 /r ib    PCMPISTRM xmm1, xmm2/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Equal Any Mode - Bit Mask Output (default)
// ============================================================================

#[test]
fn test_pcmpistrm_equal_any_bitmask() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x48, 0x10, // MOVDQA XMM1, [RAX+0x10]
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x00, // PCMPISTRM XMM0, XMM1, 0x00
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "abc\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xbyczd\0" - matches 'b' at index 1, 'c' at index 3
    let data2: [u8; 16] = [0x78, 0x62, 0x79, 0x63, 0x7a, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_equal_any_no_match() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x00,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "abc\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xyz\0" - no matches, XMM0 should be all zeros
    let data2: [u8; 16] = [0x78, 0x79, 0x7a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_equal_any_all_match() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x00,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "aaa\0"
    let data1: [u8; 16] = [0x61, 0x61, 0x61, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "aaaa\0" - all match
    let data2: [u8; 16] = [0x61, 0x61, 0x61, 0x61, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_equal_any_uwords() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x01,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: words [0x1234, 0x5678, 0x0000]
    let data1: [u8; 16] = [0x34, 0x12, 0x78, 0x56, 0x00, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: words [0xABCD, 0x5678, 0x1234, 0x0000]
    let data2: [u8; 16] = [0xCD, 0xAB, 0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Byte/Word Mask Output (bit 6 = 1)
// ============================================================================

#[test]
fn test_pcmpistrm_equal_any_bytemask() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x40,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "abc\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xbyczd\0" - matches produce 0xFF for matched bytes
    let data2: [u8; 16] = [0x78, 0x62, 0x79, 0x63, 0x7a, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_equal_any_wordmask() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x41,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: words [0x1234, 0x5678, 0x0000]
    let data1: [u8; 16] = [0x34, 0x12, 0x78, 0x56, 0x00, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: words [0xABCD, 0x5678, 0x0000] - matches produce 0xFFFF for matched words
    let data2: [u8; 16] = [0xCD, 0xAB, 0x78, 0x56, 0x00, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Ranges Mode
// ============================================================================

#[test]
fn test_pcmpistrm_ranges_bitmask() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x04,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: range 'a'-'z'
    let data1: [u8; 16] = [0x61, 0x7a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "Hello123\0" - lowercase letters match
    let data2: [u8; 16] = [0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x31, 0x32, 0x33, 0x00, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_ranges_bytemask() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x44,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: range '0'-'9'
    let data1: [u8; 16] = [0x30, 0x39, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "abc123xyz\0" - digits match with 0xFF
    let data2: [u8; 16] = [0x61, 0x62, 0x63, 0x31, 0x32, 0x33, 0x78, 0x79, 0x7a, 0x00, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_ranges_multiple() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x04,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: ranges 'a'-'z', '0'-'9'
    let data1: [u8; 16] = [0x61, 0x7a, 0x30, 0x39, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "Test123!@#\0"
    let data2: [u8; 16] = [0x54, 0x65, 0x73, 0x74, 0x31, 0x32, 0x33, 0x21, 0x40, 0x23, 0x00, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Equal Each Mode
// ============================================================================

#[test]
fn test_pcmpistrm_equal_each_match() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x08,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "abcd\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "abcd\0" - all match
    let data2: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_equal_each_mismatch() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x08,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "abcd\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "abXd\0" - mismatch at index 2
    let data2: [u8; 16] = [0x61, 0x62, 0x58, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_equal_each_bytemask() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x48,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "abcd\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "abXd\0" - matched bytes get 0xFF, mismatched get 0x00
    let data2: [u8; 16] = [0x61, 0x62, 0x58, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Equal Ordered Mode
// ============================================================================

#[test]
fn test_pcmpistrm_equal_ordered_substring() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x0c,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "cd\0" - substring to find
    let data1: [u8; 16] = [0x63, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "abcdef\0" - contains "cd" at index 2-3
    let data2: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_equal_ordered_bytemask() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x4c,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "ab\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xabxabx\0" - "ab" found at positions 1 and 4
    let data2: [u8; 16] = [0x78, 0x61, 0x62, 0x78, 0x61, 0x62, 0x78, 0x00, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_equal_ordered_no_substring() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x0c,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "xyz\0"
    let data1: [u8; 16] = [0x78, 0x79, 0x7a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "abcdef\0" - doesn't contain "xyz", all zeros in mask
    let data2: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Negative Polarity Tests
// ============================================================================

#[test]
fn test_pcmpistrm_negative_polarity_bitmask() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x10,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "aeiou\0" - vowels
    let data1: [u8; 16] = [0x61, 0x65, 0x69, 0x6f, 0x75, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "bcdfg\0" - consonants, all non-vowels
    let data2: [u8; 16] = [0x62, 0x63, 0x64, 0x66, 0x67, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_negative_polarity_bytemask() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x50,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "abc\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xayczd\0" - non-matching chars get 0xFF
    let data2: [u8; 16] = [0x78, 0x61, 0x79, 0x63, 0x7a, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_negative_polarity_ranges() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x14,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: range '0'-'9'
    let data1: [u8; 16] = [0x30, 0x39, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "123abc\0" - non-digits match
    let data2: [u8; 16] = [0x31, 0x32, 0x33, 0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Most Significant Index Tests
// ============================================================================

#[test]
fn test_pcmpistrm_msb_bitmask() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x20,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "ab\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xaxbxax\0" - multiple matches
    let data2: [u8; 16] = [0x78, 0x61, 0x78, 0x62, 0x78, 0x61, 0x78, 0x00, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_msb_bytemask() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x60,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "ab\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xaxbxax\0"
    let data2: [u8; 16] = [0x78, 0x61, 0x78, 0x62, 0x78, 0x61, 0x78, 0x00, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Signed Data Tests
// ============================================================================

#[test]
fn test_pcmpistrm_signed_bytes() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x02,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: signed bytes including negative
    let data1: [u8; 16] = [0x01, 0xFF, 0x7F, 0x80, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: test string with negative value
    let data2: [u8; 16] = [0x05, 0xFF, 0x03, 0x80, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_signed_words() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x03,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: signed words [0x1234, 0x8000, 0x0000]
    let data1: [u8; 16] = [0x34, 0x12, 0x00, 0x80, 0x00, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: words with 0x8000 (negative)
    let data2: [u8; 16] = [0x56, 0x78, 0x00, 0x80, 0x00, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_pcmpistrm_memory_operand() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x3a, 0x62, 0x40, 0x10, 0x00, // PCMPISTRM XMM0, [RAX+0x10], 0x00
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x62, 0x79, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Extended Register Tests (XMM8-XMM15)
// ============================================================================

#[test]
fn test_pcmpistrm_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x00, // MOVDQA XMM8, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x48, 0x10, // MOVDQA XMM9, [RAX+0x10]
        0x66, 0x45, 0x0f, 0x3a, 0x62, 0xc1, 0x00, // PCMPISTRM XMM8, XMM9, 0x00
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x10, // MOVDQA XMM10, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x58, 0x10, // MOVDQA XMM11, [RAX+0x10]
        0x66, 0x45, 0x0f, 0x3a, 0x62, 0xd3, 0x0c, // PCMPISTRM XMM10, XMM11, 0x0C
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x61, 0x62, 0x79, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x20, // MOVDQA XMM12, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x68, 0x10, // MOVDQA XMM13, [RAX+0x10]
        0x66, 0x45, 0x0f, 0x3a, 0x62, 0xe5, 0x44, // PCMPISTRM XMM12, XMM13, 0x44
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x30, 0x39, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x61, 0x35, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x30, // MOVDQA XMM14, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x78, 0x10, // MOVDQA XMM15, [RAX+0x10]
        0x66, 0x45, 0x0f, 0x3a, 0x62, 0xf7, 0x48, // PCMPISTRM XMM14, XMM15, 0x48
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x61, 0x62, 0x58, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_pcmpistrm_empty_strings() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x00,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let data2: [u8; 16] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_full_16_bytes() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x40,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61]; // All 'a'
    let data2: [u8; 16] = [0x62, 0x61, 0x62, 0x61, 0x62, 0x61, 0x62, 0x61,
                           0x62, 0x61, 0x62, 0x61, 0x62, 0x61, 0x62, 0x61];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Different Register Combinations
// ============================================================================

#[test]
fn test_pcmpistrm_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x6f, 0x58, 0x10, // MOVDQA XMM3, [RAX+0x10]
        0x66, 0x0f, 0x3a, 0x62, 0xd3, 0x00, // PCMPISTRM XMM2, XMM3, 0x00
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x61, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x20, // MOVDQA XMM4, [RAX]
        0x66, 0x0f, 0x6f, 0x68, 0x10, // MOVDQA XMM5, [RAX+0x10]
        0x66, 0x0f, 0x3a, 0x62, 0xe5, 0x08, // PCMPISTRM XMM4, XMM5, 0x08
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x30, // MOVDQA XMM6, [RAX]
        0x66, 0x0f, 0x6f, 0x78, 0x10, // MOVDQA XMM7, [RAX+0x10]
        0x66, 0x0f, 0x3a, 0x62, 0xf7, 0x0c, // PCMPISTRM XMM6, XMM7, 0x0C
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x61, 0x62, 0x79, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Multiple Operations and Sequences
// ============================================================================

#[test]
fn test_pcmpistrm_sequence() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x00, // PCMPISTRM XMM0, XMM1, 0x00
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x40, // PCMPISTRM XMM0, XMM1, 0x40
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x08, // PCMPISTRM XMM0, XMM1, 0x08
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x62, 0x79, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_combined_modes() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x00, // Equal any, bit mask
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x04, // Ranges, bit mask
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x08, // Equal each, bit mask
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x0c, // Equal ordered, bit mask
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x62, 0x79, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_boundary_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x00,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: contains 0x00, 0xFF, 0x7F, 0x80
    let data1: [u8; 16] = [0x00, 0xFF, 0x7F, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: test against boundary values
    let data2: [u8; 16] = [0x01, 0x80, 0x02, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistrm_alternating_patterns() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x62, 0xc1, 0x40,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x61, 0x78, 0x62, 0x79, 0x61, 0x7a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}
