use crate::*;

// MASKMOVQ - Store Selected Bytes of Quadword (MMX)
// EMMS - Empty MMX Technology State
//
// MASKMOVQ conditionally stores bytes from mm1 to memory location using mask in mm2.
// EMMS empties the MMX technology state (marks all FPU registers as empty).
//
// Opcodes:
// NP 0F F7 /r    MASKMOVQ mm1, mm2    - Store bytes with mask
// NP 0F 77       EMMS                  - Empty MMX state

const DATA_ADDR: u64 = 0x3000;

// ============================================================================
// MASKMOVQ Tests
// ============================================================================

#[test]
fn test_maskmovq_mm0_mm1() {
    let mut emu = emu64();
    let code = [0x0f, 0xf7, 0xc1, 0xf4]; // MASKMOVQ MM0, MM1; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovq_mm2_mm3() {
    let mut emu = emu64();
    let code = [0x0f, 0xf7, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovq_mm4_mm5() {
    let mut emu = emu64();
    let code = [0x0f, 0xf7, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovq_mm6_mm7() {
    let mut emu = emu64();
    let code = [0x0f, 0xf7, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovq_mm0_mm7() {
    let mut emu = emu64();
    let code = [0x0f, 0xf7, 0xc7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovq_mm7_mm0() {
    let mut emu = emu64();
    let code = [0x0f, 0xf7, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovq_mm1_mm2() {
    let mut emu = emu64();
    let code = [0x0f, 0xf7, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovq_mm3_mm4() {
    let mut emu = emu64();
    let code = [0x0f, 0xf7, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovq_mm5_mm6() {
    let mut emu = emu64();
    let code = [0x0f, 0xf7, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovq_multiple() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xf7, 0xc1, // MASKMOVQ MM0, MM1
        0x0f, 0xf7, 0xd3, // MASKMOVQ MM2, MM3
        0x0f, 0xf7, 0xe5, // MASKMOVQ MM4, MM5
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovq_all_registers() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xf7, 0xc1, // MASKMOVQ MM0, MM1
        0x0f, 0xf7, 0xca, // MASKMOVQ MM1, MM2
        0x0f, 0xf7, 0xd3, // MASKMOVQ MM2, MM3
        0x0f, 0xf7, 0xdc, // MASKMOVQ MM3, MM4
        0x0f, 0xf7, 0xe5, // MASKMOVQ MM4, MM5
        0x0f, 0xf7, 0xee, // MASKMOVQ MM5, MM6
        0x0f, 0xf7, 0xf7, // MASKMOVQ MM6, MM7
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// EMMS Tests
// ============================================================================

#[test]
fn test_emms() {
    let mut emu = emu64();
    let code = [0x0f, 0x77, 0xf4]; // EMMS; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_emms_after_mmx_ops() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x6f, 0xc1, // MOVQ MM0, MM1
        0x0f, 0xd5, 0xc2, // PMULLW MM0, MM2
        0x0f, 0x77, // EMMS
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_emms_multiple() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x77, // EMMS
        0x0f, 0x77, // EMMS
        0x0f, 0x77, // EMMS
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_emms_with_maskmovq() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xf7, 0xc1, // MASKMOVQ MM0, MM1
        0x0f, 0x77, // EMMS
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovq_emms_sequence() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xf7, 0xc1, // MASKMOVQ MM0, MM1
        0x0f, 0xf7, 0xd3, // MASKMOVQ MM2, MM3
        0x0f, 0x77, // EMMS
        0x0f, 0xf7, 0xe5, // MASKMOVQ MM4, MM5
        0x0f, 0x77, // EMMS
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_emms_before_maskmovq() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x77, // EMMS
        0x0f, 0xf7, 0xc1, // MASKMOVQ MM0, MM1
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_emms_interleaved() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x6f, 0xc1, // MOVQ MM0, MM1
        0x0f, 0x77, // EMMS
        0x0f, 0xd5, 0xd3, // PMULLW MM2, MM3
        0x0f, 0x77, // EMMS
        0x0f, 0xf7, 0xe5, // MASKMOVQ MM4, MM5
        0x0f, 0x77, // EMMS
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovq_chain() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xf7, 0xc1, // MASKMOVQ MM0, MM1
        0x0f, 0xf7, 0xca, // MASKMOVQ MM1, MM2
        0x0f, 0xf7, 0xd3, // MASKMOVQ MM2, MM3
        0x0f, 0xf7, 0xdc, // MASKMOVQ MM3, MM4
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_emms_standalone() {
    let mut emu = emu64();
    let code = [0x0f, 0x77, 0xf4]; // Just EMMS
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maskmovq_reverse_pairs() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xf7, 0xc8, // MASKMOVQ MM1, MM0
        0x0f, 0xf7, 0xda, // MASKMOVQ MM3, MM2
        0x0f, 0xf7, 0xec, // MASKMOVQ MM5, MM4
        0x0f, 0xf7, 0xfe, // MASKMOVQ MM7, MM6
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
