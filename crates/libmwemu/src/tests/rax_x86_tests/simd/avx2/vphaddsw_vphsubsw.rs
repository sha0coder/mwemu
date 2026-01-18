use crate::*;

// VPHADDSW/VPHSUBSW - Packed Horizontal Add/Subtract with Saturation (AVX2)
//
// Adds/subtracts pairs of adjacent signed 16-bit words from source and destination operands,
// saturating the results to signed 16-bit range and storing in the destination.
//
// VPHADDSW: Horizontally add 16 pairs of adjacent 16-bit signed words with signed saturation
// VPHSUBSW: Horizontally subtract 16 pairs of adjacent 16-bit signed words with signed saturation
//
// Saturation: If result > 32767, saturate to 32767; if result < -32768, saturate to -32768
//
// For VPHADDSW with YMM:
// dst[15:0]    = SATURATE(src1[31:16] + src1[15:0])
// dst[31:16]   = SATURATE(src1[63:48] + src1[47:32])
// dst[47:32]   = SATURATE(src1[95:80] + src1[79:64])
// dst[63:48]   = SATURATE(src1[127:112] + src1[111:96])
// dst[79:64]   = SATURATE(src2[31:16] + src2[15:0])
// (Similar pattern for upper 128 bits)
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F38.WIG 03 /r     VPHADDSW ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.WIG 07 /r     VPHSUBSW ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPHADDSW Tests - 16x Word Horizontal Add with Saturation (256-bit)
// ============================================================================

#[test]
fn test_vphaddsw_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPHADDSW YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc4, 0xe2, 0x75, 0x03, 0xc2, // VPHADDSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddsw_ymm3_ymm4_ymm5_all_ones() {
    let mut emu = emu64();
    // VPHADDSW YMM3, YMM4, YMM5
    let code = [
        0xc4, 0xe2, 0x5d, 0x03, 0xdd, // VPHADDSW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddsw_ymm6_ymm7_ymm8_positive_saturation() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x45, 0x03, 0xf0, // VPHADDSW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddsw_ymm9_ymm10_ymm11_negative_saturation() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x03, 0xcb, // VPHADDSW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddsw_ymm12_ymm13_ymm14_mixed() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x03, 0xe6, // VPHADDSW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddsw_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x03, 0xf9, // VPHADDSW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddsw_ymm0_ymm1_ymm2_no_saturation() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x03, 0xc2, // VPHADDSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddsw_ymm3_ymm4_ymm5_max_positive() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x03, 0xdd, // VPHADDSW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddsw_ymm6_ymm7_ymm8_max_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x45, 0x03, 0xf0, // VPHADDSW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddsw_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPHADDSW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x03, 0x00, // VPHADDSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| ((i as u16) * 0x1000).to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddsw_ymm2_ymm3_mem_saturation() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x03, 0x10, // VPHADDSW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0x4000u16; 16]
        .into_iter()
        .flat_map(|v| v.to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddsw_ymm4_ymm5_mem_negative_sat() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x03, 0x20, // VPHADDSW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0xC000u16; 16]
        .into_iter()
        .flat_map(|v| v.to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddsw_ymm6_ymm7_mem_mixed() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x45, 0x03, 0x30, // VPHADDSW YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| {
            if i % 2 == 0 {
                0x7FFFu16.to_le_bytes()
            } else {
                0x8000u16.to_le_bytes()
            }
        })
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// VPHSUBSW Tests - 16x Word Horizontal Subtract with Saturation (256-bit)
// ============================================================================

#[test]
fn test_vphsubsw_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPHSUBSW YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc4, 0xe2, 0x75, 0x07, 0xc2, // VPHSUBSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubsw_ymm3_ymm4_ymm5_all_ones() {
    let mut emu = emu64();
    // VPHSUBSW YMM3, YMM4, YMM5
    let code = [
        0xc4, 0xe2, 0x5d, 0x07, 0xdd, // VPHSUBSW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubsw_ymm6_ymm7_ymm8_positive_saturation() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x45, 0x07, 0xf0, // VPHSUBSW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubsw_ymm9_ymm10_ymm11_negative_saturation() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x07, 0xcb, // VPHSUBSW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubsw_ymm12_ymm13_ymm14_mixed() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x07, 0xe6, // VPHSUBSW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubsw_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x07, 0xf9, // VPHSUBSW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubsw_ymm0_ymm1_ymm2_no_saturation() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x07, 0xc2, // VPHSUBSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubsw_ymm3_ymm4_ymm5_max_positive() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x07, 0xdd, // VPHSUBSW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubsw_ymm6_ymm7_ymm8_max_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x45, 0x07, 0xf0, // VPHSUBSW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubsw_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPHSUBSW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x07, 0x00, // VPHSUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| ((i as u16) * 0x1000).to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubsw_ymm2_ymm3_mem_saturation() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x07, 0x10, // VPHSUBSW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    // 0x7FFF - 0xC000 = potential overflow
    let mut data = vec![0x7FFFu16; 8];
    data.extend(vec![0xC000u16; 8]);
    let bytes: Vec<u8> = data.into_iter().flat_map(|v| v.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &bytes);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubsw_ymm4_ymm5_mem_negative_sat() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x07, 0x20, // VPHSUBSW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    // 0x8000 - 0x7FFF = potential underflow
    let mut data = vec![0x8000u16; 8];
    data.extend(vec![0x7FFFu16; 8]);
    let bytes: Vec<u8> = data.into_iter().flat_map(|v| v.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &bytes);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubsw_ymm6_ymm7_mem_mixed() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x45, 0x07, 0x30, // VPHSUBSW YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| {
            if i % 2 == 0 {
                0x7FFFu16.to_le_bytes()
            } else {
                0x8000u16.to_le_bytes()
            }
        })
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_vphaddsw_chain_multiple_ops() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x03, 0xc2, // VPHADDSW YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x03, 0xc0, // VPHADDSW YMM0, YMM0, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubsw_chain_multiple_ops() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x07, 0xc2, // VPHSUBSW YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x07, 0xc0, // VPHSUBSW YMM0, YMM0, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddsw_boundary_positive() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x03, 0xc2, // VPHADDSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddsw_boundary_saturate() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x03, 0xc2, // VPHADDSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubsw_boundary_positive() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x07, 0xc2, // VPHSUBSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubsw_boundary_saturate() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x07, 0xc2, // VPHSUBSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddsw_same_register() {
    let mut emu = emu64();
    // VPHADDSW with same source registers
    let code = [
        0xc4, 0xe2, 0x75, 0x03, 0xc1, // VPHADDSW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubsw_same_register() {
    let mut emu = emu64();
    // VPHSUBSW with same source registers (should give all zeros)
    let code = [
        0xc4, 0xe2, 0x75, 0x07, 0xc1, // VPHSUBSW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddsw_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x03, 0x00, // VPHADDSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| if i % 2 == 0 { 0x3000u16 } else { 0x4FFFu16 }.to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubsw_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x07, 0x00, // VPHSUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| if i % 2 == 0 { 0x3000u16 } else { 0xD000u16 }.to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddsw_extreme_saturation() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x03, 0xc2, // VPHADDSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubsw_extreme_saturation() {
    let mut emu = emu64();
    // min - max = extreme negative
    let code = [
        0xc4, 0xe2, 0x75, 0x07, 0xc2, // VPHSUBSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddsw_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x03, 0x00, // VPHADDSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| ((i as i16 * 100) as u16).to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubsw_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x07, 0x00, // VPHSUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| ((i as i16 * 100) as u16).to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}
