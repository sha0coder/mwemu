use crate::*;

// VMOVUPS - Move Unaligned Packed Single Precision Floating-Point Values (256-bit)
// VMOVUPD - Move Unaligned Packed Double Precision Floating-Point Values (256-bit)
//
// VMOVUPS moves 256 bits (8 single-precision floating-point values) from source to destination.
// VMOVUPD moves 256 bits (4 double-precision floating-point values) from source to destination.
// Unlike VMOVAPS/VMOVAPD, these instructions work with unaligned memory addresses.
//
// Opcodes:
// VEX.256.NP 0F 10 /r    VMOVUPS ymm1, ymm2/m256   - Move unaligned packed single from ymm2/mem to ymm1
// VEX.256.NP 0F 11 /r    VMOVUPS ymm2/m256, ymm1   - Move unaligned packed single from ymm1 to ymm2/mem
// VEX.256.66 0F 10 /r    VMOVUPD ymm1, ymm2/m256   - Move unaligned packed double from ymm2/mem to ymm1
// VEX.256.66 0F 11 /r    VMOVUPD ymm2/m256, ymm1   - Move unaligned packed double from ymm1 to ymm2/mem

const UNALIGNED_ADDR: u64 = 0x3001; // Unaligned address for testing

// ============================================================================
// VMOVUPS Tests - Packed Single Precision (8x float32 - 256-bit)
// ============================================================================

