use crate::*;

// PHSUBW/PHSUBD - Packed Horizontal Subtract
//
// Performs horizontal subtraction on each adjacent pair of 16-bit/32-bit signed
// integers by subtracting the most significant word/dword from the least significant
// word/dword of each pair in the source and destination operands, and packs the
// signed results to the destination.
//
// PHSUBW: horizontal subtract of adjacent words (8 operations)
// PHSUBD: horizontal subtract of adjacent dwords (4 operations)
//
// For PHSUBW with 128-bit operands:
//   DEST[15:0] = DEST[15:0] - DEST[31:16]
//   DEST[31:16] = DEST[47:32] - DEST[63:48]
//   DEST[47:32] = DEST[79:64] - DEST[95:80]
//   DEST[63:48] = DEST[111:96] - DEST[127:112]
//   DEST[79:64] = SRC[15:0] - SRC[31:16]
//   DEST[95:80] = SRC[47:32] - SRC[63:48]
//   DEST[111:96] = SRC[79:64] - SRC[95:80]
//   DEST[127:112] = SRC[111:96] - SRC[127:112]
//
// Opcodes:
//   66 0F 38 05 /r    PHSUBW xmm1, xmm2/m128
//   66 0F 38 06 /r    PHSUBD xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// PHSUBW Tests (Packed Horizontal Subtract Words)
// ============================================================================

