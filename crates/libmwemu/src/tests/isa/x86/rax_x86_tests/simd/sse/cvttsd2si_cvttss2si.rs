use crate::*;

// CVTTSD2SI - Convert with Truncation Scalar Double FP to Signed Integer
// CVTTSS2SI - Convert with Truncation Scalar Single FP to Signed Integer
//
// CVTTSD2SI converts a single double precision floating-point value from XMM register
// or memory to a signed doubleword or quadword integer in a general purpose register
// using truncation (rounding toward zero).
//
// CVTTSS2SI converts a single single precision floating-point value from XMM register
// or memory to a signed doubleword or quadword integer in a general purpose register
// using truncation (rounding toward zero).
//
// Opcodes:
// F2 0F 2C /r    CVTTSD2SI r32, xmm/m64    - Convert one double FP to dword (truncate)
// F2 REX.W 0F 2C /r    CVTTSD2SI r64, xmm/m64    - Convert one double FP to qword (truncate)
// F3 0F 2C /r    CVTTSS2SI r32, xmm/m32    - Convert one single FP to dword (truncate)
// F3 REX.W 0F 2C /r    CVTTSS2SI r64, xmm/m32    - Convert one single FP to qword (truncate)

const DATA_ADDR: u64 = 0x3000;

// ============================================================================
// CVTTSD2SI - XMM to 32-bit Register Tests
// ============================================================================