#[test]
fn test_vmovups_ymm0_to_ymm1() {
    let mut emu = emu64();
    // VMOVUPS YMM1, YMM0
    let code = [
        0xc5, 0xfc, 0x10, 0xc8, // VMOVUPS YMM1, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_ymm1_to_ymm2() {
    let mut emu = emu64();
    // VMOVUPS YMM2, YMM1
    let code = [
        0xc5, 0xfc, 0x10, 0xd1, // VMOVUPS YMM2, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_ymm2_to_ymm3() {
    let mut emu = emu64();
    // VMOVUPS YMM3, YMM2
    let code = [
        0xc5, 0xfc, 0x10, 0xda, // VMOVUPS YMM3, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_ymm3_to_ymm4() {
    let mut emu = emu64();
    // VMOVUPS YMM4, YMM3
    let code = [
        0xc5, 0xfc, 0x10, 0xe3, // VMOVUPS YMM4, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_ymm4_to_ymm5() {
    let mut emu = emu64();
    // VMOVUPS YMM5, YMM4
    let code = [
        0xc5, 0xfc, 0x10, 0xec, // VMOVUPS YMM5, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_ymm5_to_ymm6() {
    let mut emu = emu64();
    // VMOVUPS YMM6, YMM5
    let code = [
        0xc5, 0xfc, 0x10, 0xf5, // VMOVUPS YMM6, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_ymm6_to_ymm7() {
    let mut emu = emu64();
    // VMOVUPS YMM7, YMM6
    let code = [
        0xc5, 0xfc, 0x10, 0xfe, // VMOVUPS YMM7, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_ymm7_to_ymm0() {
    let mut emu = emu64();
    // VMOVUPS YMM0, YMM7
    let code = [
        0xc5, 0xfc, 0x10, 0xc7, // VMOVUPS YMM0, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_ymm8_to_ymm9() {
    let mut emu = emu64();
    // VMOVUPS YMM9, YMM8
    let code = [
        0xc4, 0xc1, 0x7c, 0x10, 0xc8, // VMOVUPS YMM9, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_ymm10_to_ymm11() {
    let mut emu = emu64();
    // VMOVUPS YMM11, YMM10
    let code = [
        0xc4, 0xc1, 0x7c, 0x10, 0xd2, // VMOVUPS YMM11, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_ymm12_to_ymm13() {
    let mut emu = emu64();
    // VMOVUPS YMM13, YMM12
    let code = [
        0xc4, 0xc1, 0x7c, 0x10, 0xec, // VMOVUPS YMM13, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_ymm14_to_ymm15() {
    let mut emu = emu64();
    // VMOVUPS YMM15, YMM14
    let code = [
        0xc4, 0xc1, 0x7c, 0x10, 0xfe, // VMOVUPS YMM15, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMOVUPS Memory to Register Tests (Unaligned)
// ============================================================================

#[test]
fn test_vmovups_mem_to_ymm0_unaligned() {
    let mut emu = emu64();
    // VMOVUPS YMM0, [unaligned_addr]
    let code = [
        0xc5, 0xfc, 0x10, 0x05, 0x01, 0x40, 0x00, 0x00, // VMOVUPS YMM0, [rip + 0x4001]
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
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_mem_to_ymm1_unaligned() {
    let mut emu = emu64();
    // VMOVUPS YMM1, [unaligned_addr]
    let code = [
        0xc5, 0xfc, 0x10, 0x0d, 0x01, 0x40, 0x00, 0x00, // VMOVUPS YMM1, [rip + 0x4001]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x00, 0x40,
        0x00, 0x00, 0x40, 0x40, 0x00, 0x00, 0x80, 0x40,
        0x00, 0x00, 0xa0, 0x40, 0x00, 0x00, 0xc0, 0x40,
        0x00, 0x00, 0xe0, 0x40, 0x00, 0x00, 0x00, 0x41,
    ];
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_mem_to_ymm8_unaligned() {
    let mut emu = emu64();
    // VMOVUPS YMM8, [unaligned_addr]
    let code = [
        0xc4, 0xc1, 0x7c, 0x10, 0x05, 0x01, 0x40, 0x00, 0x00, // VMOVUPS YMM8, [rip + 0x4001]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc];
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_mem_to_ymm15_unaligned() {
    let mut emu = emu64();
    // VMOVUPS YMM15, [unaligned_addr]
    let code = [
        0xc4, 0xc1, 0x7c, 0x10, 0x3d, 0x01, 0x40, 0x00, 0x00, // VMOVUPS YMM15, [rip + 0x4001]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd];
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VMOVUPS Register to Memory Tests (Unaligned)
// ============================================================================

#[test]
fn test_vmovups_ymm0_to_mem_unaligned() {
    let mut emu = emu64();
    // VMOVUPS [unaligned_addr], YMM0
    let code = [
        0xc5, 0xfc, 0x11, 0x05, 0x01, 0x40, 0x00, 0x00, // VMOVUPS [rip + 0x4001], YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_ymm1_to_mem_unaligned() {
    let mut emu = emu64();
    // VMOVUPS [unaligned_addr], YMM1
    let code = [
        0xc5, 0xfc, 0x11, 0x0d, 0x01, 0x40, 0x00, 0x00, // VMOVUPS [rip + 0x4001], YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_ymm8_to_mem_unaligned() {
    let mut emu = emu64();
    // VMOVUPS [unaligned_addr], YMM8
    let code = [
        0xc4, 0xc1, 0x7c, 0x11, 0x05, 0x01, 0x40, 0x00, 0x00, // VMOVUPS [rip + 0x4001], YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_ymm15_to_mem_unaligned() {
    let mut emu = emu64();
    // VMOVUPS [unaligned_addr], YMM15
    let code = [
        0xc4, 0xc1, 0x7c, 0x11, 0x3d, 0x01, 0x40, 0x00, 0x00, // VMOVUPS [rip + 0x4001], YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMOVUPD Tests - Packed Double Precision (4x float64 - 256-bit)
// ============================================================================

#[test]
fn test_vmovupd_ymm0_to_ymm1() {
    let mut emu = emu64();
    // VMOVUPD YMM1, YMM0
    let code = [
        0xc5, 0xfd, 0x10, 0xc8, // VMOVUPD YMM1, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovupd_ymm1_to_ymm2() {
    let mut emu = emu64();
    // VMOVUPD YMM2, YMM1
    let code = [
        0xc5, 0xfd, 0x10, 0xd1, // VMOVUPD YMM2, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovupd_ymm2_to_ymm3() {
    let mut emu = emu64();
    // VMOVUPD YMM3, YMM2
    let code = [
        0xc5, 0xfd, 0x10, 0xda, // VMOVUPD YMM3, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovupd_ymm3_to_ymm4() {
    let mut emu = emu64();
    // VMOVUPD YMM4, YMM3
    let code = [
        0xc5, 0xfd, 0x10, 0xe3, // VMOVUPD YMM4, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovupd_ymm4_to_ymm5() {
    let mut emu = emu64();
    // VMOVUPD YMM5, YMM4
    let code = [
        0xc5, 0xfd, 0x10, 0xec, // VMOVUPD YMM5, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovupd_ymm5_to_ymm6() {
    let mut emu = emu64();
    // VMOVUPD YMM6, YMM5
    let code = [
        0xc5, 0xfd, 0x10, 0xf5, // VMOVUPD YMM6, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovupd_ymm6_to_ymm7() {
    let mut emu = emu64();
    // VMOVUPD YMM7, YMM6
    let code = [
        0xc5, 0xfd, 0x10, 0xfe, // VMOVUPD YMM7, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovupd_ymm7_to_ymm0() {
    let mut emu = emu64();
    // VMOVUPD YMM0, YMM7
    let code = [
        0xc5, 0xfd, 0x10, 0xc7, // VMOVUPD YMM0, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovupd_ymm8_to_ymm9() {
    let mut emu = emu64();
    // VMOVUPD YMM9, YMM8
    let code = [
        0xc4, 0xc1, 0xfd, 0x10, 0xc8, // VMOVUPD YMM9, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovupd_ymm12_to_ymm13() {
    let mut emu = emu64();
    // VMOVUPD YMM13, YMM12
    let code = [
        0xc4, 0xc1, 0xfd, 0x10, 0xec, // VMOVUPD YMM13, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovupd_ymm14_to_ymm15() {
    let mut emu = emu64();
    // VMOVUPD YMM15, YMM14
    let code = [
        0xc4, 0xc1, 0xfd, 0x10, 0xfe, // VMOVUPD YMM15, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMOVUPD Memory to Register Tests (Unaligned)
// ============================================================================

#[test]
fn test_vmovupd_mem_to_ymm0_unaligned() {
    let mut emu = emu64();
    // VMOVUPD YMM0, [unaligned_addr]
    let code = [
        0xc5, 0xfd, 0x10, 0x05, 0x01, 0x40, 0x00, 0x00, // VMOVUPD YMM0, [rip + 0x4001]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe0, 0x3f, // 0.5
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // 2.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40, // 3.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x40, // 4.0
    ];
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmovupd_mem_to_ymm8_unaligned() {
    let mut emu = emu64();
    // VMOVUPD YMM8, [unaligned_addr]
    let code = [
        0xc4, 0xc1, 0xfd, 0x10, 0x05, 0x01, 0x40, 0x00, 0x00, // VMOVUPD YMM8, [rip + 0x4001]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77];
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VMOVUPD Register to Memory Tests (Unaligned)
// ============================================================================

#[test]
fn test_vmovupd_ymm0_to_mem_unaligned() {
    let mut emu = emu64();
    // VMOVUPD [unaligned_addr], YMM0
    let code = [
        0xc5, 0xfd, 0x11, 0x05, 0x01, 0x40, 0x00, 0x00, // VMOVUPD [rip + 0x4001], YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovupd_ymm8_to_mem_unaligned() {
    let mut emu = emu64();
    // VMOVUPD [unaligned_addr], YMM8
    let code = [
        0xc4, 0xc1, 0xfd, 0x11, 0x05, 0x01, 0x40, 0x00, 0x00, // VMOVUPD [rip + 0x4001], YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovupd_ymm15_to_mem_unaligned() {
    let mut emu = emu64();
    // VMOVUPD [unaligned_addr], YMM15
    let code = [
        0xc4, 0xc1, 0xfd, 0x11, 0x3d, 0x01, 0x40, 0x00, 0x00, // VMOVUPD [rip + 0x4001], YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Cross-register VMOVUPS Tests
// ============================================================================

#[test]
fn test_vmovups_ymm0_to_ymm8() {
    let mut emu = emu64();
    // VMOVUPS YMM8, YMM0
    let code = [
        0xc4, 0xc1, 0x7c, 0x10, 0xc0, // VMOVUPS YMM8, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_ymm8_to_ymm0() {
    let mut emu = emu64();
    // VMOVUPS YMM0, YMM8
    let code = [
        0xc4, 0xc1, 0x7c, 0x10, 0xc0, // VMOVUPS YMM0, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Cross-register VMOVUPD Tests
// ============================================================================

#[test]
fn test_vmovupd_ymm0_to_ymm8() {
    let mut emu = emu64();
    // VMOVUPD YMM8, YMM0
    let code = [
        0xc4, 0xc1, 0xfd, 0x10, 0xc0, // VMOVUPD YMM8, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovupd_ymm8_to_ymm0() {
    let mut emu = emu64();
    // VMOVUPD YMM0, YMM8
    let code = [
        0xc4, 0xc1, 0xfd, 0x10, 0xc0, // VMOVUPD YMM0, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
