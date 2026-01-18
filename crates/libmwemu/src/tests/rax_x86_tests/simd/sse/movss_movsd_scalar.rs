use crate::*;

// MOVSS - Move Scalar Single Precision Floating-Point
// MOVSD - Move Scalar Double Precision Floating-Point
//
// MOVSS moves a single precision floating-point value (32 bits) between XMM registers
// or between an XMM register and memory. The upper bits remain unchanged for reg-to-reg.
//
// MOVSD moves a double precision floating-point value (64 bits) between XMM registers
// or between an XMM register and memory. The upper bits remain unchanged for reg-to-reg.
//
// Opcodes:
// F3 0F 10 /r    MOVSS xmm1, xmm2/m32    - Move scalar single from xmm2/m32 to xmm1
// F3 0F 11 /r    MOVSS xmm2/m32, xmm1    - Move scalar single from xmm1 to xmm2/m32
// F2 0F 10 /r    MOVSD xmm1, xmm2/m64    - Move scalar double from xmm2/m64 to xmm1
// F2 0F 11 /r    MOVSD xmm2/m64, xmm1    - Move scalar double from xmm1 to xmm2/m64

const DATA_ADDR: u64 = 0x3000;

// ============================================================================
// MOVSS - XMM to XMM Tests
// ============================================================================

#[test]
fn test_movss_xmm0_to_xmm1() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x10, 0xc8, 0xf4]; // MOVSS XMM1, XMM0; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movss_xmm2_to_xmm3() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x10, 0xda, 0xf4]; // MOVSS XMM3, XMM2; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movss_xmm4_to_xmm5() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x10, 0xec, 0xf4]; // MOVSS XMM5, XMM4; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movss_xmm6_to_xmm7() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x10, 0xfe, 0xf4]; // MOVSS XMM7, XMM6; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movss_xmm8_to_xmm9() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x10, 0xc8, 0xf4]; // MOVSS XMM9, XMM8; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movss_xmm14_to_xmm15() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x10, 0xfe, 0xf4]; // MOVSS XMM15, XMM14; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movss_xmm0_to_xmm15() {
    let mut emu = emu64();
    let code = [0xf3, 0x44, 0x0f, 0x10, 0xf8, 0xf4]; // MOVSS XMM15, XMM0; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movss_xmm15_to_xmm0() {
    let mut emu = emu64();
    let code = [0xf3, 0x41, 0x0f, 0x10, 0xc7, 0xf4]; // MOVSS XMM0, XMM15; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MOVSS - Memory to XMM Tests
// ============================================================================

#[test]
fn test_movss_mem_to_xmm0() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x10, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4]; // MOVSS XMM0, [0x3000]; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movss_mem_to_xmm7() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x10, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4]; // MOVSS XMM7, [0x3000]; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movss_mem_to_xmm15() {
    let mut emu = emu64();
    let code = [0xf3, 0x44, 0x0f, 0x10, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4]; // MOVSS XMM15, [0x3000]; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MOVSS - XMM to Memory Tests
// ============================================================================

#[test]
fn test_movss_xmm0_to_mem() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x11, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4]; // MOVSS [0x3000], XMM0; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movss_xmm7_to_mem() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x11, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4]; // MOVSS [0x3000], XMM7; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movss_xmm15_to_mem() {
    let mut emu = emu64();
    let code = [0xf3, 0x44, 0x0f, 0x11, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4]; // MOVSS [0x3000], XMM15; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MOVSD - XMM to XMM Tests
// ============================================================================

#[test]
fn test_movsd_xmm0_to_xmm1() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x10, 0xc8, 0xf4]; // MOVSD XMM1, XMM0; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsd_xmm2_to_xmm3() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x10, 0xda, 0xf4]; // MOVSD XMM3, XMM2; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsd_xmm4_to_xmm5() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x10, 0xec, 0xf4]; // MOVSD XMM5, XMM4; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsd_xmm6_to_xmm7() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x10, 0xfe, 0xf4]; // MOVSD XMM7, XMM6; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsd_xmm8_to_xmm9() {
    let mut emu = emu64();
    let code = [0xf2, 0x45, 0x0f, 0x10, 0xc8, 0xf4]; // MOVSD XMM9, XMM8; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsd_xmm14_to_xmm15() {
    let mut emu = emu64();
    let code = [0xf2, 0x45, 0x0f, 0x10, 0xfe, 0xf4]; // MOVSD XMM15, XMM14; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsd_xmm0_to_xmm15() {
    let mut emu = emu64();
    let code = [0xf2, 0x44, 0x0f, 0x10, 0xf8, 0xf4]; // MOVSD XMM15, XMM0; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsd_xmm15_to_xmm0() {
    let mut emu = emu64();
    let code = [0xf2, 0x41, 0x0f, 0x10, 0xc7, 0xf4]; // MOVSD XMM0, XMM15; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MOVSD - Memory to XMM Tests
