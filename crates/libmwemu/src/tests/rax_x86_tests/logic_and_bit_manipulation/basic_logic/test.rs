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
// Operation: temp = SRC1 AND SRC2; set flags; discard temp
//
// Flags: OF and CF are CLEARED.
//        SF, ZF, PF are set according to result.
//        AF is undefined.
//
// CRITICAL: TEST performs AND but discards the result - operands unchanged.
// Commonly used to test if bits are set or if register is zero.

// ============================================================================
// TEST with immediate
// ============================================================================

#[test]
fn test_test_al_imm8_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xa8, 0x0F, // TEST AL, 0x0F
        0xf4,
    ];
    emu.regs_mut().rax = 0xAB;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0xAB & 0x0F = 0x0B (non-zero)
    assert_eq!(emu.regs().rax & 0xFF, 0xAB, "AL unchanged by TEST");
    assert!(!emu.flags().f_zf, "ZF clear (result non-zero)");
    assert!(!emu.flags().f_cf, "CF cleared");
    assert!(!emu.flags().f_of, "OF cleared");
}

#[test]
fn test_test_al_imm8_zero_result() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xa8, 0x0F, 0xf4]; // TEST AL, 0x0F
    emu.regs_mut().rax = 0xF0; // No bits in common with 0x0F
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0xF0 & 0x0F = 0x00 (zero)
    assert_eq!(emu.regs().rax & 0xFF, 0xF0, "AL unchanged");
    assert!(emu.flags().f_zf, "ZF set (result is zero)");
    assert!(!emu.flags().f_cf, "CF cleared");
}

#[test]
fn test_test_al_imm8_sign_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xa8, 0xFF, 0xf4]; // TEST AL, 0xFF
    emu.regs_mut().rax = 0x80; // High bit set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x80 & 0xFF = 0x80 (high bit set)
    assert!(emu.flags().f_sf, "SF set (high bit = 1)");
}

#[test]
fn test_test_eax_imm32_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xa9, 0x00, 0xFF, 0x00, 0x00, // TEST EAX, 0x0000FF00
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x12345678 & 0x0000FF00 = 0x00005600 (non-zero)
    assert_eq!(emu.regs().rax, 0x12345678, "EAX unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_rax_imm32_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xa9, 0xFF, 0xFF, 0x00, 0x00, // TEST RAX, 0x0000FFFF
        0xf4,
    ];
    emu.regs_mut().rax = 0x123456789ABC0000; // Low 16 bits are zero
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x123456789ABC0000 & 0x0000FFFF = 0x0000 (zero)
    assert_eq!(emu.regs().rax, 0x123456789ABC0000, "RAX unchanged");
    assert!(emu.flags().f_zf, "ZF set (no bits in common)");
}

// ============================================================================
// TEST r/m with immediate
// ============================================================================

#[test]
fn test_test_rm8_imm8_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf6, 0xc3, 0x0F, // TEST BL, 0x0F (F6 /0 ib)
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0xFF & 0x0F = 0x0F (non-zero)
    assert_eq!(emu.regs().rbx & 0xFF, 0xFF, "BL unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_rm32_imm32_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf7, 0xc3, 0x00, 0x00, 0xFF, 0x00, // TEST EBX, 0x00FF0000
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x12345678 & 0x00FF0000 = 0x00340000 (non-zero)
    assert_eq!(emu.regs().rbx, 0x12345678, "EBX unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_rm64_imm32_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xf7, 0xc3, 0xFF, 0xFF, 0xFF, 0xFF, // TEST RBX, 0xFFFFFFFF
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF00000000; // High 32 bits set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0xFFFFFFFF00000000, "RBX unchanged");
    assert!(!emu.flags().f_zf, "ZF clear (sign-extended imm32 tests all bits)");
}

// ============================================================================
// TEST r/m, r
// ============================================================================

#[test]
fn test_test_rm8_r8_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x84, 0xd8, // TEST AL, BL
        0xf4,
    ];
    emu.regs_mut().rax = 0xAA; // 10101010
    emu.regs_mut().rbx = 0x55; // 01010101
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0xAA & 0x55 = 0x00 (no common bits)
    assert_eq!(emu.regs().rax & 0xFF, 0xAA, "AL unchanged");
    assert_eq!(emu.regs().rbx & 0xFF, 0x55, "BL unchanged");
    assert!(emu.flags().f_zf, "ZF set (zero result)");
}

#[test]
fn test_test_rm32_r32_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x85, 0xd8, // TEST EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0xFF00FF00;
    emu.regs_mut().rbx = 0x00FF00FF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0xFF00FF00 & 0x00FF00FF = 0x00000000
    assert_eq!(emu.regs().rax, 0xFF00FF00, "EAX unchanged");
    assert_eq!(emu.regs().rbx, 0x00FF00FF, "EBX unchanged");
    assert!(emu.flags().f_zf, "ZF set");
}

#[test]
fn test_test_rm64_r64_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x85, 0xd8, // TEST RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFF00000000;
    emu.regs_mut().rbx = 0x00000000FFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFF00000000, "RAX unchanged");
    assert_eq!(emu.regs().rbx, 0x00000000FFFFFFFF, "RBX unchanged");
    assert!(emu.flags().f_zf, "ZF set");
}

// ============================================================================
// TEST reg, reg (common idiom to check if reg is zero)
// ============================================================================

