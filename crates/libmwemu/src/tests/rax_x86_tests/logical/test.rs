use crate::*;

// TEST — Logical Compare
//
// Opcodes:
// - A8 ib           TEST AL, imm8
// - A9 iw/id        TEST AX/EAX/RAX, imm16/32
// - F6 /0 ib        TEST r/m8, imm8
// - F7 /0 iw/id     TEST r/m16/32/64, imm16/32
// - 84 /r           TEST r/m8, r8
// - 85 /r           TEST r/m16/32/64, r16/32/64
//
// Operation: TEMP := DEST AND SRC (result is not stored)
//
// Flags: OF and CF are CLEARED.
//        SF, ZF, PF are set according to result.
//        AF is undefined.
//
// CRITICAL: TEST performs AND but does NOT store the result.
// Used for testing bits without modifying the operand.

// ============================================================================
// TEST AL, imm8
// ============================================================================

#[test]
fn test_test_al_imm8_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xa8, 0x0F, 0xf4]; // TEST AL, 0x0F
    emu.regs_mut().rax = 0xAB;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL should remain unchanged
    assert_eq!(emu.regs().rax & 0xFF, 0xAB, "AL unchanged by TEST");
    assert!(!emu.flags().f_zf, "ZF clear");
    assert!(!emu.flags().f_cf, "CF clear");
    assert!(!emu.flags().f_of, "OF clear");
}

#[test]
fn test_test_al_imm8_zero_result() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xa8, 0x00, 0xf4]; // TEST AL, 0
    emu.regs_mut().rax = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "AL unchanged");
    assert!(emu.flags().f_zf, "ZF set (zero result)");
}

#[test]
fn test_test_al_imm8_bit_test() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xa8, 0x10, 0xf4]; // TEST AL, 0x10 (test bit 4)
    emu.regs_mut().rax = 0x1F; // bit 4 is set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x1F, "AL unchanged");
    assert!(!emu.flags().f_zf, "ZF clear (bit 4 is set)");
}

#[test]
fn test_test_al_imm8_bit_clear() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xa8, 0x10, 0xf4]; // TEST AL, 0x10 (test bit 4)
    emu.regs_mut().rax = 0x0F; // bit 4 is clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x0F, "AL unchanged");
    assert!(emu.flags().f_zf, "ZF set (bit 4 is clear)");
}

#[test]
fn test_test_al_imm8_sign_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xa8, 0x80, 0xf4]; // TEST AL, 0x80
    emu.regs_mut().rax = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "AL unchanged");
    assert!(emu.flags().f_sf, "SF set (result has high bit)");
}

#[test]
fn test_test_al_imm8_parity() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xa8, 0x03, 0xf4]; // TEST AL, 0x03
    emu.regs_mut().rax = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_pf, "PF set (even parity)");
}

// ============================================================================
// TEST AX/EAX/RAX, imm
// ============================================================================

#[test]
fn test_test_ax_imm16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xa9, 0x0F, 0x00, 0xf4]; // TEST AX, 0x000F
    emu.regs_mut().rax = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x1234, "AX unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_eax_imm32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xa9, 0xFF, 0x00, 0x00, 0x00, 0xf4]; // TEST EAX, 0x000000FF
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x12345678, "EAX unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_rax_imm32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xa9, 0xFF, 0xFF, 0x00, 0x00, 0xf4]; // TEST RAX, 0x0000FFFF
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "RAX unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

// ============================================================================
// TEST r/m8, imm8
// ============================================================================

#[test]
fn test_test_rm8_imm8_bl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xc3, 0x0F, 0xf4]; // TEST BL, 0x0F
    emu.regs_mut().rbx = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFF, 0xFF, "BL unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_rm8_imm8_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xc1, 0xAA, 0xf4]; // TEST CL, 0xAA
    emu.regs_mut().rcx = 0x55;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFF, 0x55, "CL unchanged");
    assert!(emu.flags().f_zf, "ZF set (no common bits)");
}

#[test]
fn test_test_rm8_imm8_dh() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xc6, 0x80, 0xf4]; // TEST DH, 0x80
    emu.regs_mut().rdx = 0xFF00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx, 0xFF00, "DH unchanged");
    assert!(emu.flags().f_sf, "SF set");
}

