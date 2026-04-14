use crate::*;

// VCVTSS2SI - Convert Scalar Single-Precision Floating-Point Value to Integer
// VCVTSD2SI - Convert Scalar Double-Precision Floating-Point Value to Integer
//
// VCVTSS2SI converts a scalar single-precision floating-point value to a signed doubleword or
// quadword integer. Rounding is controlled by MXCSR.RC (default: round to nearest even).
// VCVTSD2SI converts a scalar double-precision floating-point value to a signed doubleword or
// quadword integer. Rounding is controlled by MXCSR.RC (default: round to nearest even).
//
// Opcodes:
// VEX.LIG.F3.0F.W0 2D /r VCVTSS2SI r32, xmm1/m32 - Convert scalar single to int32
// VEX.LIG.F3.0F.W1 2D /r VCVTSS2SI r64, xmm1/m32 - Convert scalar single to int64
// VEX.LIG.F2.0F.W0 2D /r VCVTSD2SI r32, xmm1/m64 - Convert scalar double to int32
// VEX.LIG.F2.0F.W1 2D /r VCVTSD2SI r64, xmm1/m64 - Convert scalar double to int64

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VCVTSS2SI Tests - Convert scalar single to int32 (W0)
// ============================================================================

#[test]
fn test_vcvtss2si_eax_xmm0() {
    let mut emu = emu64();
    // VCVTSS2SI EAX, XMM0
    let code = [
        0xc5, 0xfa, 0x2d, 0xc0, // VCVTSS2SI EAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_ebx_xmm1() {
    let mut emu = emu64();
    // VCVTSS2SI EBX, XMM1
    let code = [
        0xc5, 0xfa, 0x2d, 0xd9, // VCVTSS2SI EBX, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_ecx_xmm2() {
    let mut emu = emu64();
    // VCVTSS2SI ECX, XMM2
    let code = [
        0xc5, 0xfa, 0x2d, 0xca, // VCVTSS2SI ECX, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_edx_xmm3() {
    let mut emu = emu64();
    // VCVTSS2SI EDX, XMM3
    let code = [
        0xc5, 0xfa, 0x2d, 0xd3, // VCVTSS2SI EDX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_esi_xmm4() {
    let mut emu = emu64();
    // VCVTSS2SI ESI, XMM4
    let code = [
        0xc5, 0xfa, 0x2d, 0xf4, // VCVTSS2SI ESI, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_edi_xmm5() {
    let mut emu = emu64();
    // VCVTSS2SI EDI, XMM5
    let code = [
        0xc5, 0xfa, 0x2d, 0xfd, // VCVTSS2SI EDI, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_ebp_xmm6() {
    let mut emu = emu64();
    // VCVTSS2SI EBP, XMM6
    let code = [
        0xc5, 0xfa, 0x2d, 0xee, // VCVTSS2SI EBP, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_esp_xmm7() {
    let mut emu = emu64();
    // VCVTSS2SI ESP, XMM7
    let code = [
        0xc5, 0xfa, 0x2d, 0xe7, // VCVTSS2SI ESP, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSS2SI Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vcvtss2si_r8d_xmm8() {
    let mut emu = emu64();
    // VCVTSS2SI R8D, XMM8
    let code = [
        0xc4, 0x41, 0x7a, 0x2d, 0xc0, // VCVTSS2SI R8D, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_r9d_xmm9() {
    let mut emu = emu64();
    // VCVTSS2SI R9D, XMM9
    let code = [
        0xc4, 0x41, 0x7a, 0x2d, 0xc9, // VCVTSS2SI R9D, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_r10d_xmm10() {
    let mut emu = emu64();
    // VCVTSS2SI R10D, XMM10
    let code = [
        0xc4, 0x41, 0x7a, 0x2d, 0xd2, // VCVTSS2SI R10D, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_r11d_xmm11() {
    let mut emu = emu64();
    // VCVTSS2SI R11D, XMM11
    let code = [
        0xc4, 0x41, 0x7a, 0x2d, 0xdb, // VCVTSS2SI R11D, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_r12d_xmm12() {
    let mut emu = emu64();
    // VCVTSS2SI R12D, XMM12
    let code = [
        0xc4, 0x41, 0x7a, 0x2d, 0xe4, // VCVTSS2SI R12D, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_r13d_xmm13() {
    let mut emu = emu64();
    // VCVTSS2SI R13D, XMM13
    let code = [
        0xc4, 0x41, 0x7a, 0x2d, 0xed, // VCVTSS2SI R13D, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_r14d_xmm14() {
    let mut emu = emu64();
    // VCVTSS2SI R14D, XMM14
    let code = [
        0xc4, 0x41, 0x7a, 0x2d, 0xf6, // VCVTSS2SI R14D, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_r15d_xmm15() {
    let mut emu = emu64();
    // VCVTSS2SI R15D, XMM15
    let code = [
        0xc4, 0x41, 0x7a, 0x2d, 0xff, // VCVTSS2SI R15D, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSS2SI Tests - Cross-domain registers
// ============================================================================

#[test]
fn test_vcvtss2si_eax_xmm8() {
    let mut emu = emu64();
    // VCVTSS2SI EAX, XMM8
    let code = [
        0xc4, 0xc1, 0x7a, 0x2d, 0xc0, // VCVTSS2SI EAX, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_r8d_xmm0() {
    let mut emu = emu64();
    // VCVTSS2SI R8D, XMM0
    let code = [
        0xc4, 0xc1, 0x7a, 0x2d, 0xc0, // VCVTSS2SI R8D, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSS2SI Tests - Memory operands (32-bit)
// ============================================================================

#[test]
fn test_vcvtss2si_eax_mem() {
    let mut emu = emu64();
    // VCVTSS2SI EAX, [mem] (reads 32 bits)
    let code = [
        0xc5, 0xfa, 0x2d, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTSS2SI EAX, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_ebx_mem_negative() {
    let mut emu = emu64();
    // VCVTSS2SI EBX, [mem]
    let code = [
        0xc5, 0xfa, 0x2d, 0x1d, 0x00, 0x40, 0x00, 0x00, // VCVTSS2SI EBX, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0x00, 0x00, 0x80, 0xbf, // -1.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_ecx_mem_round_nearest() {
    let mut emu = emu64();
    // VCVTSS2SI ECX, [mem] - Test rounding to nearest even
    let code = [
        0xc5, 0xfa, 0x2d, 0x0d, 0x00, 0x40, 0x00, 0x00, // VCVTSS2SI ECX, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0x00, 0x00, 0x00, 0x40, // 2.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_edx_mem_large() {
    let mut emu = emu64();
    // VCVTSS2SI EDX, [mem]
    let code = [
        0xc5, 0xfa, 0x2d, 0x15, 0x00, 0x40, 0x00, 0x00, // VCVTSS2SI EDX, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0x00, 0x00, 0x7a, 0x44, // 1000.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_r8d_mem() {
    let mut emu = emu64();
    // VCVTSS2SI R8D, [mem]
    let code = [
        0xc4, 0x41, 0x7a, 0x2d, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTSS2SI R8D, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0x00, 0x00, 0xc8, 0x42, // 100.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSS2SI Tests - 64-bit integer (W1)
// ============================================================================

#[test]
fn test_vcvtss2si_rax_xmm0_w1() {
    let mut emu = emu64();
    // VCVTSS2SI RAX, XMM0 (W1 for 64-bit)
    let code = [
        0xc4, 0xe1, 0xfa, 0x2d, 0xc0, // VCVTSS2SI RAX, XMM0 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_rbx_xmm1_w1() {
    let mut emu = emu64();
    // VCVTSS2SI RBX, XMM1
    let code = [
        0xc4, 0xe1, 0xfa, 0x2d, 0xd9, // VCVTSS2SI RBX, XMM1 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_r8_xmm8_w1() {
    let mut emu = emu64();
    // VCVTSS2SI R8, XMM8
    let code = [
        0xc4, 0x41, 0xfa, 0x2d, 0xc0, // VCVTSS2SI R8, XMM8 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2si_r15_xmm15_w1() {
    let mut emu = emu64();
    // VCVTSS2SI R15, XMM15
    let code = [
        0xc4, 0x41, 0xfa, 0x2d, 0xff, // VCVTSS2SI R15, XMM15 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSD2SI Tests - Convert scalar double to int32 (W0)
// ============================================================================

#[test]
fn test_vcvtsd2si_eax_xmm0() {
    let mut emu = emu64();
    // VCVTSD2SI EAX, XMM0
    let code = [
        0xc5, 0xfb, 0x2d, 0xc0, // VCVTSD2SI EAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_ebx_xmm1() {
    let mut emu = emu64();
    // VCVTSD2SI EBX, XMM1
    let code = [
        0xc5, 0xfb, 0x2d, 0xd9, // VCVTSD2SI EBX, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_ecx_xmm2() {
    let mut emu = emu64();
    // VCVTSD2SI ECX, XMM2
    let code = [
        0xc5, 0xfb, 0x2d, 0xca, // VCVTSD2SI ECX, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_edx_xmm3() {
    let mut emu = emu64();
    // VCVTSD2SI EDX, XMM3
    let code = [
        0xc5, 0xfb, 0x2d, 0xd3, // VCVTSD2SI EDX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_esi_xmm4() {
    let mut emu = emu64();
    // VCVTSD2SI ESI, XMM4
    let code = [
        0xc5, 0xfb, 0x2d, 0xf4, // VCVTSD2SI ESI, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_edi_xmm5() {
    let mut emu = emu64();
    // VCVTSD2SI EDI, XMM5
    let code = [
        0xc5, 0xfb, 0x2d, 0xfd, // VCVTSD2SI EDI, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_ebp_xmm6() {
    let mut emu = emu64();
    // VCVTSD2SI EBP, XMM6
    let code = [
        0xc5, 0xfb, 0x2d, 0xee, // VCVTSD2SI EBP, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_esp_xmm7() {
    let mut emu = emu64();
    // VCVTSD2SI ESP, XMM7
    let code = [
        0xc5, 0xfb, 0x2d, 0xe7, // VCVTSD2SI ESP, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSD2SI Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vcvtsd2si_r8d_xmm8() {
    let mut emu = emu64();
    // VCVTSD2SI R8D, XMM8
    let code = [
        0xc4, 0x41, 0x7b, 0x2d, 0xc0, // VCVTSD2SI R8D, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_r9d_xmm9() {
    let mut emu = emu64();
    // VCVTSD2SI R9D, XMM9
    let code = [
        0xc4, 0x41, 0x7b, 0x2d, 0xc9, // VCVTSD2SI R9D, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_r10d_xmm10() {
    let mut emu = emu64();
    // VCVTSD2SI R10D, XMM10
    let code = [
        0xc4, 0x41, 0x7b, 0x2d, 0xd2, // VCVTSD2SI R10D, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_r11d_xmm11() {
    let mut emu = emu64();
    // VCVTSD2SI R11D, XMM11
    let code = [
        0xc4, 0x41, 0x7b, 0x2d, 0xdb, // VCVTSD2SI R11D, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_r12d_xmm12() {
    let mut emu = emu64();
    // VCVTSD2SI R12D, XMM12
    let code = [
        0xc4, 0x41, 0x7b, 0x2d, 0xe4, // VCVTSD2SI R12D, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_r13d_xmm13() {
    let mut emu = emu64();
    // VCVTSD2SI R13D, XMM13
    let code = [
        0xc4, 0x41, 0x7b, 0x2d, 0xed, // VCVTSD2SI R13D, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_r14d_xmm14() {
    let mut emu = emu64();
    // VCVTSD2SI R14D, XMM14
    let code = [
        0xc4, 0x41, 0x7b, 0x2d, 0xf6, // VCVTSD2SI R14D, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_r15d_xmm15() {
    let mut emu = emu64();
    // VCVTSD2SI R15D, XMM15
    let code = [
        0xc4, 0x41, 0x7b, 0x2d, 0xff, // VCVTSD2SI R15D, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSD2SI Tests - Cross-domain registers
// ============================================================================

#[test]
fn test_vcvtsd2si_eax_xmm8() {
    let mut emu = emu64();
    // VCVTSD2SI EAX, XMM8
    let code = [
        0xc4, 0xc1, 0x7b, 0x2d, 0xc0, // VCVTSD2SI EAX, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_r8d_xmm0() {
    let mut emu = emu64();
    // VCVTSD2SI R8D, XMM0
    let code = [
        0xc4, 0xc1, 0x7b, 0x2d, 0xc0, // VCVTSD2SI R8D, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSD2SI Tests - Memory operands (64-bit)
// ============================================================================

#[test]
fn test_vcvtsd2si_eax_mem() {
    let mut emu = emu64();
    // VCVTSD2SI EAX, [mem] (reads 64 bits)
    let code = [
        0xc5, 0xfb, 0x2d, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTSD2SI EAX, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_ebx_mem_negative() {
    let mut emu = emu64();
    // VCVTSD2SI EBX, [mem]
    let code = [
        0xc5, 0xfb, 0x2d, 0x1d, 0x00, 0x40, 0x00, 0x00, // VCVTSD2SI EBX, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0xbf, // -1.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_ecx_mem_round_nearest() {
    let mut emu = emu64();
    // VCVTSD2SI ECX, [mem] - Test rounding to nearest even
    let code = [
        0xc5, 0xfb, 0x2d, 0x0d, 0x00, 0x40, 0x00, 0x00, // VCVTSD2SI ECX, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // 2.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_edx_mem_large() {
    let mut emu = emu64();
    // VCVTSD2SI EDX, [mem]
    let code = [
        0xc5, 0xfb, 0x2d, 0x15, 0x00, 0x40, 0x00, 0x00, // VCVTSD2SI EDX, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x8f, 0x40, // 1000.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_r8d_mem() {
    let mut emu = emu64();
    // VCVTSD2SI R8D, [mem]
    let code = [
        0xc4, 0x41, 0x7b, 0x2d, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTSD2SI R8D, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x59, 0x40, // 100.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSD2SI Tests - 64-bit integer (W1)
// ============================================================================

#[test]
fn test_vcvtsd2si_rax_xmm0_w1() {
    let mut emu = emu64();
    // VCVTSD2SI RAX, XMM0 (W1 for 64-bit)
    let code = [
        0xc4, 0xe1, 0xfb, 0x2d, 0xc0, // VCVTSD2SI RAX, XMM0 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_rbx_xmm1_w1() {
    let mut emu = emu64();
    // VCVTSD2SI RBX, XMM1
    let code = [
        0xc4, 0xe1, 0xfb, 0x2d, 0xd9, // VCVTSD2SI RBX, XMM1 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_r8_xmm8_w1() {
    let mut emu = emu64();
    // VCVTSD2SI R8, XMM8
    let code = [
        0xc4, 0x41, 0xfb, 0x2d, 0xc0, // VCVTSD2SI R8, XMM8 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_r15_xmm15_w1() {
    let mut emu = emu64();
    // VCVTSD2SI R15, XMM15
    let code = [
        0xc4, 0x41, 0xfb, 0x2d, 0xff, // VCVTSD2SI R15, XMM15 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2si_rcx_xmm2_w1() {
    let mut emu = emu64();
    // VCVTSD2SI RCX, XMM2
    let code = [
        0xc4, 0xe1, 0xfb, 0x2d, 0xca, // VCVTSD2SI RCX, XMM2 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
