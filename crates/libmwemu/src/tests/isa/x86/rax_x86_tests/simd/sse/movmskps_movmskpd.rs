use crate::*;

// MOVMSKPS - Extract Packed Single Precision Floating-Point Sign Mask
// MOVMSKPD - Extract Packed Double Precision Floating-Point Sign Mask
//
// MOVMSKPS extracts 4 sign bits from packed single-precision values to an integer register
// MOVMSKPD extracts 2 sign bits from packed double-precision values to an integer register
//
// Opcodes:
// NP 0F 50 /r             MOVMSKPS reg, xmm    - Extract 4-bit sign mask from xmm to reg
// 66 0F 50 /r             MOVMSKPD reg, xmm    - Extract 2-bit sign mask from xmm to reg

// ============================================================================
// MOVMSKPS Tests - Extract 4 Sign Bits from Single Precision
// ============================================================================

#[test]
fn test_movmskps_eax_xmm0() {
    let mut emu = emu64();
    // MOVMSKPS EAX, XMM0
    let code = [
        0x0f, 0x50, 0xc0, // MOVMSKPS EAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_ecx_xmm1() {
    let mut emu = emu64();
    // MOVMSKPS ECX, XMM1
    let code = [
        0x0f, 0x50, 0xc9, // MOVMSKPS ECX, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_edx_xmm2() {
    let mut emu = emu64();
    // MOVMSKPS EDX, XMM2
    let code = [
        0x0f, 0x50, 0xd2, // MOVMSKPS EDX, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_ebx_xmm3() {
    let mut emu = emu64();
    // MOVMSKPS EBX, XMM3
    let code = [
        0x0f, 0x50, 0xdb, // MOVMSKPS EBX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_esi_xmm4() {
    let mut emu = emu64();
    // MOVMSKPS ESI, XMM4
    let code = [
        0x0f, 0x50, 0xf4, // MOVMSKPS ESI, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_edi_xmm5() {
    let mut emu = emu64();
    // MOVMSKPS EDI, XMM5
    let code = [
        0x0f, 0x50, 0xfd, // MOVMSKPS EDI, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_r8d_xmm6() {
    let mut emu = emu64();
    // MOVMSKPS R8D, XMM6
    let code = [
        0x44, 0x0f, 0x50, 0xc6, // MOVMSKPS R8D, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_r9d_xmm7() {
    let mut emu = emu64();
    // MOVMSKPS R9D, XMM7
    let code = [
        0x44, 0x0f, 0x50, 0xcf, // MOVMSKPS R9D, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_eax_xmm8() {
    let mut emu = emu64();
    // MOVMSKPS EAX, XMM8
    let code = [
        0x41, 0x0f, 0x50, 0xc0, // MOVMSKPS EAX, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_ecx_xmm9() {
    let mut emu = emu64();
    // MOVMSKPS ECX, XMM9
    let code = [
        0x41, 0x0f, 0x50, 0xc9, // MOVMSKPS ECX, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_edx_xmm10() {
    let mut emu = emu64();
    // MOVMSKPS EDX, XMM10
    let code = [
        0x41, 0x0f, 0x50, 0xd2, // MOVMSKPS EDX, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_ebx_xmm11() {
    let mut emu = emu64();
    // MOVMSKPS EBX, XMM11
    let code = [
        0x41, 0x0f, 0x50, 0xdb, // MOVMSKPS EBX, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_esi_xmm12() {
    let mut emu = emu64();
    // MOVMSKPS ESI, XMM12
    let code = [
        0x41, 0x0f, 0x50, 0xf4, // MOVMSKPS ESI, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_edi_xmm13() {
    let mut emu = emu64();
    // MOVMSKPS EDI, XMM13
    let code = [
        0x41, 0x0f, 0x50, 0xfd, // MOVMSKPS EDI, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_r8d_xmm14() {
    let mut emu = emu64();
    // MOVMSKPS R8D, XMM14
    let code = [
        0x45, 0x0f, 0x50, 0xc6, // MOVMSKPS R8D, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_r9d_xmm15() {
    let mut emu = emu64();
    // MOVMSKPS R9D, XMM15
    let code = [
        0x45, 0x0f, 0x50, 0xcf, // MOVMSKPS R9D, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_all_positive() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x50, 0xc0, // MOVMSKPS EAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_all_negative() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x50, 0xc1, // MOVMSKPS EAX, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_sign_pattern_0001() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x50, 0xc2, // MOVMSKPS EAX, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_sign_pattern_0010() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x50, 0xc3, // MOVMSKPS EAX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_sign_pattern_0100() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x50, 0xc4, // MOVMSKPS EAX, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_sign_pattern_1000() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x50, 0xc5, // MOVMSKPS EAX, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_sign_pattern_1010() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x50, 0xc6, // MOVMSKPS EAX, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskps_sign_pattern_0101() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x50, 0xc7, // MOVMSKPS EAX, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MOVMSKPD Tests - Extract 2 Sign Bits from Double Precision
// ============================================================================

#[test]
fn test_movmskpd_eax_xmm0() {
    let mut emu = emu64();
    // MOVMSKPD EAX, XMM0
    let code = [
        0x66, 0x0f, 0x50, 0xc0, // MOVMSKPD EAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_ecx_xmm1() {
    let mut emu = emu64();
    // MOVMSKPD ECX, XMM1
    let code = [
        0x66, 0x0f, 0x50, 0xc9, // MOVMSKPD ECX, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_edx_xmm2() {
    let mut emu = emu64();
    // MOVMSKPD EDX, XMM2
    let code = [
        0x66, 0x0f, 0x50, 0xd2, // MOVMSKPD EDX, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_ebx_xmm3() {
    let mut emu = emu64();
    // MOVMSKPD EBX, XMM3
    let code = [
        0x66, 0x0f, 0x50, 0xdb, // MOVMSKPD EBX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_esi_xmm4() {
    let mut emu = emu64();
    // MOVMSKPD ESI, XMM4
    let code = [
        0x66, 0x0f, 0x50, 0xf4, // MOVMSKPD ESI, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_edi_xmm5() {
    let mut emu = emu64();
    // MOVMSKPD EDI, XMM5
    let code = [
        0x66, 0x0f, 0x50, 0xfd, // MOVMSKPD EDI, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_r8d_xmm6() {
    let mut emu = emu64();
    // MOVMSKPD R8D, XMM6
    let code = [
        0x66, 0x44, 0x0f, 0x50, 0xc6, // MOVMSKPD R8D, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_r9d_xmm7() {
    let mut emu = emu64();
    // MOVMSKPD R9D, XMM7
    let code = [
        0x66, 0x44, 0x0f, 0x50, 0xcf, // MOVMSKPD R9D, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_eax_xmm8() {
    let mut emu = emu64();
    // MOVMSKPD EAX, XMM8
    let code = [
        0x66, 0x41, 0x0f, 0x50, 0xc0, // MOVMSKPD EAX, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_ecx_xmm9() {
    let mut emu = emu64();
    // MOVMSKPD ECX, XMM9
    let code = [
        0x66, 0x41, 0x0f, 0x50, 0xc9, // MOVMSKPD ECX, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_edx_xmm10() {
    let mut emu = emu64();
    // MOVMSKPD EDX, XMM10
    let code = [
        0x66, 0x41, 0x0f, 0x50, 0xd2, // MOVMSKPD EDX, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_ebx_xmm11() {
    let mut emu = emu64();
    // MOVMSKPD EBX, XMM11
    let code = [
        0x66, 0x41, 0x0f, 0x50, 0xdb, // MOVMSKPD EBX, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_esi_xmm12() {
    let mut emu = emu64();
    // MOVMSKPD ESI, XMM12
    let code = [
        0x66, 0x41, 0x0f, 0x50, 0xf4, // MOVMSKPD ESI, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_edi_xmm13() {
    let mut emu = emu64();
    // MOVMSKPD EDI, XMM13
    let code = [
        0x66, 0x41, 0x0f, 0x50, 0xfd, // MOVMSKPD EDI, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_r8d_xmm14() {
    let mut emu = emu64();
    // MOVMSKPD R8D, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0x50, 0xc6, // MOVMSKPD R8D, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_r9d_xmm15() {
    let mut emu = emu64();
    // MOVMSKPD R9D, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x50, 0xcf, // MOVMSKPD R9D, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_all_positive() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x50, 0xc0, // MOVMSKPD EAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_all_negative() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x50, 0xc1, // MOVMSKPD EAX, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_sign_pattern_01() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x50, 0xc2, // MOVMSKPD EAX, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_sign_pattern_10() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x50, 0xc3, // MOVMSKPD EAX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_zero_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x50, 0xc4, // MOVMSKPD EAX, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movmskpd_negative_zero() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x50, 0xc5, // MOVMSKPD EAX, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
