use crate::*;

// VMOVMSKPS - Extract Packed Single-Precision Floating-Point Sign Mask
// VMOVMSKPD - Extract Packed Double-Precision Floating-Point Sign Mask
//
// VMOVMSKPS extracts the sign bits from packed single-precision floating-point values
// and stores them in a general-purpose register. Each sign bit becomes one bit in the result.
//
// VMOVMSKPD extracts the sign bits from packed double-precision floating-point values
// and stores them in a general-purpose register.
//
// For 128-bit (XMM) operands:
// - VMOVMSKPS extracts 4 sign bits (bits 3:0 of result)
// - VMOVMSKPD extracts 2 sign bits (bits 1:0 of result)
//
// For 256-bit (YMM) operands:
// - VMOVMSKPS extracts 8 sign bits (bits 7:0 of result)
// - VMOVMSKPD extracts 4 sign bits (bits 3:0 of result)
//
// Opcodes:
// VEX.128.0F.WIG 50 /r    VMOVMSKPS r32, xmm2   - Extract sign mask from XMM (4 bits)
// VEX.256.0F.WIG 50 /r    VMOVMSKPS r32, ymm2   - Extract sign mask from YMM (8 bits)
// VEX.128.66.0F.WIG 50 /r VMOVMSKPD r32, xmm2   - Extract sign mask from XMM (2 bits)
// VEX.256.66.0F.WIG 50 /r VMOVMSKPD r32, ymm2   - Extract sign mask from YMM (4 bits)

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VMOVMSKPS Tests - 128-bit XMM registers (4 sign bits)
// ============================================================================

