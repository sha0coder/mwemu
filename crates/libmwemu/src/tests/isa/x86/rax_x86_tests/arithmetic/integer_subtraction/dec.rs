use crate::*;

// DEC — Decrement by 1
//
// Opcodes:
// - FE /1       DEC r/m8      Decrement r/m8 by 1
// - REX + FE /1 DEC r/m8*     Decrement r/m8 by 1 (with REX for extended regs)
// - FF /1       DEC r/m16     Decrement r/m16 by 1
// - FF /1       DEC r/m32     Decrement r/m32 by 1
// - REX.W+FF /1 DEC r/m64     Decrement r/m64 by 1
//
// Operation: DEST := DEST - 1
// Flags: CF is NOT affected (preserved). OF, SF, ZF, AF, PF are set according to result.
//
// CRITICAL: Unlike SUB, DEC does NOT affect the CF flag. This allows loop counters
// to be updated without disturbing the carry flag used in multi-precision arithmetic.

// ============================================================================
// 8-bit DEC (opcode FE /1)
// ============================================================================

#[test]
fn test_dec_al_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xfe, 0xc8, // DEC AL (FE /1, ModRM=11_001_000)
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x41, "DEC AL: 0x42 - 1 = 0x41");
    assert!(!emu.flags().f_zf, "ZF should be clear");
    assert!(!emu.flags().f_sf, "SF should be clear (positive result)");
}

#[test]
fn test_dec_al_to_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xfe, 0xc8, 0xf4]; // DEC AL
    emu.regs_mut().rax = 0x01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0, "DEC AL: 1 - 1 = 0");
    assert!(emu.flags().f_zf, "ZF should be set (zero result)");
    assert!(!emu.flags().f_sf, "SF should be clear");
}

#[test]
fn test_dec_al_underflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0 - 1 = 0xFF (unsigned underflow)
    let code = [0xfe, 0xc8, 0xf4]; // DEC AL
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "DEC AL: 0 - 1 = 0xFF (underflow)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
    assert!(emu.flags().f_sf, "SF should be set (negative in signed interpretation)");
}

#[test]
fn test_dec_al_signed_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x80 (-128) - 1 = 0x7F (127) - this is signed overflow
    let code = [0xfe, 0xc8, 0xf4]; // DEC AL
    emu.regs_mut().rax = 0x80;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x7F, "DEC AL: 0x80 - 1 = 0x7F");
    assert!(!emu.flags().f_zf, "ZF should be clear");
    assert!(!emu.flags().f_sf, "SF should be clear (positive result)");
    assert!(emu.flags().f_of, "OF should be set (signed overflow: -128 - 1 = 127)");
}

#[test]
fn test_dec_rm8_preserves_cf_when_clear() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CRITICAL: DEC should NOT affect CF flag
    let code = [0xfe, 0xc8, 0xf4]; // DEC AL
    emu.regs_mut().rax = 0x42;
    emu.flags_mut().load(0x2); // CF=0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x41);
    assert!(!emu.flags().f_cf, "CF should remain clear");
}

#[test]
fn test_dec_rm8_preserves_cf_when_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CRITICAL: DEC should NOT affect CF flag even when set
    let code = [0xfe, 0xc8, 0xf4]; // DEC AL
    emu.regs_mut().rax = 0x42;
    emu.flags_mut().load(0x2 | flags::F_CF); // CF=1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x41);
    assert!(emu.flags().f_cf, "CF should be preserved (still set)");
}

#[test]
fn test_dec_cf_independence_with_underflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xfe, 0xc8, 0xf4]; // DEC AL
    emu.regs_mut().rax = 0x00;
    emu.flags_mut().load(0x2); // CF=0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "Underflow: 0 - 1 = 0xFF");
    assert!(!emu.flags().f_cf, "CF should remain clear (DEC doesn't affect CF)");
}

#[test]
fn test_dec_bl_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xfe, 0xcb, // DEC BL (FE /1, ModRM=11_001_011)
        0xf4,       // HLT
    ];
    emu.regs_mut().rbx = 0x10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFF, 0x0F, "DEC BL: 0x10 - 1 = 0x0F");
}

#[test]
fn test_dec_cl_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xfe, 0xc9, 0xf4]; // DEC CL
    emu.regs_mut().rcx = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFF, 0xFE, "DEC CL: 0xFF - 1 = 0xFE");
}

