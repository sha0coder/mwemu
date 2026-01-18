use crate::*;

// MOVNTDQ - Store Packed Integers Using Non-Temporal Hint
//
// MOVNTDQ moves packed integers from XMM register to memory using non-temporal hint.
// The non-temporal hint minimizes cache pollution by using write combining (WC) protocol.
//
// Memory operand must be aligned on 16-byte boundary or #GP exception occurs.
// Use SFENCE or MFENCE for ordering with weakly-ordered memory types.
//
// Opcodes:
// 66 0F E7 /r             MOVNTDQ m128, xmm1     - Move packed integers from xmm1 to m128 using non-temporal hint

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing
const UNALIGNED_ADDR: u64 = 0x3001; // Unaligned address (should cause #GP)

// ============================================================================
// MOVNTDQ Tests - Non-Temporal Store of Packed Integers
// ============================================================================

#[test]
fn test_movntdq_mem_xmm0() {
    let mut emu = emu64();
    // MOVNTDQ [ALIGNED_ADDR], XMM0
    let code = [
        0x66, 0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_mem_xmm1() {
    let mut emu = emu64();
    // MOVNTDQ [ALIGNED_ADDR], XMM1
    let code = [
        0x66, 0x0f, 0xe7, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_mem_xmm2() {
    let mut emu = emu64();
    // MOVNTDQ [ALIGNED_ADDR], XMM2
    let code = [
        0x66, 0x0f, 0xe7, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_mem_xmm3() {
    let mut emu = emu64();
    // MOVNTDQ [ALIGNED_ADDR], XMM3
    let code = [
        0x66, 0x0f, 0xe7, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_mem_xmm4() {
    let mut emu = emu64();
    // MOVNTDQ [ALIGNED_ADDR], XMM4
    let code = [
        0x66, 0x0f, 0xe7, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_mem_xmm5() {
    let mut emu = emu64();
    // MOVNTDQ [ALIGNED_ADDR], XMM5
    let code = [
        0x66, 0x0f, 0xe7, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_mem_xmm6() {
    let mut emu = emu64();
    // MOVNTDQ [ALIGNED_ADDR], XMM6
    let code = [
        0x66, 0x0f, 0xe7, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_mem_xmm7() {
    let mut emu = emu64();
    // MOVNTDQ [ALIGNED_ADDR], XMM7
    let code = [
        0x66, 0x0f, 0xe7, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_mem_xmm8() {
    let mut emu = emu64();
    // MOVNTDQ [ALIGNED_ADDR], XMM8 (requires REX prefix)
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_mem_xmm9() {
    let mut emu = emu64();
    // MOVNTDQ [ALIGNED_ADDR], XMM9
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_mem_xmm10() {
    let mut emu = emu64();
    // MOVNTDQ [ALIGNED_ADDR], XMM10
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_mem_xmm11() {
    let mut emu = emu64();
    // MOVNTDQ [ALIGNED_ADDR], XMM11
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_mem_xmm12() {
    let mut emu = emu64();
    // MOVNTDQ [ALIGNED_ADDR], XMM12
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_mem_xmm13() {
    let mut emu = emu64();
    // MOVNTDQ [ALIGNED_ADDR], XMM13
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_mem_xmm14() {
    let mut emu = emu64();
    // MOVNTDQ [ALIGNED_ADDR], XMM14
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_mem_xmm15() {
    let mut emu = emu64();
    // MOVNTDQ [ALIGNED_ADDR], XMM15
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_aligned_16byte_boundary() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_aligned_32byte_boundary() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe7, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_data_integrity_zeros() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe7, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_data_integrity_ones() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe7, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_data_integrity_pattern() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe7, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_non_temporal_hint() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe7, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_sequential_stores() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe7, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_write_combining() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe7, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_with_different_addresses_1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_with_different_addresses_2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_memory_ordering() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_cache_bypass() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_weakly_ordered_memory() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntdq_full_128bit_data() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
