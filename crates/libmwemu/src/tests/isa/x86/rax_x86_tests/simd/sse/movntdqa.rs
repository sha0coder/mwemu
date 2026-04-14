use crate::*;

// MOVNTDQA - Load Double Quadword Non-Temporal Aligned Hint
//
// Streaming load instruction with non-temporal hint for WC (write-combining) memory.
// Used for efficient loading from write-combining memory regions.
// Memory must be 16-byte aligned.
//
// Opcode:
// 66 0F 38 2A /r          MOVNTDQA xmm1, m128    - Non-temporal aligned load

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address

// ============================================================================
// MOVNTDQA Tests - Non-Temporal Load
// ============================================================================

#[test]
fn test_movntdqa_xmm0_mem() {
    let mut emu = emu64();
    // MOVNTDQA XMM0, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0x2a, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_xmm1_mem() {
    let mut emu = emu64();
    // MOVNTDQA XMM1, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0x2a, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_xmm2_mem() {
    let mut emu = emu64();
    // MOVNTDQA XMM2, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0x2a, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM2, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_xmm3_mem() {
    let mut emu = emu64();
    // MOVNTDQA XMM3, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0x2a, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM3, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_xmm4_mem() {
    let mut emu = emu64();
    // MOVNTDQA XMM4, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0x2a, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM4, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_xmm5_mem() {
    let mut emu = emu64();
    // MOVNTDQA XMM5, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0x2a, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM5, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_xmm6_mem() {
    let mut emu = emu64();
    // MOVNTDQA XMM6, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0x2a, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM6, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_xmm7_mem() {
    let mut emu = emu64();
    // MOVNTDQA XMM7, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0x2a, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_xmm8_mem() {
    let mut emu = emu64();
    // MOVNTDQA XMM8, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x2a, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM8, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_xmm9_mem() {
    let mut emu = emu64();
    // MOVNTDQA XMM9, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x2a, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM9, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_xmm10_mem() {
    let mut emu = emu64();
    // MOVNTDQA XMM10, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x2a, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM10, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_xmm11_mem() {
    let mut emu = emu64();
    // MOVNTDQA XMM11, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x2a, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM11, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_xmm12_mem() {
    let mut emu = emu64();
    // MOVNTDQA XMM12, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x2a, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM12, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_xmm13_mem() {
    let mut emu = emu64();
    // MOVNTDQA XMM13, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x2a, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM13, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_xmm14_mem() {
    let mut emu = emu64();
    // MOVNTDQA XMM14, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x2a, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM14, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_xmm15_mem() {
    let mut emu = emu64();
    // MOVNTDQA XMM15, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x2a, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Alignment Tests
// ============================================================================

#[test]
fn test_movntdqa_aligned_0x3000() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x2a, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_aligned_0x3010() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x2a, 0x04, 0x25, 0x10, 0x30, 0x00, 0x00, // MOVNTDQA XMM0, [0x3010]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_aligned_0x3020() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x2a, 0x04, 0x25, 0x20, 0x30, 0x00, 0x00, // MOVNTDQA XMM0, [0x3020]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_aligned_0x3040() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x2a, 0x04, 0x25, 0x40, 0x30, 0x00, 0x00, // MOVNTDQA XMM0, [0x3040]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Sequential Load Tests
// ============================================================================

#[test]
fn test_movntdqa_sequential_loads() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x2a, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM0, [0x3000]
        0x66, 0x0f, 0x38, 0x2a, 0x0c, 0x25, 0x10, 0x30, 0x00, 0x00, // MOVNTDQA XMM1, [0x3010]
        0x66, 0x0f, 0x38, 0x2a, 0x14, 0x25, 0x20, 0x30, 0x00, 0x00, // MOVNTDQA XMM2, [0x3020]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_streaming_pattern() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x2a, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM0, [0x3000]
        0x66, 0x0f, 0x38, 0x2a, 0x0c, 0x25, 0x10, 0x30, 0x00, 0x00, // MOVNTDQA XMM1, [0x3010]
        0x66, 0x0f, 0x38, 0x2a, 0x14, 0x25, 0x20, 0x30, 0x00, 0x00, // MOVNTDQA XMM2, [0x3020]
        0x66, 0x0f, 0x38, 0x2a, 0x1c, 0x25, 0x30, 0x30, 0x00, 0x00, // MOVNTDQA XMM3, [0x3030]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_multiple_same_addr() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x2a, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM0, [0x3000]
        0x66, 0x0f, 0x38, 0x2a, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM1, [0x3000]
        0x66, 0x0f, 0x38, 0x2a, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM2, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_high_registers() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x2a, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM8, [0x3000]
        0x66, 0x44, 0x0f, 0x38, 0x2a, 0x14, 0x25, 0x10, 0x30, 0x00, 0x00, // MOVNTDQA XMM10, [0x3010]
        0x66, 0x44, 0x0f, 0x38, 0x2a, 0x3c, 0x25, 0x20, 0x30, 0x00, 0x00, // MOVNTDQA XMM15, [0x3020]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_all_registers() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x2a, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM0, [0x3000]
        0x66, 0x0f, 0x38, 0x2a, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM1, [0x3000]
        0x66, 0x0f, 0x38, 0x2a, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM2, [0x3000]
        0x66, 0x0f, 0x38, 0x2a, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM3, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdqa_cache_line_streaming() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x2a, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQA XMM0, [0x3000]
        0x66, 0x0f, 0x38, 0x2a, 0x0c, 0x25, 0x40, 0x30, 0x00, 0x00, // MOVNTDQA XMM1, [0x3040]
        0x66, 0x0f, 0x38, 0x2a, 0x14, 0x25, 0x80, 0x30, 0x00, 0x00, // MOVNTDQA XMM2, [0x3080]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
