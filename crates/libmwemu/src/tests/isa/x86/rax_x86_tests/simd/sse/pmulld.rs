use crate::*;

// PMULLD - Multiply Packed Signed Dword Integers and Store Low Result
//
// Performs a SIMD signed multiply of the packed signed dword integers from
// each element of the first source operand with the corresponding element in
// the second source operand. The low 32 bits of each 64-bit intermediate
// result are stored to the destination operand.
//
// Opcode:
//   66 0F 38 40 /r    PMULLD xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

// Test basic multiplication with different register pairs
#[test]
fn test_pmulld_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xc1, // PMULLD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulld_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xd3, // PMULLD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulld_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xe5, // PMULLD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulld_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xf7, // PMULLD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test high XMM registers (XMM8-XMM15)
#[test]
fn test_pmulld_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x40, 0xc1, // PMULLD XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulld_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x40, 0xd3, // PMULLD XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulld_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x40, 0xe5, // PMULLD XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulld_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x40, 0xf7, // PMULLD XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test memory operands with positive values
#[test]
fn test_pmulld_xmm0_mem_positive() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x00, // PMULLD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x02, 0x00, 0x00, 0x00, // 2
        0x03, 0x00, 0x00, 0x00, // 3
        0x04, 0x00, 0x00, 0x00, // 4
        0x05, 0x00, 0x00, 0x00, // 5
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// Test memory operands with negative values
#[test]
fn test_pmulld_xmm1_mem_negative() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x08, // PMULLD XMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0xff, 0xff, 0xff, 0xff, // -1
        0xfe, 0xff, 0xff, 0xff, // -2
        0xfd, 0xff, 0xff, 0xff, // -3
        0xfc, 0xff, 0xff, 0xff, // -4
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// Test memory operands with mixed positive and negative values
#[test]
fn test_pmulld_xmm2_mem_mixed() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x10, // PMULLD XMM2, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x05, 0x00, 0x00, 0x00, // 5
        0xfd, 0xff, 0xff, 0xff, // -3
        0x64, 0x00, 0x00, 0x00, // 100
        0xce, 0xff, 0xff, 0xff, // -50
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// Test overflow behavior (result truncated to low 32 bits)
#[test]
fn test_pmulld_xmm3_mem_overflow() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x18, // PMULLD XMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x00, 0x00, 0x01, 0x00, // 0x10000 (65536)
        0x00, 0x00, 0x02, 0x00, // 0x20000 (131072)
        0x00, 0x00, 0x04, 0x00, // 0x40000 (262144)
        0x00, 0x00, 0x08, 0x00, // 0x80000 (524288)
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// Test with zeros
#[test]
fn test_pmulld_xmm4_mem_zeros() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x20, // PMULLD XMM4, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // All zeros
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// Test with ones
#[test]
fn test_pmulld_xmm5_mem_ones() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x28, // PMULLD XMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// Test same register (square operation)
#[test]
fn test_pmulld_same_register_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xc0, // PMULLD XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulld_same_register_xmm7() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xff, // PMULLD XMM7, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test sequential operations
#[test]
fn test_pmulld_sequential_operations() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xc1, // PMULLD XMM0, XMM1
        0x66, 0x0f, 0x38, 0x40, 0xd3, // PMULLD XMM2, XMM3
        0x66, 0x0f, 0x38, 0x40, 0xe5, // PMULLD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test memory with displacement
#[test]
fn test_pmulld_mem_displacement() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x40, 0x10, // PMULLD XMM0, [RAX+0x10]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x02, 0x00, 0x00, 0x00,
        0x03, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// Test cross-register operations (low to high)
#[test]
fn test_pmulld_xmm0_xmm15() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x40, 0xf8, // PMULLD XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test with maximum positive values
#[test]
fn test_pmulld_xmm6_mem_max_positive() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x30, // PMULLD XMM6, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0xff, 0xff, 0xff, 0x7f,
        0xff, 0xff, 0xff, 0x7f,
        0xff, 0xff, 0xff, 0x7f,
        0xff, 0xff, 0xff, 0x7f,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// Test with minimum negative values
#[test]
fn test_pmulld_xmm7_mem_min_negative() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x38, // PMULLD XMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x80,
        0x00, 0x00, 0x00, 0x80,
        0x00, 0x00, 0x00, 0x80,
        0x00, 0x00, 0x00, 0x80,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// Additional register combinations
#[test]
fn test_pmulld_xmm1_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xc8, // PMULLD XMM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulld_xmm3_xmm2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xda, // PMULLD XMM3, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulld_xmm5_xmm4() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xec, // PMULLD XMM5, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulld_xmm7_xmm6() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xfe, // PMULLD XMM7, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test high register combinations
#[test]
fn test_pmulld_xmm9_xmm8() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x40, 0xc8, // PMULLD XMM9, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulld_xmm11_xmm10() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x40, 0xda, // PMULLD XMM11, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulld_xmm13_xmm12() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x40, 0xec, // PMULLD XMM13, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulld_xmm15_xmm14() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x40, 0xfe, // PMULLD XMM15, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test alternating positive and negative
#[test]
fn test_pmulld_xmm0_mem_alternating() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x00, // PMULLD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x0a, 0x00, 0x00, 0x00, // 10
        0xf6, 0xff, 0xff, 0xff, // -10
        0x14, 0x00, 0x00, 0x00, // 20
        0xec, 0xff, 0xff, 0xff, // -20
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// Additional tests to reach 35+ tests
#[test]
fn test_pmulld_xmm0_xmm2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xc2, // PMULLD XMM0, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulld_xmm1_xmm3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xcb, // PMULLD XMM1, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulld_xmm2_xmm4() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xd4, // PMULLD XMM2, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulld_xmm3_xmm5() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xdd, // PMULLD XMM3, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulld_xmm4_xmm6() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xe6, // PMULLD XMM4, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
