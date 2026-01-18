use crate::*;

// AND — Logical AND
//
// Opcodes:
// - 24 ib           AND AL, imm8
// - 25 iw/id        AND AX/EAX/RAX, imm16/32
// - 80 /4 ib        AND r/m8, imm8
// - 81 /4 iw/id     AND r/m16/32/64, imm16/32
// - 83 /4 ib        AND r/m16/32/64, imm8 (sign-extended)
// - 20 /r           AND r/m8, r8
// - 21 /r           AND r/m16/32/64, r16/32/64
// - 22 /r           AND r8, r/m8
// - 23 /r           AND r16/32/64, r/m16/32/64
//
// Operation: DEST := DEST AND SRC
//
// Flags: OF and CF are CLEARED.
//        SF, ZF, PF are set according to result.
//        AF is undefined (not tested).

// ============================================================================
// AND AL, imm8 - Test accumulator with immediate
// ============================================================================

#[test]
fn test_and_al_imm8_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x24, 0x0F, 0xf4]; // AND AL, 0x0F; HLT
    emu.regs_mut().rax = 0xAB; // AL = 0xAB (10101011)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x0B, "AL: 0xAB AND 0x0F = 0x0B");
    assert!(!emu.flags().f_zf, "ZF should be clear");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_of, "OF should be clear");
}

#[test]
fn test_and_al_imm8_zero_result() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x24, 0x00, 0xf4]; // AND AL, 0
    emu.regs_mut().rax = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0, "AL: 0xFF AND 0 = 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_and_al_imm8_all_ones() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x24, 0xFF, 0xf4]; // AND AL, 0xFF
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL: 0x42 AND 0xFF = 0x42");
}

#[test]
fn test_and_al_imm8_sign_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x24, 0x80, 0xf4]; // AND AL, 0x80
    emu.regs_mut().rax = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x80, "AL: 0xFF AND 0x80 = 0x80");
    assert!(emu.flags().f_sf, "SF should be set (high bit = 1)");
}

#[test]
fn test_and_al_imm8_parity_even() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x24, 0x03, 0xf4]; // AND AL, 0x03
    emu.regs_mut().rax = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03);
    assert!(emu.flags().f_pf, "PF should be set (even parity)");
}

#[test]
fn test_and_al_imm8_parity_odd() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x24, 0x07, 0xf4]; // AND AL, 0x07
    emu.regs_mut().rax = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x07);
    assert!(!emu.flags().f_pf, "PF should be clear (odd parity)");
}

// ============================================================================
// AND AX, imm16 - 16-bit accumulator
// ============================================================================

#[test]
fn test_and_ax_imm16_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x25, 0x0F, 0x00, 0xf4]; // AND AX, 0x000F
    emu.regs_mut().rax = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0004, "AX: 0x1234 AND 0x000F = 0x0004");
}

#[test]
fn test_and_ax_imm16_high_byte() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x25, 0x00, 0xFF, 0xf4]; // AND AX, 0xFF00
    emu.regs_mut().rax = 0xABCD;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xAB00, "AX: keep only high byte");
}

#[test]
fn test_and_ax_imm16_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x25, 0x00, 0x00, 0xf4]; // AND AX, 0
    emu.regs_mut().rax = 0xFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0, "AX: AND with 0 gives 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

// ============================================================================
// AND EAX, imm32 - 32-bit accumulator
// ============================================================================

#[test]
fn test_and_eax_imm32_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x25, 0xFF, 0x00, 0x00, 0x00, 0xf4]; // AND EAX, 0x000000FF
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x00000078, "EAX: mask to low byte");
}

#[test]
fn test_and_eax_imm32_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x25, 0x00, 0xFF, 0x00, 0x00, 0xf4]; // AND EAX, 0x0000FF00
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x00005600, "EAX: mask middle byte");
}

#[test]
fn test_and_eax_imm32_high_bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x25, 0x00, 0x00, 0x00, 0x80, 0xf4]; // AND EAX, 0x80000000
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x80000000, "EAX: keep only high bit");
    assert!(emu.flags().f_sf, "SF should be set");
}

