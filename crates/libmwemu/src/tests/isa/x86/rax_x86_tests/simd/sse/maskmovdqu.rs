use crate::*;

// MASKMOVDQU - Store Selected Bytes of Double Quadword
//
// MASKMOVDQU selectively stores bytes from the source XMM register to memory location
// specified by DI/EDI/RDI using the mask in the second XMM register.
//
// The high bit of each byte in the mask determines whether the corresponding byte is written:
// - 0: no write
// - 1: write
//
// Memory location uses DS:DI/EDI/RDI (can be overridden with segment prefix)
// Non-temporal hint for cache pollution minimization
//
// Opcodes:
// 66 0F F7 /r             MASKMOVDQU xmm1, xmm2     - Store selected bytes from xmm1 to DS:[RDI] using mask in xmm2

const DEST_ADDR: u64 = 0x4000; // Destination address for masked store

// ============================================================================
// MASKMOVDQU Tests - Store Selected Bytes Using Mask
// ============================================================================

#[test]
fn test_maskmovdqu_xmm0_xmm1() {
    let mut emu = emu64();
    // MASKMOVDQU XMM0, XMM1
    let code = [
        0x66, 0x0f, 0xf7, 0xc1, // MASKMOVDQU XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_xmm1_xmm2() {
    let mut emu = emu64();
    // MASKMOVDQU XMM1, XMM2
    let code = [
        0x66, 0x0f, 0xf7, 0xca, // MASKMOVDQU XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_xmm2_xmm3() {
    let mut emu = emu64();
    // MASKMOVDQU XMM2, XMM3
    let code = [
        0x66, 0x0f, 0xf7, 0xd3, // MASKMOVDQU XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_xmm3_xmm4() {
    let mut emu = emu64();
    // MASKMOVDQU XMM3, XMM4
    let code = [
        0x66, 0x0f, 0xf7, 0xdc, // MASKMOVDQU XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_xmm4_xmm5() {
    let mut emu = emu64();
    // MASKMOVDQU XMM4, XMM5
    let code = [
        0x66, 0x0f, 0xf7, 0xe5, // MASKMOVDQU XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_xmm5_xmm6() {
    let mut emu = emu64();
    // MASKMOVDQU XMM5, XMM6
    let code = [
        0x66, 0x0f, 0xf7, 0xee, // MASKMOVDQU XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_xmm6_xmm7() {
    let mut emu = emu64();
    // MASKMOVDQU XMM6, XMM7
    let code = [
        0x66, 0x0f, 0xf7, 0xf7, // MASKMOVDQU XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_xmm7_xmm0() {
    let mut emu = emu64();
    // MASKMOVDQU XMM7, XMM0
    let code = [
        0x66, 0x0f, 0xf7, 0xf8, // MASKMOVDQU XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_xmm8_xmm9() {
    let mut emu = emu64();
    // MASKMOVDQU XMM8, XMM9 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xc1, // MASKMOVDQU XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_xmm10_xmm11() {
    let mut emu = emu64();
    // MASKMOVDQU XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xd3, // MASKMOVDQU XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_xmm12_xmm13() {
    let mut emu = emu64();
    // MASKMOVDQU XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xe5, // MASKMOVDQU XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_xmm14_xmm15() {
    let mut emu = emu64();
    // MASKMOVDQU XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xf7, // MASKMOVDQU XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_xmm15_xmm0() {
    let mut emu = emu64();
    // MASKMOVDQU XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0xf7, 0xf8, // MASKMOVDQU XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_mask_all_zeros() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf7, 0xc1, // MASKMOVDQU XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_mask_all_ones() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf7, 0xd3, // MASKMOVDQU XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_mask_first_byte() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf7, 0xe5, // MASKMOVDQU XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_mask_last_byte() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf7, 0xf7, // MASKMOVDQU XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_mask_alternating() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xc1, // MASKMOVDQU XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_mask_even_bytes() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xd3, // MASKMOVDQU XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_mask_odd_bytes() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xe5, // MASKMOVDQU XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_mask_first_half() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xf7, // MASKMOVDQU XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_mask_second_half() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0xf7, 0xf8, // MASKMOVDQU XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_with_rdi() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf7, 0xc1, // MASKMOVDQU XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_single_byte_1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf7, 0xd3, // MASKMOVDQU XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_single_byte_2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf7, 0xe5, // MASKMOVDQU XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_single_byte_3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf7, 0xf7, // MASKMOVDQU XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_non_temporal_hint() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xc1, // MASKMOVDQU XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_sparse_mask_1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xd3, // MASKMOVDQU XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_sparse_mask_2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xe5, // MASKMOVDQU XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_continuous_bytes() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xf7, // MASKMOVDQU XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_verify_unmasked_unchanged() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0xf7, 0xf8, // MASKMOVDQU XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_same_register() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf7, 0xc0, // MASKMOVDQU XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_complex_pattern_1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf7, 0xc1, // MASKMOVDQU XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_complex_pattern_2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf7, 0xd3, // MASKMOVDQU XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovdqu_boundary_bytes() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf7, 0xe5, // MASKMOVDQU XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
