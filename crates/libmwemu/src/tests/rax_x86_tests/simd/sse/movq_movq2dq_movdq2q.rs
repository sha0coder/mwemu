use crate::*;

// MOVQ - Move Quadword (MMX/SSE2)
// MOVQ2DQ - Move Quadword from MMX to XMM
// MOVDQ2Q - Move Quadword from XMM to MMX
//
// MOVQ moves 64 bits between MMX registers, XMM registers, or memory.
// MOVQ2DQ moves 64 bits from an MMX register to the low quadword of an XMM register.
// MOVDQ2Q moves 64 bits from the low quadword of an XMM register to an MMX register.
//
// Opcodes:
// NP 0F 6F /r        MOVQ mm, mm/m64           - Move qword from mm/m64 to mm
// NP 0F 7F /r        MOVQ mm/m64, mm           - Move qword from mm to mm/m64
// F3 0F 7E /r        MOVQ xmm1, xmm2/m64       - Move qword from xmm2/m64 to xmm1
// 66 0F D6 /r        MOVQ xmm2/m64, xmm1       - Move qword from xmm1 to xmm2/m64
// F3 0F D6 /r        MOVQ2DQ xmm, mm           - Move qword from mm to low qword of xmm
// F2 0F D6 /r        MOVDQ2Q mm, xmm           - Move low qword from xmm to mm

const DATA_ADDR: u64 = 0x3000;

// ============================================================================
// MOVQ - MMX to MMX Tests
// ============================================================================

