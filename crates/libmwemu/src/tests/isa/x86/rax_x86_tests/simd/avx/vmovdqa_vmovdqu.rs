use crate::*;

// VMOVDQA - Move Aligned Integer Values (256-bit)
// VMOVDQU - Move Unaligned Integer Values (256-bit)
//
// VMOVDQA moves 256 bits of integer data from source to destination.
// VMOVDQU moves 256 bits of integer data from source to destination (unaligned).
// These work with 8, 16, 32, or 64-bit integer elements.
//
// Opcodes:
// VEX.256.66 0F 6F /r    VMOVDQA ymm1, ymm2/m256   - Move aligned integer from ymm2/mem to ymm1
// VEX.256.66 0F 7F /r    VMOVDQA ymm2/m256, ymm1   - Move aligned integer from ymm1 to ymm2/mem
// VEX.256.F3 0F 6F /r    VMOVDQU ymm1, ymm2/m256   - Move unaligned integer from ymm2/mem to ymm1
// VEX.256.F3 0F 7F /r    VMOVDQU ymm2/m256, ymm1   - Move unaligned integer from ymm1 to ymm2/mem

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing
const UNALIGNED_ADDR: u64 = 0x3001; // Unaligned address for testing

// ============================================================================
// VMOVDQA Tests - Aligned Integer (256-bit)
// ============================================================================

