use crate::*;

// OR — Logical Inclusive OR
//
// Opcodes:
// - 0C ib           OR AL, imm8
// - 0D iw/id        OR AX/EAX/RAX, imm16/32
// - 80 /1 ib        OR r/m8, imm8
// - 81 /1 iw/id     OR r/m16/32/64, imm16/32
// - 83 /1 ib        OR r/m16/32/64, imm8 (sign-extended)
// - 08 /r           OR r/m8, r8
// - 09 /r           OR r/m16/32/64, r16/32/64
// - 0A /r           OR r8, r/m8
// - 0B /r           OR r16/32/64, r/m16/32/64
//
// Operation: DEST := DEST OR SRC
//
// Flags: OF and CF are CLEARED.
//        SF, ZF, PF are set according to result.
//        AF is undefined (not tested).

// ============================================================================
// OR AL, imm8
// ============================================================================

#[test]
fn test_or_al_imm8_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0C, 0x0F, 0xf4]; // OR AL, 0x0F; HLT
    emu.regs_mut().rax = 0xA0; // AL = 0xA0 (10100000)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0xA0 | 0x0F = 10100000 | 00001111 = 10101111 = 0xAF
    assert_eq!(emu.regs().rax & 0xFF, 0xAF, "AL: 0xA0 OR 0x0F = 0xAF");
    assert!(!emu.flags().f_zf, "ZF should be clear");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_of, "OF should be clear");
}

#[test]
fn test_or_al_imm8_zero_identity() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0C, 0x00, 0xf4]; // OR AL, 0
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL: 0x42 OR 0 = 0x42 (identity)");
}

#[test]
fn test_or_al_imm8_all_ones() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0C, 0xFF, 0xf4]; // OR AL, 0xFF
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "AL: 0x00 OR 0xFF = 0xFF");
}

#[test]
fn test_or_al_imm8_zero_result() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0C, 0x00, 0xf4]; // OR AL, 0
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0, "AL: 0 OR 0 = 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_or_al_imm8_sign_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0C, 0x80, 0xf4]; // OR AL, 0x80
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x80, "AL: 0x00 OR 0x80 = 0x80");
    assert!(emu.flags().f_sf, "SF should be set (high bit = 1)");
}

#[test]
fn test_or_al_imm8_parity_even() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0C, 0x03, 0xf4]; // OR AL, 0x03
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03);
    assert!(emu.flags().f_pf, "PF should be set (even parity)");
}

#[test]
fn test_or_al_imm8_parity_odd() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0C, 0x07, 0xf4]; // OR AL, 0x07
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x07);
    assert!(!emu.flags().f_pf, "PF should be clear (odd parity)");
}

// ============================================================================
// OR AX, imm16
// ============================================================================

#[test]
fn test_or_ax_imm16_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x0D, 0x0F, 0x00, 0xf4]; // OR AX, 0x000F
    emu.regs_mut().rax = 0x1230;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x123F, "AX: 0x1230 OR 0x000F = 0x123F");
}

#[test]
fn test_or_ax_imm16_high_byte() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x0D, 0x00, 0xFF, 0xf4]; // OR AX, 0xFF00
    emu.regs_mut().rax = 0x00CD;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xFFCD, "AX: set high byte");
}

#[test]
fn test_or_ax_imm16_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x0D, 0x00, 0x00, 0xf4]; // OR AX, 0
    emu.regs_mut().rax = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x1234, "AX: OR with 0 is identity");
}

// ============================================================================
// OR EAX, imm32
// ============================================================================

#[test]
fn test_or_eax_imm32_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0D, 0xFF, 0x00, 0x00, 0x00, 0xf4]; // OR EAX, 0x000000FF
    emu.regs_mut().rax = 0x12345600;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456FF, "EAX: set low byte");
}

#[test]
fn test_or_eax_imm32_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0D, 0x00, 0xFF, 0x00, 0x00, 0xf4]; // OR EAX, 0x0000FF00
    emu.regs_mut().rax = 0x12340078;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x1234FF78, "EAX: set middle byte");
}

#[test]
fn test_or_eax_imm32_high_bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0D, 0x00, 0x00, 0x00, 0x80, 0xf4]; // OR EAX, 0x80000000
    emu.regs_mut().rax = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x80000000, "EAX: set only high bit");
    assert!(emu.flags().f_sf, "SF should be set");
}