// ============================================================================
// AND RAX, imm32 - 64-bit accumulator (imm32 sign-extended)
// ============================================================================

#[test]
fn test_and_rax_imm32_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x25, 0xFF, 0xFF, 0x00, 0x00, 0xf4]; // AND RAX, 0x0000FFFF
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x000000000000DEF0, "RAX: mask to low word");
}

#[test]
fn test_and_rax_imm32_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x25, 0xFF, 0xFF, 0xFF, 0xFF, 0xf4]; // AND RAX, 0xFFFFFFFF
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "RAX: no change with all ones");
}

// ============================================================================
// AND r/m8, imm8
// ============================================================================

#[test]
fn test_and_rm8_imm8_bl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x80, 0xe3, 0x0F, 0xf4]; // AND BL, 0x0F
    emu.regs_mut().rbx = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFF, 0x0F, "BL: 0xFF AND 0x0F = 0x0F");
}

#[test]
fn test_and_rm8_imm8_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x80, 0xe1, 0xAA, 0xf4]; // AND CL, 0xAA
    emu.regs_mut().rcx = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFF, 0xAA, "CL: 0xFF AND 0xAA = 0xAA");
}

#[test]
fn test_and_rm8_imm8_dh() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x80, 0xe6, 0x55, 0xf4]; // AND DH, 0x55
    emu.regs_mut().rdx = 0xFF00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rdx >> 8) & 0xFF, 0x55, "DH: 0xFF AND 0x55 = 0x55");
}

// ============================================================================
// AND r/m16, imm16
// ============================================================================

#[test]
fn test_and_rm16_imm16_bx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x81, 0xe3, 0xF0, 0x0F, 0xf4]; // AND BX, 0x0FF0
    emu.regs_mut().rbx = 0xFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFF, 0x0FF0, "BX: 0xFFFF AND 0x0FF0 = 0x0FF0");
}

#[test]
fn test_and_rm16_imm16_si() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x81, 0xe6, 0x00, 0xFF, 0xf4]; // AND SI, 0xFF00
    emu.regs_mut().rsi = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi & 0xFFFF, 0x1200, "SI: keep only high byte");
}

// ============================================================================
// AND r/m32, imm32
// ============================================================================

#[test]
fn test_and_rm32_imm32_ebx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x81, 0xe3, 0x00, 0xFF, 0x00, 0x00, 0xf4]; // AND EBX, 0x0000FF00
    emu.regs_mut().rbx = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0x00005600, "EBX: mask middle byte");
}

#[test]
fn test_and_rm32_imm32_esi() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x81, 0xe6, 0xFF, 0x00, 0x00, 0x00, 0xf4]; // AND ESI, 0x000000FF
    emu.regs_mut().rsi = 0xABCDEF01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi, 0x00000001, "ESI: mask to low byte");
}

// ============================================================================
// AND r/m64, imm32 (sign-extended)
// ============================================================================

#[test]
fn test_and_rm64_imm32_rbx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x81, 0xe3, 0xFF, 0xFF, 0xFF, 0x00, 0xf4]; // AND RBX, 0x00FFFFFF
    emu.regs_mut().rbx = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0x0000000000BCDEF0, "RBX: mask low 3 bytes");
}

#[test]
fn test_and_rm64_imm32_r8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x49, 0x81, 0xe0, 0xFF, 0x00, 0x00, 0x00, 0xf4]; // AND R8, 0x000000FF
    emu.regs_mut().r8 = 0xFFFFFFFFFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8, 0x00000000000000FF, "R8: mask to low byte");
}

// ============================================================================
// AND r/m, imm8 (sign-extended)
// ============================================================================

#[test]
fn test_and_rm16_imm8_sign_ext() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x83, 0xe3, 0xFF, 0xf4]; // AND BX, 0xFF (sign-extended to 0xFFFF)
    emu.regs_mut().rbx = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFF, 0x1234, "BX: AND with 0xFFFF (no change)");
}

#[test]
fn test_and_rm32_imm8_sign_ext() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x83, 0xe3, 0x0F, 0xf4]; // AND EBX, 0x0F
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0x0000000F, "EBX: AND with sign-extended imm8");
}

