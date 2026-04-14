use crate::*;

// MOVNTQ - Store Packed Integers Using Non-Temporal Hint (MMX)
//
// MOVNTQ moves packed integers from MMX register to memory using non-temporal hint.
// The non-temporal hint minimizes cache pollution by using write combining (WC) protocol.
//
// Memory operand must be aligned on 8-byte boundary or #GP exception may occur (model-specific).
// Use SFENCE or MFENCE for ordering with weakly-ordered memory types.
//
// Opcodes:
// NP 0F E7 /r             MOVNTQ m64, mm         - Move packed integers from mm to m64 using non-temporal hint

const ALIGNED_ADDR: u64 = 0x3000; // 8-byte aligned address for testing

// ============================================================================
// MOVNTQ Tests - Non-Temporal Store of Packed Integers (MMX)
// ============================================================================

#[test]
fn test_movntq_mem_mm0() {
    let mut emu = emu64();
    // MOVNTQ [ALIGNED_ADDR], MM0
    let code = [
        0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_mem_mm1() {
    let mut emu = emu64();
    // MOVNTQ [ALIGNED_ADDR], MM1
    let code = [
        0x0f, 0xe7, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_mem_mm2() {
    let mut emu = emu64();
    // MOVNTQ [ALIGNED_ADDR], MM2
    let code = [
        0x0f, 0xe7, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_mem_mm3() {
    let mut emu = emu64();
    // MOVNTQ [ALIGNED_ADDR], MM3
    let code = [
        0x0f, 0xe7, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_mem_mm4() {
    let mut emu = emu64();
    // MOVNTQ [ALIGNED_ADDR], MM4
    let code = [
        0x0f, 0xe7, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_mem_mm5() {
    let mut emu = emu64();
    // MOVNTQ [ALIGNED_ADDR], MM5
    let code = [
        0x0f, 0xe7, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_mem_mm6() {
    let mut emu = emu64();
    // MOVNTQ [ALIGNED_ADDR], MM6
    let code = [
        0x0f, 0xe7, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_mem_mm7() {
    let mut emu = emu64();
    // MOVNTQ [ALIGNED_ADDR], MM7
    let code = [
        0x0f, 0xe7, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_aligned_8byte_boundary() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_aligned_16byte_boundary() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_data_integrity_zeros() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_data_integrity_ones() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_data_integrity_pattern() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_non_temporal_hint() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_sequential_stores() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM0
        0x0f, 0xe7, 0x0c, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVNTQ [0x3008], MM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_write_combining() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_with_different_addresses_1() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_with_different_addresses_2() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x04, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVNTQ [0x3008], MM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_with_different_addresses_3() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x0c, 0x25, 0x10, 0x30, 0x00, 0x00, // MOVNTQ [0x3010], MM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_memory_ordering() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_cache_bypass() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_weakly_ordered_memory() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_full_64bit_data() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_with_sfence() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM6
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_with_mfence() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM7
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_multiple_sequential() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM0
        0x0f, 0xe7, 0x0c, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVNTQ [0x3008], MM1
        0x0f, 0xe7, 0x14, 0x25, 0x10, 0x30, 0x00, 0x00, // MOVNTQ [0x3010], MM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_multiple_with_sfence() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM3
        0x0f, 0xe7, 0x24, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVNTQ [0x3008], MM4
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_packed_byte_data() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_packed_word_data() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_packed_dword_data() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_streaming_store() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM0
        0x0f, 0xe7, 0x0c, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVNTQ [0x3008], MM1
        0x0f, 0xe7, 0x14, 0x25, 0x10, 0x30, 0x00, 0x00, // MOVNTQ [0x3010], MM2
        0x0f, 0xe7, 0x1c, 0x25, 0x18, 0x30, 0x00, 0x00, // MOVNTQ [0x3018], MM3
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_high_throughput() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x24, 0x25, 0x20, 0x30, 0x00, 0x00, // MOVNTQ [0x3020], MM4
        0x0f, 0xe7, 0x2c, 0x25, 0x28, 0x30, 0x00, 0x00, // MOVNTQ [0x3028], MM5
        0x0f, 0xe7, 0x34, 0x25, 0x30, 0x30, 0x00, 0x00, // MOVNTQ [0x3030], MM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_large_buffer() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM7
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_buffer_fill() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM0
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_write_bypass() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_mixed_with_regular_stores() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM2
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_temporal_locality() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_write_through() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movntq_performance_optimization() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xe7, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM5
        0x0f, 0xe7, 0x34, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVNTQ [0x3008], MM6
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