#[test]
fn test_vmovmskps_xmm0_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPS EAX, XMM0
    let code = [
        0xc5, 0xf8, 0x50, 0xc0, // VMOVMSKPS EAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_xmm1_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPS EAX, XMM1
    let code = [
        0xc5, 0xf8, 0x50, 0xc1, // VMOVMSKPS EAX, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_xmm2_to_ebx() {
    let mut emu = emu64();
    // VMOVMSKPS EBX, XMM2
    let code = [
        0xc5, 0xf8, 0x50, 0xda, // VMOVMSKPS EBX, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_xmm3_to_ecx() {
    let mut emu = emu64();
    // VMOVMSKPS ECX, XMM3
    let code = [
        0xc5, 0xf8, 0x50, 0xcb, // VMOVMSKPS ECX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_xmm4_to_edx() {
    let mut emu = emu64();
    // VMOVMSKPS EDX, XMM4
    let code = [
        0xc5, 0xf8, 0x50, 0xd4, // VMOVMSKPS EDX, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_xmm5_to_esi() {
    let mut emu = emu64();
    // VMOVMSKPS ESI, XMM5
    let code = [
        0xc5, 0xf8, 0x50, 0xf5, // VMOVMSKPS ESI, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_xmm6_to_edi() {
    let mut emu = emu64();
    // VMOVMSKPS EDI, XMM6
    let code = [
        0xc5, 0xf8, 0x50, 0xfe, // VMOVMSKPS EDI, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_xmm7_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPS EAX, XMM7
    let code = [
        0xc5, 0xf8, 0x50, 0xc7, // VMOVMSKPS EAX, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMOVMSKPS Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vmovmskps_xmm8_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPS EAX, XMM8
    let code = [
        0xc4, 0xc1, 0x78, 0x50, 0xc0, // VMOVMSKPS EAX, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_xmm9_to_ebx() {
    let mut emu = emu64();
    // VMOVMSKPS EBX, XMM9
    let code = [
        0xc4, 0xc1, 0x78, 0x50, 0xd9, // VMOVMSKPS EBX, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_xmm10_to_ecx() {
    let mut emu = emu64();
    // VMOVMSKPS ECX, XMM10
    let code = [
        0xc4, 0xc1, 0x78, 0x50, 0xca, // VMOVMSKPS ECX, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_xmm11_to_edx() {
    let mut emu = emu64();
    // VMOVMSKPS EDX, XMM11
    let code = [
        0xc4, 0xc1, 0x78, 0x50, 0xd3, // VMOVMSKPS EDX, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_xmm12_to_esi() {
    let mut emu = emu64();
    // VMOVMSKPS ESI, XMM12
    let code = [
        0xc4, 0xc1, 0x78, 0x50, 0xf4, // VMOVMSKPS ESI, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_xmm13_to_edi() {
    let mut emu = emu64();
    // VMOVMSKPS EDI, XMM13
    let code = [
        0xc4, 0xc1, 0x78, 0x50, 0xfd, // VMOVMSKPS EDI, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_xmm14_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPS EAX, XMM14
    let code = [
        0xc4, 0xc1, 0x78, 0x50, 0xc6, // VMOVMSKPS EAX, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_xmm15_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPS EAX, XMM15
    let code = [
        0xc4, 0xc1, 0x78, 0x50, 0xc7, // VMOVMSKPS EAX, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMOVMSKPS Tests - 256-bit YMM registers (8 sign bits)
// ============================================================================

#[test]
fn test_vmovmskps_ymm0_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPS EAX, YMM0
    let code = [
        0xc5, 0xfc, 0x50, 0xc0, // VMOVMSKPS EAX, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_ymm1_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPS EAX, YMM1
    let code = [
        0xc5, 0xfc, 0x50, 0xc1, // VMOVMSKPS EAX, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_ymm2_to_ebx() {
    let mut emu = emu64();
    // VMOVMSKPS EBX, YMM2
    let code = [
        0xc5, 0xfc, 0x50, 0xda, // VMOVMSKPS EBX, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_ymm3_to_ecx() {
    let mut emu = emu64();
    // VMOVMSKPS ECX, YMM3
    let code = [
        0xc5, 0xfc, 0x50, 0xcb, // VMOVMSKPS ECX, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_ymm4_to_edx() {
    let mut emu = emu64();
    // VMOVMSKPS EDX, YMM4
    let code = [
        0xc5, 0xfc, 0x50, 0xd4, // VMOVMSKPS EDX, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_ymm5_to_esi() {
    let mut emu = emu64();
    // VMOVMSKPS ESI, YMM5
    let code = [
        0xc5, 0xfc, 0x50, 0xf5, // VMOVMSKPS ESI, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_ymm6_to_edi() {
    let mut emu = emu64();
    // VMOVMSKPS EDI, YMM6
    let code = [
        0xc5, 0xfc, 0x50, 0xfe, // VMOVMSKPS EDI, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_ymm7_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPS EAX, YMM7
    let code = [
        0xc5, 0xfc, 0x50, 0xc7, // VMOVMSKPS EAX, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_ymm8_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPS EAX, YMM8
    let code = [
        0xc4, 0xc1, 0x7c, 0x50, 0xc0, // VMOVMSKPS EAX, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_ymm9_to_ebx() {
    let mut emu = emu64();
    // VMOVMSKPS EBX, YMM9
    let code = [
        0xc4, 0xc1, 0x7c, 0x50, 0xd9, // VMOVMSKPS EBX, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_ymm10_to_ecx() {
    let mut emu = emu64();
    // VMOVMSKPS ECX, YMM10
    let code = [
        0xc4, 0xc1, 0x7c, 0x50, 0xca, // VMOVMSKPS ECX, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_ymm15_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPS EAX, YMM15
    let code = [
        0xc4, 0xc1, 0x7c, 0x50, 0xc7, // VMOVMSKPS EAX, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMOVMSKPS Tests - After comparison operations
// ============================================================================

#[test]
fn test_vmovmskps_after_vcmpps_eq() {
    let mut emu = emu64();
    // VCMPPS (EQ) followed by VMOVMSKPS
    let code = [
        0xc5, 0xf0, 0xc2, 0xc2, 0x00, // VCMPPS XMM0, XMM1, XMM2, 0 (EQ)
        0xc5, 0xf8, 0x50, 0xc0, // VMOVMSKPS EAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_after_vcmpps_lt() {
    let mut emu = emu64();
    // VCMPPS (LT) followed by VMOVMSKPS
    let code = [
        0xc5, 0xf0, 0xc2, 0xc2, 0x01, // VCMPPS XMM0, XMM1, XMM2, 1 (LT)
        0xc5, 0xf8, 0x50, 0xc0, // VMOVMSKPS EAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_after_vcmpps_ymm() {
    let mut emu = emu64();
    // VCMPPS YMM (EQ) followed by VMOVMSKPS
    let code = [
        0xc5, 0xf4, 0xc2, 0xc2, 0x00, // VCMPPS YMM0, YMM1, YMM2, 0 (EQ)
        0xc5, 0xfc, 0x50, 0xc0, // VMOVMSKPS EAX, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMOVMSKPD Tests - 128-bit XMM registers (2 sign bits)
// ============================================================================

#[test]
fn test_vmovmskpd_xmm0_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPD EAX, XMM0
    let code = [
        0xc5, 0xf9, 0x50, 0xc0, // VMOVMSKPD EAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_xmm1_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPD EAX, XMM1
    let code = [
        0xc5, 0xf9, 0x50, 0xc1, // VMOVMSKPD EAX, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_xmm2_to_ebx() {
    let mut emu = emu64();
    // VMOVMSKPD EBX, XMM2
    let code = [
        0xc5, 0xf9, 0x50, 0xda, // VMOVMSKPD EBX, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_xmm3_to_ecx() {
    let mut emu = emu64();
    // VMOVMSKPD ECX, XMM3
    let code = [
        0xc5, 0xf9, 0x50, 0xcb, // VMOVMSKPD ECX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_xmm4_to_edx() {
    let mut emu = emu64();
    // VMOVMSKPD EDX, XMM4
    let code = [
        0xc5, 0xf9, 0x50, 0xd4, // VMOVMSKPD EDX, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_xmm5_to_esi() {
    let mut emu = emu64();
    // VMOVMSKPD ESI, XMM5
    let code = [
        0xc5, 0xf9, 0x50, 0xf5, // VMOVMSKPD ESI, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_xmm6_to_edi() {
    let mut emu = emu64();
    // VMOVMSKPD EDI, XMM6
    let code = [
        0xc5, 0xf9, 0x50, 0xfe, // VMOVMSKPD EDI, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_xmm7_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPD EAX, XMM7
    let code = [
        0xc5, 0xf9, 0x50, 0xc7, // VMOVMSKPD EAX, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMOVMSKPD Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vmovmskpd_xmm8_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPD EAX, XMM8
    let code = [
        0xc4, 0xc1, 0x79, 0x50, 0xc0, // VMOVMSKPD EAX, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_xmm9_to_ebx() {
    let mut emu = emu64();
    // VMOVMSKPD EBX, XMM9
    let code = [
        0xc4, 0xc1, 0x79, 0x50, 0xd9, // VMOVMSKPD EBX, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_xmm10_to_ecx() {
    let mut emu = emu64();
    // VMOVMSKPD ECX, XMM10
    let code = [
        0xc4, 0xc1, 0x79, 0x50, 0xca, // VMOVMSKPD ECX, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_xmm11_to_edx() {
    let mut emu = emu64();
    // VMOVMSKPD EDX, XMM11
    let code = [
        0xc4, 0xc1, 0x79, 0x50, 0xd3, // VMOVMSKPD EDX, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_xmm12_to_esi() {
    let mut emu = emu64();
    // VMOVMSKPD ESI, XMM12
    let code = [
        0xc4, 0xc1, 0x79, 0x50, 0xf4, // VMOVMSKPD ESI, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_xmm13_to_edi() {
    let mut emu = emu64();
    // VMOVMSKPD EDI, XMM13
    let code = [
        0xc4, 0xc1, 0x79, 0x50, 0xfd, // VMOVMSKPD EDI, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_xmm14_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPD EAX, XMM14
    let code = [
        0xc4, 0xc1, 0x79, 0x50, 0xc6, // VMOVMSKPD EAX, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_xmm15_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPD EAX, XMM15
    let code = [
        0xc4, 0xc1, 0x79, 0x50, 0xc7, // VMOVMSKPD EAX, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMOVMSKPD Tests - 256-bit YMM registers (4 sign bits)
// ============================================================================

#[test]
fn test_vmovmskpd_ymm0_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPD EAX, YMM0
    let code = [
        0xc5, 0xfd, 0x50, 0xc0, // VMOVMSKPD EAX, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_ymm1_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPD EAX, YMM1
    let code = [
        0xc5, 0xfd, 0x50, 0xc1, // VMOVMSKPD EAX, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_ymm2_to_ebx() {
    let mut emu = emu64();
    // VMOVMSKPD EBX, YMM2
    let code = [
        0xc5, 0xfd, 0x50, 0xda, // VMOVMSKPD EBX, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_ymm3_to_ecx() {
    let mut emu = emu64();
    // VMOVMSKPD ECX, YMM3
    let code = [
        0xc5, 0xfd, 0x50, 0xcb, // VMOVMSKPD ECX, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_ymm4_to_edx() {
    let mut emu = emu64();
    // VMOVMSKPD EDX, YMM4
    let code = [
        0xc5, 0xfd, 0x50, 0xd4, // VMOVMSKPD EDX, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_ymm5_to_esi() {
    let mut emu = emu64();
    // VMOVMSKPD ESI, YMM5
    let code = [
        0xc5, 0xfd, 0x50, 0xf5, // VMOVMSKPD ESI, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_ymm6_to_edi() {
    let mut emu = emu64();
    // VMOVMSKPD EDI, YMM6
    let code = [
        0xc5, 0xfd, 0x50, 0xfe, // VMOVMSKPD EDI, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_ymm7_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPD EAX, YMM7
    let code = [
        0xc5, 0xfd, 0x50, 0xc7, // VMOVMSKPD EAX, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_ymm8_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPD EAX, YMM8
    let code = [
        0xc4, 0xc1, 0x7d, 0x50, 0xc0, // VMOVMSKPD EAX, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_ymm9_to_ebx() {
    let mut emu = emu64();
    // VMOVMSKPD EBX, YMM9
    let code = [
        0xc4, 0xc1, 0x7d, 0x50, 0xd9, // VMOVMSKPD EBX, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_ymm10_to_ecx() {
    let mut emu = emu64();
    // VMOVMSKPD ECX, YMM10
    let code = [
        0xc4, 0xc1, 0x7d, 0x50, 0xca, // VMOVMSKPD ECX, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_ymm15_to_eax() {
    let mut emu = emu64();
    // VMOVMSKPD EAX, YMM15
    let code = [
        0xc4, 0xc1, 0x7d, 0x50, 0xc7, // VMOVMSKPD EAX, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMOVMSKPD Tests - After comparison operations
// ============================================================================

#[test]
fn test_vmovmskpd_after_vcmppd_eq() {
    let mut emu = emu64();
    // VCMPPD (EQ) followed by VMOVMSKPD
    let code = [
        0xc5, 0xf1, 0xc2, 0xc2, 0x00, // VCMPPD XMM0, XMM1, XMM2, 0 (EQ)
        0xc5, 0xf9, 0x50, 0xc0, // VMOVMSKPD EAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_after_vcmppd_lt() {
    let mut emu = emu64();
    // VCMPPD (LT) followed by VMOVMSKPD
    let code = [
        0xc5, 0xf1, 0xc2, 0xc2, 0x01, // VCMPPD XMM0, XMM1, XMM2, 1 (LT)
        0xc5, 0xf9, 0x50, 0xc0, // VMOVMSKPD EAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_after_vcmppd_ymm() {
    let mut emu = emu64();
    // VCMPPD YMM (EQ) followed by VMOVMSKPD
    let code = [
        0xc5, 0xf5, 0xc2, 0xc2, 0x00, // VCMPPD YMM0, YMM1, YMM2, 0 (EQ)
        0xc5, 0xfd, 0x50, 0xc0, // VMOVMSKPD EAX, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskps_multiple_extracts() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0x50, 0xc0, // VMOVMSKPS EAX, XMM0
        0xc5, 0xf8, 0x50, 0xd9, // VMOVMSKPS EBX, XMM1
        0xc5, 0xf8, 0x50, 0xca, // VMOVMSKPS ECX, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovmskpd_multiple_extracts() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf9, 0x50, 0xc0, // VMOVMSKPD EAX, XMM0
        0xc5, 0xf9, 0x50, 0xd9, // VMOVMSKPD EBX, XMM1
        0xc5, 0xf9, 0x50, 0xca, // VMOVMSKPD ECX, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
