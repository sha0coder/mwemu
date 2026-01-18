use crate::*;

// PCMPESTRI - Packed Compare Explicit Length Strings, Return Index
//
// SSE4.2 instruction similar to PCMPISTRI but uses explicit string lengths
// provided in EAX (for first operand) and EDX (for second operand) instead
// of relying on null terminators.
//
// Returns the index of the first matching or non-matching element in ECX.
// String lengths are specified in EAX (XMM1/first operand) and EDX (XMM2/second operand).
//
// Sets flags based on the comparison result:
//   CF = 1 if any match found (IntRes2 != 0)
//   ZF = 1 if EDX <= string length (end of second operand reached)
//   SF = 1 if EAX <= string length (end of first operand reached)
//   OF = 1 if ECX is valid (result index < 16)
//
// Control byte format (imm8) - same as PCMPISTRI:
//   Bits 0-1: Source data format (00=ubytes, 01=uwords, 10=sbytes, 11=swords)
//   Bits 2-3: Aggregation (00=equal any, 01=ranges, 10=equal each, 11=equal ordered)
//   Bit 4: Polarity (0=positive, 1=negative)
//   Bit 5: Output selection (0=LSB, 1=MSB)
//   Bit 6: IntRes2 override
//
// Opcode:
//   66 0F 3A 61 /r ib    PCMPESTRI xmm1, xmm2/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Equal Any Mode - Explicit Lengths
// ============================================================================

