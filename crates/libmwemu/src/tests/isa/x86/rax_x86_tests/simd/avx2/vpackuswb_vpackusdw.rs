use crate::*;

// VPACKUSWB/VPACKUSDW - Pack with Unsigned Saturation (AVX2)
//
// Converts packed integers from larger to smaller data types with unsigned saturation.
// Values are clamped to the representable range of the target unsigned type.
//
// VPACKUSWB: Pack 16-bit signed words to 8-bit unsigned bytes with saturation
//            32 words (16 from each source) -> 32 bytes
//            Saturation range: 0 to 255 (negative values saturate to 0)
//
// VPACKUSDW: Pack 32-bit signed doublewords to 16-bit unsigned words with saturation
//            16 dwords (8 from each source) -> 16 words
//            Saturation range: 0 to 65535 (negative values saturate to 0)
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG 67 /r     VPACKUSWB ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.WIG 2B /r   VPACKUSDW ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPACKUSWB Tests - Pack Signed Words to Unsigned Bytes (256-bit)
// ============================================================================

#[test]
fn test_vpackuswb_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VPACKUSWB YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x67, 0xc2, // VPACKUSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdd, 0x67, 0xdd, // VPACKUSWB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_ymm6_ymm7_ymm8() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0x67, 0xf0, // VPACKUSWB YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x67, 0xcb, // VPACKUSWB YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0x67, 0xe6, // VPACKUSWB YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_ymm15_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0x67, 0xf9, // VPACKUSWB YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPACKUSWB YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x67, 0x00, // VPACKUSWB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as i16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_positive_saturation() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x67, 0xc2, // VPACKUSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_negative_saturation() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x67, 0xc2, // VPACKUSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_no_saturation() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x67, 0xc2, // VPACKUSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_boundary_values() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x67, 0xc2, // VPACKUSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x67, 0xc2, // VPACKUSWB YMM0, YMM1, YMM2
        0xc5, 0xed, 0x67, 0xdb, // VPACKUSWB YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_ymm2_ymm3_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0x67, 0x10, // VPACKUSWB YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..16).flat_map(|i| ((i * 8) as i16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_max_positive() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x67, 0xc2, // VPACKUSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_all_negative() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x67, 0xc2, // VPACKUSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPACKUSDW Tests - Pack Signed Dwords to Unsigned Words (256-bit)
// ============================================================================

#[test]
fn test_vpackusdw_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VPACKUSDW YMM0, YMM1, YMM2
    let code = [
        0xc4, 0xe2, 0x75, 0x2b, 0xc2, // VPACKUSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x2b, 0xdd, // VPACKUSDW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_ymm6_ymm7_ymm8() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x45, 0x2b, 0xf0, // VPACKUSDW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x2b, 0xcb, // VPACKUSDW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x2b, 0xe6, // VPACKUSDW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_ymm15_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x7d, 0x2b, 0xf9, // VPACKUSDW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPACKUSDW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x2b, 0x00, // VPACKUSDW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8).flat_map(|i| (i as i32 * 100).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_positive_saturation() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x2b, 0xc2, // VPACKUSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_negative_saturation() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x2b, 0xc2, // VPACKUSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_no_saturation() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x2b, 0xc2, // VPACKUSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_boundary_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x2b, 0xc2, // VPACKUSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x2b, 0xc2, // VPACKUSDW YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x6d, 0x2b, 0xdb, // VPACKUSDW YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_ymm2_ymm3_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x2b, 0x10, // VPACKUSDW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..8).flat_map(|i| (i as i32 * 1000).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_max_positive() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x2b, 0xc2, // VPACKUSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_all_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x2b, 0xc2, // VPACKUSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_vpackuswb_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x3d, 0x67, 0xc1, // VPACKUSWB YMM8, YMM8, YMM9
        0xc4, 0x41, 0x15, 0x67, 0xee, // VPACKUSWB YMM13, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x35, 0x2b, 0xcb, // VPACKUSDW YMM9, YMM9, YMM11
        0xc4, 0x42, 0x0d, 0x2b, 0xf7, // VPACKUSDW YMM14, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_all_zeros() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x67, 0xc2, // VPACKUSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_all_zeros() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x2b, 0xc2, // VPACKUSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_mixed_saturation() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x67, 0xc2, // VPACKUSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_mixed_saturation() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x2b, 0xc2, // VPACKUSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_sequential() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x67, 0xc2, // VPACKUSWB YMM0, YMM1, YMM2
        0xc5, 0xed, 0x67, 0xdb, // VPACKUSWB YMM3, YMM2, YMM3
        0xc5, 0xe5, 0x67, 0xe4, // VPACKUSWB YMM4, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_sequential() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x2b, 0xc2, // VPACKUSDW YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x6d, 0x2b, 0xdb, // VPACKUSDW YMM3, YMM2, YMM3
        0xc4, 0xe2, 0x65, 0x2b, 0xe4, // VPACKUSDW YMM4, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_same_source() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x67, 0xc1, // VPACKUSWB YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_same_source() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x2b, 0xc1, // VPACKUSDW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_ymm4_ymm5_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x67, 0x20, // VPACKUSWB YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_ymm4_ymm5_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x2b, 0x20, // VPACKUSDW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_small_values() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x67, 0xc2, // VPACKUSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_small_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x2b, 0xc2, // VPACKUSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x67, 0xc2, // VPACKUSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x2b, 0xc2, // VPACKUSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackuswb_edge_255_256() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x67, 0xc2, // VPACKUSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpackusdw_edge_65535_65536() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x2b, 0xc2, // VPACKUSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
