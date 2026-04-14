use crate::*;

// PBLENDW - Blend Packed Words
//
// Selects words from the source operand and destination operand using an
// immediate byte control mask. If a bit in the mask is "1", the corresponding
// word is copied from the source; otherwise, it remains unchanged.
//
// Mask bits [7:0] control which of the 8 words are selected.
//
// Opcode:
//   66 0F 3A 0E /r ib    PBLENDW xmm1, xmm2/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_pblendw_xmm0_xmm1_mask_0x00() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xc1, 0x00, // PBLENDW XMM0, XMM1, 0x00 (no blend)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm0_xmm1_mask_0xff() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xc1, 0xff, // PBLENDW XMM0, XMM1, 0xFF (all blend)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm0_xmm1_mask_0x01() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xc1, 0x01, // PBLENDW XMM0, XMM1, 0x01 (blend word 0)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm0_xmm1_mask_0x80() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xc1, 0x80, // PBLENDW XMM0, XMM1, 0x80 (blend word 7)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm0_xmm1_mask_0x0f() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xc1, 0x0f, // PBLENDW XMM0, XMM1, 0x0F (blend low 4 words)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm0_xmm1_mask_0xf0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xc1, 0xf0, // PBLENDW XMM0, XMM1, 0xF0 (blend high 4 words)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm0_xmm1_mask_0xaa() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xc1, 0xaa, // PBLENDW XMM0, XMM1, 0xAA (blend even words)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm0_xmm1_mask_0x55() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xc1, 0x55, // PBLENDW XMM0, XMM1, 0x55 (blend odd words)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm2_xmm3_mask_0x33() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xd3, 0x33, // PBLENDW XMM2, XMM3, 0x33
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm4_xmm5_mask_0xcc() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xe5, 0xcc, // PBLENDW XMM4, XMM5, 0xCC
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm6_xmm7_mask_0x11() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xf7, 0x11, // PBLENDW XMM6, XMM7, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm7_xmm0_mask_0x88() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xf8, 0x88, // PBLENDW XMM7, XMM0, 0x88
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm8_xmm9_mask_0x0f() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0e, 0xc1, 0x0f, // PBLENDW XMM8, XMM9, 0x0F
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm10_xmm11_mask_0xf0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0e, 0xd3, 0xf0, // PBLENDW XMM10, XMM11, 0xF0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm12_xmm13_mask_0xaa() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0e, 0xe5, 0xaa, // PBLENDW XMM12, XMM13, 0xAA
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm14_xmm15_mask_0x55() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0e, 0xf7, 0x55, // PBLENDW XMM14, XMM15, 0x55
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm0_mem_mask_0x00() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x0e, 0x00, 0x00, // PBLENDW XMM0, [RAX], 0x00
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
                           0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm0_mem_mask_0xff() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x0e, 0x00, 0xff, // PBLENDW XMM0, [RAX], 0xFF
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm1_mem_mask_0x0f() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x0e, 0x08, 0x0f, // PBLENDW XMM1, [RAX], 0x0F
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                           0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm2_mem_mask_0xf0() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x0e, 0x10, 0xf0, // PBLENDW XMM2, [RAX], 0xF0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm3_mem_mask_0xaa() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x0e, 0x18, 0xaa, // PBLENDW XMM3, [RAX], 0xAA
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_same_register_mask_0x00() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xc0, 0x00, // PBLENDW XMM0, XMM0, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_same_register_mask_0xff() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xc9, 0xff, // PBLENDW XMM1, XMM1, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_sequential_operations() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xc1, 0x01, // PBLENDW XMM0, XMM1, 0x01
        0x66, 0x0f, 0x3a, 0x0e, 0xd3, 0x02, // PBLENDW XMM2, XMM3, 0x02
        0x66, 0x0f, 0x3a, 0x0e, 0xe5, 0x04, // PBLENDW XMM4, XMM5, 0x04
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm0_xmm1_mask_0x02() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xc1, 0x02, // PBLENDW XMM0, XMM1, 0x02 (blend word 1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm0_xmm1_mask_0x04() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xc1, 0x04, // PBLENDW XMM0, XMM1, 0x04 (blend word 2)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm0_xmm1_mask_0x08() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xc1, 0x08, // PBLENDW XMM0, XMM1, 0x08 (blend word 3)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm0_xmm1_mask_0x10() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xc1, 0x10, // PBLENDW XMM0, XMM1, 0x10 (blend word 4)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm0_xmm1_mask_0x20() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xc1, 0x20, // PBLENDW XMM0, XMM1, 0x20 (blend word 5)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm0_xmm1_mask_0x40() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x0e, 0xc1, 0x40, // PBLENDW XMM0, XMM1, 0x40 (blend word 6)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm0_xmm15_cross_mask_0xaa() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x0e, 0xf8, 0xaa, // PBLENDW XMM0, XMM15, 0xAA
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_xmm15_xmm0_cross_mask_0x55() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x0e, 0xf8, 0x55, // PBLENDW XMM15, XMM0, 0x55
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendw_mem_displacement() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x0e, 0x40, 0x10, 0xaa, // PBLENDW XMM0, [RAX+0x10], 0xAA
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC]);
    emu.run(None).unwrap();
}
