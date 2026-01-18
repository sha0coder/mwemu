use crate::*;

// PCMPISTRI - Packed Compare Implicit Length Strings, Return Index
//
// SSE4.2 instruction that performs a string comparison of two operands using a control byte
// to specify the comparison operation. The instruction operates on implicit length strings
// (null-terminated or using all 16 bytes).
//
// Returns the index of the first matching or non-matching element in ECX.
// Sets flags based on the comparison result:
//   CF = 1 if any match found (IntRes2 != 0)
//   ZF = 1 if end of string reached in second operand
//   SF = 1 if end of string reached in first operand
//   OF = 1 if ECX is valid (result index < 16)
//
// Control byte format (imm8):
//   Bits 0-1: Source data format
//     00 = Unsigned bytes (16 elements)
//     01 = Unsigned words (8 elements)
//     10 = Signed bytes (16 elements)
//     11 = Signed words (8 elements)
//   Bits 2-3: Aggregation operation
//     00 = Equal any
//     01 = Ranges
//     10 = Equal each
//     11 = Equal ordered
//   Bit 4: Polarity
//     0 = Positive polarity
//     1 = Negative polarity
//   Bit 5: Output selection
//     0 = Least significant index
//     1 = Most significant index
//   Bit 6: IntRes2 override
//     0 = No override
//     1 = Override (invert)
//
// Opcode:
//   66 0F 3A 63 /r ib    PCMPISTRI xmm1, xmm2/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Equal Any Mode (bits 2-3 = 00) - Unsigned Bytes
// ============================================================================

