use crate::*;

// VCVTSI2SS - Convert Scalar Integer to Scalar Single-Precision Floating-Point Value
// VCVTSI2SD - Convert Scalar Integer to Scalar Double-Precision Floating-Point Value
//
// VCVTSI2SS converts a signed doubleword or quadword integer to a scalar single-precision
// floating-point value. The conversion is exact for 32-bit integers, but may round for large values.
// VCVTSI2SD converts a signed doubleword or quadword integer to a scalar double-precision
// floating-point value. The conversion is exact for 32-bit integers and most 64-bit integers.
//
// Opcodes:
// VEX.LIG.F3.0F.W0 2A /r VCVTSI2SS xmm1, xmm2, r/m32 - Convert int32 to scalar single
// VEX.LIG.F3.0F.W1 2A /r VCVTSI2SS xmm1, xmm2, r/m64 - Convert int64 to scalar single
// VEX.LIG.F2.0F.W0 2A /r VCVTSI2SD xmm1, xmm2, r/m32 - Convert int32 to scalar double
// VEX.LIG.F2.0F.W1 2A /r VCVTSI2SD xmm1, xmm2, r/m64 - Convert int64 to scalar double

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VCVTSI2SS Tests - Convert int32 to scalar single (W0 = 32-bit)
// ============================================================================