#[test]
fn test_and_rm64_imm8_sign_ext() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x83, 0xe0, 0xF0, 0xf4]; // AND RAX, 0xFFFFFFFFFFFFFFF0
    emu.regs_mut().rax = 0x123456789ABCDEF7;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "RAX: align to 16-byte boundary");
}

// ============================================================================
// AND r/m, r (destination is r/m)
// ============================================================================

#[test]
fn test_and_rm8_r8_al_bl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x20, 0xd8, 0xf4]; // AND AL, BL
    emu.regs_mut().rax = 0xFF;
    emu.regs_mut().rbx = 0x0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x0F, "AL: 0xFF AND 0x0F = 0x0F");
}

#[test]
fn test_and_rm16_r16_ax_bx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x21, 0xd8, 0xf4]; // AND AX, BX
    emu.regs_mut().rax = 0xFFFF;
    emu.regs_mut().rbx = 0x00FF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x00FF, "AX: 0xFFFF AND 0x00FF = 0x00FF");
}

#[test]
fn test_and_rm32_r32_eax_ebx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x21, 0xd8, 0xf4]; // AND EAX, EBX
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0xF0F0F0F0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x10305070, "EAX: bitwise AND with EBX");
}

#[test]
fn test_and_rm64_r64_rax_rbx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x21, 0xd8, 0xf4]; // AND RAX, RBX
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    emu.regs_mut().rbx = 0x00000000FFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x00000000FFFFFFFF, "RAX: mask to low 32 bits");
}

// ============================================================================
// AND r, r/m (destination is register)
// ============================================================================

#[test]
fn test_and_r8_rm8_al_bl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x22, 0xc3, 0xf4]; // AND AL, BL
    emu.regs_mut().rax = 0xAA;
    emu.regs_mut().rbx = 0x55;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0, "AL: 0xAA AND 0x55 = 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_and_r16_rm16_ax_bx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x23, 0xc3, 0xf4]; // AND AX, BX
    emu.regs_mut().rax = 0x1234;
    emu.regs_mut().rbx = 0x0F0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0204, "AX: 0x1234 AND 0x0F0F = 0x0204");
}

#[test]
fn test_and_r32_rm32_eax_ebx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x23, 0xc3, 0xf4]; // AND EAX, EBX
    emu.regs_mut().rax = 0xAAAAAAAA;
    emu.regs_mut().rbx = 0x55555555;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "EAX: alternating bits AND = 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_and_r64_rm64_rax_rbx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x23, 0xc3, 0xf4]; // AND RAX, RBX
    emu.regs_mut().rax = 0xFF00FF00FF00FF00;
    emu.regs_mut().rbx = 0x00FF00FF00FF00FF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "RAX: alternating bytes AND = 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

// ============================================================================
// AND with various register combinations
// ============================================================================

#[test]
fn test_and_cl_dl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x20, 0xd1, 0xf4]; // AND CL, DL
    emu.regs_mut().rcx = 0xFF;
    emu.regs_mut().rdx = 0x3C;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFF, 0x3C, "CL: 0xFF AND 0x3C = 0x3C");
}

#[test]
fn test_and_ecx_edx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x21, 0xd1, 0xf4]; // AND ECX, EDX
    emu.regs_mut().rcx = 0xF0F0F0F0;
    emu.regs_mut().rdx = 0x0F0F0F0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx, 0, "ECX: complementary patterns AND = 0");
}

#[test]
fn test_and_rsi_rdi() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x21, 0xfe, 0xf4]; // AND RSI, RDI
    emu.regs_mut().rsi = 0xAAAAAAAAAAAAAAAA;
    emu.regs_mut().rdi = 0xAAAAAAAAAAAAAAAA;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi, 0xAAAAAAAAAAAAAAAA, "RSI: AND with same value");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_and_r8b_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x41, 0x80, 0xe0, 0x0F, 0xf4]; // AND R8B, 0x0F
    emu.regs_mut().r8 = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0x0F, "R8B: 0xFF AND 0x0F = 0x0F");
}

