use crate::*;

// VCVTTSS2SI - Convert with Truncation Scalar Single-Precision to Integer
// VCVTTSD2SI - Convert with Truncation Scalar Double-Precision to Integer
//
// VCVTTSS2SI converts a scalar single-precision floating-point value to a signed doubleword or
// quadword integer using truncation (round toward zero).
// VCVTTSD2SI converts a scalar double-precision floating-point value to a signed doubleword or
// quadword integer using truncation (round toward zero).
//
// Opcodes:
// VEX.LIG.F3.0F.W0 2C /r VCVTTSS2SI r32, xmm1/m32 - Convert with truncation scalar single to int32
// VEX.LIG.F3.0F.W1 2C /r VCVTTSS2SI r64, xmm1/m32 - Convert with truncation scalar single to int64
// VEX.LIG.F2.0F.W0 2C /r VCVTTSD2SI r32, xmm1/m64 - Convert with truncation scalar double to int32
// VEX.LIG.F2.0F.W1 2C /r VCVTTSD2SI r64, xmm1/m64 - Convert with truncation scalar double to int64

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VCVTTSS2SI Tests - Convert scalar single to int32 (W0)
// ============================================================================

#[test]
fn test_vcvttss2si_eax_xmm0() {
    let mut emu = emu64();
    // VCVTTSS2SI EAX, XMM0
    let code = [
        0xc5, 0xfa, 0x2c, 0xc0, // VCVTTSS2SI EAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_ebx_xmm1() {
    let mut emu = emu64();
    // VCVTTSS2SI EBX, XMM1
    let code = [
        0xc5, 0xfa, 0x2c, 0xd9, // VCVTTSS2SI EBX, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_ecx_xmm2() {
    let mut emu = emu64();
    // VCVTTSS2SI ECX, XMM2
    let code = [
        0xc5, 0xfa, 0x2c, 0xca, // VCVTTSS2SI ECX, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_edx_xmm3() {
    let mut emu = emu64();
    // VCVTTSS2SI EDX, XMM3
    let code = [
        0xc5, 0xfa, 0x2c, 0xd3, // VCVTTSS2SI EDX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_esi_xmm4() {
    let mut emu = emu64();
    // VCVTTSS2SI ESI, XMM4
    let code = [
        0xc5, 0xfa, 0x2c, 0xf4, // VCVTTSS2SI ESI, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_edi_xmm5() {
    let mut emu = emu64();
    // VCVTTSS2SI EDI, XMM5
    let code = [
        0xc5, 0xfa, 0x2c, 0xfd, // VCVTTSS2SI EDI, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_ebp_xmm6() {
    let mut emu = emu64();
    // VCVTTSS2SI EBP, XMM6
    let code = [
        0xc5, 0xfa, 0x2c, 0xee, // VCVTTSS2SI EBP, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_esp_xmm7() {
    let mut emu = emu64();
    // VCVTTSS2SI ESP, XMM7
    let code = [
        0xc5, 0xfa, 0x2c, 0xe7, // VCVTTSS2SI ESP, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTTSS2SI Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vcvttss2si_r8d_xmm8() {
    let mut emu = emu64();
    // VCVTTSS2SI R8D, XMM8
    let code = [
        0xc4, 0x41, 0x7a, 0x2c, 0xc0, // VCVTTSS2SI R8D, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_r9d_xmm9() {
    let mut emu = emu64();
    // VCVTTSS2SI R9D, XMM9
    let code = [
        0xc4, 0x41, 0x7a, 0x2c, 0xc9, // VCVTTSS2SI R9D, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_r10d_xmm10() {
    let mut emu = emu64();
    // VCVTTSS2SI R10D, XMM10
    let code = [
        0xc4, 0x41, 0x7a, 0x2c, 0xd2, // VCVTTSS2SI R10D, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_r11d_xmm11() {
    let mut emu = emu64();
    // VCVTTSS2SI R11D, XMM11
    let code = [
        0xc4, 0x41, 0x7a, 0x2c, 0xdb, // VCVTTSS2SI R11D, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_r12d_xmm12() {
    let mut emu = emu64();
    // VCVTTSS2SI R12D, XMM12
    let code = [
        0xc4, 0x41, 0x7a, 0x2c, 0xe4, // VCVTTSS2SI R12D, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_r13d_xmm13() {
    let mut emu = emu64();
    // VCVTTSS2SI R13D, XMM13
    let code = [
        0xc4, 0x41, 0x7a, 0x2c, 0xed, // VCVTTSS2SI R13D, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_r14d_xmm14() {
    let mut emu = emu64();
    // VCVTTSS2SI R14D, XMM14
    let code = [
        0xc4, 0x41, 0x7a, 0x2c, 0xf6, // VCVTTSS2SI R14D, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_r15d_xmm15() {
    let mut emu = emu64();
    // VCVTTSS2SI R15D, XMM15
    let code = [
        0xc4, 0x41, 0x7a, 0x2c, 0xff, // VCVTTSS2SI R15D, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTTSS2SI Tests - Cross-domain registers
// ============================================================================

#[test]
fn test_vcvttss2si_eax_xmm8() {
    let mut emu = emu64();
    // VCVTTSS2SI EAX, XMM8
    let code = [
        0xc4, 0xc1, 0x7a, 0x2c, 0xc0, // VCVTTSS2SI EAX, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_r8d_xmm0() {
    let mut emu = emu64();
    // VCVTTSS2SI R8D, XMM0
    let code = [
        0xc4, 0xc1, 0x7a, 0x2c, 0xc0, // VCVTTSS2SI R8D, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTTSS2SI Tests - Memory operands (32-bit)
// ============================================================================

#[test]
fn test_vcvttss2si_eax_mem() {
    let mut emu = emu64();
    // VCVTTSS2SI EAX, [mem] (reads 32 bits)
    let code = [
        0xc5, 0xfa, 0x2c, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTTSS2SI EAX, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0x9a, 0x99, 0x99, 0x3f, // 1.2
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_ebx_mem_negative() {
    let mut emu = emu64();
    // VCVTTSS2SI EBX, [mem] - Truncate -2.6 to -2
    let code = [
        0xc5, 0xfa, 0x2c, 0x1d, 0x00, 0x40, 0x00, 0x00, // VCVTTSS2SI EBX, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0x66, 0x66, 0x26, 0xc0, // -2.6
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_ecx_mem_positive() {
    let mut emu = emu64();
    // VCVTTSS2SI ECX, [mem] - Truncate 2.9 to 2
    let code = [
        0xc5, 0xfa, 0x2c, 0x0d, 0x00, 0x40, 0x00, 0x00, // VCVTTSS2SI ECX, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0x66, 0x66, 0x3a, 0x40, // 2.9
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_edx_mem_half() {
    let mut emu = emu64();
    // VCVTTSS2SI EDX, [mem] - Truncate 0.6 to 0
    let code = [
        0xc5, 0xfa, 0x2c, 0x15, 0x00, 0x40, 0x00, 0x00, // VCVTTSS2SI EDX, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0x9a, 0x99, 0x19, 0x3f, // 0.6
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_esi_mem_large() {
    let mut emu = emu64();
    // VCVTTSS2SI ESI, [mem] - Truncate 100.8 to 100
    let code = [
        0xc5, 0xfa, 0x2c, 0x35, 0x00, 0x40, 0x00, 0x00, // VCVTTSS2SI ESI, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0x9a, 0x99, 0xc9, 0x42, // 100.8
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_r8d_mem() {
    let mut emu = emu64();
    // VCVTTSS2SI R8D, [mem]
    let code = [
        0xc4, 0x41, 0x7a, 0x2c, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTTSS2SI R8D, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0xcd, 0xcc, 0x4c, 0x40, // 3.2
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VCVTTSS2SI Tests - 64-bit integer (W1)
// ============================================================================

#[test]
fn test_vcvttss2si_rax_xmm0_w1() {
    let mut emu = emu64();
    // VCVTTSS2SI RAX, XMM0 (W1 for 64-bit)
    let code = [
        0xc4, 0xe1, 0xfa, 0x2c, 0xc0, // VCVTTSS2SI RAX, XMM0 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_rbx_xmm1_w1() {
    let mut emu = emu64();
    // VCVTTSS2SI RBX, XMM1
    let code = [
        0xc4, 0xe1, 0xfa, 0x2c, 0xd9, // VCVTTSS2SI RBX, XMM1 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_r8_xmm8_w1() {
    let mut emu = emu64();
    // VCVTTSS2SI R8, XMM8
    let code = [
        0xc4, 0x41, 0xfa, 0x2c, 0xc0, // VCVTTSS2SI R8, XMM8 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttss2si_r15_xmm15_w1() {
    let mut emu = emu64();
    // VCVTTSS2SI R15, XMM15
    let code = [
        0xc4, 0x41, 0xfa, 0x2c, 0xff, // VCVTTSS2SI R15, XMM15 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTTSD2SI Tests - Convert scalar double to int32 (W0)
// ============================================================================

#[test]
fn test_vcvttsd2si_eax_xmm0() {
    let mut emu = emu64();
    // VCVTTSD2SI EAX, XMM0
    let code = [
        0xc5, 0xfb, 0x2c, 0xc0, // VCVTTSD2SI EAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_ebx_xmm1() {
    let mut emu = emu64();
    // VCVTTSD2SI EBX, XMM1
    let code = [
        0xc5, 0xfb, 0x2c, 0xd9, // VCVTTSD2SI EBX, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_ecx_xmm2() {
    let mut emu = emu64();
    // VCVTTSD2SI ECX, XMM2
    let code = [
        0xc5, 0xfb, 0x2c, 0xca, // VCVTTSD2SI ECX, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_edx_xmm3() {
    let mut emu = emu64();
    // VCVTTSD2SI EDX, XMM3
    let code = [
        0xc5, 0xfb, 0x2c, 0xd3, // VCVTTSD2SI EDX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_esi_xmm4() {
    let mut emu = emu64();
    // VCVTTSD2SI ESI, XMM4
    let code = [
        0xc5, 0xfb, 0x2c, 0xf4, // VCVTTSD2SI ESI, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_edi_xmm5() {
    let mut emu = emu64();
    // VCVTTSD2SI EDI, XMM5
    let code = [
        0xc5, 0xfb, 0x2c, 0xfd, // VCVTTSD2SI EDI, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_ebp_xmm6() {
    let mut emu = emu64();
    // VCVTTSD2SI EBP, XMM6
    let code = [
        0xc5, 0xfb, 0x2c, 0xee, // VCVTTSD2SI EBP, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_esp_xmm7() {
    let mut emu = emu64();
    // VCVTTSD2SI ESP, XMM7
    let code = [
        0xc5, 0xfb, 0x2c, 0xe7, // VCVTTSD2SI ESP, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTTSD2SI Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vcvttsd2si_r8d_xmm8() {
    let mut emu = emu64();
    // VCVTTSD2SI R8D, XMM8
    let code = [
        0xc4, 0x41, 0x7b, 0x2c, 0xc0, // VCVTTSD2SI R8D, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_r9d_xmm9() {
    let mut emu = emu64();
    // VCVTTSD2SI R9D, XMM9
    let code = [
        0xc4, 0x41, 0x7b, 0x2c, 0xc9, // VCVTTSD2SI R9D, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_r10d_xmm10() {
    let mut emu = emu64();
    // VCVTTSD2SI R10D, XMM10
    let code = [
        0xc4, 0x41, 0x7b, 0x2c, 0xd2, // VCVTTSD2SI R10D, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_r11d_xmm11() {
    let mut emu = emu64();
    // VCVTTSD2SI R11D, XMM11
    let code = [
        0xc4, 0x41, 0x7b, 0x2c, 0xdb, // VCVTTSD2SI R11D, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_r12d_xmm12() {
    let mut emu = emu64();
    // VCVTTSD2SI R12D, XMM12
    let code = [
        0xc4, 0x41, 0x7b, 0x2c, 0xe4, // VCVTTSD2SI R12D, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_r13d_xmm13() {
    let mut emu = emu64();
    // VCVTTSD2SI R13D, XMM13
    let code = [
        0xc4, 0x41, 0x7b, 0x2c, 0xed, // VCVTTSD2SI R13D, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_r14d_xmm14() {
    let mut emu = emu64();
    // VCVTTSD2SI R14D, XMM14
    let code = [
        0xc4, 0x41, 0x7b, 0x2c, 0xf6, // VCVTTSD2SI R14D, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_r15d_xmm15() {
    let mut emu = emu64();
    // VCVTTSD2SI R15D, XMM15
    let code = [
        0xc4, 0x41, 0x7b, 0x2c, 0xff, // VCVTTSD2SI R15D, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTTSD2SI Tests - Cross-domain registers
// ============================================================================

#[test]
fn test_vcvttsd2si_eax_xmm8() {
    let mut emu = emu64();
    // VCVTTSD2SI EAX, XMM8
    let code = [
        0xc4, 0xc1, 0x7b, 0x2c, 0xc0, // VCVTTSD2SI EAX, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_r8d_xmm0() {
    let mut emu = emu64();
    // VCVTTSD2SI R8D, XMM0
    let code = [
        0xc4, 0xc1, 0x7b, 0x2c, 0xc0, // VCVTTSD2SI R8D, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTTSD2SI Tests - Memory operands (64-bit)
// ============================================================================

#[test]
fn test_vcvttsd2si_eax_mem() {
    let mut emu = emu64();
    // VCVTTSD2SI EAX, [mem] (reads 64 bits)
    let code = [
        0xc5, 0xfb, 0x2c, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTTSD2SI EAX, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0xf3, 0x3f, // 1.2
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_ebx_mem_negative() {
    let mut emu = emu64();
    // VCVTTSD2SI EBX, [mem] - Truncate -2.6 to -2
    let code = [
        0xc5, 0xfb, 0x2c, 0x1d, 0x00, 0x40, 0x00, 0x00, // VCVTTSD2SI EBX, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x9a, 0x99, 0x99, 0x99, 0x99, 0x99, 0x04, 0xc0, // -2.6
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_ecx_mem_positive() {
    let mut emu = emu64();
    // VCVTTSD2SI ECX, [mem] - Truncate 2.9 to 2
    let code = [
        0xc5, 0xfb, 0x2c, 0x0d, 0x00, 0x40, 0x00, 0x00, // VCVTTSD2SI ECX, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x07, 0x40, // 2.9
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_edx_mem_half() {
    let mut emu = emu64();
    // VCVTTSD2SI EDX, [mem] - Truncate 0.6 to 0
    let code = [
        0xc5, 0xfb, 0x2c, 0x15, 0x00, 0x40, 0x00, 0x00, // VCVTTSD2SI EDX, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0xe3, 0x3f, // 0.6
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_esi_mem_large() {
    let mut emu = emu64();
    // VCVTTSD2SI ESI, [mem] - Truncate 100.8 to 100
    let code = [
        0xc5, 0xfb, 0x2c, 0x35, 0x00, 0x40, 0x00, 0x00, // VCVTTSD2SI ESI, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x59, 0x40, // 100.8
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_r8d_mem() {
    let mut emu = emu64();
    // VCVTTSD2SI R8D, [mem]
    let code = [
        0xc4, 0x41, 0x7b, 0x2c, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTTSD2SI R8D, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x9a, 0x99, 0x99, 0x99, 0x99, 0x99, 0x09, 0x40, // 3.2
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VCVTTSD2SI Tests - 64-bit integer (W1)
// ============================================================================

#[test]
fn test_vcvttsd2si_rax_xmm0_w1() {
    let mut emu = emu64();
    // VCVTTSD2SI RAX, XMM0 (W1 for 64-bit)
    let code = [
        0xc4, 0xe1, 0xfb, 0x2c, 0xc0, // VCVTTSD2SI RAX, XMM0 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_rbx_xmm1_w1() {
    let mut emu = emu64();
    // VCVTTSD2SI RBX, XMM1
    let code = [
        0xc4, 0xe1, 0xfb, 0x2c, 0xd9, // VCVTTSD2SI RBX, XMM1 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_r8_xmm8_w1() {
    let mut emu = emu64();
    // VCVTTSD2SI R8, XMM8
    let code = [
        0xc4, 0x41, 0xfb, 0x2c, 0xc0, // VCVTTSD2SI R8, XMM8 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_r15_xmm15_w1() {
    let mut emu = emu64();
    // VCVTTSD2SI R15, XMM15
    let code = [
        0xc4, 0x41, 0xfb, 0x2c, 0xff, // VCVTTSD2SI R15, XMM15 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvttsd2si_rcx_xmm2_w1() {
    let mut emu = emu64();
    // VCVTTSD2SI RCX, XMM2
    let code = [
        0xc4, 0xe1, 0xfb, 0x2c, 0xca, // VCVTTSD2SI RCX, XMM2 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