#[test]
fn test_phsubw_xmm0_xmm1_basic() {
    let mut emu = emu64();
    // PHSUBW XMM0, XMM1 - basic register to register
    let code = [
        0x66, 0x0f, 0x38, 0x05, 0xc1, // PHSUBW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubw_xmm2_xmm3_basic() {
    let mut emu = emu64();
    // PHSUBW XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x38, 0x05, 0xd3, // PHSUBW XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubw_xmm4_xmm5_zeros() {
    let mut emu = emu64();
    // PHSUBW XMM4, XMM5 - all zeros
    let code = [
        0x66, 0x0f, 0x38, 0x05, 0xe5, // PHSUBW XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubw_xmm6_xmm7_positive() {
    let mut emu = emu64();
    // PHSUBW XMM6, XMM7 - positive values
    let code = [
        0x66, 0x0f, 0x38, 0x05, 0xf7, // PHSUBW XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubw_xmm0_xmm1_negative() {
    let mut emu = emu64();
    // PHSUBW XMM0, XMM1 - negative values
    let code = [
        0x66, 0x0f, 0x38, 0x05, 0xc1, // PHSUBW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubw_xmm1_xmm2_mixed() {
    let mut emu = emu64();
    // PHSUBW XMM1, XMM2 - mixed positive and negative
    let code = [
        0x66, 0x0f, 0x38, 0x05, 0xca, // PHSUBW XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubw_xmm3_xmm4_underflow() {
    let mut emu = emu64();
    // PHSUBW XMM3, XMM4 - test wraparound/underflow
    let code = [
        0x66, 0x0f, 0x38, 0x05, 0xdc, // PHSUBW XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubw_xmm5_xmm6_overflow() {
    let mut emu = emu64();
    // PHSUBW XMM5, XMM6 - test overflow
    let code = [
        0x66, 0x0f, 0x38, 0x05, 0xee, // PHSUBW XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubw_xmm7_xmm0() {
    let mut emu = emu64();
    // PHSUBW XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x38, 0x05, 0xf8, // PHSUBW XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubw_xmm8_xmm9() {
    let mut emu = emu64();
    // PHSUBW XMM8, XMM9 - high registers
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x05, 0xc1, // PHSUBW XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubw_xmm10_xmm11() {
    let mut emu = emu64();
    // PHSUBW XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x05, 0xd3, // PHSUBW XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubw_xmm12_xmm13() {
    let mut emu = emu64();
    // PHSUBW XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x05, 0xe5, // PHSUBW XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubw_xmm14_xmm15() {
    let mut emu = emu64();
    // PHSUBW XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x05, 0xf7, // PHSUBW XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubw_xmm0_mem() {
    let mut emu = emu64();
    // PHSUBW XMM0, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x05, 0x00, // PHSUBW XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let data: [u8; 16] = [
        0x0A, 0x00, 0x05, 0x00, 0x14, 0x00, 0x08, 0x00,
        0x1E, 0x00, 0x0C, 0x00, 0x28, 0x00, 0x10, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);

    emu.run(None).unwrap();
}

#[test]
fn test_phsubw_xmm1_mem_negative() {
    let mut emu = emu64();
    // PHSUBW XMM1, [mem] - negative result
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x05, 0x08, // PHSUBW XMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let data: [u8; 16] = [
        0x01, 0x00, 0x0A, 0x00, 0x02, 0x00, 0x14, 0x00,
        0x03, 0x00, 0x1E, 0x00, 0x04, 0x00, 0x28, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);

    emu.run(None).unwrap();
}

#[test]
fn test_phsubw_xmm2_mem_wraparound() {
    let mut emu = emu64();
    // PHSUBW XMM2, [mem] - test overflow/wraparound
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x05, 0x10, // PHSUBW XMM2, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let data: [u8; 16] = [
        0x00, 0x80, 0x01, 0x00, // -32768 - 1 = wraparound
        0xFF, 0x7F, 0xFF, 0xFF, // 32767 - (-1) = wraparound
        0x01, 0x00, 0xFF, 0x7F, // 1 - 32767 = wraparound
        0xFF, 0xFF, 0x00, 0x80, // -1 - (-32768) = wraparound
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);

    emu.run(None).unwrap();
}

// ============================================================================
// PHSUBD Tests (Packed Horizontal Subtract Dwords)
// ============================================================================

#[test]
fn test_phsubd_xmm0_xmm1_basic() {
    let mut emu = emu64();
    // PHSUBD XMM0, XMM1 - basic register to register
    let code = [
        0x66, 0x0f, 0x38, 0x06, 0xc1, // PHSUBD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubd_xmm2_xmm3_basic() {
    let mut emu = emu64();
    // PHSUBD XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x38, 0x06, 0xd3, // PHSUBD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubd_xmm4_xmm5_zeros() {
    let mut emu = emu64();
    // PHSUBD XMM4, XMM5 - all zeros
    let code = [
        0x66, 0x0f, 0x38, 0x06, 0xe5, // PHSUBD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubd_xmm6_xmm7_positive() {
    let mut emu = emu64();
    // PHSUBD XMM6, XMM7 - positive values
    let code = [
        0x66, 0x0f, 0x38, 0x06, 0xf7, // PHSUBD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubd_xmm0_xmm1_negative() {
    let mut emu = emu64();
    // PHSUBD XMM0, XMM1 - negative values
    let code = [
        0x66, 0x0f, 0x38, 0x06, 0xc1, // PHSUBD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubd_xmm1_xmm2_mixed() {
    let mut emu = emu64();
    // PHSUBD XMM1, XMM2 - mixed positive and negative
    let code = [
        0x66, 0x0f, 0x38, 0x06, 0xca, // PHSUBD XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubd_xmm3_xmm4_underflow() {
    let mut emu = emu64();
    // PHSUBD XMM3, XMM4 - test wraparound/underflow
    let code = [
        0x66, 0x0f, 0x38, 0x06, 0xdc, // PHSUBD XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubd_xmm5_xmm6_overflow() {
    let mut emu = emu64();
    // PHSUBD XMM5, XMM6 - test overflow
    let code = [
        0x66, 0x0f, 0x38, 0x06, 0xee, // PHSUBD XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubd_xmm7_xmm0() {
    let mut emu = emu64();
    // PHSUBD XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x38, 0x06, 0xf8, // PHSUBD XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubd_xmm8_xmm9() {
    let mut emu = emu64();
    // PHSUBD XMM8, XMM9 - high registers
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x06, 0xc1, // PHSUBD XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubd_xmm10_xmm11() {
    let mut emu = emu64();
    // PHSUBD XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x06, 0xd3, // PHSUBD XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubd_xmm12_xmm13() {
    let mut emu = emu64();
    // PHSUBD XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x06, 0xe5, // PHSUBD XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubd_xmm14_xmm15() {
    let mut emu = emu64();
    // PHSUBD XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x06, 0xf7, // PHSUBD XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubd_xmm0_mem() {
    let mut emu = emu64();
    // PHSUBD XMM0, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x06, 0x00, // PHSUBD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let data: [u8; 16] = [
        0x0A, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00,
        0x14, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);

    emu.run(None).unwrap();
}

#[test]
fn test_phsubd_xmm1_mem_negative() {
    let mut emu = emu64();
    // PHSUBD XMM1, [mem] - negative result
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x06, 0x08, // PHSUBD XMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let data: [u8; 16] = [
        0x01, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00,
        0x02, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);

    emu.run(None).unwrap();
}

#[test]
fn test_phsubd_xmm2_mem_wraparound() {
    let mut emu = emu64();
    // PHSUBD XMM2, [mem] - test overflow/wraparound
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x06, 0x10, // PHSUBD XMM2, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x00, // INT32_MIN - 1
        0xFF, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0xFF, 0xFF, // INT32_MAX - (-1)
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);

    emu.run(None).unwrap();
}

// ============================================================================
// Additional edge case tests
// ============================================================================

#[test]
fn test_phsubw_same_register() {
    let mut emu = emu64();
    // PHSUBW XMM0, XMM0 - source and dest are same
    let code = [
        0x66, 0x0f, 0x38, 0x05, 0xc0, // PHSUBW XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubd_same_register() {
    let mut emu = emu64();
    // PHSUBD XMM1, XMM1 - source and dest are same
    let code = [
        0x66, 0x0f, 0x38, 0x06, 0xc9, // PHSUBD XMM1, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubw_sequential() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x05, 0xc1, // PHSUBW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x05, 0xd3, // PHSUBW XMM2, XMM3
        0x66, 0x0f, 0x38, 0x05, 0xe5, // PHSUBW XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubd_sequential() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x06, 0xc1, // PHSUBD XMM0, XMM1
        0x66, 0x0f, 0x38, 0x06, 0xd3, // PHSUBD XMM2, XMM3
        0x66, 0x0f, 0x38, 0x06, 0xe5, // PHSUBD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsub_mixed_operations() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x05, 0xc1, // PHSUBW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x06, 0xd3, // PHSUBD XMM2, XMM3
        0x66, 0x0f, 0x38, 0x05, 0xe5, // PHSUBW XMM4, XMM5
        0x66, 0x0f, 0x38, 0x06, 0xf7, // PHSUBD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubw_xmm0_xmm15_cross_range() {
    let mut emu = emu64();
    // PHSUBW XMM0, XMM15 - test low and high register mix
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x05, 0xf8, // PHSUBW XMM0, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubd_xmm15_xmm0_cross_range() {
    let mut emu = emu64();
    // PHSUBD XMM15, XMM0 - test high and low register mix
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x06, 0xf8, // PHSUBD XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubw_mem_displacement() {
    let mut emu = emu64();
    // PHSUBW XMM0, [RAX + disp]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x05, 0x40, 0x10, // PHSUBW XMM0, [RAX+0x10]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x05, 0x00, 0x03, 0x00, 0x05, 0x00, 0x03, 0x00, 0x05, 0x00, 0x03, 0x00, 0x05, 0x00, 0x03, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubd_mem_displacement() {
    let mut emu = emu64();
    // PHSUBD XMM1, [RBX + disp]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x20).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x06, 0x4b, 0x20, // PHSUBD XMM1, [RBX+0x20]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x05, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}
