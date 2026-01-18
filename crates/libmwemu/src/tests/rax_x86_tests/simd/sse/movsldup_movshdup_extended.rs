use crate::*;

// MOVSLDUP/MOVSHDUP Extended Tests - Additional comprehensive coverage
//
// MOVSLDUP duplicates even-indexed (low) FP values: [0,0,2,2]
// MOVSHDUP duplicates odd-indexed (high) FP values: [1,1,3,3]
//
// Opcodes:
// F3 0F 12 /r    MOVSLDUP xmm1, xmm2/m128
// F3 0F 16 /r    MOVSHDUP xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

// Additional MOVSLDUP register combinations
#[test]
fn test_movsldup_xmm0_xmm2() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x12, 0xc2, // MOVSLDUP XMM0, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm0_xmm3() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x12, 0xc3, // MOVSLDUP XMM0, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm0_xmm4() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x12, 0xc4, // MOVSLDUP XMM0, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm0_xmm5() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x12, 0xc5, // MOVSLDUP XMM0, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm0_xmm6() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x12, 0xc6, // MOVSLDUP XMM0, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm0_xmm7() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x12, 0xc7, // MOVSLDUP XMM0, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Additional MOVSHDUP register combinations
#[test]
fn test_movshdup_xmm0_xmm2() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x16, 0xc2, // MOVSHDUP XMM0, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm0_xmm3() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x16, 0xc3, // MOVSHDUP XMM0, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm0_xmm4() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x16, 0xc4, // MOVSHDUP XMM0, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm0_xmm5() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x16, 0xc5, // MOVSHDUP XMM0, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm0_xmm6() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x16, 0xc6, // MOVSHDUP XMM0, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm1_xmm7() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x16, 0xcf, // MOVSHDUP XMM1, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// High register extended tests
#[test]
fn test_movsldup_xmm8_xmm0() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x12, 0xc0, // MOVSLDUP XMM8, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm9_xmm1() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x12, 0xc9, // MOVSLDUP XMM9, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm10_xmm2() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x12, 0xd2, // MOVSLDUP XMM10, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm11_xmm3() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x12, 0xdb, // MOVSLDUP XMM11, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm12_xmm4() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x12, 0xe4, // MOVSLDUP XMM12, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm13_xmm5() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x12, 0xed, // MOVSLDUP XMM13, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm14_xmm6() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x12, 0xf6, // MOVSLDUP XMM14, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm15_xmm7() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x12, 0xff, // MOVSLDUP XMM15, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm8_xmm0() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x16, 0xc0, // MOVSHDUP XMM8, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm9_xmm1() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x16, 0xc9, // MOVSHDUP XMM9, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm10_xmm2() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x16, 0xd2, // MOVSHDUP XMM10, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm11_xmm3() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x16, 0xdb, // MOVSHDUP XMM11, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm12_xmm4() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x16, 0xe4, // MOVSHDUP XMM12, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm13_xmm5() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x16, 0xed, // MOVSHDUP XMM13, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm14_xmm6() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x16, 0xf6, // MOVSHDUP XMM14, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm15_xmm7() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x16, 0xff, // MOVSHDUP XMM15, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Comprehensive pattern tests
#[test]
fn test_movsldup_movshdup_alternating() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x12, 0xc1, // MOVSLDUP XMM0, XMM1
        0xf3, 0x0f, 0x16, 0xd3, // MOVSHDUP XMM2, XMM3
        0xf3, 0x0f, 0x12, 0xe5, // MOVSLDUP XMM4, XMM5
        0xf3, 0x0f, 0x16, 0xf7, // MOVSHDUP XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_chain() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x12, 0xc1, // MOVSLDUP XMM0, XMM1
        0xf3, 0x0f, 0x12, 0xd0, // MOVSLDUP XMM2, XMM0
        0xf3, 0x0f, 0x12, 0xda, // MOVSLDUP XMM3, XMM2
        0xf3, 0x0f, 0x12, 0xe3, // MOVSLDUP XMM4, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_chain() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x16, 0xc1, // MOVSHDUP XMM0, XMM1
        0xf3, 0x0f, 0x16, 0xd0, // MOVSHDUP XMM2, XMM0
        0xf3, 0x0f, 0x16, 0xda, // MOVSHDUP XMM3, XMM2
        0xf3, 0x0f, 0x16, 0xe3, // MOVSHDUP XMM4, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_movshdup_interleaved() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x12, 0xc1, // MOVSLDUP XMM0, XMM1
        0xf3, 0x0f, 0x16, 0xd0, // MOVSHDUP XMM2, XMM0
        0xf3, 0x0f, 0x12, 0xda, // MOVSLDUP XMM3, XMM2
        0xf3, 0x0f, 0x16, 0xe3, // MOVSHDUP XMM4, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_high_regs_sequence() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x45, 0x0f, 0x12, 0xc1, // MOVSLDUP XMM8, XMM9
        0xf3, 0x45, 0x0f, 0x12, 0xd3, // MOVSLDUP XMM10, XMM11
        0xf3, 0x45, 0x0f, 0x12, 0xe5, // MOVSLDUP XMM12, XMM13
        0xf3, 0x45, 0x0f, 0x12, 0xf7, // MOVSLDUP XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_high_regs_sequence() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x45, 0x0f, 0x16, 0xc1, // MOVSHDUP XMM8, XMM9
        0xf3, 0x45, 0x0f, 0x16, 0xd3, // MOVSHDUP XMM10, XMM11
        0xf3, 0x45, 0x0f, 0x16, 0xe5, // MOVSHDUP XMM12, XMM13
        0xf3, 0x45, 0x0f, 0x16, 0xf7, // MOVSHDUP XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_mem_various_regs() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x12, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSLDUP XMM2, [0x3000]
        0xf3, 0x0f, 0x12, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSLDUP XMM3, [0x3000]
        0xf3, 0x0f, 0x12, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSLDUP XMM4, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_mem_various_regs() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x16, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSHDUP XMM2, [0x3000]
        0xf3, 0x0f, 0x16, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSHDUP XMM3, [0x3000]
        0xf3, 0x0f, 0x16, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSHDUP XMM4, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_cross_boundary() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x12, 0xc7, // MOVSLDUP XMM8, XMM7
        0xf3, 0x41, 0x0f, 0x12, 0xf8, // MOVSLDUP XMM7, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_cross_boundary() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x16, 0xc7, // MOVSHDUP XMM8, XMM7
        0xf3, 0x41, 0x0f, 0x16, 0xf8, // MOVSHDUP XMM7, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_movshdup_combined_pattern() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x12, 0xc1, // MOVSLDUP XMM0, XMM1
        0xf3, 0x0f, 0x16, 0xc1, // MOVSHDUP XMM0, XMM1
        0xf3, 0x0f, 0x12, 0xd3, // MOVSLDUP XMM2, XMM3
        0xf3, 0x0f, 0x16, 0xd3, // MOVSHDUP XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_all_low_to_high() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x12, 0xc0, // MOVSLDUP XMM8, XMM0
        0xf3, 0x44, 0x0f, 0x12, 0xc9, // MOVSLDUP XMM9, XMM1
        0xf3, 0x44, 0x0f, 0x12, 0xd2, // MOVSLDUP XMM10, XMM2
        0xf3, 0x44, 0x0f, 0x12, 0xdb, // MOVSLDUP XMM11, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_all_low_to_high() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x16, 0xc0, // MOVSHDUP XMM8, XMM0
        0xf3, 0x44, 0x0f, 0x16, 0xc9, // MOVSHDUP XMM9, XMM1
        0xf3, 0x44, 0x0f, 0x16, 0xd2, // MOVSHDUP XMM10, XMM2
        0xf3, 0x44, 0x0f, 0x16, 0xdb, // MOVSHDUP XMM11, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_all_high_to_low() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x41, 0x0f, 0x12, 0xc0, // MOVSLDUP XMM0, XMM8
        0xf3, 0x41, 0x0f, 0x12, 0xc9, // MOVSLDUP XMM1, XMM9
        0xf3, 0x41, 0x0f, 0x12, 0xd2, // MOVSLDUP XMM2, XMM10
        0xf3, 0x41, 0x0f, 0x12, 0xdb, // MOVSLDUP XMM3, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_all_high_to_low() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x41, 0x0f, 0x16, 0xc0, // MOVSHDUP XMM0, XMM8
        0xf3, 0x41, 0x0f, 0x16, 0xc9, // MOVSHDUP XMM1, XMM9
        0xf3, 0x41, 0x0f, 0x16, 0xd2, // MOVSHDUP XMM2, XMM10
        0xf3, 0x41, 0x0f, 0x16, 0xdb, // MOVSHDUP XMM3, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_comprehensive_all_regs() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x12, 0xc1, // MOVSLDUP XMM0, XMM1
        0xf3, 0x0f, 0x12, 0xca, // MOVSLDUP XMM1, XMM2
        0xf3, 0x0f, 0x12, 0xd3, // MOVSLDUP XMM2, XMM3
        0xf3, 0x0f, 0x12, 0xdc, // MOVSLDUP XMM3, XMM4
        0xf3, 0x0f, 0x12, 0xe5, // MOVSLDUP XMM4, XMM5
        0xf3, 0x0f, 0x12, 0xee, // MOVSLDUP XMM5, XMM6
        0xf3, 0x0f, 0x12, 0xf7, // MOVSLDUP XMM6, XMM7
        0xf3, 0x0f, 0x12, 0xf8, // MOVSLDUP XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_comprehensive_all_regs() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x16, 0xc1, // MOVSHDUP XMM0, XMM1
        0xf3, 0x0f, 0x16, 0xca, // MOVSHDUP XMM1, XMM2
        0xf3, 0x0f, 0x16, 0xd3, // MOVSHDUP XMM2, XMM3
        0xf3, 0x0f, 0x16, 0xdc, // MOVSHDUP XMM3, XMM4
        0xf3, 0x0f, 0x16, 0xe5, // MOVSHDUP XMM4, XMM5
        0xf3, 0x0f, 0x16, 0xee, // MOVSHDUP XMM5, XMM6
        0xf3, 0x0f, 0x16, 0xf7, // MOVSHDUP XMM6, XMM7
        0xf3, 0x0f, 0x16, 0xf8, // MOVSHDUP XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_movshdup_mixed_mem_reg() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x12, 0xc1, // MOVSLDUP XMM0, XMM1
        0xf3, 0x0f, 0x16, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSHDUP XMM2, [0x3000]
        0xf3, 0x0f, 0x12, 0xdb, // MOVSLDUP XMM3, XMM3
        0xf3, 0x0f, 0x16, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSHDUP XMM4, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
