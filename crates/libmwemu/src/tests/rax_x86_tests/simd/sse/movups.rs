use crate::*;

// MOVUPS - Move Unaligned Packed Single Precision Floating-Point Values
//
// Moves 128 bits (4 single-precision floating-point values) from source to destination.
// Unlike MOVAPS, this instruction does NOT require 16-byte alignment.
// Can move data to/from unaligned memory addresses without causing #GP.
//
// Opcodes:
// NP 0F 10 /r    MOVUPS xmm1, xmm2/m128    - Move unaligned packed single from xmm2/mem to xmm1
// NP 0F 11 /r    MOVUPS xmm2/m128, xmm1    - Move unaligned packed single from xmm1 to xmm2/mem

const UNALIGNED_ADDR: u64 = 0x3001; // Intentionally unaligned (offset by 1)
const ALIGNED_ADDR: u64 = 0x3000;   // 16-byte aligned (MOVUPS works with aligned too)

// ============================================================================
// Register to Register Tests
// ============================================================================

#[test]
fn test_movups_xmm0_to_xmm1() {
    let mut emu = emu64();
    // MOVUPS XMM1, XMM0
    let code = [
        0x0f, 0x10, 0xc8, // MOVUPS XMM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_xmm2_to_xmm3() {
    let mut emu = emu64();
    // MOVUPS XMM3, XMM2
    let code = [
        0x0f, 0x10, 0xda, // MOVUPS XMM3, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_xmm4_to_xmm5() {
    let mut emu = emu64();
    // MOVUPS XMM5, XMM4
    let code = [
        0x0f, 0x10, 0xec, // MOVUPS XMM5, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_xmm6_to_xmm7() {
    let mut emu = emu64();
    // MOVUPS XMM7, XMM6
    let code = [
        0x0f, 0x10, 0xfe, // MOVUPS XMM7, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_xmm8_to_xmm9() {
    let mut emu = emu64();
    // MOVUPS XMM9, XMM8 (requires REX prefix)
    let code = [
        0x45, 0x0f, 0x10, 0xc8, // MOVUPS XMM9, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_xmm10_to_xmm11() {
    let mut emu = emu64();
    // MOVUPS XMM11, XMM10
    let code = [
        0x45, 0x0f, 0x10, 0xda, // MOVUPS XMM11, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_xmm12_to_xmm13() {
    let mut emu = emu64();
    // MOVUPS XMM13, XMM12
    let code = [
        0x45, 0x0f, 0x10, 0xec, // MOVUPS XMM13, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_xmm14_to_xmm15() {
    let mut emu = emu64();
    // MOVUPS XMM15, XMM14
    let code = [
        0x45, 0x0f, 0x10, 0xfe, // MOVUPS XMM15, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_xmm0_to_xmm15() {
    let mut emu = emu64();
    // MOVUPS XMM15, XMM0
    let code = [
        0x44, 0x0f, 0x10, 0xf8, // MOVUPS XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_xmm15_to_xmm0() {
    let mut emu = emu64();
    // MOVUPS XMM0, XMM15
    let code = [
        0x44, 0x0f, 0x10, 0xc7, // MOVUPS XMM0, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Unaligned Memory to Register Tests
// ============================================================================

#[test]
fn test_movups_unaligned_mem_to_xmm0() {
    let mut emu = emu64();
    // MOVUPS XMM0, [unaligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10]);

    emu.run(None).unwrap();
}

#[test]
fn test_movups_unaligned_offset_1() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 1, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_unaligned_offset_2() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 2).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 2, &[0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA]);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_unaligned_offset_3() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 3).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 3, &[0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55]);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_unaligned_offset_7() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 7).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 7, &[0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33]);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_unaligned_offset_15() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 15).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 15, &[0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77]);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_aligned_mem_to_xmm1() {
    let mut emu = emu64();
    // MOVUPS also works with aligned memory
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x08, // MOVUPS XMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB]);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_mem_to_xmm8_unaligned() {
    let mut emu = emu64();
    // MOVUPS XMM8, [unaligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x10, 0x00, // MOVUPS XMM8, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &[0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC]);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_mem_to_xmm15_unaligned() {
    let mut emu = emu64();
    // MOVUPS XMM15, [unaligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x10, 0x38, // MOVUPS XMM15, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &[0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD]);
    emu.run(None).unwrap();
}

// ============================================================================
// Register to Unaligned Memory Tests
// ============================================================================

#[test]
fn test_movups_xmm0_to_unaligned_mem() {
    let mut emu = emu64();
    // MOVUPS [unaligned_addr], XMM0
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x11, 0x00, // MOVUPS [RAX], XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();

    let mut result = [0u8; 16];
    emu.maps.read_bytes_buff(&mut result, UNALIGNED_ADDR);
}

#[test]
fn test_movups_xmm1_to_unaligned_mem() {
    let mut emu = emu64();
    // MOVUPS [unaligned_addr], XMM1
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x11, 0x08, // MOVUPS [RAX], XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_xmm7_to_unaligned_mem() {
    let mut emu = emu64();
    // MOVUPS [unaligned_addr], XMM7
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x11, 0x38, // MOVUPS [RAX], XMM7
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_xmm8_to_unaligned_mem() {
    let mut emu = emu64();
    // MOVUPS [unaligned_addr], XMM8
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x11, 0x00, // MOVUPS [RAX], XMM8
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_xmm15_to_unaligned_mem() {
    let mut emu = emu64();
    // MOVUPS [unaligned_addr], XMM15
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x11, 0x38, // MOVUPS [RAX], XMM15
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

// ============================================================================
// Data Integrity Tests with Various Offsets
// ============================================================================

#[test]
fn test_movups_data_integrity_offset_1() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    let test_addr = ALIGNED_ADDR + 1;
    full_code.extend_from_slice(&test_addr.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let test_data = [0x10, 0x32, 0x54, 0x76, 0x98, 0xBA, 0xDC, 0xFE,
                     0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    emu.maps.write_bytes_slice(test_addr, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_movups_data_integrity_offset_4() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    let test_addr = ALIGNED_ADDR + 4;
    full_code.extend_from_slice(&test_addr.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let test_data = [0xF0, 0xE1, 0xD2, 0xC3, 0xB4, 0xA5, 0x96, 0x87,
                     0x78, 0x69, 0x5A, 0x4B, 0x3C, 0x2D, 0x1E, 0x0F];
    emu.maps.write_bytes_slice(test_addr, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_movups_data_integrity_offset_8() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    let test_addr = ALIGNED_ADDR + 8;
    full_code.extend_from_slice(&test_addr.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let test_data = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
                     0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00];
    emu.maps.write_bytes_slice(test_addr, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// Pattern Tests
// ============================================================================

#[test]
fn test_movups_all_zeros_unaligned() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_all_ones_unaligned() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_alternating_pattern_unaligned() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &[0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55]);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_float_values_unaligned() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let float1: f32 = 1.5;
    let float2: f32 = 2.5;
    let float3: f32 = 3.5;
    let float4: f32 = 4.5;

    let mut data = Vec::new();
    data.extend_from_slice(&float1.to_le_bytes());
    data.extend_from_slice(&float2.to_le_bytes());
    data.extend_from_slice(&float3.to_le_bytes());
    data.extend_from_slice(&float4.to_le_bytes());

    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Round-trip Tests
// ============================================================================

#[test]
fn test_movups_roundtrip_unaligned() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0x0f, 0x11, 0x40, 0x20, // MOVUPS [RAX+0x20], XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let test_data = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0,
                     0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_movups_chain_with_different_offsets() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0x48, 0xb8, // MOV RAX, imm64
    ]);
    full_code.extend_from_slice(&(ALIGNED_ADDR + 5).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x11, 0x00, // MOVUPS [RAX], XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 1, &[0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE]);
    emu.run(None).unwrap();
}

// ============================================================================
// Addressing Mode Tests with Unaligned Memory
// ============================================================================

#[test]
fn test_movups_base_displacement_unaligned() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(UNALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x40, 0x10, // MOVUPS XMM0, [RAX + 0x10]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &[0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99]);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_with_rbx_base_unaligned() {
    let mut emu = emu64();
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x03, // MOVUPS XMM0, [RBX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &[0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88]);
    emu.run(None).unwrap();
}

#[test]
fn test_movups_sequential_operations() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX] (aligned)
        0x0f, 0x10, 0x48, 0x01, // MOVUPS XMM1, [RAX+1] (unaligned)
        0x0f, 0x10, 0x50, 0x05, // MOVUPS XMM2, [RAX+5] (unaligned)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11]);
    emu.run(None).unwrap();
}
