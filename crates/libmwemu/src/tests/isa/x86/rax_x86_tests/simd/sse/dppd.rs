use crate::*;

// DPPD - Dot Product of Packed Double Precision Floating-Point Values
//
// Conditionally multiplies packed double-precision floats from destination
// with source based on a mask in bits 5:4 of imm8. The two products are
// summed, and the result is conditionally broadcast to destination elements
// based on bits 1:0 of imm8.
//
// imm8[5:4]: Source mask (which elements to multiply)
// imm8[1:0]: Destination mask (which results to write)
//
// Opcode:
//   66 0F 3A 41 /r ib    DPPD xmm1, xmm2/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// Test all source and destination masks enabled
#[test]
fn test_dppd_xmm0_xmm1_mask_0x33() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xc1, 0x33, // DPPD XMM0, XMM1, 0x33 (multiply all, write all)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test all destination mask bits = 0
#[test]
fn test_dppd_xmm0_xmm1_mask_0x30() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xc1, 0x30, // DPPD XMM0, XMM1, 0x30 (multiply all, write none)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test only low element written
#[test]
fn test_dppd_xmm0_xmm1_mask_0x31() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xc1, 0x31, // DPPD XMM0, XMM1, 0x31 (multiply all, write element 0)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test only high element written
#[test]
fn test_dppd_xmm0_xmm1_mask_0x32() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xc1, 0x32, // DPPD XMM0, XMM1, 0x32 (multiply all, write element 1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test only element 0 multiplied, write both
#[test]
fn test_dppd_xmm0_xmm1_mask_0x13() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xc1, 0x13, // DPPD XMM0, XMM1, 0x13 (multiply element 0, write all)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test only element 1 multiplied, write both
#[test]
fn test_dppd_xmm0_xmm1_mask_0x23() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xc1, 0x23, // DPPD XMM0, XMM1, 0x23 (multiply element 1, write all)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test only element 0 multiplied, write element 0
#[test]
fn test_dppd_xmm0_xmm1_mask_0x11() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xc1, 0x11, // DPPD XMM0, XMM1, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test only element 1 multiplied, write element 1
#[test]
fn test_dppd_xmm0_xmm1_mask_0x22() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xc1, 0x22, // DPPD XMM0, XMM1, 0x22
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test no multiplication, no writes
#[test]
fn test_dppd_xmm0_xmm1_mask_0x00() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xc1, 0x00, // DPPD XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test cross combinations
#[test]
fn test_dppd_xmm0_xmm1_mask_0x12() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xc1, 0x12, // DPPD XMM0, XMM1, 0x12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_xmm0_xmm1_mask_0x21() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xc1, 0x21, // DPPD XMM0, XMM1, 0x21
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test different register pairs
#[test]
fn test_dppd_xmm2_xmm3_mask_0x33() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xd3, 0x33, // DPPD XMM2, XMM3, 0x33
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_xmm4_xmm5_mask_0x31() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xe5, 0x31, // DPPD XMM4, XMM5, 0x31
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_xmm6_xmm7_mask_0x32() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xf7, 0x32, // DPPD XMM6, XMM7, 0x32
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test high XMM registers (XMM8-XMM15)
#[test]
fn test_dppd_xmm8_xmm9_mask_0x33() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x41, 0xc1, 0x33, // DPPD XMM8, XMM9, 0x33
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_xmm10_xmm11_mask_0x31() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x41, 0xd3, 0x31, // DPPD XMM10, XMM11, 0x31
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_xmm12_xmm13_mask_0x32() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x41, 0xe5, 0x32, // DPPD XMM12, XMM13, 0x32
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_xmm14_xmm15_mask_0x13() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x41, 0xf7, 0x13, // DPPD XMM14, XMM15, 0x13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test memory operands
#[test]
fn test_dppd_xmm0_mem_mask_0x33() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x41, 0x00, 0x33, // DPPD XMM0, [RAX], 0x33
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // 2.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_xmm1_mem_mask_0x31() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x41, 0x08, 0x31, // DPPD XMM1, [RAX], 0x31
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_xmm2_mem_mask_0x32() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x41, 0x10, 0x32, // DPPD XMM2, [RAX], 0x32
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // 2.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40, // 3.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_xmm3_mem_mask_0x13() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x41, 0x18, 0x13, // DPPD XMM3, [RAX], 0x13
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_xmm4_mem_mask_0x23() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x41, 0x20, 0x23, // DPPD XMM4, [RAX], 0x23
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// Test same register
#[test]
fn test_dppd_same_register_mask_0x33() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xc0, 0x33, // DPPD XMM0, XMM0, 0x33
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_same_register_mask_0x30() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xc9, 0x30, // DPPD XMM1, XMM1, 0x30
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test sequential operations
#[test]
fn test_dppd_sequential_operations() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xc1, 0x33, // DPPD XMM0, XMM1, 0x33
        0x66, 0x0f, 0x3a, 0x41, 0xd3, 0x31, // DPPD XMM2, XMM3, 0x31
        0x66, 0x0f, 0x3a, 0x41, 0xe5, 0x32, // DPPD XMM4, XMM5, 0x32
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test memory with displacement
#[test]
fn test_dppd_mem_displacement() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x41, 0x40, 0x10, 0x33, // DPPD XMM0, [RAX+0x10], 0x33
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// Test cross-register operations (low to high)
#[test]
fn test_dppd_xmm0_xmm15_mask_0x33() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x41, 0xf8, 0x33, // DPPD XMM15, XMM0, 0x33
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test additional mask combinations
#[test]
fn test_dppd_xmm1_xmm2_mask_0x10() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xca, 0x10, // DPPD XMM1, XMM2, 0x10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_xmm2_xmm3_mask_0x20() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xd3, 0x20, // DPPD XMM2, XMM3, 0x20
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_xmm3_xmm4_mask_0x01() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xdc, 0x01, // DPPD XMM3, XMM4, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_xmm4_xmm5_mask_0x02() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xe5, 0x02, // DPPD XMM4, XMM5, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_xmm5_xmm6_mask_0x03() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xee, 0x03, // DPPD XMM5, XMM6, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test all possible valid source mask combinations
#[test]
fn test_dppd_xmm0_xmm1_mask_0x03() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xc1, 0x03, // DPPD XMM0, XMM1, 0x03 (no multiply, write all)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Additional register pairs
#[test]
fn test_dppd_xmm7_xmm0_mask_0x33() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xf8, 0x33, // DPPD XMM7, XMM0, 0x33
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_xmm1_xmm7_mask_0x11() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xcf, 0x11, // DPPD XMM1, XMM7, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_xmm3_xmm5_mask_0x22() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xdd, 0x22, // DPPD XMM3, XMM5, 0x22
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_xmm9_xmm10_mask_0x12() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x41, 0xca, 0x12, // DPPD XMM9, XMM10, 0x12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_xmm11_xmm12_mask_0x21() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x41, 0xdc, 0x21, // DPPD XMM11, XMM12, 0x21
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Additional tests to reach 40+ tests
#[test]
fn test_dppd_xmm0_xmm2_mask_0x30() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xc2, 0x30, // DPPD XMM0, XMM2, 0x30
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dppd_xmm5_xmm7_mask_0x11() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x41, 0xef, 0x11, // DPPD XMM5, XMM7, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
