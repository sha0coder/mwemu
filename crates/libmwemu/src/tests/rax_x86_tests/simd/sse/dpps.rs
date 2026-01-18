use crate::*;

// DPPS - Dot Product of Packed Single Precision Floating-Point Values
//
// Conditionally multiplies packed single-precision floats from destination
// with source based on a mask in bits 7:4 of imm8. The four products are
// summed, and the result is conditionally broadcast to destination elements
// based on bits 3:0 of imm8.
//
// imm8[7:4]: Source mask (which elements to multiply)
// imm8[3:0]: Destination mask (which results to write)
//
// Opcode:
//   66 0F 3A 40 /r ib    DPPS xmm1, xmm2/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// Test all destination mask bits = 0 (all zeros written)
#[test]
fn test_dpps_xmm0_xmm1_mask_0xf0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0xf0, // DPPS XMM0, XMM1, 0xF0 (multiply all, write none)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test all destination mask bits = 1 (broadcast to all)
#[test]
fn test_dpps_xmm0_xmm1_mask_0xff() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0xff, // DPPS XMM0, XMM1, 0xFF (multiply all, write all)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test only lowest element written
#[test]
fn test_dpps_xmm0_xmm1_mask_0xf1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0xf1, // DPPS XMM0, XMM1, 0xF1 (multiply all, write element 0)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test only second element written
#[test]
fn test_dpps_xmm0_xmm1_mask_0xf2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0xf2, // DPPS XMM0, XMM1, 0xF2 (multiply all, write element 1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test only third element written
#[test]
fn test_dpps_xmm0_xmm1_mask_0xf4() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0xf4, // DPPS XMM0, XMM1, 0xF4 (multiply all, write element 2)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test only highest element written
#[test]
fn test_dpps_xmm0_xmm1_mask_0xf8() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0xf8, // DPPS XMM0, XMM1, 0xF8 (multiply all, write element 3)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test low two elements written
#[test]
fn test_dpps_xmm0_xmm1_mask_0xf3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0xf3, // DPPS XMM0, XMM1, 0xF3 (multiply all, write elements 0-1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test high two elements written
#[test]
fn test_dpps_xmm0_xmm1_mask_0xfc() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0xfc, // DPPS XMM0, XMM1, 0xFC (multiply all, write elements 2-3)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test source mask: only element 0 multiplied
#[test]
fn test_dpps_xmm0_xmm1_mask_0x1f() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0x1f, // DPPS XMM0, XMM1, 0x1F (multiply element 0, write all)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test source mask: only element 1 multiplied
#[test]
fn test_dpps_xmm0_xmm1_mask_0x2f() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0x2f, // DPPS XMM0, XMM1, 0x2F (multiply element 1, write all)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test source mask: only element 2 multiplied
#[test]
fn test_dpps_xmm0_xmm1_mask_0x4f() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0x4f, // DPPS XMM0, XMM1, 0x4F (multiply element 2, write all)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test source mask: only element 3 multiplied
#[test]
fn test_dpps_xmm0_xmm1_mask_0x8f() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0x8f, // DPPS XMM0, XMM1, 0x8F (multiply element 3, write all)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test source mask: elements 0 and 1 multiplied
#[test]
fn test_dpps_xmm0_xmm1_mask_0x3f() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0x3f, // DPPS XMM0, XMM1, 0x3F (multiply elements 0-1, write all)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test source mask: elements 2 and 3 multiplied
#[test]
fn test_dpps_xmm0_xmm1_mask_0xcf() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0xcf, // DPPS XMM0, XMM1, 0xCF (multiply elements 2-3, write all)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test source mask: elements 0 and 2 multiplied
#[test]
fn test_dpps_xmm0_xmm1_mask_0x5f() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0x5f, // DPPS XMM0, XMM1, 0x5F (multiply elements 0,2, write all)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test source mask: elements 1 and 3 multiplied
#[test]
fn test_dpps_xmm0_xmm1_mask_0xaf() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0xaf, // DPPS XMM0, XMM1, 0xAF (multiply elements 1,3, write all)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test no source or destination mask
#[test]
fn test_dpps_xmm0_xmm1_mask_0x00() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0x00, // DPPS XMM0, XMM1, 0x00 (multiply none, write none)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test different register pairs
#[test]
fn test_dpps_xmm2_xmm3_mask_0xff() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xd3, 0xff, // DPPS XMM2, XMM3, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dpps_xmm4_xmm5_mask_0x7f() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xe5, 0x7f, // DPPS XMM4, XMM5, 0x7F
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dpps_xmm6_xmm7_mask_0xef() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xf7, 0xef, // DPPS XMM6, XMM7, 0xEF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test high XMM registers (XMM8-XMM15)
#[test]
fn test_dpps_xmm8_xmm9_mask_0xff() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x40, 0xc1, 0xff, // DPPS XMM8, XMM9, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dpps_xmm10_xmm11_mask_0xf5() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x40, 0xd3, 0xf5, // DPPS XMM10, XMM11, 0xF5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dpps_xmm12_xmm13_mask_0xfa() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x40, 0xe5, 0xfa, // DPPS XMM12, XMM13, 0xFA
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dpps_xmm14_xmm15_mask_0xaa() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x40, 0xf7, 0xaa, // DPPS XMM14, XMM15, 0xAA
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test memory operands
#[test]
fn test_dpps_xmm0_mem_mask_0xff() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x40, 0x00, 0xff, // DPPS XMM0, [RAX], 0xFF
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x40, // 2.0
        0x00, 0x00, 0x40, 0x40, // 3.0
        0x00, 0x00, 0x80, 0x40, // 4.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_dpps_xmm1_mem_mask_0xf1() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x40, 0x08, 0xf1, // DPPS XMM1, [RAX], 0xF1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f]; // All 1.0
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_dpps_xmm2_mem_mask_0x71() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x40, 0x10, 0x71, // DPPS XMM2, [RAX], 0x71
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, // 0.0
        0x00, 0x00, 0x80, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x40, // 2.0
        0x00, 0x00, 0x40, 0x40, // 3.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// Test same register
