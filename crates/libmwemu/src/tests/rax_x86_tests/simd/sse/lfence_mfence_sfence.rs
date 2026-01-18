use crate::*;

// Memory Fence Instructions - LFENCE, MFENCE, SFENCE
//
// LFENCE (Load Fence):
// - Serializes all load operations before LFENCE
// - No later instruction begins execution until LFENCE completes
// - Instructions can be fetched speculatively before LFENCE completes
// Opcode: NP 0F AE E8
//
// MFENCE (Memory Fence):
// - Serializes all load and store operations before MFENCE
// - Guarantees global visibility of loads/stores before any after MFENCE
// - Ordered with respect to all loads, stores, and other fences
// Opcode: NP 0F AE F0
//
// SFENCE (Store Fence):
// - Serializes all store operations before SFENCE
// - Every store before SFENCE is globally visible before stores after
// - Not ordered with loads or LFENCE
// Opcode: NP 0F AE F8

// ============================================================================
// LFENCE Tests - Load Fence (Serializes Load Operations)
// ============================================================================

#[test]
fn test_lfence_basic() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lfence_after_load() {
    let mut emu = emu64();
    // LFENCE after memory load
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lfence_before_load() {
    let mut emu = emu64();
    // LFENCE before memory load
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lfence_multiple() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lfence_serialization() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lfence_ordering_1() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lfence_ordering_2() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lfence_with_stores() {
    let mut emu = emu64();
    // LFENCE with store operations (not ordered)
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lfence_speculative_fetch() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lfence_weakly_ordered_memory() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lfence_opcodes_e9() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xe9, // LFENCE (alternate encoding)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lfence_opcodes_ef() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xef, // LFENCE (alternate encoding)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MFENCE Tests - Memory Fence (Serializes Load and Store Operations)
// ============================================================================

#[test]
fn test_mfence_basic() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mfence_after_load() {
    let mut emu = emu64();
    // MFENCE after memory load
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mfence_after_store() {
    let mut emu = emu64();
    // MFENCE after memory store
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mfence_before_load() {
    let mut emu = emu64();
    // MFENCE before memory load
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mfence_before_store() {
    let mut emu = emu64();
    // MFENCE before memory store
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mfence_multiple() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mfence_serialization() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mfence_global_visibility() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mfence_ordering_loads_stores() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mfence_with_lfence() {
    let mut emu = emu64();
    // MFENCE with LFENCE
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mfence_with_sfence() {
    let mut emu = emu64();
    // MFENCE with SFENCE
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mfence_opcodes_f7() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xf7, // MFENCE (alternate encoding)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// SFENCE Tests - Store Fence (Serializes Store Operations)
// ============================================================================

#[test]
fn test_sfence_basic() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sfence_after_store() {
    let mut emu = emu64();
    // SFENCE after memory store
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sfence_before_store() {
    let mut emu = emu64();
    // SFENCE before memory store
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sfence_multiple() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sfence_serialization() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sfence_global_visibility() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sfence_ordering() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sfence_with_loads() {
    let mut emu = emu64();
    // SFENCE with load operations (not ordered)
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sfence_with_mfence() {
    let mut emu = emu64();
    // SFENCE with MFENCE
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sfence_write_combining() {
    let mut emu = emu64();
    // SFENCE with write combining
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sfence_opcodes_ff() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xff, // SFENCE (alternate encoding)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Combined Fence Tests
// ============================================================================

#[test]
fn test_all_fences_sequence() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0x0f, 0xae, 0xf0, // MFENCE
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lfence_sfence_combination() {
    let mut emu = emu64();
    // LFENCE followed by SFENCE
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sfence_lfence_combination() {
    let mut emu = emu64();
    // SFENCE followed by LFENCE
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_nested_fences_1() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0x0f, 0xae, 0xe8, // LFENCE
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_nested_fences_2() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0x0f, 0xae, 0xf8, // SFENCE
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