#[test]
fn test_pcmpistri_equal_any_ubytes_match_first() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x48, 0x10, // MOVDQA XMM1, [RAX+0x10]
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x00, // PCMPISTRI XMM0, XMM1, 0x00
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "abcd\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "cabc\0" - 'c' matches at index 0
    let data2: [u8; 16] = [0x63, 0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistri_equal_any_ubytes_no_match() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x00,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "abc\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xyz\0" - no matches
    let data2: [u8; 16] = [0x78, 0x79, 0x7a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistri_equal_any_ubytes_all_match() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x00,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "aaaa\0"
    let data1: [u8; 16] = [0x61, 0x61, 0x61, 0x61, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "aaaa\0" - all match
    let data2: [u8; 16] = [0x61, 0x61, 0x61, 0x61, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistri_equal_any_uwords() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x01,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: words [0x1234, 0x5678, 0x0000, ...]
    let data1: [u8; 16] = [0x34, 0x12, 0x78, 0x56, 0x00, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: words [0x5678, 0x1234, 0x0000, ...]
    let data2: [u8; 16] = [0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Ranges Mode (bits 2-3 = 01)
// ============================================================================

#[test]
fn test_pcmpistri_ranges_ubytes() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x04,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: range pairs 'a'-'z', 'A'-'Z'
    let data1: [u8; 16] = [0x61, 0x7a, 0x41, 0x5a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "Hello123\0"
    let data2: [u8; 16] = [0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x31, 0x32, 0x33, 0x00, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistri_ranges_digits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x04,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: range pair '0'-'9'
    let data1: [u8; 16] = [0x30, 0x39, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "abc123xyz\0"
    let data2: [u8; 16] = [0x61, 0x62, 0x63, 0x31, 0x32, 0x33, 0x78, 0x79, 0x7a, 0x00, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Equal Each Mode (bits 2-3 = 10)
// ============================================================================

#[test]
fn test_pcmpistri_equal_each_match() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x08,
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
fn test_pcmpistri_equal_each_mismatch() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x08,
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
fn test_pcmpistri_equal_each_partial() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x08,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "abcd\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "ab\0" - shorter string
    let data2: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Equal Ordered Mode (bits 2-3 = 11)
// ============================================================================

#[test]
fn test_pcmpistri_equal_ordered_substring() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x0c,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "cd\0" - substring to find
    let data1: [u8; 16] = [0x63, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "abcdef\0" - contains "cd" at index 2
    let data2: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistri_equal_ordered_no_substring() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x0c,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "xyz\0"
    let data1: [u8; 16] = [0x78, 0x79, 0x7a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "abcdef\0" - doesn't contain "xyz"
    let data2: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistri_equal_ordered_at_start() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x0c,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "ab\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "abcdef\0" - contains "ab" at index 0
    let data2: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Negative Polarity Tests (bit 4 = 1)
// ============================================================================

#[test]
fn test_pcmpistri_negative_polarity() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x10,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "aeiou\0" - vowels
    let data1: [u8; 16] = [0x61, 0x65, 0x69, 0x6f, 0x75, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "bcdfg\0" - consonants, first non-vowel at index 0
    let data2: [u8; 16] = [0x62, 0x63, 0x64, 0x66, 0x67, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistri_negative_polarity_ranges() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x14,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: range '0'-'9'
    let data1: [u8; 16] = [0x30, 0x39, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "123abc\0" - first non-digit at index 3 ('a')
    let data2: [u8; 16] = [0x31, 0x32, 0x33, 0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Most Significant Index (bit 5 = 1)
// ============================================================================

#[test]
fn test_pcmpistri_most_significant_index() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x20,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "ab\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xaxbxax\0" - last 'a' or 'b' at index 6 ('a')
    let data2: [u8; 16] = [0x78, 0x61, 0x78, 0x62, 0x78, 0x61, 0x78, 0x00, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistri_msb_equal_ordered() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x2c,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "ab\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xabxabx\0" - last occurrence of "ab" at index 4
    let data2: [u8; 16] = [0x78, 0x61, 0x62, 0x78, 0x61, 0x62, 0x78, 0x00, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Signed Data Tests
// ============================================================================

#[test]
fn test_pcmpistri_signed_bytes() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x02,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: signed bytes including negative
    let data1: [u8; 16] = [0x01, 0xFF, 0x7F, 0x80, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: test string with negative value
    let data2: [u8; 16] = [0x05, 0xFF, 0x03, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistri_signed_words() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x03,
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
fn test_pcmpistri_memory_operand() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x3a, 0x63, 0x40, 0x10, 0x00, // PCMPISTRI XMM0, [RAX+0x10], 0x00
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
fn test_pcmpistri_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x00, // MOVDQA XMM8, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x48, 0x10, // MOVDQA XMM9, [RAX+0x10]
        0x66, 0x45, 0x0f, 0x3a, 0x63, 0xc1, 0x00, // PCMPISTRI XMM8, XMM9, 0x00
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
fn test_pcmpistri_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x10, // MOVDQA XMM10, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x58, 0x10, // MOVDQA XMM11, [RAX+0x10]
        0x66, 0x45, 0x0f, 0x3a, 0x63, 0xd3, 0x0c, // PCMPISTRI XMM10, XMM11, 0x0C
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
fn test_pcmpistri_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x20, // MOVDQA XMM12, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x68, 0x10, // MOVDQA XMM13, [RAX+0x10]
        0x66, 0x45, 0x0f, 0x3a, 0x63, 0xe5, 0x04, // PCMPISTRI XMM12, XMM13, 0x04
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
fn test_pcmpistri_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x30, // MOVDQA XMM14, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x78, 0x10, // MOVDQA XMM15, [RAX+0x10]
        0x66, 0x45, 0x0f, 0x3a, 0x63, 0xf7, 0x08, // PCMPISTRI XMM14, XMM15, 0x08
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_pcmpistri_empty_strings() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x00,
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
fn test_pcmpistri_full_16_bytes() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x00,
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

#[test]
fn test_pcmpistri_override_bit() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x40,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x62, 0x63, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Multiple Operations
// ============================================================================

#[test]
fn test_pcmpistri_sequence() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x00, // PCMPISTRI XMM0, XMM1, 0x00
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x08, // PCMPISTRI XMM0, XMM1, 0x08
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x0c, // PCMPISTRI XMM0, XMM1, 0x0C
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
// Different Register Combinations
// ============================================================================

#[test]
fn test_pcmpistri_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x6f, 0x58, 0x10, // MOVDQA XMM3, [RAX+0x10]
        0x66, 0x0f, 0x3a, 0x63, 0xd3, 0x00, // PCMPISTRI XMM2, XMM3, 0x00
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
fn test_pcmpistri_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x20, // MOVDQA XMM4, [RAX]
        0x66, 0x0f, 0x6f, 0x68, 0x10, // MOVDQA XMM5, [RAX+0x10]
        0x66, 0x0f, 0x3a, 0x63, 0xe5, 0x08, // PCMPISTRI XMM4, XMM5, 0x08
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
fn test_pcmpistri_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x30, // MOVDQA XMM6, [RAX]
        0x66, 0x0f, 0x6f, 0x78, 0x10, // MOVDQA XMM7, [RAX+0x10]
        0x66, 0x0f, 0x3a, 0x63, 0xf7, 0x0c, // PCMPISTRI XMM6, XMM7, 0x0C
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
// Additional Control Byte Variations
// ============================================================================

#[test]
fn test_pcmpistri_control_0x18() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x18,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x61, 0x62, 0x58, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistri_control_0x1c() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x1c,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x79, 0x7a, 0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistri_control_0x30() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x30,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x62, 0x78, 0x79, 0x7a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistri_ranges_multiple() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x04,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: ranges 'a'-'z', '0'-'9'
    let data1: [u8; 16] = [0x61, 0x7a, 0x30, 0x39, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "Test123!@#"
    let data2: [u8; 16] = [0x54, 0x65, 0x73, 0x74, 0x31, 0x32, 0x33, 0x21, 0x40, 0x23, 0x00, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistri_case_insensitive_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x04,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: 'A'-'Z', 'a'-'z' ranges
    let data1: [u8; 16] = [0x41, 0x5a, 0x61, 0x7a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "Hello123!@#"
    let data2: [u8; 16] = [0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x31, 0x32, 0x33, 0x21, 0x40, 0x23, 0x00, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpistri_boundary_characters() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x00,
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
fn test_pcmpistri_xmm0_xmm1_various_modes() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x00, // Equal any
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x04, // Ranges
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x08, // Equal each
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x0c, // Equal ordered
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x62, 0x79, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}