#[test]
fn test_movq_mm0_to_mm1() {
    let mut emu = emu64();
    // MOVQ MM1, MM0
    let code = [
        0x0f, 0x6f, 0xc8, // MOVQ MM1, MM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_mm2_to_mm3() {
    let mut emu = emu64();
    // MOVQ MM3, MM2
    let code = [
        0x0f, 0x6f, 0xda, // MOVQ MM3, MM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_mm4_to_mm5() {
    let mut emu = emu64();
    // MOVQ MM5, MM4
    let code = [
        0x0f, 0x6f, 0xec, // MOVQ MM5, MM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_mm6_to_mm7() {
    let mut emu = emu64();
    // MOVQ MM7, MM6
    let code = [
        0x0f, 0x6f, 0xfe, // MOVQ MM7, MM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_mm7_to_mm0() {
    let mut emu = emu64();
    // MOVQ MM0, MM7
    let code = [
        0x0f, 0x6f, 0xc7, // MOVQ MM0, MM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MOVQ - MMX to Memory Tests
// ============================================================================

#[test]
fn test_movq_mm0_to_mem() {
    let mut emu = emu64();
    // MOVQ [0x3000], MM0
    let code = [
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_mm1_to_mem() {
    let mut emu = emu64();
    // MOVQ [0x3000], MM1
    let code = [
        0x0f, 0x7f, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_mm7_to_mem() {
    let mut emu = emu64();
    // MOVQ [0x3000], MM7
    let code = [
        0x0f, 0x7f, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MOVQ - Memory to MMX Tests
// ============================================================================

#[test]
fn test_movq_mem_to_mm0() {
    let mut emu = emu64();
    // MOVQ MM0, [0x3000]
    let code = [
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ MM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_mem_to_mm3() {
    let mut emu = emu64();
    // MOVQ MM3, [0x3000]
    let code = [
        0x0f, 0x6f, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ MM3, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_mem_to_mm7() {
    let mut emu = emu64();
    // MOVQ MM7, [0x3000]
    let code = [
        0x0f, 0x6f, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ MM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MOVQ - XMM to XMM Tests (SSE2)
// ============================================================================

#[test]
fn test_movq_xmm0_to_xmm1() {
    let mut emu = emu64();
    // MOVQ XMM1, XMM0
    let code = [
        0xf3, 0x0f, 0x7e, 0xc8, // MOVQ XMM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm2_to_xmm3() {
    let mut emu = emu64();
    // MOVQ XMM3, XMM2
    let code = [
        0xf3, 0x0f, 0x7e, 0xda, // MOVQ XMM3, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm4_to_xmm5() {
    let mut emu = emu64();
    // MOVQ XMM5, XMM4
    let code = [
        0xf3, 0x0f, 0x7e, 0xec, // MOVQ XMM5, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm6_to_xmm7() {
    let mut emu = emu64();
    // MOVQ XMM7, XMM6
    let code = [
        0xf3, 0x0f, 0x7e, 0xfe, // MOVQ XMM7, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm8_to_xmm9() {
    let mut emu = emu64();
    // MOVQ XMM9, XMM8
    let code = [
        0xf3, 0x45, 0x0f, 0x7e, 0xc8, // MOVQ XMM9, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm14_to_xmm15() {
    let mut emu = emu64();
    // MOVQ XMM15, XMM14
    let code = [
        0xf3, 0x45, 0x0f, 0x7e, 0xfe, // MOVQ XMM15, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MOVQ - XMM to Memory Tests (SSE2)
// ============================================================================

#[test]
fn test_movq_xmm0_to_mem_sse2() {
    let mut emu = emu64();
    // MOVQ [0x3000], XMM0
    let code = [
        0x66, 0x0f, 0xd6, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm7_to_mem_sse2() {
    let mut emu = emu64();
    // MOVQ [0x3000], XMM7
    let code = [
        0x66, 0x0f, 0xd6, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm15_to_mem_sse2() {
    let mut emu = emu64();
    // MOVQ [0x3000], XMM15
    let code = [
        0x66, 0x44, 0x0f, 0xd6, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MOVQ - Memory to XMM Tests (SSE2)
// ============================================================================

#[test]
fn test_movq_mem_to_xmm0() {
    let mut emu = emu64();
    // MOVQ XMM0, [0x3000]
    let code = [
        0xf3, 0x0f, 0x7e, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_mem_to_xmm7() {
    let mut emu = emu64();
    // MOVQ XMM7, [0x3000]
    let code = [
        0xf3, 0x0f, 0x7e, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_mem_to_xmm15() {
    let mut emu = emu64();
    // MOVQ XMM15, [0x3000]
    let code = [
        0xf3, 0x44, 0x0f, 0x7e, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MOVQ2DQ - MMX to XMM Tests
// ============================================================================

#[test]
fn test_movq2dq_mm0_to_xmm0() {
    let mut emu = emu64();
    // MOVQ2DQ XMM0, MM0
    let code = [
        0xf3, 0x0f, 0xd6, 0xc0, // MOVQ2DQ XMM0, MM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq2dq_mm1_to_xmm1() {
    let mut emu = emu64();
    // MOVQ2DQ XMM1, MM1
    let code = [
        0xf3, 0x0f, 0xd6, 0xc9, // MOVQ2DQ XMM1, MM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq2dq_mm2_to_xmm2() {
    let mut emu = emu64();
    // MOVQ2DQ XMM2, MM2
    let code = [
        0xf3, 0x0f, 0xd6, 0xd2, // MOVQ2DQ XMM2, MM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq2dq_mm3_to_xmm3() {
    let mut emu = emu64();
    // MOVQ2DQ XMM3, MM3
    let code = [
        0xf3, 0x0f, 0xd6, 0xdb, // MOVQ2DQ XMM3, MM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq2dq_mm4_to_xmm4() {
    let mut emu = emu64();
    // MOVQ2DQ XMM4, MM4
    let code = [
        0xf3, 0x0f, 0xd6, 0xe4, // MOVQ2DQ XMM4, MM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq2dq_mm5_to_xmm5() {
    let mut emu = emu64();
    // MOVQ2DQ XMM5, MM5
    let code = [
        0xf3, 0x0f, 0xd6, 0xed, // MOVQ2DQ XMM5, MM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq2dq_mm6_to_xmm6() {
    let mut emu = emu64();
    // MOVQ2DQ XMM6, MM6
    let code = [
        0xf3, 0x0f, 0xd6, 0xf6, // MOVQ2DQ XMM6, MM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq2dq_mm7_to_xmm7() {
    let mut emu = emu64();
    // MOVQ2DQ XMM7, MM7
    let code = [
        0xf3, 0x0f, 0xd6, 0xff, // MOVQ2DQ XMM7, MM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq2dq_mm0_to_xmm15() {
    let mut emu = emu64();
    // MOVQ2DQ XMM15, MM0
    let code = [
        0xf3, 0x44, 0x0f, 0xd6, 0xf8, // MOVQ2DQ XMM15, MM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MOVDQ2Q - XMM to MMX Tests
// ============================================================================

#[test]
fn test_movdq2q_xmm0_to_mm0() {
    let mut emu = emu64();
    // MOVDQ2Q MM0, XMM0
    let code = [
        0xf2, 0x0f, 0xd6, 0xc0, // MOVDQ2Q MM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdq2q_xmm1_to_mm1() {
    let mut emu = emu64();
    // MOVDQ2Q MM1, XMM1
    let code = [
        0xf2, 0x0f, 0xd6, 0xc9, // MOVDQ2Q MM1, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdq2q_xmm2_to_mm2() {
    let mut emu = emu64();
    // MOVDQ2Q MM2, XMM2
    let code = [
        0xf2, 0x0f, 0xd6, 0xd2, // MOVDQ2Q MM2, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdq2q_xmm3_to_mm3() {
    let mut emu = emu64();
    // MOVDQ2Q MM3, XMM3
    let code = [
        0xf2, 0x0f, 0xd6, 0xdb, // MOVDQ2Q MM3, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdq2q_xmm4_to_mm4() {
    let mut emu = emu64();
    // MOVDQ2Q MM4, XMM4
    let code = [
        0xf2, 0x0f, 0xd6, 0xe4, // MOVDQ2Q MM4, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdq2q_xmm5_to_mm5() {
    let mut emu = emu64();
    // MOVDQ2Q MM5, XMM5
    let code = [
        0xf2, 0x0f, 0xd6, 0xed, // MOVDQ2Q MM5, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdq2q_xmm6_to_mm6() {
    let mut emu = emu64();
    // MOVDQ2Q MM6, XMM6
    let code = [
        0xf2, 0x0f, 0xd6, 0xf6, // MOVDQ2Q MM6, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdq2q_xmm7_to_mm7() {
    let mut emu = emu64();
    // MOVDQ2Q MM7, XMM7
    let code = [
        0xf2, 0x0f, 0xd6, 0xff, // MOVDQ2Q MM7, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdq2q_xmm15_to_mm7() {
    let mut emu = emu64();
    // MOVDQ2Q MM7, XMM15
    let code = [
        0xf2, 0x41, 0x0f, 0xd6, 0xff, // MOVDQ2Q MM7, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Mixed Conversion Chains
// ============================================================================

#[test]
fn test_movq2dq_movdq2q_round_trip() {
    let mut emu = emu64();
    // MOVQ2DQ XMM0, MM0 then MOVDQ2Q MM1, XMM0
    let code = [
        0xf3, 0x0f, 0xd6, 0xc0, // MOVQ2DQ XMM0, MM0
        0xf2, 0x0f, 0xd6, 0xc8, // MOVDQ2Q MM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdq2q_movq2dq_round_trip() {
    let mut emu = emu64();
    // MOVDQ2Q MM0, XMM0 then MOVQ2DQ XMM1, MM0
    let code = [
        0xf2, 0x0f, 0xd6, 0xc0, // MOVDQ2Q MM0, XMM0
        0xf3, 0x0f, 0xd6, 0xc8, // MOVQ2DQ XMM1, MM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_multiple_movq_mmx() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x6f, 0xc8, // MOVQ MM1, MM0
        0x0f, 0x6f, 0xd1, // MOVQ MM2, MM1
        0x0f, 0x6f, 0xda, // MOVQ MM3, MM2
        0x0f, 0x6f, 0xe3, // MOVQ MM4, MM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_multiple_movq_xmm() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x7e, 0xc8, // MOVQ XMM1, XMM0
        0xf3, 0x0f, 0x7e, 0xd1, // MOVQ XMM2, XMM1
        0xf3, 0x0f, 0x7e, 0xda, // MOVQ XMM3, XMM2
        0xf3, 0x0f, 0x7e, 0xe3, // MOVQ XMM4, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_multiple_movq2dq() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0xd6, 0xc0, // MOVQ2DQ XMM0, MM0
        0xf3, 0x0f, 0xd6, 0xc9, // MOVQ2DQ XMM1, MM1
        0xf3, 0x0f, 0xd6, 0xd2, // MOVQ2DQ XMM2, MM2
        0xf3, 0x0f, 0xd6, 0xdb, // MOVQ2DQ XMM3, MM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_multiple_movdq2q() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0xd6, 0xc0, // MOVDQ2Q MM0, XMM0
        0xf2, 0x0f, 0xd6, 0xc9, // MOVDQ2Q MM1, XMM1
        0xf2, 0x0f, 0xd6, 0xd2, // MOVDQ2Q MM2, XMM2
        0xf2, 0x0f, 0xd6, 0xdb, // MOVDQ2Q MM3, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
