use crate::*;

// AVX-512 Compress and Expand Instructions
//
// VCOMPRESSPD - Store Sparse Packed Double-Precision Floating-Point Values
// VCOMPRESSPS - Store Sparse Packed Single-Precision Floating-Point Values
// VEXPANDPD - Load Sparse Packed Double-Precision Floating-Point Values
// VEXPANDPS - Load Sparse Packed Single-Precision Floating-Point Values
//
// These instructions compress or expand vector data using opmask registers
// Compress: Pack active elements to the low end, controlled by opmask
// Expand: Unpack elements from memory/register to positions indicated by opmask
//
// Encodings:
//   EVEX.128.66.0F38.W1 8A /r   VCOMPRESSPD xmm1/m128{k1}{z}, xmm2
//   EVEX.256.66.0F38.W1 8A /r   VCOMPRESSPD ymm1/m256{k1}{z}, ymm2
//   EVEX.512.66.0F38.W1 8A /r   VCOMPRESSPD zmm1/m512{k1}{z}, zmm2
//
//   EVEX.128.66.0F38.W0 8A /r   VCOMPRESSPS xmm1/m128{k1}{z}, xmm2
//   EVEX.256.66.0F38.W0 8A /r   VCOMPRESSPS ymm1/m256{k1}{z}, ymm2
//   EVEX.512.66.0F38.W0 8A /r   VCOMPRESSPS zmm1/m512{k1}{z}, zmm2
//
//   EVEX.128.66.0F38.W1 88 /r   VEXPANDPD xmm1{k1}{z}, xmm2/m128
//   EVEX.256.66.0F38.W1 88 /r   VEXPANDPD ymm1{k1}{z}, ymm2/m256
//   EVEX.512.66.0F38.W1 88 /r   VEXPANDPD zmm1{k1}{z}, zmm2/m512
//
//   EVEX.128.66.0F38.W0 88 /r   VEXPANDPS xmm1{k1}{z}, xmm2/m128
//   EVEX.256.66.0F38.W0 88 /r   VEXPANDPS ymm1{k1}{z}, ymm2/m256
//   EVEX.512.66.0F38.W0 88 /r   VEXPANDPS zmm1{k1}{z}, zmm2/m512

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VCOMPRESSPD XMM Tests (128-bit, 2 doubles)
// ============================================================================

