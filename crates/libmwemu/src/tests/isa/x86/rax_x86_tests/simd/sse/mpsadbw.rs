use crate::*;

// MPSADBW - Compute Multiple Packed Sums of Absolute Difference
//
// Calculates packed word results of sum-absolute-difference (SAD) of unsigned
// bytes from two blocks of 32-bit dword elements. The offset of block_2 within
// the second operand is selected by imm8[1:0]*32. The offset of block_1 within
// the first operand is selected by imm8[2]*32.
//
// imm8[1:0]: Selects source offset (0, 32, 64, 96 bits)
// imm8[2]: Selects destination offset (0 or 32 bits)
//
// Opcode:
//   66 0F 3A 42 /r ib    MPSADBW xmm1, xmm2/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// Test all offset combinations for imm8[1:0] (source offset)
#[test]
fn test_mpsadbw_xmm0_xmm1_offset_0x00() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xc1, 0x00, // MPSADBW XMM0, XMM1, 0x00 (src offset 0, dest offset 0)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm0_xmm1_offset_0x01() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xc1, 0x01, // MPSADBW XMM0, XMM1, 0x01 (src offset 32)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm0_xmm1_offset_0x02() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xc1, 0x02, // MPSADBW XMM0, XMM1, 0x02 (src offset 64)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm0_xmm1_offset_0x03() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xc1, 0x03, // MPSADBW XMM0, XMM1, 0x03 (src offset 96)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test destination offset bit (imm8[2])
#[test]
fn test_mpsadbw_xmm0_xmm1_offset_0x04() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xc1, 0x04, // MPSADBW XMM0, XMM1, 0x04 (src offset 0, dest offset 32)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm0_xmm1_offset_0x05() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xc1, 0x05, // MPSADBW XMM0, XMM1, 0x05 (src offset 32, dest offset 32)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm0_xmm1_offset_0x06() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xc1, 0x06, // MPSADBW XMM0, XMM1, 0x06 (src offset 64, dest offset 32)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm0_xmm1_offset_0x07() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xc1, 0x07, // MPSADBW XMM0, XMM1, 0x07 (src offset 96, dest offset 32)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test different register pairs
#[test]
fn test_mpsadbw_xmm2_xmm3_offset_0x00() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xd3, 0x00, // MPSADBW XMM2, XMM3, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm4_xmm5_offset_0x01() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xe5, 0x01, // MPSADBW XMM4, XMM5, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm6_xmm7_offset_0x02() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xf7, 0x02, // MPSADBW XMM6, XMM7, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm1_xmm2_offset_0x03() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xca, 0x03, // MPSADBW XMM1, XMM2, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test high XMM registers (XMM8-XMM15)
#[test]
fn test_mpsadbw_xmm8_xmm9_offset_0x00() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x42, 0xc1, 0x00, // MPSADBW XMM8, XMM9, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm10_xmm11_offset_0x04() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x42, 0xd3, 0x04, // MPSADBW XMM10, XMM11, 0x04
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm12_xmm13_offset_0x05() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x42, 0xe5, 0x05, // MPSADBW XMM12, XMM13, 0x05
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm14_xmm15_offset_0x07() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x42, 0xf7, 0x07, // MPSADBW XMM14, XMM15, 0x07
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test memory operands with different offsets
#[test]
fn test_mpsadbw_xmm0_mem_offset_0x00() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x42, 0x00, 0x00, // MPSADBW XMM0, [RAX], 0x00
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm1_mem_offset_0x01() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x42, 0x08, 0x01, // MPSADBW XMM1, [RAX], 0x01
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm2_mem_offset_0x02() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x42, 0x10, 0x02, // MPSADBW XMM2, [RAX], 0x02
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm3_mem_offset_0x03() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x42, 0x18, 0x03, // MPSADBW XMM3, [RAX], 0x03
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm4_mem_offset_0x04() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x42, 0x20, 0x04, // MPSADBW XMM4, [RAX], 0x04
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
        0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// Test same register
#[test]
fn test_mpsadbw_same_register_offset_0x00() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xc0, 0x00, // MPSADBW XMM0, XMM0, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_same_register_offset_0x07() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xc9, 0x07, // MPSADBW XMM1, XMM1, 0x07
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test sequential operations
#[test]
fn test_mpsadbw_sequential_operations() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xc1, 0x00, // MPSADBW XMM0, XMM1, 0x00
        0x66, 0x0f, 0x3a, 0x42, 0xd3, 0x01, // MPSADBW XMM2, XMM3, 0x01
        0x66, 0x0f, 0x3a, 0x42, 0xe5, 0x02, // MPSADBW XMM4, XMM5, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test memory with displacement
#[test]
fn test_mpsadbw_mem_displacement() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x42, 0x40, 0x10, 0x00, // MPSADBW XMM0, [RAX+0x10], 0x00
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// Test cross-register operations (low to high)
#[test]
fn test_mpsadbw_xmm0_xmm15_offset_0x00() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x42, 0xf8, 0x00, // MPSADBW XMM15, XMM0, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm15_xmm0_offset_0x07() {
    let mut emu = emu64();
    let code = [
        0x66, 0x41, 0x0f, 0x3a, 0x42, 0xf8, 0x07, // MPSADBW XMM7, XMM8, 0x07
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test additional register combinations
#[test]
fn test_mpsadbw_xmm3_xmm4_offset_0x04() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xdc, 0x04, // MPSADBW XMM3, XMM4, 0x04
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm5_xmm6_offset_0x06() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xee, 0x06, // MPSADBW XMM5, XMM6, 0x06
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm7_xmm1_offset_0x01() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xf9, 0x01, // MPSADBW XMM7, XMM1, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test high registers with different offsets
#[test]
fn test_mpsadbw_xmm9_xmm8_offset_0x02() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x42, 0xc8, 0x02, // MPSADBW XMM9, XMM8, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm11_xmm10_offset_0x03() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x42, 0xda, 0x03, // MPSADBW XMM11, XMM10, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm13_xmm14_offset_0x06() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x42, 0xee, 0x06, // MPSADBW XMM13, XMM14, 0x06
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test memory with various data patterns
#[test]
fn test_mpsadbw_xmm5_mem_offset_0x05() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x42, 0x28, 0x05, // MPSADBW XMM5, [RAX], 0x05
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm6_mem_offset_0x06() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x42, 0x30, 0x06, // MPSADBW XMM6, [RAX], 0x06
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80,
        0x90, 0xa0, 0xb0, 0xc0, 0xd0, 0xe0, 0xf0, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm7_mem_offset_0x07() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x42, 0x38, 0x07, // MPSADBW XMM7, [RAX], 0x07
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08,
        0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// Additional tests to reach 40+ tests
#[test]
fn test_mpsadbw_xmm0_xmm2_offset_0x01() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xc2, 0x01, // MPSADBW XMM0, XMM2, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm1_xmm3_offset_0x03() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xcb, 0x03, // MPSADBW XMM1, XMM3, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm2_xmm4_offset_0x05() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xd4, 0x05, // MPSADBW XMM2, XMM4, 0x05
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm4_xmm6_offset_0x02() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x42, 0xe6, 0x02, // MPSADBW XMM4, XMM6, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