#[test]
fn test_cvttsd2si_xmm0_to_eax() {
    let mut emu = emu64();
    // CVTTSD2SI EAX, XMM0
    let code = [
        0xf2, 0x0f, 0x2c, 0xc0, // CVTTSD2SI EAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttsd2si_xmm1_to_ebx() {
    let mut emu = emu64();
    // CVTTSD2SI EBX, XMM1
    let code = [
        0xf2, 0x0f, 0x2c, 0xd9, // CVTTSD2SI EBX, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttsd2si_xmm2_to_ecx() {
    let mut emu = emu64();
    // CVTTSD2SI ECX, XMM2
    let code = [
        0xf2, 0x0f, 0x2c, 0xca, // CVTTSD2SI ECX, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttsd2si_xmm3_to_edx() {
    let mut emu = emu64();
    // CVTTSD2SI EDX, XMM3
    let code = [
        0xf2, 0x0f, 0x2c, 0xd3, // CVTTSD2SI EDX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttsd2si_xmm4_to_esi() {
    let mut emu = emu64();
    // CVTTSD2SI ESI, XMM4
    let code = [
        0xf2, 0x0f, 0x2c, 0xf4, // CVTTSD2SI ESI, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttsd2si_xmm5_to_edi() {
    let mut emu = emu64();
    // CVTTSD2SI EDI, XMM5
    let code = [
        0xf2, 0x0f, 0x2c, 0xfd, // CVTTSD2SI EDI, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttsd2si_xmm6_to_ebp() {
    let mut emu = emu64();
    // CVTTSD2SI EBP, XMM6
    let code = [
        0xf2, 0x0f, 0x2c, 0xee, // CVTTSD2SI EBP, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttsd2si_xmm7_to_esp() {
    let mut emu = emu64();
    // CVTTSD2SI ESP, XMM7
    let code = [
        0xf2, 0x0f, 0x2c, 0xe7, // CVTTSD2SI ESP, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttsd2si_xmm8_to_r8d() {
    let mut emu = emu64();
    // CVTTSD2SI R8D, XMM8
    let code = [
        0xf2, 0x45, 0x0f, 0x2c, 0xc0, // CVTTSD2SI R8D, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttsd2si_xmm15_to_r15d() {
    let mut emu = emu64();
    // CVTTSD2SI R15D, XMM15
    let code = [
        0xf2, 0x45, 0x0f, 0x2c, 0xff, // CVTTSD2SI R15D, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// CVTTSD2SI - XMM to 64-bit Register Tests
// ============================================================================

#[test]
fn test_cvttsd2si_xmm0_to_rax() {
    let mut emu = emu64();
    // CVTTSD2SI RAX, XMM0
    let code = [
        0xf2, 0x48, 0x0f, 0x2c, 0xc0, // CVTTSD2SI RAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttsd2si_xmm1_to_rbx() {
    let mut emu = emu64();
    // CVTTSD2SI RBX, XMM1
    let code = [
        0xf2, 0x48, 0x0f, 0x2c, 0xd9, // CVTTSD2SI RBX, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttsd2si_xmm2_to_rcx() {
    let mut emu = emu64();
    // CVTTSD2SI RCX, XMM2
    let code = [
        0xf2, 0x48, 0x0f, 0x2c, 0xca, // CVTTSD2SI RCX, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttsd2si_xmm8_to_r8() {
    let mut emu = emu64();
    // CVTTSD2SI R8, XMM8
    let code = [
        0xf2, 0x4d, 0x0f, 0x2c, 0xc0, // CVTTSD2SI R8, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttsd2si_xmm15_to_r15() {
    let mut emu = emu64();
    // CVTTSD2SI R15, XMM15
    let code = [
        0xf2, 0x4d, 0x0f, 0x2c, 0xff, // CVTTSD2SI R15, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// CVTTSD2SI - Memory to Register Tests
// ============================================================================

#[test]
fn test_cvttsd2si_mem_to_eax() {
    let mut emu = emu64();
    // CVTTSD2SI EAX, [0x3000]
    let code = [
        0xf2, 0x0f, 0x2c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTTSD2SI EAX, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttsd2si_mem_to_rax() {
    let mut emu = emu64();
    // CVTTSD2SI RAX, [0x3000]
    let code = [
        0xf2, 0x48, 0x0f, 0x2c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTTSD2SI RAX, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttsd2si_mem_to_r15() {
    let mut emu = emu64();
    // CVTTSD2SI R15, [0x3000]
    let code = [
        0xf2, 0x4c, 0x0f, 0x2c, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTTSD2SI R15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// CVTTSS2SI - XMM to 32-bit Register Tests
// ============================================================================

#[test]
fn test_cvttss2si_xmm0_to_eax() {
    let mut emu = emu64();
    // CVTTSS2SI EAX, XMM0
    let code = [
        0xf3, 0x0f, 0x2c, 0xc0, // CVTTSS2SI EAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttss2si_xmm1_to_ebx() {
    let mut emu = emu64();
    // CVTTSS2SI EBX, XMM1
    let code = [
        0xf3, 0x0f, 0x2c, 0xd9, // CVTTSS2SI EBX, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttss2si_xmm2_to_ecx() {
    let mut emu = emu64();
    // CVTTSS2SI ECX, XMM2
    let code = [
        0xf3, 0x0f, 0x2c, 0xca, // CVTTSS2SI ECX, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttss2si_xmm3_to_edx() {
    let mut emu = emu64();
    // CVTTSS2SI EDX, XMM3
    let code = [
        0xf3, 0x0f, 0x2c, 0xd3, // CVTTSS2SI EDX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttss2si_xmm4_to_esi() {
    let mut emu = emu64();
    // CVTTSS2SI ESI, XMM4
    let code = [
        0xf3, 0x0f, 0x2c, 0xf4, // CVTTSS2SI ESI, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttss2si_xmm5_to_edi() {
    let mut emu = emu64();
    // CVTTSS2SI EDI, XMM5
    let code = [
        0xf3, 0x0f, 0x2c, 0xfd, // CVTTSS2SI EDI, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttss2si_xmm6_to_ebp() {
    let mut emu = emu64();
    // CVTTSS2SI EBP, XMM6
    let code = [
        0xf3, 0x0f, 0x2c, 0xee, // CVTTSS2SI EBP, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttss2si_xmm7_to_esp() {
    let mut emu = emu64();
    // CVTTSS2SI ESP, XMM7
    let code = [
        0xf3, 0x0f, 0x2c, 0xe7, // CVTTSS2SI ESP, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttss2si_xmm8_to_r8d() {
    let mut emu = emu64();
    // CVTTSS2SI R8D, XMM8
    let code = [
        0xf3, 0x45, 0x0f, 0x2c, 0xc0, // CVTTSS2SI R8D, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttss2si_xmm15_to_r15d() {
    let mut emu = emu64();
    // CVTTSS2SI R15D, XMM15
    let code = [
        0xf3, 0x45, 0x0f, 0x2c, 0xff, // CVTTSS2SI R15D, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// CVTTSS2SI - XMM to 64-bit Register Tests
// ============================================================================

#[test]
fn test_cvttss2si_xmm0_to_rax() {
    let mut emu = emu64();
    // CVTTSS2SI RAX, XMM0
    let code = [
        0xf3, 0x48, 0x0f, 0x2c, 0xc0, // CVTTSS2SI RAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttss2si_xmm1_to_rbx() {
    let mut emu = emu64();
    // CVTTSS2SI RBX, XMM1
    let code = [
        0xf3, 0x48, 0x0f, 0x2c, 0xd9, // CVTTSS2SI RBX, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttss2si_xmm2_to_rcx() {
    let mut emu = emu64();
    // CVTTSS2SI RCX, XMM2
    let code = [
        0xf3, 0x48, 0x0f, 0x2c, 0xca, // CVTTSS2SI RCX, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttss2si_xmm8_to_r8() {
    let mut emu = emu64();
    // CVTTSS2SI R8, XMM8
    let code = [
        0xf3, 0x4d, 0x0f, 0x2c, 0xc0, // CVTTSS2SI R8, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttss2si_xmm15_to_r15() {
    let mut emu = emu64();
    // CVTTSS2SI R15, XMM15
    let code = [
        0xf3, 0x4d, 0x0f, 0x2c, 0xff, // CVTTSS2SI R15, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// CVTTSS2SI - Memory to Register Tests
// ============================================================================

#[test]
fn test_cvttss2si_mem_to_eax() {
    let mut emu = emu64();
    // CVTTSS2SI EAX, [0x3000]
    let code = [
        0xf3, 0x0f, 0x2c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTTSS2SI EAX, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttss2si_mem_to_rax() {
    let mut emu = emu64();
    // CVTTSS2SI RAX, [0x3000]
    let code = [
        0xf3, 0x48, 0x0f, 0x2c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTTSS2SI RAX, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttss2si_mem_to_r15() {
    let mut emu = emu64();
    // CVTTSS2SI R15, [0x3000]
    let code = [
        0xf3, 0x4c, 0x0f, 0x2c, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTTSS2SI R15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Mixed Truncation Conversions
// ============================================================================

#[test]
fn test_multiple_cvttsd2si() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x2c, 0xc0, // CVTTSD2SI EAX, XMM0
        0xf2, 0x0f, 0x2c, 0xd9, // CVTTSD2SI EBX, XMM1
        0xf2, 0x0f, 0x2c, 0xca, // CVTTSD2SI ECX, XMM2
        0xf2, 0x0f, 0x2c, 0xd3, // CVTTSD2SI EDX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_multiple_cvttss2si() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x2c, 0xc0, // CVTTSS2SI EAX, XMM0
        0xf3, 0x0f, 0x2c, 0xd9, // CVTTSS2SI EBX, XMM1
        0xf3, 0x0f, 0x2c, 0xca, // CVTTSS2SI ECX, XMM2
        0xf3, 0x0f, 0x2c, 0xd3, // CVTTSS2SI EDX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mixed_cvttsd2si_cvttss2si() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x2c, 0xc0, // CVTTSD2SI EAX, XMM0
        0xf3, 0x0f, 0x2c, 0xd9, // CVTTSS2SI EBX, XMM1
        0xf2, 0x0f, 0x2c, 0xca, // CVTTSD2SI ECX, XMM2
        0xf3, 0x0f, 0x2c, 0xd3, // CVTTSS2SI EDX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttsd2si_32_and_64_bit() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x2c, 0xc0, // CVTTSD2SI EAX, XMM0
        0xf2, 0x48, 0x0f, 0x2c, 0xd9, // CVTTSD2SI RBX, XMM1
        0xf2, 0x0f, 0x2c, 0xca, // CVTTSD2SI ECX, XMM2
        0xf2, 0x48, 0x0f, 0x2c, 0xd3, // CVTTSD2SI RDX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttss2si_32_and_64_bit() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x2c, 0xc0, // CVTTSS2SI EAX, XMM0
        0xf3, 0x48, 0x0f, 0x2c, 0xd9, // CVTTSS2SI RBX, XMM1
        0xf3, 0x0f, 0x2c, 0xca, // CVTTSS2SI ECX, XMM2
        0xf3, 0x48, 0x0f, 0x2c, 0xd3, // CVTTSS2SI RDX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