// ============================================================================
// OR RAX, imm32 (sign-extended)
// ============================================================================

#[test]
fn test_or_rax_imm32_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x0D, 0xFF, 0xFF, 0x00, 0x00, 0xf4]; // OR RAX, 0x0000FFFF
    emu.regs_mut().rax = 0x1234567800000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456780000FFFF, "RAX: set low word");
}

#[test]
fn test_or_rax_imm32_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x0D, 0xFF, 0xFF, 0xFF, 0xFF, 0xf4]; // OR RAX, 0xFFFFFFFF
    emu.regs_mut().rax = 0x0000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "RAX: all bits set");
}

// ============================================================================
// OR r/m8, imm8
// ============================================================================

#[test]
fn test_or_rm8_imm8_bl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x80, 0xcb, 0x0F, 0xf4]; // OR BL, 0x0F
    emu.regs_mut().rbx = 0xA0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFF, 0xAF, "BL: 0xA0 OR 0x0F = 0xAF");
}

#[test]
fn test_or_rm8_imm8_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x80, 0xc9, 0xAA, 0xf4]; // OR CL, 0xAA
    emu.regs_mut().rcx = 0x55;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFF, 0xFF, "CL: 0x55 OR 0xAA = 0xFF");
}

#[test]
fn test_or_rm8_imm8_dh() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x80, 0xce, 0x55, 0xf4]; // OR DH, 0x55
    emu.regs_mut().rdx = 0xAA00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rdx >> 8) & 0xFF, 0xFF, "DH: 0xAA OR 0x55 = 0xFF");
}

// ============================================================================
// OR r/m16, imm16
// ============================================================================

#[test]
fn test_or_rm16_imm16_bx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x81, 0xcb, 0x0F, 0x00, 0xf4]; // OR BX, 0x000F
    emu.regs_mut().rbx = 0xFF00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFF, 0xFF0F, "BX: 0xFF00 OR 0x000F = 0xFF0F");
}

#[test]
fn test_or_rm16_imm16_si() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x81, 0xce, 0x00, 0xFF, 0xf4]; // OR SI, 0xFF00
    emu.regs_mut().rsi = 0x0034;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi & 0xFFFF, 0xFF34, "SI: set high byte");
}

// ============================================================================
// OR r/m32, imm32
// ============================================================================

#[test]
fn test_or_rm32_imm32_ebx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x81, 0xcb, 0x00, 0xFF, 0x00, 0x00, 0xf4]; // OR EBX, 0x0000FF00
    emu.regs_mut().rbx = 0x12340078;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0x1234FF78, "EBX: set middle byte");
}

#[test]
fn test_or_rm32_imm32_esi() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x81, 0xce, 0xFF, 0x00, 0x00, 0x00, 0xf4]; // OR ESI, 0x000000FF
    emu.regs_mut().rsi = 0xABCDEF00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi, 0xABCDEFFF, "ESI: set low byte");
}

// ============================================================================
// OR r/m64, imm32 (sign-extended)
// ============================================================================

#[test]
fn test_or_rm64_imm32_rbx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x81, 0xcb, 0xFF, 0xFF, 0xFF, 0x00, 0xf4]; // OR RBX, 0x00FFFFFF
    emu.regs_mut().rbx = 0x1234567800000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0x1234567800FFFFFF, "RBX: set low 3 bytes");
}

#[test]
fn test_or_rm64_imm32_r8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x49, 0x81, 0xc8, 0xFF, 0x00, 0x00, 0x00, 0xf4]; // OR R8, 0x000000FF
    emu.regs_mut().r8 = 0x0000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8, 0x00000000000000FF, "R8: set low byte");
}

// ============================================================================
// OR r/m, imm8 (sign-extended)
// ============================================================================

#[test]
fn test_or_rm16_imm8_sign_ext() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x83, 0xcb, 0xFF, 0xf4]; // OR BX, 0xFF (sign-extended to 0xFFFF)
    emu.regs_mut().rbx = 0x0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFF, 0xFFFF, "BX: OR with 0xFFFF");
}