// ============================================================================

#[test]
fn test_movsd_mem_to_xmm0() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x10, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4]; // MOVSD XMM0, [0x3000]; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsd_mem_to_xmm7() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x10, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4]; // MOVSD XMM7, [0x3000]; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsd_mem_to_xmm15() {
    let mut emu = emu64();
    let code = [0xf2, 0x44, 0x0f, 0x10, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4]; // MOVSD XMM15, [0x3000]; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MOVSD - XMM to Memory Tests
// ============================================================================

#[test]
fn test_movsd_xmm0_to_mem() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x11, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4]; // MOVSD [0x3000], XMM0; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsd_xmm7_to_mem() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x11, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4]; // MOVSD [0x3000], XMM7; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsd_xmm15_to_mem() {
    let mut emu = emu64();
    let code = [0xf2, 0x44, 0x0f, 0x11, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4]; // MOVSD [0x3000], XMM15; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Mixed MOVSS/MOVSD Tests
// ============================================================================

#[test]
fn test_multiple_movss() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x10, 0xc8, // MOVSS XMM1, XMM0
        0xf3, 0x0f, 0x10, 0xd1, // MOVSS XMM2, XMM1
        0xf3, 0x0f, 0x10, 0xda, // MOVSS XMM3, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_multiple_movsd() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x10, 0xc8, // MOVSD XMM1, XMM0
        0xf2, 0x0f, 0x10, 0xd1, // MOVSD XMM2, XMM1
        0xf2, 0x0f, 0x10, 0xda, // MOVSD XMM3, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movss_movsd_mixed() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x10, 0xc8, // MOVSS XMM1, XMM0
        0xf2, 0x0f, 0x10, 0xd1, // MOVSD XMM2, XMM1
        0xf3, 0x0f, 0x10, 0xda, // MOVSS XMM3, XMM2
        0xf2, 0x0f, 0x10, 0xe3, // MOVSD XMM4, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movss_mem_round_trip() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x11, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSS [0x3000], XMM0
        0xf3, 0x0f, 0x10, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSS XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsd_mem_round_trip() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x11, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSD [0x3000], XMM0
        0xf2, 0x0f, 0x10, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSD XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movss_all_reg_pairs() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x10, 0xc8, // MOVSS XMM1, XMM0
        0xf3, 0x0f, 0x10, 0xda, // MOVSS XMM3, XMM2
        0xf3, 0x0f, 0x10, 0xec, // MOVSS XMM5, XMM4
        0xf3, 0x0f, 0x10, 0xfe, // MOVSS XMM7, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsd_all_reg_pairs() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x10, 0xc8, // MOVSD XMM1, XMM0
        0xf2, 0x0f, 0x10, 0xda, // MOVSD XMM3, XMM2
        0xf2, 0x0f, 0x10, 0xec, // MOVSD XMM5, XMM4
        0xf2, 0x0f, 0x10, 0xfe, // MOVSD XMM7, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movss_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x45, 0x0f, 0x10, 0xc8, // MOVSS XMM9, XMM8
        0xf3, 0x45, 0x0f, 0x10, 0xda, // MOVSS XMM11, XMM10
        0xf3, 0x45, 0x0f, 0x10, 0xec, // MOVSS XMM13, XMM12
        0xf3, 0x45, 0x0f, 0x10, 0xfe, // MOVSS XMM15, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsd_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x45, 0x0f, 0x10, 0xc8, // MOVSD XMM9, XMM8
        0xf2, 0x45, 0x0f, 0x10, 0xda, // MOVSD XMM11, XMM10
        0xf2, 0x45, 0x0f, 0x10, 0xec, // MOVSD XMM13, XMM12
        0xf2, 0x45, 0x0f, 0x10, 0xfe, // MOVSD XMM15, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