#[test]
fn test_pcmpestri_equal_any_basic() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x48, 0x10, // MOVDQA XMM1, [RAX+0x10]
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3 (length of first string)
        0xba, 0x05, 0x00, 0x00, 0x00, // MOV EDX, 5 (length of second string)
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x00, // PCMPESTRI XMM0, XMM1, 0x00
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "abc" (length 3, but no null terminator needed)
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xbycz" (length 5)
    let data2: [u8; 16] = [0x78, 0x62, 0x79, 0x63, 0x7a, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpestri_zero_length_first() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0 (zero length)
        0xba, 0x03, 0x00, 0x00, 0x00, // MOV EDX, 3
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x00,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x79, 0x7a, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpestri_zero_length_second() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0xba, 0x00, 0x00, 0x00, 0x00, // MOV EDX, 0 (zero length)
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x00,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x79, 0x7a, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpestri_full_16_bytes() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0xb8, 0x10, 0x00, 0x00, 0x00, // MOV EAX, 16 (full length)
        0xba, 0x10, 0x00, 0x00, 0x00, // MOV EDX, 16 (full length)
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x00,
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
fn test_pcmpestri_partial_lengths() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0xb8, 0x02, 0x00, 0x00, 0x00, // MOV EAX, 2 (only first 2 chars)
        0xba, 0x04, 0x00, 0x00, 0x00, // MOV EDX, 4 (only first 4 chars)
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x00,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x99, 0x99, 0x99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x62, 0x79, 0x61, 0x99, 0x99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Word Operations
// ============================================================================

#[test]
fn test_pcmpestri_uwords() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0xb8, 0x02, 0x00, 0x00, 0x00, // MOV EAX, 2 (2 words)
        0xba, 0x03, 0x00, 0x00, 0x00, // MOV EDX, 3 (3 words)
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x01,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: words [0x1234, 0x5678]
    let data1: [u8; 16] = [0x34, 0x12, 0x78, 0x56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: words [0xABCD, 0x5678, 0x1234]
    let data2: [u8; 16] = [0xCD, 0xAB, 0x78, 0x56, 0x34, 0x12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpestri_sword_length_8() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0xb8, 0x08, 0x00, 0x00, 0x00, // MOV EAX, 8 (all 8 words)
        0xba, 0x08, 0x00, 0x00, 0x00, // MOV EDX, 8
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x03,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x34, 0x12, 0x00, 0x80, 0xFF, 0xFF, 0x00, 0x00,
                           0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00];
    let data2: [u8; 16] = [0x56, 0x78, 0x00, 0x80, 0xFF, 0xFF, 0x00, 0x00,
                           0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Ranges Mode
// ============================================================================

#[test]
fn test_pcmpestri_ranges() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0xb8, 0x02, 0x00, 0x00, 0x00, // MOV EAX, 2 (1 range pair)
        0xba, 0x08, 0x00, 0x00, 0x00, // MOV EDX, 8
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x04,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: range 'a'-'z'
    let data1: [u8; 16] = [0x61, 0x7a, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "Hello123"
    let data2: [u8; 16] = [0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x31, 0x32, 0x33, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpestri_ranges_multiple() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0xb8, 0x04, 0x00, 0x00, 0x00, // MOV EAX, 4 (2 range pairs)
        0xba, 0x0a, 0x00, 0x00, 0x00, // MOV EDX, 10
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x04,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: ranges 'a'-'z', '0'-'9'
    let data1: [u8; 16] = [0x61, 0x7a, 0x30, 0x39, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "Test123!@#"
    let data2: [u8; 16] = [0x54, 0x65, 0x73, 0x74, 0x31, 0x32, 0x33, 0x21, 0x40, 0x23, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Equal Each Mode
// ============================================================================

#[test]
fn test_pcmpestri_equal_each() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0xb8, 0x04, 0x00, 0x00, 0x00, // MOV EAX, 4
        0xba, 0x04, 0x00, 0x00, 0x00, // MOV EDX, 4
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x08,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "abcd"
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "abXd"
    let data2: [u8; 16] = [0x61, 0x62, 0x58, 0x64, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpestri_equal_each_different_lengths() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xba, 0x03, 0x00, 0x00, 0x00, // MOV EDX, 3 (shorter)
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x08,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x61, 0x62, 0x63, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Equal Ordered Mode
// ============================================================================

#[test]
fn test_pcmpestri_equal_ordered_substring() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0xb8, 0x02, 0x00, 0x00, 0x00, // MOV EAX, 2 (substring "cd")
        0xba, 0x06, 0x00, 0x00, 0x00, // MOV EDX, 6 ("abcdef")
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x0c,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "cd" (substring to find)
    let data1: [u8; 16] = [0x63, 0x64, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "abcdef" (contains "cd" at index 2-3)
    let data2: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpestri_equal_ordered_no_match() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0xba, 0x06, 0x00, 0x00, 0x00, // MOV EDX, 6
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x0c,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "xyz"
    let data1: [u8; 16] = [0x78, 0x79, 0x7a, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "abcdef" (doesn't contain "xyz")
    let data2: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Negative Polarity
// ============================================================================

#[test]
fn test_pcmpestri_negative_polarity() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xba, 0x05, 0x00, 0x00, 0x00, // MOV EDX, 5
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x10,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "aeiou" (vowels)
    let data1: [u8; 16] = [0x61, 0x65, 0x69, 0x6f, 0x75, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "bcdfg" (consonants, first non-vowel at index 0)
    let data2: [u8; 16] = [0x62, 0x63, 0x64, 0x66, 0x67, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpestri_negative_ranges() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0xb8, 0x02, 0x00, 0x00, 0x00, // MOV EAX, 2
        0xba, 0x06, 0x00, 0x00, 0x00, // MOV EDX, 6
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x14,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: range '0'-'9'
    let data1: [u8; 16] = [0x30, 0x39, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "123abc" - first non-digit at index 3 ('a')
    let data2: [u8; 16] = [0x31, 0x32, 0x33, 0x61, 0x62, 0x63, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Most Significant Index
// ============================================================================

#[test]
fn test_pcmpestri_msb() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0xb8, 0x02, 0x00, 0x00, 0x00, // MOV EAX, 2
        0xba, 0x07, 0x00, 0x00, 0x00, // MOV EDX, 7
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x20,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // XMM0: "ab"
    let data1: [u8; 16] = [0x61, 0x62, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xaxbxax" - last 'a' or 'b' at index 5
    let data2: [u8; 16] = [0x78, 0x61, 0x78, 0x62, 0x78, 0x61, 0x78, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Extended Register Tests
// ============================================================================

#[test]
fn test_pcmpestri_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x00, // MOVDQA XMM8, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x48, 0x10, // MOVDQA XMM9, [RAX+0x10]
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0xba, 0x03, 0x00, 0x00, 0x00, // MOV EDX, 3
        0x66, 0x45, 0x0f, 0x3a, 0x61, 0xc1, 0x00, // PCMPESTRI XMM8, XMM9, 0x00
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x62, 0x79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpestri_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x10, // MOVDQA XMM10, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x58, 0x10, // MOVDQA XMM11, [RAX+0x10]
        0xb8, 0x02, 0x00, 0x00, 0x00, // MOV EAX, 2
        0xba, 0x04, 0x00, 0x00, 0x00, // MOV EDX, 4
        0x66, 0x45, 0x0f, 0x3a, 0x61, 0xd3, 0x0c, // PCMPESTRI XMM10, XMM11, 0x0C
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x61, 0x62, 0x79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpestri_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x20, // MOVDQA XMM12, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x68, 0x10, // MOVDQA XMM13, [RAX+0x10]
        0xb8, 0x02, 0x00, 0x00, 0x00, // MOV EAX, 2
        0xba, 0x03, 0x00, 0x00, 0x00, // MOV EDX, 3
        0x66, 0x45, 0x0f, 0x3a, 0x61, 0xe5, 0x04, // PCMPESTRI XMM12, XMM13, 0x04
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x30, 0x39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x61, 0x35, 0x62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpestri_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x30, // MOVDQA XMM14, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x78, 0x10, // MOVDQA XMM15, [RAX+0x10]
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0xba, 0x03, 0x00, 0x00, 0x00, // MOV EDX, 3
        0x66, 0x45, 0x0f, 0x3a, 0x61, 0xf7, 0x08, // PCMPESTRI XMM14, XMM15, 0x08
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x61, 0x62, 0x63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_pcmpestri_memory_operand() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0xba, 0x03, 0x00, 0x00, 0x00, // MOV EDX, 3
        0x66, 0x0f, 0x3a, 0x61, 0x40, 0x10, 0x00, // PCMPESTRI XMM0, [RAX+0x10], 0x00
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x62, 0x79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional Register Combinations
// ============================================================================

#[test]
fn test_pcmpestri_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x6f, 0x58, 0x10, // MOVDQA XMM3, [RAX+0x10]
        0xb8, 0x02, 0x00, 0x00, 0x00,
        0xba, 0x02, 0x00, 0x00, 0x00,
        0x66, 0x0f, 0x3a, 0x61, 0xd3, 0x00, // PCMPESTRI XMM2, XMM3, 0x00
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpestri_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x20, // MOVDQA XMM4, [RAX]
        0x66, 0x0f, 0x6f, 0x68, 0x10, // MOVDQA XMM5, [RAX+0x10]
        0xb8, 0x03, 0x00, 0x00, 0x00,
        0xba, 0x03, 0x00, 0x00, 0x00,
        0x66, 0x0f, 0x3a, 0x61, 0xe5, 0x08, // PCMPESTRI XMM4, XMM5, 0x08
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x61, 0x62, 0x63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpestri_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x30, // MOVDQA XMM6, [RAX]
        0x66, 0x0f, 0x6f, 0x78, 0x10, // MOVDQA XMM7, [RAX+0x10]
        0xb8, 0x02, 0x00, 0x00, 0x00,
        0xba, 0x04, 0x00, 0x00, 0x00,
        0x66, 0x0f, 0x3a, 0x61, 0xf7, 0x0c, // PCMPESTRI XMM6, XMM7, 0x0C
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x61, 0x62, 0x79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Length Edge Cases
// ============================================================================

#[test]
fn test_pcmpestri_max_byte_length() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0xb8, 0x10, 0x00, 0x00, 0x00, // MOV EAX, 16 (max for bytes)
        0xba, 0x10, 0x00, 0x00, 0x00, // MOV EDX, 16
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x00,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68,
                           0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70];
    let data2: [u8; 16] = [0x71, 0x62, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77,
                           0x78, 0x79, 0x7a, 0x61, 0x62, 0x63, 0x64, 0x65];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpestri_max_word_length() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0xb8, 0x08, 0x00, 0x00, 0x00, // MOV EAX, 8 (max for words)
        0xba, 0x08, 0x00, 0x00, 0x00, // MOV EDX, 8
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x01,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00,
                           0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00];
    let data2: [u8; 16] = [0x09, 0x00, 0x02, 0x00, 0x0a, 0x00, 0x0b, 0x00,
                           0x0c, 0x00, 0x0d, 0x00, 0x0e, 0x00, 0x0f, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Sequence and Multiple Operations
// ============================================================================

#[test]
fn test_pcmpestri_sequence_operations() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0xb8, 0x03, 0x00, 0x00, 0x00,
        0xba, 0x03, 0x00, 0x00, 0x00,
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x00, // PCMPESTRI XMM0, XMM1, 0x00
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x08, // PCMPESTRI XMM0, XMM1, 0x08
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x0c, // PCMPESTRI XMM0, XMM1, 0x0C
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x62, 0x79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpestri_varying_lengths() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x48, 0x10,
        0xb8, 0x01, 0x00, 0x00, 0x00, // Length 1
        0xba, 0x01, 0x00, 0x00, 0x00,
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x00,
        0xb8, 0x05, 0x00, 0x00, 0x00, // Length 5
        0xba, 0x05, 0x00, 0x00, 0x00,
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x00,
        0xb8, 0x10, 0x00, 0x00, 0x00, // Length 16
        0xba, 0x10, 0x00, 0x00, 0x00,
        0x66, 0x0f, 0x3a, 0x61, 0xc1, 0x00,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68,
                           0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70];
    let data2: [u8; 16] = [0x71, 0x62, 0x72, 0x63, 0x73, 0x74, 0x75, 0x76,
                           0x77, 0x78, 0x79, 0x7a, 0x61, 0x62, 0x63, 0x64];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 0x10, &data2);
    emu.run(None).unwrap();
}