// ============================================================================
// TEST r/m16, imm16
// ============================================================================

#[test]
fn test_test_rm16_imm16_bx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xc3, 0xF0, 0x0F, 0xf4]; // TEST BX, 0x0FF0
    emu.regs_mut().rbx = 0xFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFF, 0xFFFF, "BX unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_rm16_imm16_si() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xc6, 0x00, 0xFF, 0xf4]; // TEST SI, 0xFF00
    emu.regs_mut().rsi = 0x00FF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi & 0xFFFF, 0x00FF, "SI unchanged");
    assert!(emu.flags().f_zf, "ZF set (no common bits)");
}

// ============================================================================
// TEST r/m32, imm32
// ============================================================================

#[test]
fn test_test_rm32_imm32_ebx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xc3, 0x00, 0xFF, 0x00, 0x00, 0xf4]; // TEST EBX, 0x0000FF00
    emu.regs_mut().rbx = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0x12345678, "EBX unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_rm32_imm32_esi() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xc6, 0xFF, 0x00, 0x00, 0x00, 0xf4]; // TEST ESI, 0x000000FF
    emu.regs_mut().rsi = 0xABCDEF00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi, 0xABCDEF00, "ESI unchanged");
    assert!(emu.flags().f_zf, "ZF set");
}

// ============================================================================
// TEST r/m64, imm32
// ============================================================================

#[test]
fn test_test_rm64_imm32_rbx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xc3, 0xFF, 0xFF, 0xFF, 0x00, 0xf4]; // TEST RBX, 0x00FFFFFF
    emu.regs_mut().rbx = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0x123456789ABCDEF0, "RBX unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

// ============================================================================
// TEST r/m, r
// ============================================================================

#[test]
fn test_test_rm8_r8_al_bl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x84, 0xd8, 0xf4]; // TEST AL, BL
    emu.regs_mut().rax = 0xFF;
    emu.regs_mut().rbx = 0x0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "AL unchanged");
    assert_eq!(emu.regs().rbx & 0xFF, 0x0F, "BL unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_rm8_r8_no_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x84, 0xd8, 0xf4]; // TEST AL, BL
    emu.regs_mut().rax = 0xAA;
    emu.regs_mut().rbx = 0x55;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xAA, "AL unchanged");
    assert_eq!(emu.regs().rbx & 0xFF, 0x55, "BL unchanged");
    assert!(emu.flags().f_zf, "ZF set (no common bits)");
}

#[test]
fn test_test_rm16_r16_ax_bx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x85, 0xd8, 0xf4]; // TEST AX, BX
    emu.regs_mut().rax = 0xFFFF;
    emu.regs_mut().rbx = 0x00FF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xFFFF, "AX unchanged");
    assert_eq!(emu.regs().rbx & 0xFFFF, 0x00FF, "BX unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_rm32_r32_eax_ebx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x85, 0xd8, 0xf4]; // TEST EAX, EBX
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0xF0F0F0F0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x12345678, "EAX unchanged");
    assert_eq!(emu.regs().rbx, 0xF0F0F0F0, "EBX unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_rm64_r64_rax_rbx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x85, 0xd8, 0xf4]; // TEST RAX, RBX
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    emu.regs_mut().rbx = 0x00000000FFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "RAX unchanged");
    assert_eq!(emu.regs().rbx, 0x00000000FFFFFFFF, "RBX unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

// ============================================================================
// Different register combinations
// ============================================================================

#[test]
fn test_test_cl_dl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x84, 0xd1, 0xf4]; // TEST CL, DL
    emu.regs_mut().rcx = 0xFF;
    emu.regs_mut().rdx = 0x3C;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFF, 0xFF, "CL unchanged");
    assert_eq!(emu.regs().rdx & 0xFF, 0x3C, "DL unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_ecx_edx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x85, 0xd1, 0xf4]; // TEST ECX, EDX
    emu.regs_mut().rcx = 0xF0F0F0F0;
    emu.regs_mut().rdx = 0x0F0F0F0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx, 0xF0F0F0F0, "ECX unchanged");
    assert_eq!(emu.regs().rdx, 0x0F0F0F0F, "EDX unchanged");
    assert!(emu.flags().f_zf, "ZF set (complementary)");
}