#[test]
fn test_vmovdqa_ymm0_to_ymm1() {
    let mut emu = emu64();
    // VMOVDQA YMM1, YMM0
    let code = [
        0xc5, 0xfd, 0x6f, 0xc8, // VMOVDQA YMM1, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqa_ymm1_to_ymm2() {
    let mut emu = emu64();
    // VMOVDQA YMM2, YMM1
    let code = [
        0xc5, 0xfd, 0x6f, 0xd1, // VMOVDQA YMM2, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqa_ymm2_to_ymm3() {
    let mut emu = emu64();
    // VMOVDQA YMM3, YMM2
    let code = [
        0xc5, 0xfd, 0x6f, 0xda, // VMOVDQA YMM3, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqa_ymm3_to_ymm4() {
    let mut emu = emu64();
    // VMOVDQA YMM4, YMM3
    let code = [
        0xc5, 0xfd, 0x6f, 0xe3, // VMOVDQA YMM4, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqa_ymm4_to_ymm5() {
    let mut emu = emu64();
    // VMOVDQA YMM5, YMM4
    let code = [
        0xc5, 0xfd, 0x6f, 0xec, // VMOVDQA YMM5, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqa_ymm5_to_ymm6() {
    let mut emu = emu64();
    // VMOVDQA YMM6, YMM5
    let code = [
        0xc5, 0xfd, 0x6f, 0xf5, // VMOVDQA YMM6, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqa_ymm6_to_ymm7() {
    let mut emu = emu64();
    // VMOVDQA YMM7, YMM6
    let code = [
        0xc5, 0xfd, 0x6f, 0xfe, // VMOVDQA YMM7, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqa_ymm7_to_ymm0() {
    let mut emu = emu64();
    // VMOVDQA YMM0, YMM7
    let code = [
        0xc5, 0xfd, 0x6f, 0xc7, // VMOVDQA YMM0, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqa_ymm8_to_ymm9() {
    let mut emu = emu64();
    // VMOVDQA YMM9, YMM8
    let code = [
        0xc4, 0xc1, 0xfd, 0x6f, 0xc8, // VMOVDQA YMM9, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqa_ymm12_to_ymm13() {
    let mut emu = emu64();
    // VMOVDQA YMM13, YMM12
    let code = [
        0xc4, 0xc1, 0xfd, 0x6f, 0xec, // VMOVDQA YMM13, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqa_ymm14_to_ymm15() {
    let mut emu = emu64();
    // VMOVDQA YMM15, YMM14
    let code = [
        0xc4, 0xc1, 0xfd, 0x6f, 0xfe, // VMOVDQA YMM15, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMOVDQA Memory to Register Tests (Aligned)
// ============================================================================

#[test]
fn test_vmovdqa_mem_to_ymm0_aligned() {
    let mut emu = emu64();
    // VMOVDQA YMM0, [aligned_addr]
    let code = [
        0xc5, 0xfd, 0x6f, 0x05, 0xf7, 0x1f, 0x00, 0x00, // VMOVDQA YMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqa_mem_to_ymm1_aligned() {
    let mut emu = emu64();
    // VMOVDQA YMM1, [aligned_addr]
    let code = [
        0xc5, 0xfd, 0x6f, 0x0d, 0xf7, 0x1f, 0x00, 0x00, // VMOVDQA YMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqa_mem_to_ymm8_aligned() {
    let mut emu = emu64();
    // VMOVDQA YMM8, [aligned_addr]
    let code = [
        0xc4, 0xc1, 0xfd, 0x6f, 0x05, 0xf6, 0x1f, 0x00, 0x00, // VMOVDQA YMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VMOVDQA Register to Memory Tests (Aligned)
// ============================================================================

#[test]
fn test_vmovdqa_ymm0_to_mem_aligned() {
    let mut emu = emu64();
    // VMOVDQA [aligned_addr], YMM0
    let code = [
        0xc5, 0xfd, 0x7f, 0x05, 0xf7, 0x1f, 0x00, 0x00, // VMOVDQA [rip + 0x4000], YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqa_ymm1_to_mem_aligned() {
    let mut emu = emu64();
    // VMOVDQA [aligned_addr], YMM1
    let code = [
        0xc5, 0xfd, 0x7f, 0x0d, 0xf7, 0x1f, 0x00, 0x00, // VMOVDQA [rip + 0x4000], YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqa_ymm8_to_mem_aligned() {
    let mut emu = emu64();
    // VMOVDQA [aligned_addr], YMM8
    let code = [
        0xc4, 0xc1, 0xfd, 0x7f, 0x05, 0xf6, 0x1f, 0x00, 0x00, // VMOVDQA [rip + 0x4000], YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqa_ymm15_to_mem_aligned() {
    let mut emu = emu64();
    // VMOVDQA [aligned_addr], YMM15
    let code = [
        0xc4, 0xc1, 0xfd, 0x7f, 0x3d, 0xf6, 0x1f, 0x00, 0x00, // VMOVDQA [rip + 0x4000], YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMOVDQU Tests - Unaligned Integer (256-bit)
// ============================================================================

#[test]
fn test_vmovdqu_ymm0_to_ymm1() {
    let mut emu = emu64();
    // VMOVDQU YMM1, YMM0
    let code = [
        0xc5, 0xfe, 0x6f, 0xc8, // VMOVDQU YMM1, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqu_ymm1_to_ymm2() {
    let mut emu = emu64();
    // VMOVDQU YMM2, YMM1
    let code = [
        0xc5, 0xfe, 0x6f, 0xd1, // VMOVDQU YMM2, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqu_ymm2_to_ymm3() {
    let mut emu = emu64();
    // VMOVDQU YMM3, YMM2
    let code = [
        0xc5, 0xfe, 0x6f, 0xda, // VMOVDQU YMM3, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqu_ymm3_to_ymm4() {
    let mut emu = emu64();
    // VMOVDQU YMM4, YMM3
    let code = [
        0xc5, 0xfe, 0x6f, 0xe3, // VMOVDQU YMM4, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqu_ymm4_to_ymm5() {
    let mut emu = emu64();
    // VMOVDQU YMM5, YMM4
    let code = [
        0xc5, 0xfe, 0x6f, 0xec, // VMOVDQU YMM5, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqu_ymm5_to_ymm6() {
    let mut emu = emu64();
    // VMOVDQU YMM6, YMM5
    let code = [
        0xc5, 0xfe, 0x6f, 0xf5, // VMOVDQU YMM6, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqu_ymm6_to_ymm7() {
    let mut emu = emu64();
    // VMOVDQU YMM7, YMM6
    let code = [
        0xc5, 0xfe, 0x6f, 0xfe, // VMOVDQU YMM7, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqu_ymm7_to_ymm0() {
    let mut emu = emu64();
    // VMOVDQU YMM0, YMM7
    let code = [
        0xc5, 0xfe, 0x6f, 0xc7, // VMOVDQU YMM0, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqu_ymm8_to_ymm9() {
    let mut emu = emu64();
    // VMOVDQU YMM9, YMM8
    let code = [
        0xc4, 0xc1, 0xfe, 0x6f, 0xc8, // VMOVDQU YMM9, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqu_ymm12_to_ymm13() {
    let mut emu = emu64();
    // VMOVDQU YMM13, YMM12
    let code = [
        0xc4, 0xc1, 0xfe, 0x6f, 0xec, // VMOVDQU YMM13, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqu_ymm14_to_ymm15() {
    let mut emu = emu64();
    // VMOVDQU YMM15, YMM14
    let code = [
        0xc4, 0xc1, 0xfe, 0x6f, 0xfe, // VMOVDQU YMM15, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMOVDQU Memory to Register Tests (Unaligned)
// ============================================================================

#[test]
fn test_vmovdqu_mem_to_ymm0_unaligned() {
    let mut emu = emu64();
    // VMOVDQU YMM0, [unaligned_addr]
    let code = [
        0xc5, 0xfe, 0x6f, 0x05, 0x01, 0x40, 0x00, 0x00, // VMOVDQU YMM0, [rip + 0x4001]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22,
        0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44,
        0x55, 0x55, 0x55, 0x55, 0x66, 0x66, 0x66, 0x66,
        0x77, 0x77, 0x77, 0x77, 0x88, 0x88, 0x88, 0x88,
    ];
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqu_mem_to_ymm1_unaligned() {
    let mut emu = emu64();
    // VMOVDQU YMM1, [unaligned_addr]
    let code = [
        0xc5, 0xfe, 0x6f, 0x0d, 0x01, 0x40, 0x00, 0x00, // VMOVDQU YMM1, [rip + 0x4001]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc];
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqu_mem_to_ymm8_unaligned() {
    let mut emu = emu64();
    // VMOVDQU YMM8, [unaligned_addr]
    let code = [
        0xc4, 0xc1, 0xfe, 0x6f, 0x05, 0x01, 0x40, 0x00, 0x00, // VMOVDQU YMM8, [rip + 0x4001]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd];
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqu_mem_to_ymm15_unaligned() {
    let mut emu = emu64();
    // VMOVDQU YMM15, [unaligned_addr]
    let code = [
        0xc4, 0xc1, 0xfe, 0x6f, 0x3d, 0x01, 0x40, 0x00, 0x00, // VMOVDQU YMM15, [rip + 0x4001]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee];
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VMOVDQU Register to Memory Tests (Unaligned)
// ============================================================================

#[test]
fn test_vmovdqu_ymm0_to_mem_unaligned() {
    let mut emu = emu64();
    // VMOVDQU [unaligned_addr], YMM0
    let code = [
        0xc5, 0xfe, 0x7f, 0x05, 0x01, 0x40, 0x00, 0x00, // VMOVDQU [rip + 0x4001], YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqu_ymm1_to_mem_unaligned() {
    let mut emu = emu64();
    // VMOVDQU [unaligned_addr], YMM1
    let code = [
        0xc5, 0xfe, 0x7f, 0x0d, 0x01, 0x40, 0x00, 0x00, // VMOVDQU [rip + 0x4001], YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqu_ymm8_to_mem_unaligned() {
    let mut emu = emu64();
    // VMOVDQU [unaligned_addr], YMM8
    let code = [
        0xc4, 0xc1, 0xfe, 0x7f, 0x05, 0x01, 0x40, 0x00, 0x00, // VMOVDQU [rip + 0x4001], YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqu_ymm15_to_mem_unaligned() {
    let mut emu = emu64();
    // VMOVDQU [unaligned_addr], YMM15
    let code = [
        0xc4, 0xc1, 0xfe, 0x7f, 0x3d, 0x01, 0x40, 0x00, 0x00, // VMOVDQU [rip + 0x4001], YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Cross-register VMOVDQA Tests
// ============================================================================

#[test]
fn test_vmovdqa_ymm0_to_ymm8() {
    let mut emu = emu64();
    // VMOVDQA YMM8, YMM0
    let code = [
        0xc4, 0xc1, 0xfd, 0x6f, 0xc0, // VMOVDQA YMM8, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqa_ymm8_to_ymm0() {
    let mut emu = emu64();
    // VMOVDQA YMM0, YMM8
    let code = [
        0xc4, 0xc1, 0xfd, 0x6f, 0xc0, // VMOVDQA YMM0, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Cross-register VMOVDQU Tests
// ============================================================================

#[test]
fn test_vmovdqu_ymm0_to_ymm8() {
    let mut emu = emu64();
    // VMOVDQU YMM8, YMM0
    let code = [
        0xc4, 0xc1, 0xfe, 0x6f, 0xc0, // VMOVDQU YMM8, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovdqu_ymm8_to_ymm0() {
    let mut emu = emu64();
    // VMOVDQU YMM0, YMM8
    let code = [
        0xc4, 0xc1, 0xfe, 0x6f, 0xc0, // VMOVDQU YMM0, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