#[test]
fn test_vcompresspd_xmm_all_active() {
    let mut emu = emu64();
    // VCOMPRESSPD with all mask bits set (no compression)
    // K1 = 0b11 (both elements active)
    let code = [
        // Setup mask K1 = 0b11
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VCOMPRESSPD XMM0{k1}, XMM1
        0x62, 0xf2, 0xfd, 0x09, 0x8a, 0xc1, // VCOMPRESSPD XMM0{k1}, XMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcompresspd_xmm_partial_mask() {
    let mut emu = emu64();
    // VCOMPRESSPD with partial mask (only first element)
    // K1 = 0b01
    let code = [
        // Setup mask K1 = 0b01
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VCOMPRESSPD XMM0{k1}, XMM1
        0x62, 0xf2, 0xfd, 0x09, 0x8a, 0xc1, // VCOMPRESSPD XMM0{k1}, XMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcompresspd_xmm_zeroing() {
    let mut emu = emu64();
    // VCOMPRESSPD with zeroing (inactive elements zeroed)
    // K1 = 0b01, z=1
    let code = [
        // Setup mask K1 = 0b01
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VCOMPRESSPD XMM0{k1}{z}, XMM1
        0x62, 0xf2, 0xfd, 0x89, 0x8a, 0xc1, // VCOMPRESSPD XMM0{k1}{z}, XMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcompresspd_xmm_to_memory() {
    let mut emu = emu64();
    // VCOMPRESSPD to memory destination
    let code = [
        // Setup mask K1 = 0b11
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VCOMPRESSPD [0x3000]{k1}, XMM1
        0x62, 0xf2, 0xfd, 0x09, 0x8a, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // VCOMPRESSPD [0x3000]{k1}, XMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCOMPRESSPD YMM Tests (256-bit, 4 doubles)
// ============================================================================

#[test]
fn test_vcompresspd_ymm_all_active() {
    let mut emu = emu64();
    // K1 = 0b1111 (all 4 elements active)
    let code = [
        // Setup mask K1 = 0b1111
        0xb8, 0x0f, 0x00, 0x00, 0x00, // MOV EAX, 15
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VCOMPRESSPD YMM0{k1}, YMM1
        0x62, 0xf2, 0xfd, 0x29, 0x8a, 0xc1, // VCOMPRESSPD YMM0{k1}, YMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcompresspd_ymm_sparse_mask() {
    let mut emu = emu64();
    // K1 = 0b1010 (elements 1 and 3)
    let code = [
        // Setup mask K1 = 0b1010
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VCOMPRESSPD YMM0{k1}, YMM1
        0x62, 0xf2, 0xfd, 0x29, 0x8a, 0xc1, // VCOMPRESSPD YMM0{k1}, YMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcompresspd_ymm_alternating() {
    let mut emu = emu64();
    // K1 = 0b0101 (alternating pattern)
    let code = [
        // Setup mask K1 = 0b0101
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VCOMPRESSPD YMM0{k1}{z}, YMM1
        0x62, 0xf2, 0xfd, 0xa9, 0x8a, 0xc1, // VCOMPRESSPD YMM0{k1}{z}, YMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCOMPRESSPD ZMM Tests (512-bit, 8 doubles)
// ============================================================================

#[test]
fn test_vcompresspd_zmm_all_active() {
    let mut emu = emu64();
    // K1 = 0xFF (all 8 elements active)
    let code = [
        // Setup mask K1 = 0xFF
        0xb8, 0xff, 0x00, 0x00, 0x00, // MOV EAX, 255
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VCOMPRESSPD ZMM0{k1}, ZMM1
        0x62, 0xf2, 0xfd, 0x49, 0x8a, 0xc1, // VCOMPRESSPD ZMM0{k1}, ZMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcompresspd_zmm_partial() {
    let mut emu = emu64();
    // K1 = 0b11110000 (upper 4 elements only)
    let code = [
        // Setup mask K1 = 0xF0
        0xb8, 0xf0, 0x00, 0x00, 0x00, // MOV EAX, 240
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VCOMPRESSPD ZMM0{k1}, ZMM1
        0x62, 0xf2, 0xfd, 0x49, 0x8a, 0xc1, // VCOMPRESSPD ZMM0{k1}, ZMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCOMPRESSPS XMM Tests (128-bit, 4 floats)
// ============================================================================

#[test]
fn test_vcompressps_xmm_all_active() {
    let mut emu = emu64();
    // K1 = 0b1111 (all 4 elements active)
    let code = [
        // Setup mask K1 = 0b1111
        0xb8, 0x0f, 0x00, 0x00, 0x00, // MOV EAX, 15
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VCOMPRESSPS XMM0{k1}, XMM1
        0x62, 0xf2, 0x7d, 0x09, 0x8a, 0xc1, // VCOMPRESSPS XMM0{k1}, XMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcompressps_xmm_first_two() {
    let mut emu = emu64();
    // K1 = 0b0011 (first two elements)
    let code = [
        // Setup mask K1 = 0b0011
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VCOMPRESSPS XMM0{k1}, XMM1
        0x62, 0xf2, 0x7d, 0x09, 0x8a, 0xc1, // VCOMPRESSPS XMM0{k1}, XMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcompressps_xmm_to_memory() {
    let mut emu = emu64();
    // VCOMPRESSPS to memory
    let code = [
        // Setup mask K1 = 0b1111
        0xb8, 0x0f, 0x00, 0x00, 0x00, // MOV EAX, 15
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VCOMPRESSPS [0x3000]{k1}, XMM1
        0x62, 0xf2, 0x7d, 0x09, 0x8a, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // VCOMPRESSPS [0x3000]{k1}, XMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCOMPRESSPS YMM Tests (256-bit, 8 floats)
// ============================================================================

#[test]
fn test_vcompressps_ymm_all_active() {
    let mut emu = emu64();
    // K1 = 0xFF (all 8 elements)
    let code = [
        // Setup mask K1 = 0xFF
        0xb8, 0xff, 0x00, 0x00, 0x00, // MOV EAX, 255
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VCOMPRESSPS YMM0{k1}, YMM1
        0x62, 0xf2, 0x7d, 0x29, 0x8a, 0xc1, // VCOMPRESSPS YMM0{k1}, YMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcompressps_ymm_even_elements() {
    let mut emu = emu64();
    // K1 = 0b10101010 (even elements only)
    let code = [
        // Setup mask K1 = 0xAA
        0xb8, 0xaa, 0x00, 0x00, 0x00, // MOV EAX, 170
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VCOMPRESSPS YMM0{k1}, YMM1
        0x62, 0xf2, 0x7d, 0x29, 0x8a, 0xc1, // VCOMPRESSPS YMM0{k1}, YMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCOMPRESSPS ZMM Tests (512-bit, 16 floats)
// ============================================================================

#[test]
fn test_vcompressps_zmm_all_active() {
    let mut emu = emu64();
    // K1 = 0xFFFF (all 16 elements)
    let code = [
        // Setup mask K1 = 0xFFFF
        0xb8, 0xff, 0xff, 0x00, 0x00, // MOV EAX, 65535
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VCOMPRESSPS ZMM0{k1}, ZMM1
        0x62, 0xf2, 0x7d, 0x49, 0x8a, 0xc1, // VCOMPRESSPS ZMM0{k1}, ZMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcompressps_zmm_sparse() {
    let mut emu = emu64();
    // K1 = 0x5555 (alternating pattern)
    let code = [
        // Setup mask K1 = 0x5555
        0xb8, 0x55, 0x55, 0x00, 0x00, // MOV EAX, 21845
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VCOMPRESSPS ZMM0{k1}, ZMM1
        0x62, 0xf2, 0x7d, 0x49, 0x8a, 0xc1, // VCOMPRESSPS ZMM0{k1}, ZMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VEXPANDPD XMM Tests (128-bit, 2 doubles)
// ============================================================================

#[test]
fn test_vexpandpd_xmm_all_active() {
    let mut emu = emu64();
    // VEXPANDPD with all mask bits set
    // K1 = 0b11
    let code = [
        // Setup mask K1 = 0b11
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VEXPANDPD XMM0{k1}, XMM1
        0x62, 0xf2, 0xfd, 0x09, 0x88, 0xc1, // VEXPANDPD XMM0{k1}, XMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vexpandpd_xmm_partial() {
    let mut emu = emu64();
    // K1 = 0b10 (second element only)
    let code = [
        // Setup mask K1 = 0b10
        0xb8, 0x02, 0x00, 0x00, 0x00, // MOV EAX, 2
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VEXPANDPD XMM0{k1}, XMM1
        0x62, 0xf2, 0xfd, 0x09, 0x88, 0xc1, // VEXPANDPD XMM0{k1}, XMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vexpandpd_xmm_from_memory() {
    let mut emu = emu64();
    // VEXPANDPD from memory source
    let code = [
        // Setup mask K1 = 0b11
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VEXPANDPD XMM0{k1}, [0x3000]
        0x62, 0xf2, 0xfd, 0x09, 0x88, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VEXPANDPD XMM0{k1}, [0x3000]

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vexpandpd_xmm_zeroing() {
    let mut emu = emu64();
    // VEXPANDPD with zeroing
    // K1 = 0b01, z=1
    let code = [
        // Setup mask K1 = 0b01
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VEXPANDPD XMM0{k1}{z}, XMM1
        0x62, 0xf2, 0xfd, 0x89, 0x88, 0xc1, // VEXPANDPD XMM0{k1}{z}, XMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VEXPANDPD YMM Tests (256-bit, 4 doubles)
// ============================================================================

#[test]
fn test_vexpandpd_ymm_all_active() {
    let mut emu = emu64();
    // K1 = 0b1111
    let code = [
        // Setup mask K1 = 0b1111
        0xb8, 0x0f, 0x00, 0x00, 0x00, // MOV EAX, 15
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VEXPANDPD YMM0{k1}, YMM1
        0x62, 0xf2, 0xfd, 0x29, 0x88, 0xc1, // VEXPANDPD YMM0{k1}, YMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vexpandpd_ymm_alternating() {
    let mut emu = emu64();
    // K1 = 0b0101
    let code = [
        // Setup mask K1 = 0b0101
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VEXPANDPD YMM0{k1}, YMM1
        0x62, 0xf2, 0xfd, 0x29, 0x88, 0xc1, // VEXPANDPD YMM0{k1}, YMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VEXPANDPD ZMM Tests (512-bit, 8 doubles)
// ============================================================================

#[test]
fn test_vexpandpd_zmm_all_active() {
    let mut emu = emu64();
    // K1 = 0xFF
    let code = [
        // Setup mask K1 = 0xFF
        0xb8, 0xff, 0x00, 0x00, 0x00, // MOV EAX, 255
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VEXPANDPD ZMM0{k1}, ZMM1
        0x62, 0xf2, 0xfd, 0x49, 0x88, 0xc1, // VEXPANDPD ZMM0{k1}, ZMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vexpandpd_zmm_sparse() {
    let mut emu = emu64();
    // K1 = 0b10110100
    let code = [
        // Setup mask K1 = 0xB4
        0xb8, 0xb4, 0x00, 0x00, 0x00, // MOV EAX, 180
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VEXPANDPD ZMM0{k1}, ZMM1
        0x62, 0xf2, 0xfd, 0x49, 0x88, 0xc1, // VEXPANDPD ZMM0{k1}, ZMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VEXPANDPS XMM Tests (128-bit, 4 floats)
// ============================================================================

#[test]
fn test_vexpandps_xmm_all_active() {
    let mut emu = emu64();
    // K1 = 0b1111
    let code = [
        // Setup mask K1 = 0b1111
        0xb8, 0x0f, 0x00, 0x00, 0x00, // MOV EAX, 15
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VEXPANDPS XMM0{k1}, XMM1
        0x62, 0xf2, 0x7d, 0x09, 0x88, 0xc1, // VEXPANDPS XMM0{k1}, XMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vexpandps_xmm_odd_elements() {
    let mut emu = emu64();
    // K1 = 0b1010 (odd elements)
    let code = [
        // Setup mask K1 = 0b1010
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VEXPANDPS XMM0{k1}, XMM1
        0x62, 0xf2, 0x7d, 0x09, 0x88, 0xc1, // VEXPANDPS XMM0{k1}, XMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vexpandps_xmm_from_memory() {
    let mut emu = emu64();
    // VEXPANDPS from memory
    let code = [
        // Setup mask K1 = 0b1111
        0xb8, 0x0f, 0x00, 0x00, 0x00, // MOV EAX, 15
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VEXPANDPS XMM0{k1}, [0x3000]
        0x62, 0xf2, 0x7d, 0x09, 0x88, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VEXPANDPS XMM0{k1}, [0x3000]

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VEXPANDPS YMM Tests (256-bit, 8 floats)
// ============================================================================

#[test]
fn test_vexpandps_ymm_all_active() {
    let mut emu = emu64();
    // K1 = 0xFF
    let code = [
        // Setup mask K1 = 0xFF
        0xb8, 0xff, 0x00, 0x00, 0x00, // MOV EAX, 255
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VEXPANDPS YMM0{k1}, YMM1
        0x62, 0xf2, 0x7d, 0x29, 0x88, 0xc1, // VEXPANDPS YMM0{k1}, YMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vexpandps_ymm_pattern() {
    let mut emu = emu64();
    // K1 = 0b11001100
    let code = [
        // Setup mask K1 = 0xCC
        0xb8, 0xcc, 0x00, 0x00, 0x00, // MOV EAX, 204
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VEXPANDPS YMM0{k1}, YMM1
        0x62, 0xf2, 0x7d, 0x29, 0x88, 0xc1, // VEXPANDPS YMM0{k1}, YMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VEXPANDPS ZMM Tests (512-bit, 16 floats)
// ============================================================================

#[test]
fn test_vexpandps_zmm_all_active() {
    let mut emu = emu64();
    // K1 = 0xFFFF
    let code = [
        // Setup mask K1 = 0xFFFF
        0xb8, 0xff, 0xff, 0x00, 0x00, // MOV EAX, 65535
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VEXPANDPS ZMM0{k1}, ZMM1
        0x62, 0xf2, 0x7d, 0x49, 0x88, 0xc1, // VEXPANDPS ZMM0{k1}, ZMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vexpandps_zmm_checkerboard() {
    let mut emu = emu64();
    // K1 = 0xAAAA (alternating pattern)
    let code = [
        // Setup mask K1 = 0xAAAA
        0xb8, 0xaa, 0xaa, 0x00, 0x00, // MOV EAX, 43690
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VEXPANDPS ZMM0{k1}, ZMM1
        0x62, 0xf2, 0x7d, 0x49, 0x88, 0xc1, // VEXPANDPS ZMM0{k1}, ZMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Combined Compress/Expand Tests
// ============================================================================

#[test]
fn test_compress_then_expand_roundtrip() {
    let mut emu = emu64();
    let code = [
        // Setup mask K1 = 0xFF
        0xb8, 0xff, 0x00, 0x00, 0x00, // MOV EAX, 255
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VCOMPRESSPS YMM2{k1}, YMM1
        0x62, 0xf2, 0x7d, 0x29, 0x8a, 0xd1, // VCOMPRESSPS YMM2{k1}, YMM1

        // VEXPANDPS YMM3{k1}, YMM2
        0x62, 0xf2, 0x7d, 0x29, 0x88, 0xda, // VEXPANDPS YMM3{k1}, YMM2

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_compress_expand_different_masks() {
    let mut emu = emu64();
    let code = [
        // Setup mask K1 = 0x0F (lower 4 elements)
        0xb8, 0x0f, 0x00, 0x00, 0x00, // MOV EAX, 15
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX

        // VCOMPRESSPS YMM2{k1}, YMM1
        0x62, 0xf2, 0x7d, 0x29, 0x8a, 0xd1, // VCOMPRESSPS YMM2{k1}, YMM1

        // Setup mask K2 = 0xF0 (upper 4 elements)
        0xb8, 0xf0, 0x00, 0x00, 0x00, // MOV EAX, 240
        0xc5, 0xf8, 0x92, 0xd0, // KMOVW K2, EAX

        // VEXPANDPS YMM3{k2}, YMM2
        0x62, 0xf2, 0x7d, 0x2a, 0x88, 0xda, // VEXPANDPS YMM3{k2}, YMM2

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_compress_expand_different_sizes() {
    let mut emu = emu64();
    let code = [
        // XMM: K1 = 0x0F
        0xb8, 0x0f, 0x00, 0x00, 0x00, // MOV EAX, 15
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX
        0x62, 0xf2, 0x7d, 0x09, 0x8a, 0xd1, // VCOMPRESSPS XMM2{k1}, XMM1
        0x62, 0xf2, 0x7d, 0x09, 0x88, 0xda, // VEXPANDPS XMM3{k1}, XMM2

        // YMM: K2 = 0xFF
        0xb8, 0xff, 0x00, 0x00, 0x00, // MOV EAX, 255
        0xc5, 0xf8, 0x92, 0xd0, // KMOVW K2, EAX
        0x62, 0xf2, 0x7d, 0x2a, 0x8a, 0xec, // VCOMPRESSPS YMM4{k2}, YMM5
        0x62, 0xf2, 0x7d, 0x2a, 0x88, 0xf4, // VEXPANDPS YMM6{k2}, YMM4

        // ZMM: K3 = 0xFFFF
        0xb8, 0xff, 0xff, 0x00, 0x00, // MOV EAX, 65535
        0xc5, 0xf8, 0x92, 0xd8, // KMOVW K3, EAX
        0x62, 0xf2, 0x7d, 0x4b, 0x8a, 0xf8, // VCOMPRESSPS ZMM7{k3}, ZMM0
        0x62, 0xf2, 0x7d, 0x4b, 0x88, 0xc7, // VEXPANDPS ZMM0{k3}, ZMM7

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
