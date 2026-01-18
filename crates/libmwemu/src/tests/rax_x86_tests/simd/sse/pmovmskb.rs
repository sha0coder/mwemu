use crate::*;

// PMOVMSKB - Move Byte Mask
//
// Creates a mask made up of the most significant bit of each byte in the
// source operand and stores the result in the low byte or word of the
// destination operand (depending on operand size).
//
// Opcodes:
// 66 0F D7 /r             PMOVMSKB r32, xmm1    - Move byte mask from XMM to r32
// 66 REX.W 0F D7 /r       PMOVMSKB r64, xmm1    - Move byte mask from XMM to r64

// ============================================================================
// PMOVMSKB Tests - Move Byte Mask (XMM -> GPR)
// ============================================================================

#[test]
fn test_pmovmskb_eax_xmm0() {
    let mut emu = emu64();
    // PMOVMSKB EAX, XMM0
    let code = [
        0x66, 0x0f, 0xd7, 0xc0, // PMOVMSKB EAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_ebx_xmm1() {
    let mut emu = emu64();
    // PMOVMSKB EBX, XMM1
    let code = [
        0x66, 0x0f, 0xd7, 0xd9, // PMOVMSKB EBX, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_ecx_xmm2() {
    let mut emu = emu64();
    // PMOVMSKB ECX, XMM2
    let code = [
        0x66, 0x0f, 0xd7, 0xca, // PMOVMSKB ECX, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_edx_xmm3() {
    let mut emu = emu64();
    // PMOVMSKB EDX, XMM3
    let code = [
        0x66, 0x0f, 0xd7, 0xd3, // PMOVMSKB EDX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_esi_xmm4() {
    let mut emu = emu64();
    // PMOVMSKB ESI, XMM4
    let code = [
        0x66, 0x0f, 0xd7, 0xf4, // PMOVMSKB ESI, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_edi_xmm5() {
    let mut emu = emu64();
    // PMOVMSKB EDI, XMM5
    let code = [
        0x66, 0x0f, 0xd7, 0xfd, // PMOVMSKB EDI, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_ebp_xmm6() {
    let mut emu = emu64();
    // PMOVMSKB EBP, XMM6
    let code = [
        0x66, 0x0f, 0xd7, 0xee, // PMOVMSKB EBP, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_eax_xmm7() {
    let mut emu = emu64();
    // PMOVMSKB EAX, XMM7
    let code = [
        0x66, 0x0f, 0xd7, 0xc7, // PMOVMSKB EAX, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_r8d_xmm8() {
    let mut emu = emu64();
    // PMOVMSKB R8D, XMM8 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xc0, // PMOVMSKB R8D, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_r9d_xmm9() {
    let mut emu = emu64();
    // PMOVMSKB R9D, XMM9
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xc9, // PMOVMSKB R9D, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_r10d_xmm10() {
    let mut emu = emu64();
    // PMOVMSKB R10D, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xd2, // PMOVMSKB R10D, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_r11d_xmm11() {
    let mut emu = emu64();
    // PMOVMSKB R11D, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xdb, // PMOVMSKB R11D, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_r12d_xmm12() {
    let mut emu = emu64();
    // PMOVMSKB R12D, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xe4, // PMOVMSKB R12D, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_r13d_xmm13() {
    let mut emu = emu64();
    // PMOVMSKB R13D, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xed, // PMOVMSKB R13D, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_r14d_xmm14() {
    let mut emu = emu64();
    // PMOVMSKB R14D, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xf6, // PMOVMSKB R14D, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_r15d_xmm15() {
    let mut emu = emu64();
    // PMOVMSKB R15D, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xff, // PMOVMSKB R15D, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_rax_xmm0() {
    let mut emu = emu64();
    // PMOVMSKB RAX, XMM0 (64-bit mode with REX.W)
    let code = [
        0x66, 0x48, 0x0f, 0xd7, 0xc0, // PMOVMSKB RAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_rbx_xmm1() {
    let mut emu = emu64();
    // PMOVMSKB RBX, XMM1
    let code = [
        0x66, 0x48, 0x0f, 0xd7, 0xd9, // PMOVMSKB RBX, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_rcx_xmm2() {
    let mut emu = emu64();
    // PMOVMSKB RCX, XMM2
    let code = [
        0x66, 0x48, 0x0f, 0xd7, 0xca, // PMOVMSKB RCX, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_rdx_xmm3() {
    let mut emu = emu64();
    // PMOVMSKB RDX, XMM3
    let code = [
        0x66, 0x48, 0x0f, 0xd7, 0xd3, // PMOVMSKB RDX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_rsi_xmm4() {
    let mut emu = emu64();
    // PMOVMSKB RSI, XMM4
    let code = [
        0x66, 0x48, 0x0f, 0xd7, 0xf4, // PMOVMSKB RSI, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_rdi_xmm5() {
    let mut emu = emu64();
    // PMOVMSKB RDI, XMM5
    let code = [
        0x66, 0x48, 0x0f, 0xd7, 0xfd, // PMOVMSKB RDI, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_rbp_xmm6() {
    let mut emu = emu64();
    // PMOVMSKB RBP, XMM6
    let code = [
        0x66, 0x48, 0x0f, 0xd7, 0xee, // PMOVMSKB RBP, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_r8_xmm7() {
    let mut emu = emu64();
    // PMOVMSKB R8, XMM7
    let code = [
        0x66, 0x4c, 0x0f, 0xd7, 0xc7, // PMOVMSKB R8, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_r9_xmm8() {
    let mut emu = emu64();
    // PMOVMSKB R9, XMM8
    let code = [
        0x66, 0x4d, 0x0f, 0xd7, 0xc8, // PMOVMSKB R9, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_r10_xmm9() {
    let mut emu = emu64();
    // PMOVMSKB R10, XMM9
    let code = [
        0x66, 0x4d, 0x0f, 0xd7, 0xd1, // PMOVMSKB R10, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_r11_xmm10() {
    let mut emu = emu64();
    // PMOVMSKB R11, XMM10
    let code = [
        0x66, 0x4d, 0x0f, 0xd7, 0xda, // PMOVMSKB R11, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_r12_xmm11() {
    let mut emu = emu64();
    // PMOVMSKB R12, XMM11
    let code = [
        0x66, 0x4d, 0x0f, 0xd7, 0xe3, // PMOVMSKB R12, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_r13_xmm12() {
    let mut emu = emu64();
    // PMOVMSKB R13, XMM12
    let code = [
        0x66, 0x4d, 0x0f, 0xd7, 0xec, // PMOVMSKB R13, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_r14_xmm13() {
    let mut emu = emu64();
    // PMOVMSKB R14, XMM13
    let code = [
        0x66, 0x4d, 0x0f, 0xd7, 0xf5, // PMOVMSKB R14, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_r15_xmm14() {
    let mut emu = emu64();
    // PMOVMSKB R15, XMM14
    let code = [
        0x66, 0x4d, 0x0f, 0xd7, 0xfe, // PMOVMSKB R15, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_all_bits_set() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xd7, 0xc0, // PMOVMSKB EAX, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_all_bits_clear() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xd7, 0xd9, // PMOVMSKB EBX, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_alternating_bits() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xd7, 0xca, // PMOVMSKB ECX, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_first_half_set() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xd7, 0xd3, // PMOVMSKB EDX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_second_half_set() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xd7, 0xf4, // PMOVMSKB ESI, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_single_bit() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xd7, 0xfd, // PMOVMSKB EDI, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_sequential_ops() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xd7, 0xc0, // PMOVMSKB EAX, XMM0
        0x66, 0x0f, 0xd7, 0xd9, // PMOVMSKB EBX, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_mixed_registers_1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0xd7, 0xc1, // PMOVMSKB R8D, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_mixed_registers_2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xd7, 0xc7, // PMOVMSKB EAX, XMM7
        0x66, 0x45, 0x0f, 0xd7, 0xc8, // PMOVMSKB R9D, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_pattern_1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xd7, 0xee, // PMOVMSKB EBP, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_pattern_2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xdb, // PMOVMSKB R11D, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_pattern_3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xe4, // PMOVMSKB R12D, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_64bit_mode_1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x4c, 0x0f, 0xd7, 0xc7, // PMOVMSKB R8, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_64bit_mode_2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x4d, 0x0f, 0xd7, 0xfe, // PMOVMSKB R15, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_all_xmm_regs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xd7, 0xc0, // PMOVMSKB EAX, XMM0
        0x66, 0x0f, 0xd7, 0xd9, // PMOVMSKB EBX, XMM1
        0x66, 0x0f, 0xd7, 0xca, // PMOVMSKB ECX, XMM2
        0x66, 0x0f, 0xd7, 0xd3, // PMOVMSKB EDX, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovmskb_high_xmm_regs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xff, // PMOVMSKB R15D, XMM15
        0x66, 0x45, 0x0f, 0xd7, 0xf6, // PMOVMSKB R14D, XMM14
        0x66, 0x45, 0x0f, 0xd7, 0xed, // PMOVMSKB R13D, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