#[test]
fn test_and_r9w_imm16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x41, 0x81, 0xe1, 0xF0, 0x0F, 0xf4]; // AND R9W, 0x0FF0
    emu.regs_mut().r9 = 0xFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r9 & 0xFFFF, 0x0FF0, "R9W: mask");
}

#[test]
fn test_and_r10d_imm32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x41, 0x81, 0xe2, 0xFF, 0x00, 0x00, 0x00, 0xf4]; // AND R10D, 0x000000FF
    emu.regs_mut().r10 = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10, 0x00000078, "R10D: mask to low byte");
}

#[test]
fn test_and_r11_imm32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x49, 0x81, 0xe3, 0xFF, 0xFF, 0x00, 0x00, 0xf4]; // AND R11, 0x0000FFFF
    emu.regs_mut().r11 = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r11, 0x000000000000DEF0, "R11: mask to low word");
}

#[test]
fn test_and_r12d_r13d() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x45, 0x21, 0xec, 0xf4]; // AND R12D, R13D
    emu.regs_mut().r12 = 0xFFFFFFFF;
    emu.regs_mut().r13 = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r12, 0x12345678, "R12D: AND with R13D");
}

#[test]
fn test_and_r14_r15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x4d, 0x21, 0xfe, 0xf4]; // AND R14, R15
    emu.regs_mut().r14 = 0xFFFFFFFF00000000;
    emu.regs_mut().r15 = 0x00000000FFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r14, 0, "R14: no overlapping bits");
    assert!(emu.flags().f_zf, "ZF should be set");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_and_byte_ptr_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x80, 0x25, 0xf9, 0x0f, 0x00, 0x00, 0x0F, // AND BYTE PTR [rip+0x0FF9], 0x0F
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0xFF);

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0x0F, "Memory: 0xFF AND 0x0F = 0x0F");
}

#[test]
fn test_and_word_ptr_imm16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0x81, 0x25, 0xf7, 0x0f, 0x00, 0x00, 0xF0, 0x0F, // AND WORD PTR [rip+0x0FF7], 0x0FF0
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0xFFFF);

    emu.run(None).unwrap();
    let result = emu.maps.read_word(DATA_ADDR).unwrap();

    assert_eq!(result, 0x0FF0, "Memory: word AND");
}

#[test]
fn test_and_dword_ptr_imm32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x81, 0x25, 0xf6, 0x0f, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, // AND DWORD PTR [rip+0x0FF6], 0x000000FF
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x12345678);

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x00000078, "Memory: dword mask to low byte");
}

#[test]
fn test_and_qword_ptr_imm32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x81, 0x25, 0xf5, 0x0f, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, // AND QWORD PTR [rip+0x0FF5], 0x0000FFFF
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x123456789ABCDEF0);

    emu.run(None).unwrap();
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x000000000000DEF0, "Memory: qword mask to low word");
}

// ============================================================================
// Flag behavior tests
// ============================================================================

#[test]
fn test_and_clears_of_cf() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x24, 0xFF, 0xf4]; // AND AL, 0xFF
    emu.regs_mut().rax = 0xFF;
    emu.flags_mut().load(0x2 | flags::F_OF | flags::F_CF);
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_of, "OF cleared by AND");
    assert!(!emu.flags().f_cf, "CF cleared by AND");
}

// ============================================================================
// Practical use cases
// ============================================================================

#[test]
fn test_and_mask_low_nibble() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x24, 0x0F, 0xf4]; // AND AL, 0x0F
    emu.regs_mut().rax = 0xB7;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x07, "Extract low nibble");
}

#[test]
fn test_and_check_bit_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x24, 0x10, 0xf4]; // AND AL, 0x10
    emu.regs_mut().rax = 0x1F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x10, "Bit 4 is set");
    assert!(!emu.flags().f_zf, "ZF clear means bit was set");
}

#[test]
fn test_and_check_bit_clear() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x24, 0x10, 0xf4]; // AND AL, 0x10
    emu.regs_mut().rax = 0x0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0, "Bit 4 is clear");
    assert!(emu.flags().f_zf, "ZF set means bit was clear");
}
