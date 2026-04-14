use crate::*;

// CLFLUSHOPT - Optimized Cache Line Flush
//
// CLFLUSHOPT flushes a cache line from all levels of the processor cache hierarchy.
// It is similar to CLFLUSH but with optimizations:
// - Allows more flexible ordering (can be ordered with SFENCE)
// - May have better performance characteristics
// - Invalidates the cache line containing the specified memory address
//
// CLFLUSHOPT is weakly-ordered and can be ordered with:
// - SFENCE for stores
// - MFENCE for loads and stores
//
// Unlike CLFLUSH, CLFLUSHOPT does not serialize the processor pipeline by default.
//
// Opcode:
// 66 0F AE /7             CLFLUSHOPT m8          - Flush cache line (optimized)

const ADDR: u64 = 0x3000; // Address for testing

// ============================================================================
// CLFLUSHOPT Tests - Optimized Cache Line Flush
// ============================================================================

#[test]
fn test_clflushopt_basic() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_single_line() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_offset_1() {
    let mut emu = emu64();
    // CLFLUSHOPT with offset 1
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x10, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3010]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_offset_2() {
    let mut emu = emu64();
    // CLFLUSHOPT with offset 2
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x20, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3020]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_offset_3() {
    let mut emu = emu64();
    // CLFLUSHOPT with offset 3
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x40, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3040]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_sequential() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x40, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3040]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_multiple_lines() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x40, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3040]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x80, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3080]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_cache_line_aligned() {
    let mut emu = emu64();
    // CLFLUSHOPT on cache line boundary
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_unaligned() {
    let mut emu = emu64();
    // CLFLUSHOPT on unaligned address
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x01, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3001]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_with_sfence() {
    let mut emu = emu64();
    // CLFLUSHOPT followed by SFENCE
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_with_mfence() {
    let mut emu = emu64();
    // CLFLUSHOPT followed by MFENCE
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_ordering_1() {
    let mut emu = emu64();
    // CLFLUSHOPT ordering test 1
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_ordering_2() {
    let mut emu = emu64();
    // CLFLUSHOPT ordering test 2
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_invalidate() {
    let mut emu = emu64();
    // CLFLUSHOPT invalidating cache line
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_writeback() {
    let mut emu = emu64();
    // CLFLUSHOPT with writeback
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_all_levels() {
    let mut emu = emu64();
    // CLFLUSHOPT from all cache levels
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_performance() {
    let mut emu = emu64();
    // CLFLUSHOPT performance pattern
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_weakly_ordered() {
    let mut emu = emu64();
    // CLFLUSHOPT weak ordering
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_persistence() {
    let mut emu = emu64();
    // CLFLUSHOPT for persistence (PMEM)
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_modified_line() {
    let mut emu = emu64();
    // CLFLUSHOPT on modified cache line
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_shared_line() {
    let mut emu = emu64();
    // CLFLUSHOPT on shared cache line
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_exclusive_line() {
    let mut emu = emu64();
    // CLFLUSHOPT on exclusive cache line
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_range_1() {
    let mut emu = emu64();
    // CLFLUSHOPT range pattern 1
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x40, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3040]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_range_2() {
    let mut emu = emu64();
    // CLFLUSHOPT range pattern 2
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x40, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3040]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x80, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3080]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_buffer_flush() {
    let mut emu = emu64();
    // CLFLUSHOPT for buffer flush
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_data_sync() {
    let mut emu = emu64();
    // CLFLUSHOPT for data synchronization
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_coherency() {
    let mut emu = emu64();
    // CLFLUSHOPT for cache coherency
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_dma_buffer() {
    let mut emu = emu64();
    // CLFLUSHOPT for DMA buffer
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_io_coherency() {
    let mut emu = emu64();
    // CLFLUSHOPT for I/O coherency
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_large_buffer() {
    let mut emu = emu64();
    // CLFLUSHOPT for large buffer
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x40, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3040]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x80, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3080]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0xc0, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x30c0]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_streaming_data() {
    let mut emu = emu64();
    // CLFLUSHOPT for streaming data
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_memory_mapped_io() {
    let mut emu = emu64();
    // CLFLUSHOPT for memory-mapped I/O
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_cache_control() {
    let mut emu = emu64();
    // CLFLUSHOPT for cache control
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_batch_flush_1() {
    let mut emu = emu64();
    // CLFLUSHOPT batch flush pattern 1
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x40, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3040]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_batch_flush_2() {
    let mut emu = emu64();
    // CLFLUSHOPT batch flush pattern 2
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x40, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3040]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_nvdimm() {
    let mut emu = emu64();
    // CLFLUSHOPT for NVDIMM
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_persistent_memory() {
    let mut emu = emu64();
    // CLFLUSHOPT for persistent memory
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_transaction_log() {
    let mut emu = emu64();
    // CLFLUSHOPT for transaction log
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_durability() {
    let mut emu = emu64();
    // CLFLUSHOPT for durability guarantee
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_optimized_ordering() {
    let mut emu = emu64();
    // CLFLUSHOPT with optimized ordering
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x40, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3040]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x80, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3080]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_hierarchical_flush() {
    let mut emu = emu64();
    // CLFLUSHOPT hierarchical flush
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_bypass_cache() {
    let mut emu = emu64();
    // CLFLUSHOPT bypassing cache
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_memory_barrier() {
    let mut emu = emu64();
    // CLFLUSHOPT with memory barrier
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_clflushopt_store_barrier() {
    let mut emu = emu64();
    // CLFLUSHOPT with store barrier
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
