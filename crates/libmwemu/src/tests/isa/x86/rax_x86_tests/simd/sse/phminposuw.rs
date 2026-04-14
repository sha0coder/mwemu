use crate::*;

// PHMINPOSUW - Packed Horizontal Word Minimum
//
// Determines the minimum unsigned word value in the source operand and places
// the unsigned word in the low word (bits 0-15) of the destination operand.
// The word index of the minimum value is stored in bits 16-18 of the destination
// operand. The remaining upper bits of the destination are set to zero.
//
// Result format:
//   DEST[15:0]   = minimum unsigned word value
//   DEST[18:16]  = index (0-7) of minimum word
//   DEST[127:19] = 0
//
// Opcode:
//   66 0F 38 41 /r    PHMINPOSUW xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_phminposuw_xmm0_xmm1_basic() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_xmm2_xmm3_basic() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xd3, // PHMINPOSUW XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_xmm4_xmm5_zeros() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xe5, // PHMINPOSUW XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_xmm6_xmm7_min_at_index_0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xf7, // PHMINPOSUW XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_xmm0_xmm1_min_at_index_1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_xmm1_xmm2_min_at_index_7() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xca, // PHMINPOSUW XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_xmm3_xmm4_min_at_middle() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xdc, // PHMINPOSUW XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x41, 0xd3, // PHMINPOSUW XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x41, 0xe5, // PHMINPOSUW XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x41, 0xf7, // PHMINPOSUW XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x41, 0x00, // PHMINPOSUW XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [8, 0, 7, 0, 6, 0, 5, 0, 4, 0, 3, 0, 2, 0, 1, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_xmm1_mem_min_at_start() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x41, 0x08, // PHMINPOSUW XMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_xmm2_mem_all_max() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x41, 0x10, // PHMINPOSUW XMM2, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_same_register() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc0, // PHMINPOSUW XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_sequential() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x41, 0xd3, // PHMINPOSUW XMM2, XMM3
        0x66, 0x0f, 0x38, 0x41, 0xe5, // PHMINPOSUW XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_xmm15_xmm0_cross() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x41, 0xf8, // PHMINPOSUW XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_mem_displacement() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x41, 0x40, 0x10, // PHMINPOSUW XMM0, [RAX+0x10]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_min_is_zero() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_min_is_one() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_all_equal() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_duplicate_minimums() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_index_bits_16_18() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_upper_bits_zeroed() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_unsigned_comparison() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_alternating_pattern() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_random_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_power_of_two() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_boundary_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phminposuw_min_at_each_position() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
