use crate::*;

// VMOVAPS - Move Aligned Packed Single Precision Floating-Point Values (256-bit)
// VMOVAPD - Move Aligned Packed Double Precision Floating-Point Values (256-bit)
//
// VMOVAPS moves 256 bits (8 single-precision floating-point values) from source to destination.
// VMOVAPD moves 256 bits (4 double-precision floating-point values) from source to destination.
// When the operand is a memory location, it must be aligned on a 32-byte boundary.
// Otherwise, a general-protection exception (#GP) is generated.
//
// Opcodes:
// VEX.256.NP 0F 28 /r    VMOVAPS ymm1, ymm2/m256   - Move aligned packed single from ymm2/mem to ymm1
// VEX.256.NP 0F 29 /r    VMOVAPS ymm2/m256, ymm1   - Move aligned packed single from ymm1 to ymm2/mem
// VEX.256.66 0F 28 /r    VMOVAPD ymm1, ymm2/m256   - Move aligned packed double from ymm2/mem to ymm1
// VEX.256.66 0F 29 /r    VMOVAPD ymm2/m256, ymm1   - Move aligned packed double from ymm1 to ymm2/mem

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VMOVAPS Tests - Packed Single Precision (8x float32 - 256-bit)
// ============================================================================

#[test]
fn test_vmovaps_ymm0_to_ymm1() {
    let mut emu = emu64();
    // VMOVAPS YMM1, YMM0
    let code = [
        0xc5, 0xfc, 0x28, 0xc8, // VMOVAPS YMM1, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovaps_ymm1_to_ymm2() {
    let mut emu = emu64();
    // VMOVAPS YMM2, YMM1
    let code = [
        0xc5, 0xfc, 0x28, 0xd1, // VMOVAPS YMM2, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovaps_ymm2_to_ymm3() {
    let mut emu = emu64();
    // VMOVAPS YMM3, YMM2
    let code = [
        0xc5, 0xfc, 0x28, 0xda, // VMOVAPS YMM3, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovaps_ymm3_to_ymm4() {
    let mut emu = emu64();
    // VMOVAPS YMM4, YMM3
    let code = [
        0xc5, 0xfc, 0x28, 0xe3, // VMOVAPS YMM4, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovaps_ymm4_to_ymm5() {
    let mut emu = emu64();
    // VMOVAPS YMM5, YMM4
    let code = [
        0xc5, 0xfc, 0x28, 0xec, // VMOVAPS YMM5, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovaps_ymm5_to_ymm6() {
    let mut emu = emu64();
    // VMOVAPS YMM6, YMM5
    let code = [
        0xc5, 0xfc, 0x28, 0xf5, // VMOVAPS YMM6, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovaps_ymm6_to_ymm7() {
    let mut emu = emu64();
    // VMOVAPS YMM7, YMM6
    let code = [
        0xc5, 0xfc, 0x28, 0xfe, // VMOVAPS YMM7, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovaps_ymm7_to_ymm0() {
    let mut emu = emu64();
    // VMOVAPS YMM0, YMM7
    let code = [
        0xc5, 0xfc, 0x28, 0xc7, // VMOVAPS YMM0, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovaps_ymm8_to_ymm9() {
    let mut emu = emu64();
    // VMOVAPS YMM9, YMM8 (requires EVEX prefix with REX.R)
    let code = [
        0xc4, 0xc1, 0x7c, 0x28, 0xc8, // VMOVAPS YMM9, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovaps_ymm10_to_ymm11() {
    let mut emu = emu64();
    // VMOVAPS YMM11, YMM10
    let code = [
        0xc4, 0xc1, 0x7c, 0x28, 0xd2, // VMOVAPS YMM11, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovaps_ymm12_to_ymm13() {
    let mut emu = emu64();
    // VMOVAPS YMM13, YMM12
    let code = [
        0xc4, 0xc1, 0x7c, 0x28, 0xec, // VMOVAPS YMM13, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovaps_ymm14_to_ymm15() {
    let mut emu = emu64();
    // VMOVAPS YMM15, YMM14
    let code = [
        0xc4, 0xc1, 0x7c, 0x28, 0xfe, // VMOVAPS YMM15, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovaps_ymm0_to_ymm15() {
    let mut emu = emu64();
    // VMOVAPS YMM15, YMM0
    let code = [
        0xc4, 0xc1, 0x7c, 0x28, 0xf8, // VMOVAPS YMM15, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovaps_ymm15_to_ymm0() {
    let mut emu = emu64();
    // VMOVAPS YMM0, YMM15
    let code = [
        0xc4, 0xc1, 0x7c, 0x28, 0xc7, // VMOVAPS YMM0, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMOVAPS Memory to Register Tests (Aligned)
// ============================================================================

#[test]
fn test_vmovaps_mem_to_ymm0_aligned() {
    let mut emu = emu64();
    // VMOVAPS YMM0, [aligned_addr]
    let code = [
        0xc5, 0xfc, 0x28, 0x05, 0xf7, 0x1f, 0x00, 0x00, // VMOVAPS YMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x00, 0x3f, // 0.5 as float32
        0x00, 0x00, 0x00, 0x40, // 2.0 as float32
        0x00, 0x00, 0x00, 0x40, // 2.0 as float32
        0x00, 0x00, 0x80, 0x3f, // 1.0 as float32
        0x00, 0x00, 0x00, 0x41, // 8.0 as float32
        0x00, 0x00, 0x00, 0x41, // 8.0 as float32
        0x00, 0x00, 0x80, 0x40, // 4.0 as float32
        0x00, 0x00, 0x00, 0xbf, // -0.5 as float32
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmovaps_mem_to_ymm1_aligned() {
    let mut emu = emu64();
    // VMOVAPS YMM1, [aligned_addr]
    let code = [
        0xc5, 0xfc, 0x28, 0x0d, 0xf7, 0x1f, 0x00, 0x00, // VMOVAPS YMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x00, 0x40,
        0x00, 0x00, 0x40, 0x40, 0x00, 0x00, 0x80, 0x40,
        0x00, 0x00, 0xa0, 0x40, 0x00, 0x00, 0xc0, 0x40,
        0x00, 0x00, 0xe0, 0x40, 0x00, 0x00, 0x00, 0x41,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmovaps_mem_to_ymm8_aligned() {
    let mut emu = emu64();
    // VMOVAPS YMM8, [aligned_addr]
    let code = [
        0xc4, 0xc1, 0x7c, 0x28, 0x05, 0xf6, 0x1f, 0x00, 0x00, // VMOVAPS YMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VMOVAPS Register to Memory Tests (Aligned)
// ============================================================================

#[test]
fn test_vmovaps_ymm0_to_mem_aligned() {
    let mut emu = emu64();
    // VMOVAPS [aligned_addr], YMM0
    let code = [
        0xc5, 0xfc, 0x29, 0x05, 0xf7, 0x1f, 0x00, 0x00, // VMOVAPS [rip + 0x4000], YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovaps_ymm1_to_mem_aligned() {
    let mut emu = emu64();
    // VMOVAPS [aligned_addr], YMM1
    let code = [
        0xc5, 0xfc, 0x29, 0x0d, 0xf7, 0x1f, 0x00, 0x00, // VMOVAPS [rip + 0x4000], YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovaps_ymm15_to_mem_aligned() {
    let mut emu = emu64();
    // VMOVAPS [aligned_addr], YMM15
    let code = [
        0xc4, 0xc1, 0x7c, 0x29, 0x3d, 0xf6, 0x1f, 0x00, 0x00, // VMOVAPS [rip + 0x4000], YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMOVAPD Tests - Packed Double Precision (4x float64 - 256-bit)
// ============================================================================

#[test]
fn test_vmovapd_ymm0_to_ymm1() {
    let mut emu = emu64();
    // VMOVAPD YMM1, YMM0
    let code = [
        0xc5, 0xfd, 0x28, 0xc8, // VMOVAPD YMM1, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovapd_ymm1_to_ymm2() {
    let mut emu = emu64();
    // VMOVAPD YMM2, YMM1
    let code = [
        0xc5, 0xfd, 0x28, 0xd1, // VMOVAPD YMM2, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovapd_ymm2_to_ymm3() {
    let mut emu = emu64();
    // VMOVAPD YMM3, YMM2
    let code = [
        0xc5, 0xfd, 0x28, 0xda, // VMOVAPD YMM3, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovapd_ymm3_to_ymm4() {
    let mut emu = emu64();
    // VMOVAPD YMM4, YMM3
    let code = [
        0xc5, 0xfd, 0x28, 0xe3, // VMOVAPD YMM4, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovapd_ymm4_to_ymm5() {
    let mut emu = emu64();
    // VMOVAPD YMM5, YMM4
    let code = [
        0xc5, 0xfd, 0x28, 0xec, // VMOVAPD YMM5, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovapd_ymm5_to_ymm6() {
    let mut emu = emu64();
    // VMOVAPD YMM6, YMM5
    let code = [
        0xc5, 0xfd, 0x28, 0xf5, // VMOVAPD YMM6, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovapd_ymm6_to_ymm7() {
    let mut emu = emu64();
    // VMOVAPD YMM7, YMM6
    let code = [
        0xc5, 0xfd, 0x28, 0xfe, // VMOVAPD YMM7, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovapd_ymm7_to_ymm0() {
    let mut emu = emu64();
    // VMOVAPD YMM0, YMM7
    let code = [
        0xc5, 0xfd, 0x28, 0xc7, // VMOVAPD YMM0, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovapd_ymm8_to_ymm9() {
    let mut emu = emu64();
    // VMOVAPD YMM9, YMM8
    let code = [
        0xc4, 0xc1, 0xfd, 0x28, 0xc8, // VMOVAPD YMM9, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovapd_ymm12_to_ymm13() {
    let mut emu = emu64();
    // VMOVAPD YMM13, YMM12
    let code = [
        0xc4, 0xc1, 0xfd, 0x28, 0xec, // VMOVAPD YMM13, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovapd_ymm14_to_ymm15() {
    let mut emu = emu64();
    // VMOVAPD YMM15, YMM14
    let code = [
        0xc4, 0xc1, 0xfd, 0x28, 0xfe, // VMOVAPD YMM15, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMOVAPD Memory to Register Tests (Aligned)
// ============================================================================

#[test]
fn test_vmovapd_mem_to_ymm0_aligned() {
    let mut emu = emu64();
    // VMOVAPD YMM0, [aligned_addr]
    let code = [
        0xc5, 0xfd, 0x28, 0x05, 0xf7, 0x1f, 0x00, 0x00, // VMOVAPD YMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe0, 0x3f, // 0.5
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // 2.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40, // 3.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x40, // 4.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmovapd_mem_to_ymm8_aligned() {
    let mut emu = emu64();
    // VMOVAPD YMM8, [aligned_addr]
    let code = [
        0xc4, 0xc1, 0xfd, 0x28, 0x05, 0xf6, 0x1f, 0x00, 0x00, // VMOVAPD YMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VMOVAPD Register to Memory Tests (Aligned)
// ============================================================================

#[test]
fn test_vmovapd_ymm0_to_mem_aligned() {
    let mut emu = emu64();
    // VMOVAPD [aligned_addr], YMM0
    let code = [
        0xc5, 0xfd, 0x29, 0x05, 0xf7, 0x1f, 0x00, 0x00, // VMOVAPD [rip + 0x4000], YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovapd_ymm8_to_mem_aligned() {
    let mut emu = emu64();
    // VMOVAPD [aligned_addr], YMM8
    let code = [
        0xc4, 0xc1, 0xfd, 0x29, 0x05, 0xf6, 0x1f, 0x00, 0x00, // VMOVAPD [rip + 0x4000], YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovapd_ymm15_to_mem_aligned() {
    let mut emu = emu64();
    // VMOVAPD [aligned_addr], YMM15
    let code = [
        0xc4, 0xc1, 0xfd, 0x29, 0x3d, 0xf6, 0x1f, 0x00, 0x00, // VMOVAPD [rip + 0x4000], YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Cross-register VMOVAPS Tests
// ============================================================================

#[test]
fn test_vmovaps_ymm0_to_ymm8() {
    let mut emu = emu64();
    // VMOVAPS YMM8, YMM0 (cross register domains)
    let code = [
        0xc4, 0xc1, 0x7c, 0x28, 0xc0, // VMOVAPS YMM8, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovaps_ymm8_to_ymm0() {
    let mut emu = emu64();
    // VMOVAPS YMM0, YMM8
    let code = [
        0xc4, 0xc1, 0x7c, 0x28, 0xc0, // VMOVAPS YMM0, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Cross-register VMOVAPD Tests
// ============================================================================

#[test]
fn test_vmovapd_ymm0_to_ymm8() {
    let mut emu = emu64();
    // VMOVAPD YMM8, YMM0
    let code = [
        0xc4, 0xc1, 0xfd, 0x28, 0xc0, // VMOVAPD YMM8, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovapd_ymm8_to_ymm0() {
    let mut emu = emu64();
    // VMOVAPD YMM0, YMM8
    let code = [
        0xc4, 0xc1, 0xfd, 0x28, 0xc0, // VMOVAPD YMM0, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