#[test]
fn test_or_rm32_imm8_sign_ext() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x83, 0xcb, 0x0F, 0xf4]; // OR EBX, 0x0F
    emu.regs_mut().rbx = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0x0000000F, "EBX: OR with sign-extended imm8");
}

#[test]
fn test_or_rm64_imm8_sign_ext() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x83, 0xc8, 0x0F, 0xf4]; // OR RAX, 0x0F
    emu.regs_mut().rax = 0x1234567800000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456780000000F, "RAX: set low nibble");
}

// ============================================================================
// OR r/m, r (destination is r/m)
// ============================================================================

#[test]
fn test_or_rm8_r8_al_bl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x08, 0xd8, 0xf4]; // OR AL, BL
    emu.regs_mut().rax = 0xA0;
    emu.regs_mut().rbx = 0x0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xAF, "AL: 0xA0 OR 0x0F = 0xAF");
}

#[test]
fn test_or_rm16_r16_ax_bx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x09, 0xd8, 0xf4]; // OR AX, BX
    emu.regs_mut().rax = 0xFF00;
    emu.regs_mut().rbx = 0x00FF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xFFFF, "AX: 0xFF00 OR 0x00FF = 0xFFFF");
}

#[test]
fn test_or_rm32_r32_eax_ebx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x09, 0xd8, 0xf4]; // OR EAX, EBX
    emu.regs_mut().rax = 0x12340000;
    emu.regs_mut().rbx = 0x00005678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x12345678, "EAX: combine with EBX");
}

#[test]
fn test_or_rm64_r64_rax_rbx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x09, 0xd8, 0xf4]; // OR RAX, RBX
    emu.regs_mut().rax = 0xFFFFFFFF00000000;
    emu.regs_mut().rbx = 0x00000000FFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "RAX: all bits set");
}

// ============================================================================
// OR r, r/m (destination is register)
// ============================================================================

#[test]
fn test_or_r8_rm8_al_bl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0A, 0xc3, 0xf4]; // OR AL, BL
    emu.regs_mut().rax = 0xAA;
    emu.regs_mut().rbx = 0x55;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "AL: 0xAA OR 0x55 = 0xFF");
}

#[test]
fn test_or_r16_rm16_ax_bx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x0B, 0xc3, 0xf4]; // OR AX, BX
    emu.regs_mut().rax = 0x1234;
    emu.regs_mut().rbx = 0x0F0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x1F3F, "AX: 0x1234 OR 0x0F0F = 0x1F3F");
}

#[test]
fn test_or_r32_rm32_eax_ebx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0B, 0xc3, 0xf4]; // OR EAX, EBX
    emu.regs_mut().rax = 0xAAAAAAAA;
    emu.regs_mut().rbx = 0x55555555;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFF, "EAX: alternating bits OR = all ones");
}

#[test]
fn test_or_r64_rm64_rax_rbx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x0B, 0xc3, 0xf4]; // OR RAX, RBX
    emu.regs_mut().rax = 0xFF00FF00FF00FF00;
    emu.regs_mut().rbx = 0x00FF00FF00FF00FF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "RAX: alternating bytes OR = all ones");
}

// ============================================================================
// OR with various register combinations
// ============================================================================

#[test]
fn test_or_cl_dl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x08, 0xd1, 0xf4]; // OR CL, DL
    emu.regs_mut().rcx = 0xF0;
    emu.regs_mut().rdx = 0x0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFF, 0xFF, "CL: 0xF0 OR 0x0F = 0xFF");
}

#[test]
fn test_or_ecx_edx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x09, 0xd1, 0xf4]; // OR ECX, EDX
    emu.regs_mut().rcx = 0xF0F0F0F0;
    emu.regs_mut().rdx = 0x0F0F0F0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx, 0xFFFFFFFF, "ECX: complementary patterns OR = all ones");
}

#[test]
fn test_or_rsi_rdi() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x09, 0xfe, 0xf4]; // OR RSI, RDI
    emu.regs_mut().rsi = 0xAAAAAAAAAAAAAAAA;
    emu.regs_mut().rdi = 0x5555555555555555;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi, 0xFFFFFFFFFFFFFFFF, "RSI: OR all bits set");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_or_r8b_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x41, 0x80, 0xc8, 0x0F, 0xf4]; // OR R8B, 0x0F
    emu.regs_mut().r8 = 0xA0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0xAF, "R8B: 0xA0 OR 0x0F = 0xAF");
}

