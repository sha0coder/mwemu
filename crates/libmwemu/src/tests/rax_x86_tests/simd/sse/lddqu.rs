use crate::*;

// LDDQU - Load Unaligned Integer 128 Bits
//
// Special load instruction for unaligned data that may cross cache line boundaries.
// Functionally similar to MOVDQU but may have better performance for cache line splits.
//
// Opcode:
// F2 0F F0 /r             LDDQU xmm1, mem    - Load unaligned integer 128 bits

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address
const UNALIGNED_ADDR: u64 = 0x3001; // Unaligned address

// ============================================================================
// LDDQU Tests - Load Unaligned Integer
// ============================================================================

#[test]
fn test_lddqu_xmm0_mem_aligned() {
    let mut emu = emu64();
    // LDDQU XMM0, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0xf0, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm1_mem_aligned() {
    let mut emu = emu64();
    // LDDQU XMM1, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0xf0, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm2_mem_aligned() {
    let mut emu = emu64();
    // LDDQU XMM2, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0xf0, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM2, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm3_mem_aligned() {
    let mut emu = emu64();
    // LDDQU XMM3, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0xf0, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM3, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm4_mem_aligned() {
    let mut emu = emu64();
    // LDDQU XMM4, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0xf0, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM4, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm5_mem_aligned() {
    let mut emu = emu64();
    // LDDQU XMM5, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0xf0, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM5, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm6_mem_aligned() {
    let mut emu = emu64();
    // LDDQU XMM6, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0xf0, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM6, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm7_mem_aligned() {
    let mut emu = emu64();
    // LDDQU XMM7, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0xf0, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm8_mem_aligned() {
    let mut emu = emu64();
    // LDDQU XMM8, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x44, 0x0f, 0xf0, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM8, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm9_mem_aligned() {
    let mut emu = emu64();
    // LDDQU XMM9, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x44, 0x0f, 0xf0, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM9, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm10_mem_aligned() {
    let mut emu = emu64();
    // LDDQU XMM10, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x44, 0x0f, 0xf0, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM10, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm11_mem_aligned() {
    let mut emu = emu64();
    // LDDQU XMM11, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x44, 0x0f, 0xf0, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM11, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm12_mem_aligned() {
    let mut emu = emu64();
    // LDDQU XMM12, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x44, 0x0f, 0xf0, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM12, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm13_mem_aligned() {
    let mut emu = emu64();
    // LDDQU XMM13, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x44, 0x0f, 0xf0, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM13, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm14_mem_aligned() {
    let mut emu = emu64();
    // LDDQU XMM14, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x44, 0x0f, 0xf0, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM14, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm15_mem_aligned() {
    let mut emu = emu64();
    // LDDQU XMM15, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x44, 0x0f, 0xf0, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Unaligned Access Tests
// ============================================================================

#[test]
fn test_lddqu_xmm0_mem_unaligned_1byte() {
    let mut emu = emu64();
    // LDDQU XMM0, [0x3001] - 1 byte offset
    let code = [
        0xf2, 0x0f, 0xf0, 0x04, 0x25, 0x01, 0x30, 0x00, 0x00, // LDDQU XMM0, [0x3001]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm1_mem_unaligned_2byte() {
    let mut emu = emu64();
    // LDDQU XMM1, [0x3002] - 2 byte offset
    let code = [
        0xf2, 0x0f, 0xf0, 0x0c, 0x25, 0x02, 0x30, 0x00, 0x00, // LDDQU XMM1, [0x3002]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm2_mem_unaligned_4byte() {
    let mut emu = emu64();
    // LDDQU XMM2, [0x3004] - 4 byte offset
    let code = [
        0xf2, 0x0f, 0xf0, 0x14, 0x25, 0x04, 0x30, 0x00, 0x00, // LDDQU XMM2, [0x3004]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm3_mem_unaligned_8byte() {
    let mut emu = emu64();
    // LDDQU XMM3, [0x3008] - 8 byte offset
    let code = [
        0xf2, 0x0f, 0xf0, 0x1c, 0x25, 0x08, 0x30, 0x00, 0x00, // LDDQU XMM3, [0x3008]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm4_mem_unaligned_odd() {
    let mut emu = emu64();
    // LDDQU XMM4, [0x3003] - odd offset
    let code = [
        0xf2, 0x0f, 0xf0, 0x24, 0x25, 0x03, 0x30, 0x00, 0x00, // LDDQU XMM4, [0x3003]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm5_mem_unaligned_5byte() {
    let mut emu = emu64();
    // LDDQU XMM5, [0x3005] - 5 byte offset
    let code = [
        0xf2, 0x0f, 0xf0, 0x2c, 0x25, 0x05, 0x30, 0x00, 0x00, // LDDQU XMM5, [0x3005]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm6_mem_unaligned_7byte() {
    let mut emu = emu64();
    // LDDQU XMM6, [0x3007] - 7 byte offset
    let code = [
        0xf2, 0x0f, 0xf0, 0x34, 0x25, 0x07, 0x30, 0x00, 0x00, // LDDQU XMM6, [0x3007]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_xmm7_mem_unaligned_15byte() {
    let mut emu = emu64();
    // LDDQU XMM7, [0x300f] - 15 byte offset (maximum misalignment)
    let code = [
        0xf2, 0x0f, 0xf0, 0x3c, 0x25, 0x0f, 0x30, 0x00, 0x00, // LDDQU XMM7, [0x300f]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Multiple Load Tests
// ============================================================================

#[test]
fn test_lddqu_multiple_loads() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0xf0, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM0, [0x3000]
        0xf2, 0x0f, 0xf0, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM1, [0x3000]
        0xf2, 0x0f, 0xf0, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM2, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_sequential_addresses() {
    let mut emu = emu64();
    // LDDQU from sequential memory addresses
    let code = [
        0xf2, 0x0f, 0xf0, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM0, [0x3000]
        0xf2, 0x0f, 0xf0, 0x0c, 0x25, 0x10, 0x30, 0x00, 0x00, // LDDQU XMM1, [0x3010]
        0xf2, 0x0f, 0xf0, 0x14, 0x25, 0x20, 0x30, 0x00, 0x00, // LDDQU XMM2, [0x3020]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_high_regs() {
    let mut emu = emu64();
    // LDDQU with high XMM registers
    let code = [
        0xf2, 0x44, 0x0f, 0xf0, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM8, [0x3000]
        0xf2, 0x44, 0x0f, 0xf0, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM10, [0x3000]
        0xf2, 0x44, 0x0f, 0xf0, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // LDDQU XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lddqu_cache_line_boundary() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0xf0, 0x04, 0x25, 0x38, 0x30, 0x00, 0x00, // LDDQU XMM0, [0x3038]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