#[test]
fn test_test_rsi_rdi() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x85, 0xfe, 0xf4]; // TEST RSI, RDI
    emu.regs_mut().rsi = 0xAAAAAAAAAAAAAAAA;
    emu.regs_mut().rdi = 0xAAAAAAAAAAAAAAAA;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi, 0xAAAAAAAAAAAAAAAA, "RSI unchanged");
    assert_eq!(emu.regs().rdi, 0xAAAAAAAAAAAAAAAA, "RDI unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_test_r8b_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x41, 0xf6, 0xc0, 0x0F, 0xf4]; // TEST R8B, 0x0F
    emu.regs_mut().r8 = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0xFF, "R8B unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_r9w_imm16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x41, 0xf7, 0xc1, 0xF0, 0x0F, 0xf4]; // TEST R9W, 0x0FF0
    emu.regs_mut().r9 = 0xFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r9 & 0xFFFF, 0xFFFF, "R9W unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_r10d_imm32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x41, 0xf7, 0xc2, 0xFF, 0x00, 0x00, 0x00, 0xf4]; // TEST R10D, 0x000000FF
    emu.regs_mut().r10 = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10, 0x12345678, "R10D unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_r11_imm32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x49, 0xf7, 0xc3, 0xFF, 0xFF, 0x00, 0x00, 0xf4]; // TEST R11, 0x0000FFFF
    emu.regs_mut().r11 = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r11, 0x123456789ABCDEF0, "R11 unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_r12d_r13d() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x45, 0x85, 0xec, 0xf4]; // TEST R12D, R13D
    emu.regs_mut().r12 = 0xFFFFFFFF;
    emu.regs_mut().r13 = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r12, 0xFFFFFFFF, "R12D unchanged");
    assert_eq!(emu.regs().r13, 0x12345678, "R13D unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_r14_r15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x4d, 0x85, 0xfe, 0xf4]; // TEST R14, R15
    emu.regs_mut().r14 = 0xFFFFFFFF00000000;
    emu.regs_mut().r15 = 0x00000000FFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r14, 0xFFFFFFFF00000000, "R14 unchanged");
    assert_eq!(emu.regs().r15, 0x00000000FFFFFFFF, "R15 unchanged");
    assert!(emu.flags().f_zf, "ZF set (no common bits)");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_test_byte_ptr_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf6, 0x05, 0xf9, 0x0f, 0x00, 0x00, 0x0F, // TEST BYTE PTR [rip+0x0FF9], 0x0F
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0xFF);

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0xFF, "Memory unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_dword_ptr_imm32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf7, 0x05, 0xf6, 0x0f, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, // TEST DWORD PTR [rip+0x0FF6], 0x000000FF
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x12345678);

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x12345678, "Memory unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

// ============================================================================
// Flag behavior tests
// ============================================================================

#[test]
fn test_test_clears_of_cf() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xa8, 0xFF, 0xf4]; // TEST AL, 0xFF
    emu.regs_mut().rax = 0xFF;
    emu.flags_mut().load(0x2 | flags::F_OF | flags::F_CF);
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_of, "OF cleared by TEST");
    assert!(!emu.flags().f_cf, "CF cleared by TEST");
}

// ============================================================================
// Practical use cases
// ============================================================================

#[test]
fn test_test_check_multiple_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xa8, 0x18, 0xf4]; // TEST AL, 0x18 (bits 3 and 4)
    emu.regs_mut().rax = 0x1F; // bits 3 and 4 are set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_zf, "ZF clear (at least one bit is set)");
}

#[test]
fn test_test_register_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x85, 0xc0, 0xf4]; // TEST EAX, EAX
    emu.regs_mut().rax = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_zf, "ZF set (register is zero)");
    assert!(!emu.flags().f_sf, "SF clear");
}

#[test]
fn test_test_register_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x85, 0xc0, 0xf4]; // TEST RAX, RAX
    emu.regs_mut().rax = 0x8000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_zf, "ZF clear");
    assert!(emu.flags().f_sf, "SF set (high bit set)");
}
