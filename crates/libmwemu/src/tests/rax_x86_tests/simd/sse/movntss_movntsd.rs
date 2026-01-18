use crate::*;

// MOVNTSS/MOVNTSD - Store Scalar Single/Double Precision Floating-Point Using Non-Temporal Hint
//
// Note: MOVNTSS and MOVNTSD are part of SSE4A (AMD extension).
// These instructions store scalar floating-point values using non-temporal hint.
// The non-temporal hint minimizes cache pollution.
//
// Memory operand alignment requirements may vary by implementation.
// Use SFENCE or MFENCE for ordering with weakly-ordered memory types.
//
// Opcodes:
// F3 0F 2B /r             MOVNTSS m32, xmm1      - Move scalar single-precision from xmm1 to m32 using non-temporal hint
// F2 0F 2B /r             MOVNTSD m64, xmm1      - Move scalar double-precision from xmm1 to m64 using non-temporal hint

const ADDR: u64 = 0x3000; // Address for testing

// ============================================================================
// MOVNTSS Tests - Non-Temporal Store of Scalar Single-Precision
// ============================================================================

#[test]
fn test_movntss_mem_xmm0() {
    let mut emu = emu64();
    // MOVNTSS [ADDR], XMM0
    let code = [
        0xf3, 0x0f, 0x2b, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntss_mem_xmm1() {
    let mut emu = emu64();
    // MOVNTSS [ADDR], XMM1
    let code = [
        0xf3, 0x0f, 0x2b, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntss_mem_xmm2() {
    let mut emu = emu64();
    // MOVNTSS [ADDR], XMM2
    let code = [
        0xf3, 0x0f, 0x2b, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntss_mem_xmm3() {
    let mut emu = emu64();
    // MOVNTSS [ADDR], XMM3
    let code = [
        0xf3, 0x0f, 0x2b, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntss_mem_xmm4() {
    let mut emu = emu64();
    // MOVNTSS [ADDR], XMM4
    let code = [
        0xf3, 0x0f, 0x2b, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntss_mem_xmm5() {
    let mut emu = emu64();
    // MOVNTSS [ADDR], XMM5
    let code = [
        0xf3, 0x0f, 0x2b, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntss_mem_xmm6() {
    let mut emu = emu64();
    // MOVNTSS [ADDR], XMM6
    let code = [
        0xf3, 0x0f, 0x2b, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntss_mem_xmm7() {
    let mut emu = emu64();
    // MOVNTSS [ADDR], XMM7
    let code = [
        0xf3, 0x0f, 0x2b, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntss_mem_xmm8() {
    let mut emu = emu64();
    // MOVNTSS [ADDR], XMM8 (requires REX prefix)
    let code = [
        0xf3, 0x44, 0x0f, 0x2b, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntss_mem_xmm9() {
    let mut emu = emu64();
    // MOVNTSS [ADDR], XMM9
    let code = [
        0xf3, 0x44, 0x0f, 0x2b, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntss_mem_xmm10() {
    let mut emu = emu64();
    // MOVNTSS [ADDR], XMM10
    let code = [
        0xf3, 0x44, 0x0f, 0x2b, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntss_mem_xmm11() {
    let mut emu = emu64();
    // MOVNTSS [ADDR], XMM11
    let code = [
        0xf3, 0x44, 0x0f, 0x2b, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntss_mem_xmm12() {
    let mut emu = emu64();
    // MOVNTSS [ADDR], XMM12
    let code = [
        0xf3, 0x44, 0x0f, 0x2b, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntss_mem_xmm13() {
    let mut emu = emu64();
    // MOVNTSS [ADDR], XMM13
    let code = [
        0xf3, 0x44, 0x0f, 0x2b, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntss_mem_xmm14() {
    let mut emu = emu64();
    // MOVNTSS [ADDR], XMM14
    let code = [
        0xf3, 0x44, 0x0f, 0x2b, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntss_mem_xmm15() {
    let mut emu = emu64();
    // MOVNTSS [ADDR], XMM15
    let code = [
        0xf3, 0x44, 0x0f, 0x2b, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MOVNTSD Tests - Non-Temporal Store of Scalar Double-Precision
// ============================================================================

#[test]
fn test_movntsd_mem_xmm0() {
    let mut emu = emu64();
    // MOVNTSD [ADDR], XMM0
    let code = [
        0xf2, 0x0f, 0x2b, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_mem_xmm1() {
    let mut emu = emu64();
    // MOVNTSD [ADDR], XMM1
    let code = [
        0xf2, 0x0f, 0x2b, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_mem_xmm2() {
    let mut emu = emu64();
    // MOVNTSD [ADDR], XMM2
    let code = [
        0xf2, 0x0f, 0x2b, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_mem_xmm3() {
    let mut emu = emu64();
    // MOVNTSD [ADDR], XMM3
    let code = [
        0xf2, 0x0f, 0x2b, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_mem_xmm4() {
    let mut emu = emu64();
    // MOVNTSD [ADDR], XMM4
    let code = [
        0xf2, 0x0f, 0x2b, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_mem_xmm5() {
    let mut emu = emu64();
    // MOVNTSD [ADDR], XMM5
    let code = [
        0xf2, 0x0f, 0x2b, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_mem_xmm6() {
    let mut emu = emu64();
    // MOVNTSD [ADDR], XMM6
    let code = [
        0xf2, 0x0f, 0x2b, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_mem_xmm7() {
    let mut emu = emu64();
    // MOVNTSD [ADDR], XMM7
    let code = [
        0xf2, 0x0f, 0x2b, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_mem_xmm8() {
    let mut emu = emu64();
    // MOVNTSD [ADDR], XMM8 (requires REX prefix)
    let code = [
        0xf2, 0x44, 0x0f, 0x2b, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_mem_xmm9() {
    let mut emu = emu64();
    // MOVNTSD [ADDR], XMM9
    let code = [
        0xf2, 0x44, 0x0f, 0x2b, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_mem_xmm10() {
    let mut emu = emu64();
    // MOVNTSD [ADDR], XMM10
    let code = [
        0xf2, 0x44, 0x0f, 0x2b, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_mem_xmm11() {
    let mut emu = emu64();
    // MOVNTSD [ADDR], XMM11
    let code = [
        0xf2, 0x44, 0x0f, 0x2b, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_mem_xmm12() {
    let mut emu = emu64();
    // MOVNTSD [ADDR], XMM12
    let code = [
        0xf2, 0x44, 0x0f, 0x2b, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_mem_xmm13() {
    let mut emu = emu64();
    // MOVNTSD [ADDR], XMM13
    let code = [
        0xf2, 0x44, 0x0f, 0x2b, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_mem_xmm14() {
    let mut emu = emu64();
    // MOVNTSD [ADDR], XMM14
    let code = [
        0xf2, 0x44, 0x0f, 0x2b, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_mem_xmm15() {
    let mut emu = emu64();
    // MOVNTSD [ADDR], XMM15
    let code = [
        0xf2, 0x44, 0x0f, 0x2b, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional Tests - Non-Temporal Scalar Stores
// ============================================================================

#[test]
fn test_movntss_sequential_stores() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x2b, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM0
        0xf3, 0x0f, 0x2b, 0x0c, 0x25, 0x04, 0x30, 0x00, 0x00, // MOVNTSS [0x3004], XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_sequential_stores() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x2b, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM0
        0xf2, 0x0f, 0x2b, 0x0c, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVNTSD [0x3008], XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntss_with_sfence() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x2b, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM2
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_with_sfence() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x2b, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM2
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntss_with_mfence() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x2b, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM3
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_with_mfence() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x2b, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM3
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntss_cache_bypass() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x2b, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_cache_bypass() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x2b, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntss_memory_ordering() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x2b, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSS [0x3000], XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntsd_memory_ordering() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x2b, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTSD [0x3000], XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
