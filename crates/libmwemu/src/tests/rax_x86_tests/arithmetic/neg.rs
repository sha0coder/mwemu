use crate::*;

// NEG — Two's Complement Negation
//
// Opcodes:
// - F6 /3       NEG r/m8      Two's complement negate r/m8
// - REX + F6 /3 NEG r/m8*     Two's complement negate r/m8 (with REX for extended regs)
// - F7 /3       NEG r/m16     Two's complement negate r/m16
// - F7 /3       NEG r/m32     Two's complement negate r/m32
// - REX.W+F7 /3 NEG r/m64     Two's complement negate r/m64
//
// Operation: IF DEST = 0 THEN CF := 0; ELSE CF := 1; FI; DEST := -(DEST)
//            (Equivalent to: DEST := 0 - DEST)
//
// Flags: CF is set to 0 if source is 0, otherwise 1.
//        OF, SF, ZF, AF, PF are set according to result.
//
// CRITICAL: NEG of the most negative value (e.g., 0x80 for i8) causes signed overflow
// because the positive equivalent cannot be represented.

// ============================================================================
// 8-bit NEG (opcode F6 /3)
// ============================================================================

#[test]
fn test_neg_al_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf6, 0xd8, // NEG AL (F6 /3, ModRM=11_011_000)
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0x42; // 66
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xBE, "NEG 0x42 (66) = 0xBE (-66 in two's complement)");
    assert!(emu.flags().f_cf, "CF should be set (operand was non-zero)");
    assert!(emu.flags().f_sf, "SF should be set (negative result)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_neg_al_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    emu.regs_mut().rax = 0x01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "NEG 1 = 0xFF (-1 in two's complement)");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_neg_al_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // NEG of -1 (0xFF) should give 1
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    emu.regs_mut().rax = 0xFF; // -1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x01, "NEG 0xFF (-1) = 1");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(!emu.flags().f_sf, "SF should be clear (positive result)");
}

#[test]
fn test_neg_al_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CRITICAL: NEG 0 = 0, and CF should be CLEAR (special case)
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0, "NEG 0 = 0");
    assert!(!emu.flags().f_cf, "CF should be CLEAR (operand was zero)");
    assert!(emu.flags().f_zf, "ZF should be set");
    assert!(!emu.flags().f_sf, "SF should be clear");
}

#[test]
fn test_neg_al_signed_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // NEG of 0x80 (-128, the most negative i8) = 0x80 (cannot represent +128)
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    emu.regs_mut().rax = 0x80; // -128
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x80, "NEG 0x80 (-128) = 0x80 (overflow)");
    assert!(emu.flags().f_cf, "CF should be set (non-zero operand)");
    assert!(emu.flags().f_of, "OF should be set (signed overflow)");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_neg_al_max_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // NEG of 0x7F (127, max positive i8) = 0x81 (-127)
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    emu.regs_mut().rax = 0x7F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x81, "NEG 0x7F (127) = 0x81 (-127)");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_sf, "SF should be set");
    assert!(!emu.flags().f_of, "OF should be clear (no overflow)");
}

#[test]
fn test_neg_bl_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf6, 0xdb, // NEG BL (F6 /3, ModRM=11_011_011)
        0xf4,
    ];
    emu.regs_mut().rbx = 0x05;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFF, 0xFB, "NEG 5 = 0xFB (-5)");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_neg_cl_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xd9, 0xf4]; // NEG CL
    emu.regs_mut().rcx = 0x0A;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFF, 0xF6, "NEG 10 = 0xF6 (-10)");
}

#[test]
fn test_neg_preserves_high_bytes_8bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    emu.regs_mut().rax = 0xDEADBEEF_12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x88, "AL: NEG 0x78 = 0x88");
    assert_eq!(emu.regs().rax & !0xFF, 0xDEADBEEF_12345600, "High bytes preserved");
}

// ============================================================================
// 16-bit NEG (opcode F7 /3 with 0x66 prefix)
// ============================================================================

#[test]
fn test_neg_ax_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0xf7, 0xd8, // NEG AX (66 F7 /3)
        0xf4,
    ];
    emu.regs_mut().rax = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xEDCC, "NEG 0x1234 = 0xEDCC");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_neg_ax_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xd8, 0xf4]; // NEG AX
    emu.regs_mut().rax = 0x0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0, "NEG 0 = 0");
    assert!(!emu.flags().f_cf, "CF should be clear (zero operand)");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_neg_ax_signed_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // NEG 0x8000 (-32768) = 0x8000 (overflow)
    let code = [0x66, 0xf7, 0xd8, 0xf4]; // NEG AX
    emu.regs_mut().rax = 0x8000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x8000, "NEG 0x8000 = 0x8000 (overflow)");
    assert!(emu.flags().f_of, "OF should be set");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_neg_ax_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // NEG -100 (0xFF9C) = 100 (0x0064)
    let code = [0x66, 0xf7, 0xd8, 0xf4]; // NEG AX
    emu.regs_mut().rax = 0xFF9C; // -100
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0064, "NEG -100 = 100");
    assert!(!emu.flags().f_sf, "SF should be clear");
}