#[test]
fn test_test_eax_eax_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TEST EAX, EAX is common idiom to test if EAX is zero
    let code = [
        0x85, 0xc0, // TEST EAX, EAX
        0xf4,
    ];
    emu.regs_mut().rax = 0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "EAX unchanged");
    assert!(emu.flags().f_zf, "ZF set (EAX is zero)");
    assert!(!emu.flags().f_sf, "SF clear");
    assert!(!emu.flags().f_cf, "CF clear");
    assert!(!emu.flags().f_of, "OF clear");
}

#[test]
fn test_test_eax_eax_non_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x85, 0xc0, 0xf4]; // TEST EAX, EAX
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x12345678, "EAX unchanged");
    assert!(!emu.flags().f_zf, "ZF clear (EAX is non-zero)");
}

#[test]
fn test_test_rax_rax_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x85, 0xc0, 0xf4]; // TEST RAX, RAX
    emu.regs_mut().rax = 0x8000000000000000; // High bit set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_zf, "ZF clear (non-zero)");
    assert!(emu.flags().f_sf, "SF set (high bit = 1)");
}

// ============================================================================
// Bit testing use cases
// ============================================================================

#[test]
fn test_test_check_bit_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xa8, 0x10, 0xf4]; // TEST AL, 0x10
    emu.regs_mut().rax = 0x1F; // bit 4 is set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_zf, "ZF clear means bit 4 is set");
}

#[test]
fn test_test_check_bit_clear() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xa8, 0x10, 0xf4]; // TEST AL, 0x10
    emu.regs_mut().rax = 0x0F; // bit 4 is clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_zf, "ZF set means bit 4 is clear");
}

#[test]
fn test_test_check_multiple_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xa8, 0x15, 0xf4]; // TEST AL, 0x15 (00010101)
    emu.regs_mut().rax = 0x04; // Only bit 2 is set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_zf, "ZF clear (at least one bit matches)");
}

#[test]
fn test_test_check_all_bits_clear() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xa8, 0x15, 0xf4]; // TEST AL, 0x15
    emu.regs_mut().rax = 0xEA; // bits 1, 3, 5, 6, 7 set (not 0, 2, 4)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_zf, "ZF set (all tested bits are clear)");
}

// ============================================================================
// Parity flag tests
// ============================================================================

#[test]
fn test_test_parity_even() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xa8, 0x03, 0xf4]; // TEST AL, 0x03
    emu.regs_mut().rax = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_pf, "PF set (even parity)");
}

#[test]
fn test_test_parity_odd() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xa8, 0x07, 0xf4]; // TEST AL, 0x07
    emu.regs_mut().rax = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_pf, "PF clear (odd parity)");
}

// ============================================================================
// OF and CF always cleared
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
// Different registers
// ============================================================================

#[test]
fn test_test_different_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // TEST CL, DL
    let code = [0x84, 0xd1, 0xf4];
    emu.regs_mut().rcx = 0x0F;
    emu.regs_mut().rdx = 0xF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFF, 0x0F, "CL unchanged");
    assert_eq!(emu.regs().rdx & 0xFF, 0xF0, "DL unchanged");
    assert!(emu.flags().f_zf, "ZF set (no common bits)");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_test_r8b_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x41, 0xf6, 0xc0, 0x0F, // TEST R8B, 0x0F
        0xf4,
    ];
    emu.regs_mut().r8 = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0xFF, "R8B unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_r10d_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x41, 0xf7, 0xc2, 0xFF, 0xFF, 0x00, 0x00, // TEST R10D, 0x0000FFFF
        0xf4,
    ];
    emu.regs_mut().r10 = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10, 0x12345678, "R10D unchanged");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_test_r11_r12() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x4d, 0x85, 0xe3, // TEST R11, R12
        0xf4,
    ];
    emu.regs_mut().r11 = 0xAAAAAAAAAAAAAAAA;
    emu.regs_mut().r12 = 0x5555555555555555;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r11, 0xAAAAAAAAAAAAAAAA, "R11 unchanged");
    assert_eq!(emu.regs().r12, 0x5555555555555555, "R12 unchanged");
    assert!(emu.flags().f_zf, "ZF set (no common bits)");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_test_byte_ptr_mem() {
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
    assert!(!emu.flags().f_zf, "ZF clear (0xFF & 0x0F = 0x0F)");
}

#[test]
fn test_test_dword_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf7, 0x05, 0xf6, 0x0f, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, // TEST DWORD PTR [rip+0x0FF6], 0x000000FF
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x12345600); // Low byte is zero

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x12345600, "Memory unchanged");
    assert!(emu.flags().f_zf, "ZF set (low byte is zero)");
}

// ============================================================================
// Practical use cases
// ============================================================================

#[test]
fn test_test_validate_alignment() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xa9, 0x0F, 0x00, 0x00, 0x00, // TEST RAX, 0x0000000F
        0xf4,
    ];
    emu.regs_mut().rax = 0x123456789ABCDEF0; // Aligned (low 4 bits = 0)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_zf, "ZF set means aligned");
}

#[test]
fn test_test_validate_not_aligned() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xa9, 0x0F, 0x00, 0x00, 0x00, // TEST RAX, 0x0000000F
        0xf4,
    ];
    emu.regs_mut().rax = 0x123456789ABCDEF7; // Not aligned
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_zf, "ZF clear means not aligned");
}

#[test]
fn test_test_check_sign_bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0xf4,
    ];
    emu.regs_mut().rax = 0x8000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_sf, "SF set means high bit is set (negative)");
}
