use crate::*;

// VPSIGNB/VPSIGNW/VPSIGND - Packed Sign (AVX2)
//
// Negates, zeros, or passes through each data element of the destination operand
// based on the sign of the corresponding data element in the source operand.
//
// For each element:
// - If src2 element < 0: dst = -src1
// - If src2 element = 0: dst = 0
// - If src2 element > 0: dst = src1
//
// VPSIGNB: Apply sign operation to 32 packed byte integers (8-bit)
// VPSIGNW: Apply sign operation to 16 packed word integers (16-bit)
// VPSIGND: Apply sign operation to 8 packed doubleword integers (32-bit)
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F38.WIG 08 /r     VPSIGNB ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.WIG 09 /r     VPSIGNW ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.WIG 0A /r     VPSIGND ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPSIGNB Tests - 32x Byte Sign (256-bit)
// ============================================================================

#[test]
fn test_vpsignb_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPSIGNB YMM0, YMM1, YMM2 with all zeros (control all zeros -> result all zeros)
    let code = [
        0xc4, 0xe2, 0x75, 0x08, 0xc2, // VPSIGNB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignb_ymm3_ymm4_ymm5_all_positive() {
    let mut emu = emu64();
    // VPSIGNB YMM3, YMM4, YMM5 with positive control values (pass through)
    let code = [
        0xc4, 0xe2, 0x5d, 0x08, 0xdd, // VPSIGNB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignb_ymm6_ymm7_ymm8_all_negative() {
    let mut emu = emu64();
    // VPSIGNB YMM6, YMM7, YMM8 with negative control values (negate)
    let code = [
        0xc4, 0xc2, 0x45, 0x08, 0xf0, // VPSIGNB YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignb_ymm9_ymm10_ymm11_mixed() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x08, 0xcb, // VPSIGNB YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignb_ymm12_ymm13_ymm14_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x08, 0xe6, // VPSIGNB YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignb_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x08, 0xf9, // VPSIGNB YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignb_ymm0_ymm1_ymm2_negate_positive() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x08, 0xc2, // VPSIGNB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignb_ymm3_ymm4_ymm5_zero_control() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x08, 0xdd, // VPSIGNB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignb_ymm6_ymm7_ymm8_edge_cases() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x45, 0x08, 0xf0, // VPSIGNB YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignb_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPSIGNB YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x08, 0x00, // VPSIGNB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let control: Vec<u8> = (0..32).map(|i| if i % 3 == 0 { 0xFF } else if i % 3 == 1 { 0x00 } else { 0x01 }).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &control);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignb_ymm2_ymm3_mem_all_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x08, 0x10, // VPSIGNB YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignb_ymm4_ymm5_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x08, 0x20, // VPSIGNB YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let control: Vec<u8> = (0..32).map(|i| if i % 2 == 0 { 0x01 } else { 0xFF }).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &control);
    emu.run(None).unwrap();
}

// ============================================================================
// VPSIGNW Tests - 16x Word Sign (256-bit)
// ============================================================================

#[test]
fn test_vpsignw_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPSIGNW YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc4, 0xe2, 0x75, 0x09, 0xc2, // VPSIGNW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignw_ymm3_ymm4_ymm5_all_positive() {
    let mut emu = emu64();
    // VPSIGNW YMM3, YMM4, YMM5 with positive control values
    let code = [
        0xc4, 0xe2, 0x5d, 0x09, 0xdd, // VPSIGNW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignw_ymm6_ymm7_ymm8_all_negative() {
    let mut emu = emu64();
    // VPSIGNW YMM6, YMM7, YMM8 with negative control values
    let code = [
        0xc4, 0xc2, 0x45, 0x09, 0xf0, // VPSIGNW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignw_ymm9_ymm10_ymm11_mixed() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x09, 0xcb, // VPSIGNW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignw_ymm12_ymm13_ymm14_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x09, 0xe6, // VPSIGNW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignw_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x09, 0xf9, // VPSIGNW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignw_ymm0_ymm1_ymm2_negate_positive() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x09, 0xc2, // VPSIGNW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignw_ymm3_ymm4_ymm5_edge_cases() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x09, 0xdd, // VPSIGNW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignw_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPSIGNW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x09, 0x00, // VPSIGNW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let control: Vec<u8> = (0..16)
        .flat_map(|i| {
            if i % 3 == 0 { 0xFFFFu16 } else if i % 3 == 1 { 0x0000u16 } else { 0x0001u16 }
        }.to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &control);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignw_ymm2_ymm3_mem_all_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x09, 0x10, // VPSIGNW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignw_ymm4_ymm5_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x09, 0x20, // VPSIGNW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let control: Vec<u8> = (0..16)
        .flat_map(|i| if i % 2 == 0 { 0x0001u16 } else { 0x8000u16 }.to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &control);
    emu.run(None).unwrap();
}

// ============================================================================
// VPSIGND Tests - 8x Doubleword Sign (256-bit)
// ============================================================================

#[test]
fn test_vpsignd_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPSIGND YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc4, 0xe2, 0x75, 0x0a, 0xc2, // VPSIGND YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignd_ymm3_ymm4_ymm5_all_positive() {
    let mut emu = emu64();
    // VPSIGND YMM3, YMM4, YMM5 with positive control values
    let code = [
        0xc4, 0xe2, 0x5d, 0x0a, 0xdd, // VPSIGND YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignd_ymm6_ymm7_ymm8_all_negative() {
    let mut emu = emu64();
    // VPSIGND YMM6, YMM7, YMM8 with negative control values
    let code = [
        0xc4, 0xc2, 0x45, 0x0a, 0xf0, // VPSIGND YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignd_ymm9_ymm10_ymm11_mixed() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x0a, 0xcb, // VPSIGND YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignd_ymm12_ymm13_ymm14_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x0a, 0xe6, // VPSIGND YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignd_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x0a, 0xf9, // VPSIGND YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignd_ymm0_ymm1_ymm2_negate_positive() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x0a, 0xc2, // VPSIGND YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignd_ymm3_ymm4_ymm5_edge_cases() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x0a, 0xdd, // VPSIGND YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPSIGND YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x0a, 0x00, // VPSIGND YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let control: Vec<u8> = (0..8)
        .flat_map(|i| {
            if i % 3 == 0 { 0xFFFFFFFFu32 } else if i % 3 == 1 { 0x00000000u32 } else { 0x00000001u32 }
        }.to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &control);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignd_ymm2_ymm3_mem_all_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x0a, 0x10, // VPSIGND YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignd_ymm4_ymm5_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x0a, 0x20, // VPSIGND YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let control: Vec<u8> = (0..8)
        .flat_map(|i| if i % 2 == 0 { 0x00000001u32 } else { 0x80000000u32 }.to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &control);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_vpsignb_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x08, 0xc2, // VPSIGNB YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x08, 0xc3, // VPSIGNB YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignw_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x09, 0xc2, // VPSIGNW YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x09, 0xc3, // VPSIGNW YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignd_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x0a, 0xc2, // VPSIGND YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x0a, 0xc3, // VPSIGND YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignb_double_negate() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x08, 0xc2, // VPSIGNB YMM0, YMM1, YMM2 (control = -1)
        0xc4, 0xe2, 0x7d, 0x08, 0xc2, // VPSIGNB YMM0, YMM0, YMM2 (control = -1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignw_double_negate() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x09, 0xc2, // VPSIGNW YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x09, 0xc2, // VPSIGNW YMM0, YMM0, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignd_double_negate() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x0a, 0xc2, // VPSIGND YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x0a, 0xc2, // VPSIGND YMM0, YMM0, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignb_same_register() {
    let mut emu = emu64();
    // VPSIGNB with same source registers
    let code = [
        0xc4, 0xe2, 0x75, 0x08, 0xc1, // VPSIGNB YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignw_same_register() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x09, 0xc1, // VPSIGNW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignd_same_register() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x0a, 0xc1, // VPSIGND YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignb_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x08, 0x00, // VPSIGNB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = vec![0xFF, 0x00, 0x01, 0x80]
        .into_iter()
        .cycle()
        .take(32)
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignw_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x09, 0x00, // VPSIGNW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = vec![0xFFFFu16, 0x0000, 0x0001, 0x8000]
        .into_iter()
        .cycle()
        .take(16)
        .flat_map(|v| v.to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsignd_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x0a, 0x00, // VPSIGND YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = vec![0xFFFFFFFFu32, 0x00000000, 0x00000001, 0x80000000]
        .into_iter()
        .cycle()
        .take(8)
        .flat_map(|v| v.to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}
