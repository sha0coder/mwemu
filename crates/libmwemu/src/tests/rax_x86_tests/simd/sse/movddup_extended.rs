use crate::*;

// MOVDDUP Extended Tests - Additional comprehensive test coverage
//
// This file provides extended test coverage for MOVDDUP instruction
// beyond the basic tests in movddup.rs
//
// MOVDDUP duplicates the low 64-bit FP value to the high 64-bit
//
// Opcode:
// F2 0F 12 /r    MOVDDUP xmm1, xmm2/m64

const ALIGNED_ADDR: u64 = 0x3000;

// Extended register combination tests
#[test]
fn test_movddup_xmm0_xmm2() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xc2, // MOVDDUP XMM0, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm0_xmm3() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xc3, // MOVDDUP XMM0, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm0_xmm4() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xc4, // MOVDDUP XMM0, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm0_xmm5() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xc5, // MOVDDUP XMM0, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm0_xmm6() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xc6, // MOVDDUP XMM0, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm0_xmm7() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xc7, // MOVDDUP XMM0, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm1_xmm0() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xc8, // MOVDDUP XMM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm1_xmm3() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xcb, // MOVDDUP XMM1, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm1_xmm4() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xcc, // MOVDDUP XMM1, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm1_xmm5() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xcd, // MOVDDUP XMM1, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Extended high register tests
#[test]
fn test_movddup_xmm8_xmm0() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0xc0, // MOVDDUP XMM8, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm9_xmm1() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0xc9, // MOVDDUP XMM9, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm10_xmm2() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0xd2, // MOVDDUP XMM10, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm11_xmm3() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0xdb, // MOVDDUP XMM11, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm12_xmm4() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0xe4, // MOVDDUP XMM12, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm13_xmm5() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0xed, // MOVDDUP XMM13, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm14_xmm6() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0xf6, // MOVDDUP XMM14, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm15_xmm7() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0xff, // MOVDDUP XMM15, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Memory operand tests with various addressing modes
#[test]
fn test_movddup_xmm2_mem() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM2, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm3_mem() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM3, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm4_mem() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM4, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm5_mem() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM5, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm6_mem() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM6, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm8_mem() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM8, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm9_mem() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM9, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm10_mem() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM10, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm11_mem() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM11, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm12_mem() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM12, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm13_mem() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM13, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm14_mem() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM14, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Complex pattern tests
#[test]
fn test_movddup_alternating_low_high() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xc1, // MOVDDUP XMM0, XMM1
        0xf2, 0x45, 0x0f, 0x12, 0xc1, // MOVDDUP XMM8, XMM9
        0xf2, 0x0f, 0x12, 0xd3, // MOVDDUP XMM2, XMM3
        0xf2, 0x45, 0x0f, 0x12, 0xd3, // MOVDDUP XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_all_to_xmm0() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xc1, // MOVDDUP XMM0, XMM1
        0xf2, 0x0f, 0x12, 0xc2, // MOVDDUP XMM0, XMM2
        0xf2, 0x0f, 0x12, 0xc3, // MOVDDUP XMM0, XMM3
        0xf2, 0x0f, 0x12, 0xc4, // MOVDDUP XMM0, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_xmm0_to_all() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xc8, // MOVDDUP XMM1, XMM0
        0xf2, 0x0f, 0x12, 0xd0, // MOVDDUP XMM2, XMM0
        0xf2, 0x0f, 0x12, 0xd8, // MOVDDUP XMM3, XMM0
        0xf2, 0x0f, 0x12, 0xe0, // MOVDDUP XMM4, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_circular_pattern() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xc1, // MOVDDUP XMM0, XMM1
        0xf2, 0x0f, 0x12, 0xca, // MOVDDUP XMM1, XMM2
        0xf2, 0x0f, 0x12, 0xd3, // MOVDDUP XMM2, XMM3
        0xf2, 0x0f, 0x12, 0xd8, // MOVDDUP XMM3, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_cascading() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xc1, // MOVDDUP XMM0, XMM1
        0xf2, 0x0f, 0x12, 0xd0, // MOVDDUP XMM2, XMM0
        0xf2, 0x0f, 0x12, 0xda, // MOVDDUP XMM3, XMM2
        0xf2, 0x0f, 0x12, 0xe3, // MOVDDUP XMM4, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_pair_swap() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xf8, // MOVDDUP XMM7, XMM0
        0xf2, 0x0f, 0x12, 0xc7, // MOVDDUP XMM0, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_interleaved() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xc1, // MOVDDUP XMM0, XMM1
        0xf2, 0x0f, 0x12, 0xe5, // MOVDDUP XMM4, XMM5
        0xf2, 0x0f, 0x12, 0xd3, // MOVDDUP XMM2, XMM3
        0xf2, 0x0f, 0x12, 0xf7, // MOVDDUP XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_high_to_low() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x41, 0x0f, 0x12, 0xc0, // MOVDDUP XMM0, XMM8
        0xf2, 0x41, 0x0f, 0x12, 0xc9, // MOVDDUP XMM1, XMM9
        0xf2, 0x41, 0x0f, 0x12, 0xd2, // MOVDDUP XMM2, XMM10
        0xf2, 0x41, 0x0f, 0x12, 0xdb, // MOVDDUP XMM3, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_low_to_high() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0xc0, // MOVDDUP XMM8, XMM0
        0xf2, 0x44, 0x0f, 0x12, 0xc9, // MOVDDUP XMM9, XMM1
        0xf2, 0x44, 0x0f, 0x12, 0xd2, // MOVDDUP XMM10, XMM2
        0xf2, 0x44, 0x0f, 0x12, 0xdb, // MOVDDUP XMM11, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_mixed_mem_reg() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xc1, // MOVDDUP XMM0, XMM1
        0xf2, 0x0f, 0x12, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM2, [0x3000]
        0xf2, 0x0f, 0x12, 0xdb, // MOVDDUP XMM3, XMM3
        0xf2, 0x0f, 0x12, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM4, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_all_high_regs() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x45, 0x0f, 0x12, 0xc1, // MOVDDUP XMM8, XMM9
        0xf2, 0x45, 0x0f, 0x12, 0xd3, // MOVDDUP XMM10, XMM11
        0xf2, 0x45, 0x0f, 0x12, 0xe5, // MOVDDUP XMM12, XMM13
        0xf2, 0x45, 0x0f, 0x12, 0xf7, // MOVDDUP XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_mixed_cross_boundary() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0xc7, // MOVDDUP XMM8, XMM7
        0xf2, 0x41, 0x0f, 0x12, 0xf8, // MOVDDUP XMM7, XMM8
        0xf2, 0x44, 0x0f, 0x12, 0xce, // MOVDDUP XMM9, XMM6
        0xf2, 0x41, 0x0f, 0x12, 0xf1, // MOVDDUP XMM6, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_long_chain() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xc1, // MOVDDUP XMM0, XMM1
        0xf2, 0x0f, 0x12, 0xca, // MOVDDUP XMM1, XMM2
        0xf2, 0x0f, 0x12, 0xd3, // MOVDDUP XMM2, XMM3
        0xf2, 0x0f, 0x12, 0xdc, // MOVDDUP XMM3, XMM4
        0xf2, 0x0f, 0x12, 0xe5, // MOVDDUP XMM4, XMM5
        0xf2, 0x0f, 0x12, 0xee, // MOVDDUP XMM5, XMM6
        0xf2, 0x0f, 0x12, 0xf7, // MOVDDUP XMM6, XMM7
        0xf2, 0x0f, 0x12, 0xf8, // MOVDDUP XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movddup_bidirectional() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x12, 0xc1, // MOVDDUP XMM0, XMM1
        0xf2, 0x0f, 0x12, 0xc8, // MOVDDUP XMM1, XMM0
        0xf2, 0x0f, 0x12, 0xd3, // MOVDDUP XMM2, XMM3
        0xf2, 0x0f, 0x12, 0xda, // MOVDDUP XMM3, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