#[test]
fn test_dec_preserves_high_bytes_8bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xfe, 0xc8, 0xf4]; // DEC AL
    emu.regs_mut().rax = 0xDEADBEEF_12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x77, "AL: 0x78 - 1 = 0x77");
    assert_eq!(emu.regs().rax & !0xFF, 0xDEADBEEF_12345600, "High bytes should be preserved");
}

// ============================================================================
// 16-bit DEC (opcode FF /1 with 0x66 prefix)
// ============================================================================

#[test]
fn test_dec_ax_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0xff, 0xc8, // DEC AX (66 FF /1, ModRM=11_001_000)
        0xf4,             // HLT
    ];
    emu.regs_mut().rax = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x1233, "DEC AX: 0x1234 - 1 = 0x1233");
}

#[test]
fn test_dec_ax_to_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xff, 0xc8, 0xf4]; // DEC AX
    emu.regs_mut().rax = 0x0001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0, "DEC AX: 1 - 1 = 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_dec_ax_underflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xff, 0xc8, 0xf4]; // DEC AX
    emu.regs_mut().rax = 0x0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xFFFF, "DEC AX: 0 - 1 = 0xFFFF");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_dec_ax_signed_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x8000 (-32768) - 1 = 0x7FFF (32767)
    let code = [0x66, 0xff, 0xc8, 0xf4]; // DEC AX
    emu.regs_mut().rax = 0x8000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x7FFF, "DEC AX: 0x8000 - 1 = 0x7FFF");
    assert!(emu.flags().f_of, "OF should be set (signed overflow)");
}

#[test]
fn test_dec_ax_preserves_cf() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xff, 0xc8, 0xf4]; // DEC AX
    emu.regs_mut().rax = 0x1234;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be preserved");
}

#[test]
fn test_dec_bx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xff, 0xcb, 0xf4]; // DEC BX
    emu.regs_mut().rbx = 0x5678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFF, 0x5677, "DEC BX: 0x5678 - 1 = 0x5677");
}

#[test]
fn test_dec_preserves_high_bytes_16bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xff, 0xc8, 0xf4]; // DEC AX
    emu.regs_mut().rax = 0xDEADBEEF_12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x5677);
    assert_eq!(emu.regs().rax & !0xFFFF, 0xDEADBEEF_12340000, "Upper bits preserved");
}

// ============================================================================
// 32-bit DEC (opcode FF /1, no prefix in 64-bit mode)
// ============================================================================

#[test]
fn test_dec_eax_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xff, 0xc8, // DEC EAX (FF /1, ModRM=11_001_000)
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x12345677, "DEC EAX: 0x12345678 - 1 = 0x12345677");
}

#[test]
fn test_dec_eax_to_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xff, 0xc8, 0xf4]; // DEC EAX
    emu.regs_mut().rax = 0x00000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "DEC EAX: 1 - 1 = 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_dec_eax_underflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xff, 0xc8, 0xf4]; // DEC EAX
    emu.regs_mut().rax = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFF, "DEC EAX: 0 - 1 = 0xFFFFFFFF");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_dec_eax_signed_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x80000000 (-2147483648) - 1 = 0x7FFFFFFF (2147483647)
    let code = [0xff, 0xc8, 0xf4]; // DEC EAX
    emu.regs_mut().rax = 0x80000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x7FFFFFFF, "DEC EAX: 0x80000000 - 1 = 0x7FFFFFFF");
    assert!(emu.flags().f_of, "OF should be set (signed overflow)");
}

#[test]
fn test_dec_eax_preserves_cf() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xff, 0xc8, 0xf4]; // DEC EAX
    emu.regs_mut().rax = 0x12345678;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x12345677);
    assert!(emu.flags().f_cf, "CF should be preserved");
}

#[test]
fn test_dec_ebx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xff, 0xcb, 0xf4]; // DEC EBX
    emu.regs_mut().rbx = 0xAABBCCDD;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0xAABBCCDC, "DEC EBX: 0xAABBCCDD - 1 = 0xAABBCCDC");
}

#[test]
fn test_dec_ecx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xff, 0xc9, 0xf4]; // DEC ECX
    emu.regs_mut().rcx = 0x100;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx, 0xFF, "DEC ECX: 0x100 - 1 = 0xFF");
}

