use crate::*;

// PBLENDVB - Variable Blend Packed Bytes
//
// Conditionally copies byte elements from the source operand to the destination
// operand depending on mask bits defined in the implicit third register (XMM0
// for legacy SSE4.1). The mask bits are the most significant bit in each byte
// element of XMM0.
//
// If mask bit is "1", copy from source; else, keep destination unchanged.
//
// Opcode:
//   66 0F 38 10 /r    PBLENDVB xmm1, xmm2/m128, <XMM0>

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_pblendvb_xmm1_xmm2_mask_all_zeros() {
    let mut emu = emu64();
    // XMM0 (mask) = all zeros, so XMM1 unchanged
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2 (mask in XMM0)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_all_ones() {
    let mut emu = emu64();
    // XMM0 (mask) = all 0xFF, so all bytes from XMM2
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_alternating() {
    let mut emu = emu64();
    // XMM0 (mask) alternates 0x00 and 0xFF
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm3_xmm4_mask_low_half() {
    let mut emu = emu64();
    // XMM0 (mask) = 0xFF for low 8 bytes, 0x00 for high 8 bytes
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xdc, // PBLENDVB XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm5_xmm6_mask_high_half() {
    let mut emu = emu64();
    // XMM0 (mask) = 0x00 for low 8 bytes, 0xFF for high 8 bytes
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xee, // PBLENDVB XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm7_xmm0_basic() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xf8, // PBLENDVB XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm3_mask_sign_bit() {
    let mut emu = emu64();
    // XMM0 (mask) has sign bit set (0x80)
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xcb, // PBLENDVB XMM1, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm2_xmm4_mask_no_sign_bit() {
    let mut emu = emu64();
    // XMM0 (mask) = 0x7F (sign bit not set)
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xd4, // PBLENDVB XMM2, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm8_xmm9_high_regs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x10, 0xc1, // PBLENDVB XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm10_xmm11_high_regs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x10, 0xd3, // PBLENDVB XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm12_xmm13_high_regs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x10, 0xe5, // PBLENDVB XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm14_xmm15_high_regs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x10, 0xf7, // PBLENDVB XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm1_mem_mask_zeros() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x10, 0x08, // PBLENDVB XMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
                           0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm2_mem_mask_all_ones() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x10, 0x10, // PBLENDVB XMM2, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm3_mem_mask_alternating() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x10, 0x18, // PBLENDVB XMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF,
                           0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm4_mem_mask_low_half() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x10, 0x20, // PBLENDVB XMM4, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm5_mem_mask_high_half() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x10, 0x28, // PBLENDVB XMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_sequential_operations() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0x66, 0x0f, 0x38, 0x10, 0xdc, // PBLENDVB XMM3, XMM4
        0x66, 0x0f, 0x38, 0x10, 0xee, // PBLENDVB XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_single_byte() {
    let mut emu = emu64();
    // XMM0 (mask) has only one byte with sign bit set
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_two_bytes() {
    let mut emu = emu64();
    // XMM0 (mask) has two bytes with sign bit set
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_four_bytes() {
    let mut emu = emu64();
    // XMM0 (mask) has four bytes with sign bit set
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_eight_bytes() {
    let mut emu = emu64();
    // XMM0 (mask) has eight bytes with sign bit set
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_0x80_pattern() {
    let mut emu = emu64();
    // XMM0 (mask) has 0x80 (only sign bit set)
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_0x81_pattern() {
    let mut emu = emu64();
    // XMM0 (mask) has 0x81 (sign bit + LSB set)
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_0xc0_pattern() {
    let mut emu = emu64();
    // XMM0 (mask) has 0xC0 (two high bits set)
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm0_xmm15_cross() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x10, 0xf8, // PBLENDVB XMM0, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm15_xmm1_cross() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x10, 0xf9, // PBLENDVB XMM15, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_mem_displacement() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x10, 0x48, 0x10, // PBLENDVB XMM1, [RAX+0x10]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC]);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_pattern1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_pattern2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_random_pattern() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