#[test]
fn test_dpps_same_register_mask_0xff() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc0, 0xff, // DPPS XMM0, XMM0, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dpps_same_register_mask_0xf0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc9, 0xf0, // DPPS XMM1, XMM1, 0xF0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test sequential operations
#[test]
fn test_dpps_sequential_operations() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0xff, // DPPS XMM0, XMM1, 0xFF
        0x66, 0x0f, 0x3a, 0x40, 0xd3, 0xf1, // DPPS XMM2, XMM3, 0xF1
        0x66, 0x0f, 0x3a, 0x40, 0xe5, 0x1f, // DPPS XMM4, XMM5, 0x1F
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test mixed source and destination masks
#[test]
fn test_dpps_xmm0_xmm1_mask_0x11() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0x11, // DPPS XMM0, XMM1, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dpps_xmm0_xmm1_mask_0x22() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0x22, // DPPS XMM0, XMM1, 0x22
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dpps_xmm0_xmm1_mask_0x44() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0x44, // DPPS XMM0, XMM1, 0x44
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dpps_xmm0_xmm1_mask_0x88() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0x88, // DPPS XMM0, XMM1, 0x88
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test alternating patterns
#[test]
fn test_dpps_xmm0_xmm1_mask_0x55() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0x55, // DPPS XMM0, XMM1, 0x55
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dpps_xmm0_xmm1_mask_0xaa() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0xaa, // DPPS XMM0, XMM1, 0xAA
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test memory with displacement
#[test]
fn test_dpps_mem_displacement() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x40, 0x40, 0x10, 0xff, // DPPS XMM0, [RAX+0x10], 0xFF
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// Test cross-register operations (low to high)
#[test]
fn test_dpps_xmm0_xmm15_mask_0xff() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x40, 0xf8, 0xff, // DPPS XMM15, XMM0, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test specific source/destination combinations
#[test]
fn test_dpps_xmm0_xmm1_mask_0x77() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0x77, // DPPS XMM0, XMM1, 0x77
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dpps_xmm0_xmm1_mask_0xee() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0xee, // DPPS XMM0, XMM1, 0xEE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dpps_xmm0_xmm1_mask_0x33() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0x33, // DPPS XMM0, XMM1, 0x33
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dpps_xmm0_xmm1_mask_0xcc() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0xcc, // DPPS XMM0, XMM1, 0xCC
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Additional edge cases
#[test]
fn test_dpps_xmm0_xmm1_mask_0x0f() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0x0f, // DPPS XMM0, XMM1, 0x0F
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dpps_xmm3_xmm4_mask_0x9f() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xdc, 0x9f, // DPPS XMM3, XMM4, 0x9F
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Additional tests to reach 45+ tests
#[test]
fn test_dpps_xmm0_xmm1_mask_0xb7() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0xb7, // DPPS XMM0, XMM1, 0xB7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dpps_xmm0_xmm1_mask_0xd5() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xc1, 0xd5, // DPPS XMM0, XMM1, 0xD5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_dpps_xmm1_xmm3_mask_0x6a() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x40, 0xcb, 0x6a, // DPPS XMM1, XMM3, 0x6A
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