#[test]
fn test_dec_edx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xff, 0xca, 0xf4]; // DEC EDX
    emu.regs_mut().rdx = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx, 0xFFFFFFFE, "DEC EDX: 0xFFFFFFFF - 1 = 0xFFFFFFFE");
}

// ============================================================================
// 64-bit DEC (opcode REX.W + FF /1)
// ============================================================================

#[test]
fn test_dec_rax_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xff, 0xc8, // DEC RAX (REX.W FF /1)
        0xf4,             // HLT
    ];
    emu.regs_mut().rax = 0x1234567890ABCDEF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x1234567890ABCDEE, "DEC RAX: full 64-bit decrement");
}

#[test]
fn test_dec_rax_to_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xff, 0xc8, 0xf4]; // DEC RAX
    emu.regs_mut().rax = 0x0000000000000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "DEC RAX: 1 - 1 = 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_dec_rax_underflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xff, 0xc8, 0xf4]; // DEC RAX
    emu.regs_mut().rax = 0x0000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "DEC RAX: 0 - 1 = 0xFFFFFFFFFFFFFFFF");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_dec_rax_signed_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x8000000000000000 (most negative i64) - 1 = 0x7FFFFFFFFFFFFFFF (max i64)
    let code = [0x48, 0xff, 0xc8, 0xf4]; // DEC RAX
    emu.regs_mut().rax = 0x8000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x7FFFFFFFFFFFFFFF, "DEC RAX: signed overflow");
    assert!(emu.flags().f_of, "OF should be set");
}

#[test]
fn test_dec_rax_preserves_cf() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xff, 0xc8, 0xf4]; // DEC RAX
    emu.regs_mut().rax = 0x1234567890ABCDEF;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be preserved");
}

#[test]
fn test_dec_rbx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xff, 0xcb, 0xf4]; // DEC RBX
    emu.regs_mut().rbx = 0xFEDCBA9876543210;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0xFEDCBA987654320F, "DEC RBX works");
}

#[test]
fn test_dec_rcx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xff, 0xc9, 0xf4]; // DEC RCX
    emu.regs_mut().rcx = 0x1000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx, 0x0FFFFFFFFFFFFFFF, "DEC RCX works");
}

#[test]
fn test_dec_rdx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xff, 0xca, 0xf4]; // DEC RDX
    emu.regs_mut().rdx = 0xFFFFFFFFFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx, 0xFFFFFFFFFFFFFFFE, "DEC RDX works");
}

// ============================================================================
// Extended registers (R8-R15) with REX prefix
// ============================================================================

#[test]
fn test_dec_r8b_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // DEC R8B requires REX prefix (REX + FE /1)
    let code = [
        0x41, 0xfe, 0xc8, // DEC R8B (REX.B + FE /1, ModRM=11_001_000)
        0xf4,             // HLT
    ];
    emu.regs_mut().r8 = 0x99;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0x98, "DEC R8B: 0x99 - 1 = 0x98");
}

#[test]
fn test_dec_r9w_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // DEC R9W
    let code = [
        0x66, 0x41, 0xff, 0xc9, // DEC R9W (66 REX.B FF /1)
        0xf4,
    ];
    emu.regs_mut().r9 = 0x5678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r9 & 0xFFFF, 0x5677, "DEC R9W works");
}

#[test]
fn test_dec_r10d_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // DEC R10D
    let code = [
        0x41, 0xff, 0xca, // DEC R10D (REX.B FF /1)
        0xf4,
    ];
    emu.regs_mut().r10 = 0xAABBCCDD;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10, 0xAABBCCDC, "DEC R10D works");
}

#[test]
fn test_dec_r11_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // DEC R11 (64-bit)
    let code = [
        0x49, 0xff, 0xcb, // DEC R11 (REX.WB FF /1)
        0xf4,
    ];
    emu.regs_mut().r11 = 0x1234567890ABCDEF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r11, 0x1234567890ABCDEE, "DEC R11 works");
}

#[test]
fn test_dec_r15_to_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x49, 0xff, 0xcf, 0xf4]; // DEC R15
    emu.regs_mut().r15 = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 0, "DEC R15: 1 - 1 = 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_dec_byte_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xfe, 0x0d, 0xfa, 0x0f, 0x00, 0x00, // DEC BYTE PTR [rip+0x0FFA] (FE /1 with RIP-relative)
        0xf4,                               // HLT
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0x42);

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0x41, "DEC byte [mem]: 0x42 - 1 = 0x41");
}

