use crate::*;

// OR — Logical Inclusive OR
//
// Opcodes: Similar structure to AND but opcode /1 instead of /4
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
//        AF is undefined.
//
// CRITICAL: OR is bitwise inclusive OR. Each bit is 1 if EITHER (or both)
// corresponding bits are 1. Commonly used to set specific bits.

// ============================================================================
// OR with immediate
// ============================================================================

#[test]
fn test_or_al_imm8_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0c, 0x0F, // OR AL, 0x0F
        0xf4,
    ];
    emu.regs_mut().rax = 0xA0; // 10100000
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0xA0 | 0x0F = 10100000 | 00001111 = 10101111 = 0xAF
    assert_eq!(emu.regs().rax & 0xFF, 0xAF, "AL: 0xA0 OR 0x0F = 0xAF");
    assert!(!emu.flags().f_zf, "ZF should be clear");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_of, "OF should be clear");
    assert!(emu.flags().f_sf, "SF should be set (high bit = 1)");
}

#[test]
fn test_or_al_imm8_zero_with_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // OR with 0 preserves value (identity operation)
    let code = [0x0c, 0x00, 0xf4]; // OR AL, 0
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL: 0x42 OR 0 = 0x42 (identity)");
}

#[test]
fn test_or_al_imm8_with_all_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // OR with 0xFF sets all bits
    let code = [0x0c, 0xFF, 0xf4]; // OR AL, 0xFF
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "AL: 0 OR 0xFF = 0xFF");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_or_eax_imm32_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0d, 0x00, 0xFF, 0x00, 0x00, // OR EAX, 0x0000FF00
        0xf4,
    ];
    emu.regs_mut().rax = 0x12000034;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x1200FF34, "EAX: set byte 1");
}

#[test]
fn test_or_rax_imm32_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x0d, 0x00, 0x00, 0xFF, 0x00, // OR RAX, 0x00FF0000 (sign-extended)
        0xf4,
    ];
    emu.regs_mut().rax = 0x1234567800000078;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x1234567800FF0078, "RAX: set byte 2");
}

// ============================================================================
// OR r/m with immediate
// ============================================================================

#[test]
fn test_or_rm8_imm8_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x80, 0xcb, 0x0F, // OR BL, 0x0F
        0xf4,
    ];
    emu.regs_mut().rbx = 0xF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFF, 0xFF, "BL: 0xF0 OR 0x0F = 0xFF");
}

#[test]
fn test_or_rm32_imm32_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x81, 0xcb, 0x00, 0x00, 0xFF, 0x00, // OR EBX, 0x00FF0000
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0x00FF0000, "EBX: set byte 2");
}

#[test]
fn test_or_rm64_imm8_sign_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // OR RBX, -1 (sign-extended to 0xFFFFFFFFFFFFFFFF)
    let code = [
        0x48, 0x83, 0xcb, 0xFF, // OR RBX, 0xFF (sign-extended to -1)
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0xFFFFFFFFFFFFFFFF, "RBX: OR with -1 sets all bits");
}

// ============================================================================
// OR r/m, r
// ============================================================================

#[test]
fn test_or_rm8_r8_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x08, 0xd8, // OR AL, BL
        0xf4,
    ];
    emu.regs_mut().rax = 0xAA; // 10101010
    emu.regs_mut().rbx = 0x55; // 01010101
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "AL: 0xAA OR 0x55 = 0xFF (all bits)");
}

#[test]
fn test_or_rm32_r32_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x09, 0xd8, // OR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0xFF00FF00;
    emu.regs_mut().rbx = 0x00FF00FF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFF, "EAX: complementary bytes OR");
}

#[test]
fn test_or_rm64_r64_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x09, 0xd8, // OR RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFF00000000;
    emu.regs_mut().rbx = 0x00000000FFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "RAX: all bits set");
}

// ============================================================================
// OR r, r/m
// ============================================================================

#[test]
fn test_or_r8_rm8_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0a, 0xc3, // OR AL, BL
        0xf4,
    ];
    emu.regs_mut().rax = 0x0F;
    emu.regs_mut().rbx = 0xF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "AL: 0x0F OR 0xF0 = 0xFF");
}

#[test]
fn test_or_r32_rm32_same_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // OR EAX, EAX is a common idiom to test if EAX is zero
    let code = [
        0x0b, 0xc0, // OR EAX, EAX
        0xf4,
    ];
    emu.regs_mut().rax = 0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "EAX: 0 OR 0 = 0");
    assert!(emu.flags().f_zf, "ZF set (zero result)");
}

#[test]
fn test_or_r32_rm32_non_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0b, 0xc0, // OR EAX, EAX
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x12345678, "EAX unchanged");
    assert!(!emu.flags().f_zf, "ZF clear (non-zero)");
}

// ============================================================================
// Bit setting use cases
// ============================================================================