#[test]
fn test_neg_bx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xdb, 0xf4]; // NEG BX
    emu.regs_mut().rbx = 0x0001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFF, 0xFFFF, "NEG 1 = 0xFFFF (-1)");
}

#[test]
fn test_neg_preserves_high_bytes_16bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xd8, 0xf4]; // NEG AX
    emu.regs_mut().rax = 0xDEADBEEF_12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xA988, "AX: NEG 0x5678");
    assert_eq!(emu.regs().rax & !0xFFFF, 0xDEADBEEF_12340000, "Upper bits preserved");
}

// ============================================================================
// 32-bit NEG (opcode F7 /3, no prefix in 64-bit mode)
// ============================================================================

#[test]
fn test_neg_eax_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf7, 0xd8, // NEG EAX (F7 /3)
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xEDCBA988, "NEG 0x12345678 = 0xEDCBA988");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_neg_eax_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xd8, 0xf4]; // NEG EAX
    emu.regs_mut().rax = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "NEG 0 = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_neg_eax_signed_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // NEG 0x80000000 (-2147483648) = 0x80000000 (overflow)
    let code = [0xf7, 0xd8, 0xf4]; // NEG EAX
    emu.regs_mut().rax = 0x80000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x80000000, "NEG 0x80000000 = 0x80000000 (overflow)");
    assert!(emu.flags().f_of, "OF should be set");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_neg_eax_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xd8, 0xf4]; // NEG EAX
    emu.regs_mut().rax = 0x00000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFF, "NEG 1 = 0xFFFFFFFF (-1)");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_neg_eax_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // NEG -1 (0xFFFFFFFF) = 1
    let code = [0xf7, 0xd8, 0xf4]; // NEG EAX
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x00000001, "NEG -1 = 1");
    assert!(!emu.flags().f_sf, "SF should be clear");
}

#[test]
fn test_neg_ebx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xdb, 0xf4]; // NEG EBX
    emu.regs_mut().rbx = 0x000000FF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0xFFFFFF01, "NEG 255 = 0xFFFFFF01");
}

#[test]
fn test_neg_ecx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xd9, 0xf4]; // NEG ECX
    emu.regs_mut().rcx = 0x7FFFFFFF; // Max positive i32
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx, 0x80000001, "NEG 0x7FFFFFFF = 0x80000001");
}

// ============================================================================
// 64-bit NEG (opcode REX.W + F7 /3)
// ============================================================================

#[test]
fn test_neg_rax_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xf7, 0xd8, // NEG RAX (REX.W F7 /3)
        0xf4,
    ];
    emu.regs_mut().rax = 0x1234567890ABCDEF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xEDCBA9876F543211, "NEG 0x1234567890ABCDEF");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_neg_rax_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xd8, 0xf4]; // NEG RAX
    emu.regs_mut().rax = 0x0000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "NEG 0 = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_neg_rax_signed_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // NEG 0x8000000000000000 (most negative i64) = overflow
    let code = [0x48, 0xf7, 0xd8, 0xf4]; // NEG RAX
    emu.regs_mut().rax = 0x8000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x8000000000000000, "NEG 0x8000...000 = overflow");
    assert!(emu.flags().f_of, "OF should be set");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_neg_rax_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xd8, 0xf4]; // NEG RAX
    emu.regs_mut().rax = 0x0000000000000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "NEG 1 = -1");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_neg_rax_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // NEG -1 = 1
    let code = [0x48, 0xf7, 0xd8, 0xf4]; // NEG RAX
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000001, "NEG -1 = 1");
    assert!(!emu.flags().f_sf, "SF should be clear");
}

#[test]
fn test_neg_rbx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xdb, 0xf4]; // NEG RBX
    emu.regs_mut().rbx = 0x0000000000000100;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0xFFFFFFFFFFFFFF00, "NEG 256");
}

#[test]
fn test_neg_rcx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xd9, 0xf4]; // NEG RCX
    emu.regs_mut().rcx = 0x7FFFFFFFFFFFFFFF; // Max positive i64
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx, 0x8000000000000001, "NEG max_i64");
}

#[test]
fn test_neg_rdx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xda, 0xf4]; // NEG RDX
    emu.regs_mut().rdx = 0xFFFFFFFFFFFFFFFF; // -1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx, 1, "NEG -1 = 1");
}

// ============================================================================
// Extended registers (R8-R15) with REX prefix
// ============================================================================

#[test]
fn test_neg_r8b_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x41, 0xf6, 0xd8, // NEG R8B (REX.B F6 /3)
        0xf4,
    ];
    emu.regs_mut().r8 = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0xBE, "NEG R8B: 0x42 -> 0xBE");
}

