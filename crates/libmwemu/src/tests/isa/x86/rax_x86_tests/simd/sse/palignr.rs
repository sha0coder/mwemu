use crate::*;

// PALIGNR - Packed Align Right
//
// Concatenates the destination operand and the source operand into an intermediate
// composite, shifts the composite at byte granularity to the right by a constant
// immediate, and extracts the right-aligned result into the destination.
//
// For 128-bit operands, imm8 > 31 produces zero result.
//
// Opcode:
//   66 0F 3A 0F /r ib    PALIGNR xmm1, xmm2/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_palignr_xmm0_xmm1_shift0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc1, 0x00, // PALIGNR XMM0, XMM1, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm0_xmm1_shift1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc1, 0x01, // PALIGNR XMM0, XMM1, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm0_xmm1_shift4() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc1, 0x04, // PALIGNR XMM0, XMM1, 4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm0_xmm1_shift8() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc1, 0x08, // PALIGNR XMM0, XMM1, 8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm0_xmm1_shift12() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc1, 0x0c, // PALIGNR XMM0, XMM1, 12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm0_xmm1_shift15() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc1, 0x0f, // PALIGNR XMM0, XMM1, 15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm0_xmm1_shift16() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc1, 0x10, // PALIGNR XMM0, XMM1, 16
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm0_xmm1_shift24() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc1, 0x18, // PALIGNR XMM0, XMM1, 24
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm0_xmm1_shift31() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc1, 0x1f, // PALIGNR XMM0, XMM1, 31
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm0_xmm1_shift32_zero() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc1, 0x20, // PALIGNR XMM0, XMM1, 32 (produces zero)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm2_xmm3_shift2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xd3, 0x02, // PALIGNR XMM2, XMM3, 2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm4_xmm5_shift6() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xe5, 0x06, // PALIGNR XMM4, XMM5, 6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm6_xmm7_shift10() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xf7, 0x0a, // PALIGNR XMM6, XMM7, 10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm7_xmm0_shift14() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xf8, 0x0e, // PALIGNR XMM7, XMM0, 14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm8_xmm9_shift0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0f, 0xc1, 0x00, // PALIGNR XMM8, XMM9, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm8_xmm9_shift8() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0f, 0xc1, 0x08, // PALIGNR XMM8, XMM9, 8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm10_xmm11_shift3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0f, 0xd3, 0x03, // PALIGNR XMM10, XMM11, 3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm12_xmm13_shift7() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0f, 0xe5, 0x07, // PALIGNR XMM12, XMM13, 7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm14_xmm15_shift16() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0f, 0xf7, 0x10, // PALIGNR XMM14, XMM15, 16
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm0_mem_shift0() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x0f, 0x00, 0x00, // PALIGNR XMM0, [RAX], 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm0_mem_shift1() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x0f, 0x00, 0x01, // PALIGNR XMM0, [RAX], 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
                           0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm1_mem_shift4() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x0f, 0x08, 0x04, // PALIGNR XMM1, [RAX], 4
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
                           0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm2_mem_shift8() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x0f, 0x10, 0x08, // PALIGNR XMM2, [RAX], 8
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm3_mem_shift16() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x0f, 0x18, 0x10, // PALIGNR XMM3, [RAX], 16
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_same_register_shift0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc0, 0x00, // PALIGNR XMM0, XMM0, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_same_register_shift8() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc9, 0x08, // PALIGNR XMM1, XMM1, 8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_sequential_operations() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc1, 0x01, // PALIGNR XMM0, XMM1, 1
        0x66, 0x0f, 0x3a, 0x0f, 0xd3, 0x02, // PALIGNR XMM2, XMM3, 2
        0x66, 0x0f, 0x3a, 0x0f, 0xe5, 0x04, // PALIGNR XMM4, XMM5, 4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm0_xmm1_shift5() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc1, 0x05, // PALIGNR XMM0, XMM1, 5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm0_xmm1_shift9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc1, 0x09, // PALIGNR XMM0, XMM1, 9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm0_xmm1_shift11() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc1, 0x0b, // PALIGNR XMM0, XMM1, 11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm0_xmm1_shift13() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc1, 0x0d, // PALIGNR XMM0, XMM1, 13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm0_xmm1_shift17() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc1, 0x11, // PALIGNR XMM0, XMM1, 17
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm0_xmm1_shift20() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc1, 0x14, // PALIGNR XMM0, XMM1, 20
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm0_xmm1_shift255_zero() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0f, 0xc1, 0xff, // PALIGNR XMM0, XMM1, 255 (produces zero)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm0_xmm15_cross_shift8() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x0f, 0xf8, 0x08, // PALIGNR XMM0, XMM15, 8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_xmm15_xmm0_cross_shift4() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x0f, 0xf8, 0x04, // PALIGNR XMM15, XMM0, 4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_palignr_mem_displacement() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x0f, 0x40, 0x10, 0x08, // PALIGNR XMM0, [RAX+0x10], 8
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55]);
    emu.run(None).unwrap();
}