#[test]
fn test_or_r9w_imm16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x41, 0x81, 0xc9, 0x0F, 0x00, 0xf4]; // OR R9W, 0x000F
    emu.regs_mut().r9 = 0xFF00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r9 & 0xFFFF, 0xFF0F, "R9W: set low nibble");
}

#[test]
fn test_or_r10d_imm32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x41, 0x81, 0xca, 0xFF, 0x00, 0x00, 0x00, 0xf4]; // OR R10D, 0x000000FF
    emu.regs_mut().r10 = 0x12345600;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10, 0x123456FF, "R10D: set low byte");
}

#[test]
fn test_or_r11_imm32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x49, 0x81, 0xcb, 0xFF, 0xFF, 0x00, 0x00, 0xf4]; // OR R11, 0x0000FFFF
    emu.regs_mut().r11 = 0x1234567800000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r11, 0x123456780000FFFF, "R11: set low word");
}

#[test]
fn test_or_r12d_r13d() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x45, 0x09, 0xec, 0xf4]; // OR R12D, R13D
    emu.regs_mut().r12 = 0xFFFF0000;
    emu.regs_mut().r13 = 0x0000FFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r12, 0xFFFFFFFF, "R12D: OR with R13D");
}

#[test]
fn test_or_r14_r15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x4d, 0x09, 0xfe, 0xf4]; // OR R14, R15
    emu.regs_mut().r14 = 0xFFFFFFFF00000000;
    emu.regs_mut().r15 = 0x00000000FFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r14, 0xFFFFFFFFFFFFFFFF, "R14: all bits set");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_or_byte_ptr_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x80, 0x0d, 0xf9, 0x0f, 0x00, 0x00, 0x0F, // OR BYTE PTR [rip+0x0FF9], 0x0F
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0xA0);

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0xAF, "Memory: 0xA0 OR 0x0F = 0xAF");
}

#[test]
fn test_or_word_ptr_imm16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0x81, 0x0d, 0xf7, 0x0f, 0x00, 0x00, 0x0F, 0x00, // OR WORD PTR [rip+0x0FF7], 0x000F
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0xFF00);

    emu.run(None).unwrap();
    let result = emu.maps.read_word(DATA_ADDR).unwrap();

    assert_eq!(result, 0xFF0F, "Memory: word OR");
}

#[test]
fn test_or_dword_ptr_imm32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x81, 0x0d, 0xf6, 0x0f, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, // OR DWORD PTR [rip+0x0FF6], 0x000000FF
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x12345600);

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x123456FF, "Memory: dword set low byte");
}

#[test]
fn test_or_qword_ptr_imm32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x81, 0x0d, 0xf5, 0x0f, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, // OR QWORD PTR [rip+0x0FF5], 0x0000FFFF
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x1234567800000000);

    emu.run(None).unwrap();
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x123456780000FFFF, "Memory: qword set low word");
}

// ============================================================================
// Flag behavior tests
// ============================================================================

#[test]
fn test_or_clears_of_cf() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0C, 0xFF, 0xf4]; // OR AL, 0xFF
    emu.regs_mut().rax = 0x00;
    emu.flags_mut().load(0x2 | flags::F_OF | flags::F_CF);
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_of, "OF cleared by OR");
    assert!(!emu.flags().f_cf, "CF cleared by OR");
}

// ============================================================================
// Practical use cases
// ============================================================================

#[test]
fn test_or_set_specific_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0C, 0x10, 0xf4]; // OR AL, 0x10
    emu.regs_mut().rax = 0x0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x1F, "Set bit 4");
}

#[test]
fn test_or_combine_flags() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0C, 0x04, 0xf4]; // OR AL, 0x04
    emu.regs_mut().rax = 0x01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x05, "Combine flags: 0x01 | 0x04 = 0x05");
}

#[test]
fn test_or_idempotent() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // OR is idempotent: a OR a = a
    let code = [0x08, 0xc0, 0xf4]; // OR AL, AL
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "OR is idempotent");
}