#[test]
fn test_neg_r9w_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0x41, 0xf7, 0xd9, // NEG R9W
        0xf4,
    ];
    emu.regs_mut().r9 = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r9 & 0xFFFF, 0xEDCC, "NEG R9W works");
}

#[test]
fn test_neg_r10d_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x41, 0xf7, 0xda, // NEG R10D
        0xf4,
    ];
    emu.regs_mut().r10 = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10, 0xEDCBA988, "NEG R10D works");
}

#[test]
fn test_neg_r11_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x49, 0xf7, 0xdb, // NEG R11 (REX.WB F7 /3)
        0xf4,
    ];
    emu.regs_mut().r11 = 0x1234567890ABCDEF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r11, 0xEDCBA9876F543211, "NEG R11 works");
}

#[test]
fn test_neg_r15_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x49, 0xf7, 0xdf, 0xf4]; // NEG R15
    emu.regs_mut().r15 = 0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 0, "NEG 0 = 0");
    assert!(!emu.flags().f_cf, "CF should be clear (zero operand)");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_neg_byte_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf6, 0x1d, 0xfa, 0x0f, 0x00, 0x00, // NEG BYTE PTR [rip+0x0FFA]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0x42);

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0xBE, "NEG byte [mem]: 0x42 -> 0xBE");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_neg_word_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0xf7, 0x1d, 0xf9, 0x0f, 0x00, 0x00, // NEG WORD PTR [rip+0x0FF9]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0x1234);

    emu.run(None).unwrap();
    let result = emu.maps.read_word(DATA_ADDR).unwrap();

    assert_eq!(result, 0xEDCC, "NEG word [mem]: 0x1234 -> 0xEDCC");
}

#[test]
fn test_neg_dword_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf7, 0x1d, 0xfa, 0x0f, 0x00, 0x00, // NEG DWORD PTR [rip+0x0FFA]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x12345678);

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(result, 0xEDCBA988, "NEG dword [mem] works");
}

#[test]
fn test_neg_qword_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xf7, 0x1d, 0xf9, 0x0f, 0x00, 0x00, // NEG QWORD PTR [rip+0x0FF9]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x1234567890ABCDEF);

    emu.run(None).unwrap();
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();

    assert_eq!(result, 0xEDCBA9876F543211, "NEG qword [mem] works");
}

#[test]
fn test_neg_mem_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf6, 0x1d, 0xfa, 0x0f, 0x00, 0x00, // NEG BYTE PTR [rip+0x0FFA]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0x00);

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0, "NEG 0 = 0");
    assert!(!emu.flags().f_cf, "CF should be clear (zero operand)");
    assert!(emu.flags().f_zf, "ZF should be set");
}

// ============================================================================
// Parity flag tests
// ============================================================================

#[test]
fn test_neg_parity_flag_even() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // NEG 3 = 0xFD (0b11111101, seven 1-bits = odd parity, PF=0)
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    emu.regs_mut().rax = 0x03;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFD);
    assert!(!emu.flags().f_pf, "PF should be clear (odd parity)");
}

#[test]
fn test_neg_parity_flag_odd() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // NEG 1 = 0xFF (0b11111111, eight 1-bits = even parity, PF=1)
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    emu.regs_mut().rax = 0x01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF);
    assert!(emu.flags().f_pf, "PF should be set (even parity)");
}

// ============================================================================
// Auxiliary carry flag tests
// ============================================================================

#[test]
fn test_neg_auxiliary_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // NEG changes AF based on borrow from bit 3
    // NEG 0x0F = 0xF1: 0 - 0x0F requires borrow, AF should be set
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    emu.regs_mut().rax = 0x0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xF1);
    assert!(emu.flags().f_af, "AF should be set");
}

// ============================================================================
// Edge cases and special scenarios
// ============================================================================

#[test]
fn test_neg_double_negation() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // NEG(NEG(x)) should equal x (except for overflow cases)
    let code = [
        0xf6, 0xd8, // NEG AL (first time)
        0xf6, 0xd8, // NEG AL (second time)
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "NEG(NEG(0x42)) = 0x42");
}

#[test]
fn test_neg_all_operand_sizes_non_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();

    // 8-bit
    let code = [0xf6, 0xd8, 0xf4];
    emu.regs_mut().rax = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert!(emu.flags().f_cf, "8-bit: CF should be set for non-zero");

    // 16-bit
    let code = [0x66, 0xf7, 0xd8, 0xf4];
    emu.regs_mut().rax = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert!(emu.flags().f_cf, "16-bit: CF should be set for non-zero");

    // 32-bit
    let code = [0xf7, 0xd8, 0xf4];
    emu.regs_mut().rax = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert!(emu.flags().f_cf, "32-bit: CF should be set for non-zero");

    // 64-bit
    let code = [0x48, 0xf7, 0xd8, 0xf4];
    emu.regs_mut().rax = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert!(emu.flags().f_cf, "64-bit: CF should be set for non-zero");
}