#[test]
fn test_vcvtsi2ss_xmm0_xmm1_eax() {
    let mut emu = emu64();
    // VCVTSI2SS XMM0, XMM1, EAX
    let code = [
        0xc5, 0xf2, 0x2a, 0xc0, // VCVTSI2SS XMM0, XMM1, EAX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm1_xmm2_ebx() {
    let mut emu = emu64();
    // VCVTSI2SS XMM1, XMM2, EBX
    let code = [
        0xc5, 0xea, 0x2a, 0xcb, // VCVTSI2SS XMM1, XMM2, EBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm2_xmm3_ecx() {
    let mut emu = emu64();
    // VCVTSI2SS XMM2, XMM3, ECX
    let code = [
        0xc5, 0xe2, 0x2a, 0xd1, // VCVTSI2SS XMM2, XMM3, ECX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm3_xmm4_edx() {
    let mut emu = emu64();
    // VCVTSI2SS XMM3, XMM4, EDX
    let code = [
        0xc5, 0xda, 0x2a, 0xda, // VCVTSI2SS XMM3, XMM4, EDX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm4_xmm5_esi() {
    let mut emu = emu64();
    // VCVTSI2SS XMM4, XMM5, ESI
    let code = [
        0xc5, 0xd2, 0x2a, 0xe6, // VCVTSI2SS XMM4, XMM5, ESI
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm5_xmm6_edi() {
    let mut emu = emu64();
    // VCVTSI2SS XMM5, XMM6, EDI
    let code = [
        0xc5, 0xca, 0x2a, 0xef, // VCVTSI2SS XMM5, XMM6, EDI
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm6_xmm7_ebp() {
    let mut emu = emu64();
    // VCVTSI2SS XMM6, XMM7, EBP
    let code = [
        0xc5, 0xc2, 0x2a, 0xf5, // VCVTSI2SS XMM6, XMM7, EBP
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm7_xmm0_esp() {
    let mut emu = emu64();
    // VCVTSI2SS XMM7, XMM0, ESP
    let code = [
        0xc5, 0xfa, 0x2a, 0xfc, // VCVTSI2SS XMM7, XMM0, ESP
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSI2SS Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vcvtsi2ss_xmm8_xmm9_r8d() {
    let mut emu = emu64();
    // VCVTSI2SS XMM8, XMM9, R8D
    let code = [
        0xc4, 0x41, 0x32, 0x2a, 0xc0, // VCVTSI2SS XMM8, XMM9, R8D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm9_xmm10_r9d() {
    let mut emu = emu64();
    // VCVTSI2SS XMM9, XMM10, R9D
    let code = [
        0xc4, 0x41, 0x2a, 0x2a, 0xc9, // VCVTSI2SS XMM9, XMM10, R9D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm10_xmm11_r10d() {
    let mut emu = emu64();
    // VCVTSI2SS XMM10, XMM11, R10D
    let code = [
        0xc4, 0x41, 0x22, 0x2a, 0xd2, // VCVTSI2SS XMM10, XMM11, R10D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm11_xmm12_r11d() {
    let mut emu = emu64();
    // VCVTSI2SS XMM11, XMM12, R11D
    let code = [
        0xc4, 0x41, 0x1a, 0x2a, 0xdb, // VCVTSI2SS XMM11, XMM12, R11D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm12_xmm13_r12d() {
    let mut emu = emu64();
    // VCVTSI2SS XMM12, XMM13, R12D
    let code = [
        0xc4, 0x41, 0x12, 0x2a, 0xe4, // VCVTSI2SS XMM12, XMM13, R12D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm13_xmm14_r13d() {
    let mut emu = emu64();
    // VCVTSI2SS XMM13, XMM14, R13D
    let code = [
        0xc4, 0x41, 0x0a, 0x2a, 0xed, // VCVTSI2SS XMM13, XMM14, R13D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm14_xmm15_r14d() {
    let mut emu = emu64();
    // VCVTSI2SS XMM14, XMM15, R14D
    let code = [
        0xc4, 0x41, 0x02, 0x2a, 0xf6, // VCVTSI2SS XMM14, XMM15, R14D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm15_xmm8_r15d() {
    let mut emu = emu64();
    // VCVTSI2SS XMM15, XMM8, R15D
    let code = [
        0xc4, 0x41, 0x3a, 0x2a, 0xff, // VCVTSI2SS XMM15, XMM8, R15D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSI2SS Tests - Cross-domain registers
// ============================================================================

#[test]
fn test_vcvtsi2ss_xmm0_xmm8_eax() {
    let mut emu = emu64();
    // VCVTSI2SS XMM0, XMM8, EAX
    let code = [
        0xc4, 0xc1, 0x3a, 0x2a, 0xc0, // VCVTSI2SS XMM0, XMM8, EAX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm8_xmm0_r8d() {
    let mut emu = emu64();
    // VCVTSI2SS XMM8, XMM0, R8D
    let code = [
        0xc4, 0x41, 0x7a, 0x2a, 0xc0, // VCVTSI2SS XMM8, XMM0, R8D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSI2SS Tests - Memory operands (32-bit)
// ============================================================================

#[test]
fn test_vcvtsi2ss_xmm0_xmm1_mem32() {
    let mut emu = emu64();
    // VCVTSI2SS XMM0, XMM1, [mem] (reads 32 bits)
    let code = [
        0xc5, 0xf2, 0x2a, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTSI2SS XMM0, XMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0x01, 0x00, 0x00, 0x00, // 1
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm8_xmm9_mem32_negative() {
    let mut emu = emu64();
    // VCVTSI2SS XMM8, XMM9, [mem]
    let code = [
        0xc4, 0x41, 0x32, 0x2a, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTSI2SS XMM8, XMM9, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0xff, 0xff, 0xff, 0xff, // -1
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm2_xmm3_mem32_large() {
    let mut emu = emu64();
    // VCVTSI2SS XMM2, XMM3, [mem]
    let code = [
        0xc5, 0xe2, 0x2a, 0x15, 0x00, 0x40, 0x00, 0x00, // VCVTSI2SS XMM2, XMM3, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0xe8, 0x03, 0x00, 0x00, // 1000
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm4_xmm5_mem32_max() {
    let mut emu = emu64();
    // VCVTSI2SS XMM4, XMM5, [mem]
    let code = [
        0xc5, 0xd2, 0x2a, 0x25, 0x00, 0x40, 0x00, 0x00, // VCVTSI2SS XMM4, XMM5, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0xff, 0xff, 0xff, 0x7f, // 2147483647 (INT32_MAX)
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm6_xmm7_mem32_min() {
    let mut emu = emu64();
    // VCVTSI2SS XMM6, XMM7, [mem]
    let code = [
        0xc5, 0xc2, 0x2a, 0x35, 0x00, 0x40, 0x00, 0x00, // VCVTSI2SS XMM6, XMM7, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0x00, 0x00, 0x00, 0x80, // -2147483648 (INT32_MIN)
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSI2SS Tests - 64-bit integer (W1)
// ============================================================================

#[test]
fn test_vcvtsi2ss_xmm0_xmm1_rax_w1() {
    let mut emu = emu64();
    // VCVTSI2SS XMM0, XMM1, RAX (W1 for 64-bit)
    let code = [
        0xc4, 0xe1, 0xf2, 0x2a, 0xc0, // VCVTSI2SS XMM0, XMM1, RAX (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm1_xmm2_rbx_w1() {
    let mut emu = emu64();
    // VCVTSI2SS XMM1, XMM2, RBX
    let code = [
        0xc4, 0xe1, 0xea, 0x2a, 0xcb, // VCVTSI2SS XMM1, XMM2, RBX (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2ss_xmm8_xmm9_r8_w1() {
    let mut emu = emu64();
    // VCVTSI2SS XMM8, XMM9, R8
    let code = [
        0xc4, 0x41, 0xb2, 0x2a, 0xc0, // VCVTSI2SS XMM8, XMM9, R8 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSI2SD Tests - Convert int32 to scalar double (W0 = 32-bit)
// ============================================================================

#[test]
fn test_vcvtsi2sd_xmm0_xmm1_eax() {
    let mut emu = emu64();
    // VCVTSI2SD XMM0, XMM1, EAX
    let code = [
        0xc5, 0xf3, 0x2a, 0xc0, // VCVTSI2SD XMM0, XMM1, EAX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm1_xmm2_ebx() {
    let mut emu = emu64();
    // VCVTSI2SD XMM1, XMM2, EBX
    let code = [
        0xc5, 0xeb, 0x2a, 0xcb, // VCVTSI2SD XMM1, XMM2, EBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm2_xmm3_ecx() {
    let mut emu = emu64();
    // VCVTSI2SD XMM2, XMM3, ECX
    let code = [
        0xc5, 0xe3, 0x2a, 0xd1, // VCVTSI2SD XMM2, XMM3, ECX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm3_xmm4_edx() {
    let mut emu = emu64();
    // VCVTSI2SD XMM3, XMM4, EDX
    let code = [
        0xc5, 0xdb, 0x2a, 0xda, // VCVTSI2SD XMM3, XMM4, EDX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm4_xmm5_esi() {
    let mut emu = emu64();
    // VCVTSI2SD XMM4, XMM5, ESI
    let code = [
        0xc5, 0xd3, 0x2a, 0xe6, // VCVTSI2SD XMM4, XMM5, ESI
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm5_xmm6_edi() {
    let mut emu = emu64();
    // VCVTSI2SD XMM5, XMM6, EDI
    let code = [
        0xc5, 0xcb, 0x2a, 0xef, // VCVTSI2SD XMM5, XMM6, EDI
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm6_xmm7_ebp() {
    let mut emu = emu64();
    // VCVTSI2SD XMM6, XMM7, EBP
    let code = [
        0xc5, 0xc3, 0x2a, 0xf5, // VCVTSI2SD XMM6, XMM7, EBP
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm7_xmm0_esp() {
    let mut emu = emu64();
    // VCVTSI2SD XMM7, XMM0, ESP
    let code = [
        0xc5, 0xfb, 0x2a, 0xfc, // VCVTSI2SD XMM7, XMM0, ESP
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSI2SD Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vcvtsi2sd_xmm8_xmm9_r8d() {
    let mut emu = emu64();
    // VCVTSI2SD XMM8, XMM9, R8D
    let code = [
        0xc4, 0x41, 0x33, 0x2a, 0xc0, // VCVTSI2SD XMM8, XMM9, R8D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm9_xmm10_r9d() {
    let mut emu = emu64();
    // VCVTSI2SD XMM9, XMM10, R9D
    let code = [
        0xc4, 0x41, 0x2b, 0x2a, 0xc9, // VCVTSI2SD XMM9, XMM10, R9D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm10_xmm11_r10d() {
    let mut emu = emu64();
    // VCVTSI2SD XMM10, XMM11, R10D
    let code = [
        0xc4, 0x41, 0x23, 0x2a, 0xd2, // VCVTSI2SD XMM10, XMM11, R10D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm11_xmm12_r11d() {
    let mut emu = emu64();
    // VCVTSI2SD XMM11, XMM12, R11D
    let code = [
        0xc4, 0x41, 0x1b, 0x2a, 0xdb, // VCVTSI2SD XMM11, XMM12, R11D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm12_xmm13_r12d() {
    let mut emu = emu64();
    // VCVTSI2SD XMM12, XMM13, R12D
    let code = [
        0xc4, 0x41, 0x13, 0x2a, 0xe4, // VCVTSI2SD XMM12, XMM13, R12D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm13_xmm14_r13d() {
    let mut emu = emu64();
    // VCVTSI2SD XMM13, XMM14, R13D
    let code = [
        0xc4, 0x41, 0x0b, 0x2a, 0xed, // VCVTSI2SD XMM13, XMM14, R13D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm14_xmm15_r14d() {
    let mut emu = emu64();
    // VCVTSI2SD XMM14, XMM15, R14D
    let code = [
        0xc4, 0x41, 0x03, 0x2a, 0xf6, // VCVTSI2SD XMM14, XMM15, R14D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm15_xmm8_r15d() {
    let mut emu = emu64();
    // VCVTSI2SD XMM15, XMM8, R15D
    let code = [
        0xc4, 0x41, 0x3b, 0x2a, 0xff, // VCVTSI2SD XMM15, XMM8, R15D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSI2SD Tests - Cross-domain registers
// ============================================================================

#[test]
fn test_vcvtsi2sd_xmm0_xmm8_eax() {
    let mut emu = emu64();
    // VCVTSI2SD XMM0, XMM8, EAX
    let code = [
        0xc4, 0xc1, 0x3b, 0x2a, 0xc0, // VCVTSI2SD XMM0, XMM8, EAX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm8_xmm0_r8d() {
    let mut emu = emu64();
    // VCVTSI2SD XMM8, XMM0, R8D
    let code = [
        0xc4, 0x41, 0x7b, 0x2a, 0xc0, // VCVTSI2SD XMM8, XMM0, R8D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSI2SD Tests - Memory operands (32-bit)
// ============================================================================

#[test]
fn test_vcvtsi2sd_xmm0_xmm1_mem32() {
    let mut emu = emu64();
    // VCVTSI2SD XMM0, XMM1, [mem] (reads 32 bits)
    let code = [
        0xc5, 0xf3, 0x2a, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTSI2SD XMM0, XMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0x01, 0x00, 0x00, 0x00, // 1
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm8_xmm9_mem32_negative() {
    let mut emu = emu64();
    // VCVTSI2SD XMM8, XMM9, [mem]
    let code = [
        0xc4, 0x41, 0x33, 0x2a, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTSI2SD XMM8, XMM9, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0xff, 0xff, 0xff, 0xff, // -1
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm2_xmm3_mem32_large() {
    let mut emu = emu64();
    // VCVTSI2SD XMM2, XMM3, [mem]
    let code = [
        0xc5, 0xe3, 0x2a, 0x15, 0x00, 0x40, 0x00, 0x00, // VCVTSI2SD XMM2, XMM3, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0xe8, 0x03, 0x00, 0x00, // 1000
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm4_xmm5_mem32_max() {
    let mut emu = emu64();
    // VCVTSI2SD XMM4, XMM5, [mem]
    let code = [
        0xc5, 0xd3, 0x2a, 0x25, 0x00, 0x40, 0x00, 0x00, // VCVTSI2SD XMM4, XMM5, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0xff, 0xff, 0xff, 0x7f, // 2147483647 (INT32_MAX)
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm6_xmm7_mem32_min() {
    let mut emu = emu64();
    // VCVTSI2SD XMM6, XMM7, [mem]
    let code = [
        0xc5, 0xc3, 0x2a, 0x35, 0x00, 0x40, 0x00, 0x00, // VCVTSI2SD XMM6, XMM7, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0x00, 0x00, 0x00, 0x80, // -2147483648 (INT32_MIN)
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSI2SD Tests - 64-bit integer (W1)
// ============================================================================

#[test]
fn test_vcvtsi2sd_xmm0_xmm1_rax_w1() {
    let mut emu = emu64();
    // VCVTSI2SD XMM0, XMM1, RAX (W1 for 64-bit)
    let code = [
        0xc4, 0xe1, 0xf3, 0x2a, 0xc0, // VCVTSI2SD XMM0, XMM1, RAX (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm1_xmm2_rbx_w1() {
    let mut emu = emu64();
    // VCVTSI2SD XMM1, XMM2, RBX
    let code = [
        0xc4, 0xe1, 0xeb, 0x2a, 0xcb, // VCVTSI2SD XMM1, XMM2, RBX (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm8_xmm9_r8_w1() {
    let mut emu = emu64();
    // VCVTSI2SD XMM8, XMM9, R8
    let code = [
        0xc4, 0x41, 0xb3, 0x2a, 0xc0, // VCVTSI2SD XMM8, XMM9, R8 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm2_xmm3_rcx_w1() {
    let mut emu = emu64();
    // VCVTSI2SD XMM2, XMM3, RCX
    let code = [
        0xc4, 0xe1, 0xe3, 0x2a, 0xd1, // VCVTSI2SD XMM2, XMM3, RCX (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsi2sd_xmm9_xmm10_r15_w1() {
    let mut emu = emu64();
    // VCVTSI2SD XMM9, XMM10, R15
    let code = [
        0xc4, 0x41, 0xab, 0x2a, 0xcf, // VCVTSI2SD XMM9, XMM10, R15 (W1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