#[test]
fn test_or_set_specific_bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0c, 0x10, 0xf4]; // OR AL, 0x10
    emu.regs_mut().rax = 0x0F; // bit 4 is clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x1F, "Set bit 4: 0x0F | 0x10 = 0x1F");
}

#[test]
fn test_or_set_multiple_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0c, 0x50, 0xf4]; // OR AL, 0x50 (01010000)
    emu.regs_mut().rax = 0x0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x5F, "Set bits 4 and 6");
}

#[test]
fn test_or_set_high_byte() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0d, 0x00, 0x00, 0x00, 0xFF, // OR EAX, 0xFF000000
        0xf4,
    ];
    emu.regs_mut().rax = 0x00123456;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFF123456, "EAX: set high byte");
}

// ============================================================================
// OF and CF always cleared
// ============================================================================

#[test]
fn test_or_clears_of_cf() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0c, 0xFF, 0xf4]; // OR AL, 0xFF
    emu.regs_mut().rax = 0x00;
    emu.flags_mut().load(0x2 | flags::F_OF | flags::F_CF);
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_of, "OF cleared by OR");
    assert!(!emu.flags().f_cf, "CF cleared by OR");
}

// ============================================================================
// Parity flag tests
// ============================================================================

#[test]
fn test_or_parity_even() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0c, 0x03, 0xf4]; // OR AL, 0x03
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03);
    assert!(emu.flags().f_pf, "PF set (even parity)");
}

#[test]
fn test_or_parity_odd() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0c, 0x07, 0xf4]; // OR AL, 0x07
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x07);
    assert!(!emu.flags().f_pf, "PF clear (odd parity)");
}

// ============================================================================
// Zero flag tests
// ============================================================================

#[test]
fn test_or_zero_result_only_from_zero_operands() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x08, 0xd8, 0xf4]; // OR AL, BL
    emu.regs_mut().rax = 0;
    emu.regs_mut().rbx = 0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0, "AL: 0 OR 0 = 0");
    assert!(emu.flags().f_zf, "ZF set");
}

// ============================================================================
// Different registers
// ============================================================================

#[test]
fn test_or_different_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // OR CL, DL
    let code = [0x08, 0xd1, 0xf4];
    emu.regs_mut().rcx = 0x0F;
    emu.regs_mut().rdx = 0xF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFF, 0xFF, "CL: 0x0F OR 0xF0 = 0xFF");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_or_r8b_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x41, 0x80, 0xc8, 0x0F, // OR R8B, 0x0F
        0xf4,
    ];
    emu.regs_mut().r8 = 0xF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0xFF, "R8B: 0xF0 OR 0x0F = 0xFF");
}

#[test]
fn test_or_r10d_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x41, 0x81, 0xca, 0x00, 0xFF, 0x00, 0x00, // OR R10D, 0x0000FF00
        0xf4,
    ];
    emu.regs_mut().r10 = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10, 0x0000FF00, "R10D: set byte 1");
}

#[test]
fn test_or_r11_r12() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x4d, 0x09, 0xe3, // OR R11, R12
        0xf4,
    ];
    emu.regs_mut().r11 = 0xFF00FF00FF00FF00;
    emu.regs_mut().r12 = 0x00FF00FF00FF00FF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r11, 0xFFFFFFFFFFFFFFFF, "R11: all bits set");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_or_byte_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x80, 0x0d, 0xf9, 0x0f, 0x00, 0x00, 0x0F, // OR BYTE PTR [rip+0x0FF9], 0x0F
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0xF0);

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0xFF, "Memory: 0xF0 OR 0x0F = 0xFF");
}

#[test]
fn test_or_dword_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x81, 0x0d, 0xf6, 0x0f, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, // OR DWORD PTR [rip+0x0FF6], 0x00FF0000
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x00000000);

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x00FF0000, "Memory: set byte 2");
}

// ============================================================================
// Practical use cases
// ============================================================================

#[test]
fn test_or_combine_flags() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x0c, 0x15, 0xf4]; // OR AL, 0x15 (bits 0, 2, 4)
    emu.regs_mut().rax = 0x0A; // bits 1, 3 already set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x0A | 0x15 = 0b00001010 | 0b00010101 = 0b00011111 = 0x1F
    assert_eq!(emu.regs().rax & 0xFF, 0x1F, "Combined flags");
}

#[test]
fn test_or_idempotent() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // OR is idempotent: x OR x = x
    let code = [0x08, 0xc0, 0xf4]; // OR AL, AL
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL: x OR x = x (idempotent)");
}

#[test]
fn test_or_test_for_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x0b, 0xc0, 0xf4]; // OR RAX, RAX
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "RAX unchanged");
    assert!(!emu.flags().f_zf, "ZF clear (non-zero)");
    assert!(!emu.flags().f_sf, "SF clear (high bit = 0)");
    assert!(!emu.flags().f_of, "OF cleared");
    assert!(!emu.flags().f_cf, "CF cleared");
}