#[test]
fn test_dec_word_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0xff, 0x0d, 0xf9, 0x0f, 0x00, 0x00, // DEC WORD PTR [rip+0x0FF9]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0x1234);

    emu.run(None).unwrap();
    let result = emu.maps.read_word(DATA_ADDR).unwrap();

    assert_eq!(result, 0x1233, "DEC word [mem]: 0x1234 - 1 = 0x1233");
}

#[test]
fn test_dec_dword_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xff, 0x0d, 0xfa, 0x0f, 0x00, 0x00, // DEC DWORD PTR [rip+0x0FFA]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x12345678);

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x12345677, "DEC dword [mem]: 0x12345678 - 1 = 0x12345677");
}

#[test]
fn test_dec_qword_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xff, 0x0d, 0xf9, 0x0f, 0x00, 0x00, // DEC QWORD PTR [rip+0x0FF9]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x1234567890ABCDEF);

    emu.run(None).unwrap();
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x1234567890ABCDEE, "DEC qword [mem] works");
}

#[test]
fn test_dec_mem_underflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xfe, 0x0d, 0xfa, 0x0f, 0x00, 0x00, // DEC BYTE PTR [rip+0x0FFA]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0x00);

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0xFF, "DEC byte [mem]: 0 - 1 = 0xFF");
    assert!(emu.flags().f_sf, "SF should be set");
}

// ============================================================================
// Parity flag tests
// ============================================================================

#[test]
fn test_dec_parity_flag_even() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xfe, 0xc8, 0xf4]; // DEC AL
    emu.regs_mut().rax = 0x04; // 4 - 1 = 3 (0b00000011, two 1-bits = even parity)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03);
    assert!(emu.flags().f_pf, "PF should be set (even parity)");
}

#[test]
fn test_dec_parity_flag_odd() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xfe, 0xc8, 0xf4]; // DEC AL
    emu.regs_mut().rax = 0x02; // 2 - 1 = 1 (0b00000001, one 1-bit = odd parity)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x01);
    assert!(!emu.flags().f_pf, "PF should be clear (odd parity)");
}

// ============================================================================
// Auxiliary carry flag tests
// ============================================================================

#[test]
fn test_dec_auxiliary_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AF is set when there's a borrow from bit 3 to bit 4
    // 0x10 - 1 = 0x0F: borrow from bit 4, so AF should be set
    let code = [0xfe, 0xc8, 0xf4]; // DEC AL
    emu.regs_mut().rax = 0x10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x0F);
    assert!(emu.flags().f_af, "AF should be set (borrow from bit 4)");
}

#[test]
fn test_dec_no_auxiliary_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x12 - 1 = 0x11: no borrow from bit 4, so AF should be clear
    let code = [0xfe, 0xc8, 0xf4]; // DEC AL
    emu.regs_mut().rax = 0x12;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x11);
    assert!(!emu.flags().f_af, "AF should be clear");
}

// ============================================================================
// Use case: Loop counter that preserves CF for multi-precision arithmetic
// ============================================================================

#[test]
fn test_dec_as_loop_counter_with_multiprecision() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        // Iteration 1: ADC (sets CF)
        0x48, 0x01, 0xc8,       // ADD RAX, RCX (might set CF)
        0x49, 0xff, 0xca,       // DEC R10 (loop counter - must preserve CF!)
        0x48, 0x11, 0xd3,       // ADC RBX, RDX (uses CF from previous ADD)
        0xf4,                   // HLT
    ];
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF; // Will cause carry
    emu.regs_mut().rcx = 0x0000000000000002;
    emu.regs_mut().rbx = 0x0000000000000005;
    emu.regs_mut().rdx = 0x0000000000000003;
    emu.regs_mut().r10 = 10; // Loop counter
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 1, "Low: 0xFFFF...FFFF + 2 = 1 (with carry)");
    assert_eq!(emu.regs().r10, 9, "Loop counter decremented");
    assert_eq!(emu.regs().rbx, 9, "High: 5 + 3 + carry(1) = 9");
    // so ADC could correctly add the carry
}
