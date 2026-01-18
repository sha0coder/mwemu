use crate::*;

// SAR — Shift Arithmetic Right
//
// Opcodes:
// - D0 /7       SAR r/m8, 1
// - D2 /7       SAR r/m8, CL
// - C0 /7 ib    SAR r/m8, imm8
// - D1 /7       SAR r/m16/32/64, 1
// - D3 /7       SAR r/m16/32/64, CL
// - C1 /7 ib    SAR r/m16/32/64, imm8
//
// Flags:
// - CF: Last bit shifted out
// - OF: Only for 1-bit shifts (always cleared for SAR)
// - SF, ZF, PF: Set according to result
// - Count is 0: No flags affected
// - Count is masked to 5 bits (0x1F) for 8/16/32-bit, 6 bits (0x3F) for 64-bit
//
// CRITICAL: SAR preserves sign bit (sign extension from left)

// ============================================================================
// 8-bit SAR tests
// ============================================================================

#[test]
fn test_sar_al_1_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xd0, 0xf8, 0xf4]; // SAR AL, 1
    emu.regs_mut().rax = 0x42; // 0100_0010 (positive)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x21, "AL: 0x42 >> 1 = 0x21");
    assert!(!emu.flags().f_cf, "CF clear (LSB was 0)");
    assert!(!emu.flags().f_of, "OF clear for SAR");
}

#[test]
fn test_sar_al_1_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xd0, 0xf8, 0xf4]; // SAR AL, 1
    emu.regs_mut().rax = 0x82; // 1000_0010 (negative in signed interpretation)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xC1, "AL: 0x82 >> 1 = 0xC1 (sign extended)");
    assert!(!emu.flags().f_cf, "CF clear (LSB was 0)");
    assert!(emu.flags().f_sf, "SF set (result is negative)");
}

#[test]
fn test_sar_al_1_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xd0, 0xf8, 0xf4]; // SAR AL, 1
    emu.regs_mut().rax = 0x43; // 0100_0011
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x21, "AL: 0x43 >> 1 = 0x21");
    assert!(emu.flags().f_cf, "CF set (LSB was 1)");
}

#[test]
fn test_sar_bl_cl_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xd2, 0xfb, 0xf4]; // SAR BL, CL
    emu.regs_mut().rbx = 0x80; // 1000_0000 (negative)
    emu.regs_mut().rcx = 0x07;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFF, 0xFF, "BL: 0x80 >> 7 = 0xFF (sign extended)");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
    assert!(emu.flags().f_sf, "SF set");
}

#[test]
fn test_sar_cl_imm8_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xc0, 0xf9, 0x03, 0xf4]; // SAR CL, 3
    emu.regs_mut().rcx = 0x48; // 0100_1000 (positive)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFF, 0x09, "CL: 0x48 >> 3 = 0x09");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
}

#[test]
fn test_sar_cl_imm8_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xc0, 0xf9, 0x03, 0xf4]; // SAR CL, 3
    emu.regs_mut().rcx = 0x88; // 1000_1000 (negative)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFF, 0xF1, "CL: 0x88 >> 3 = 0xF1 (sign extended)");
}

#[test]
fn test_sar_al_to_zero_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xc0, 0xf8, 0x08, 0xf4]; // SAR AL, 8
    emu.regs_mut().rax = 0x42; // Positive
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL: positive value >> 8 = 0");
    assert!(emu.flags().f_zf, "ZF set");
}

#[test]
fn test_sar_al_to_ff_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xc0, 0xf8, 0x08, 0xf4]; // SAR AL, 8
    emu.regs_mut().rax = 0x80; // Negative
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "AL: negative value >> 8 = 0xFF");
    assert!(emu.flags().f_sf, "SF set");
    assert!(!emu.flags().f_zf, "ZF clear");
}

#[test]
fn test_sar_count_masked_8bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xd2, 0xf8, 0xf4]; // SAR AL, CL
    emu.regs_mut().rax = 0x88;
    emu.regs_mut().rcx = 0x23; // 35 & 0x1F = 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xF1, "AL: 0x88 >> 3 = 0xF1 (count masked)");
}

#[test]
fn test_sar_count_zero_preserves_flags() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xc0, 0xf8, 0x00, 0xf4]; // SAR AL, 0
    emu.regs_mut().rax = 0x42;
    emu.flags_mut().load(0x2 | flags::F_CF | flags::F_ZF | flags::F_OF);
    let initial_flags = emu.flags().dump();
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL unchanged");
    assert_eq!(emu.flags().dump(), initial_flags, "Flags unchanged when count is 0");
}

#[test]
fn test_sar_dh_1_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xd0, 0xfe, 0xf4]; // SAR DH, 1
    emu.regs_mut().rdx = 0x8200; // DH = 0x82
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rdx >> 8) & 0xFF, 0xC1, "DH: 0x82 >> 1 = 0xC1 (sign extended)");
}

// ============================================================================
// 16-bit SAR tests
// ============================================================================

#[test]
fn test_sar_ax_1_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xd1, 0xf8, 0xf4]; // SAR AX, 1
    emu.regs_mut().rax = 0x4321;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x2190, "AX: 0x4321 >> 1 = 0x2190");
    assert!(emu.flags().f_cf, "CF set (LSB was 1)");
}

#[test]
fn test_sar_ax_1_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xd1, 0xf8, 0xf4]; // SAR AX, 1
    emu.regs_mut().rax = 0x8642;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xC321, "AX: 0x8642 >> 1 = 0xC321 (sign extended)");
    assert!(!emu.flags().f_cf, "CF clear");
    assert!(emu.flags().f_sf, "SF set");
}

#[test]
fn test_sar_ax_cl_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xd3, 0xf8, 0xf4]; // SAR AX, CL
    emu.regs_mut().rax = 0x8000;
    emu.regs_mut().rcx = 0x0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xFFFF, "AX: 0x8000 >> 15 = 0xFFFF (sign extended)");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
    assert!(emu.flags().f_sf, "SF set");
}

#[test]
fn test_sar_bx_imm8_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xc1, 0xfb, 0x04, 0xf4]; // SAR BX, 4
    emu.regs_mut().rbx = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFF, 0x0123, "BX: 0x1234 >> 4 = 0x0123");
}

#[test]
fn test_sar_bx_imm8_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xc1, 0xfb, 0x04, 0xf4]; // SAR BX, 4
    emu.regs_mut().rbx = 0x9234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFF, 0xF923, "BX: 0x9234 >> 4 = 0xF923 (sign extended)");
    assert!(emu.flags().f_sf, "SF set");
}

#[test]
fn test_sar_cx_to_zero_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xc1, 0xf9, 0x10, 0xf4]; // SAR CX, 16
    emu.regs_mut().rcx = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFFFF, 0x0000, "CX: positive >> 16 = 0");
    assert!(emu.flags().f_zf, "ZF set");
}

#[test]
fn test_sar_cx_to_ff_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xc1, 0xf9, 0x10, 0xf4]; // SAR CX, 16
    emu.regs_mut().rcx = 0x8234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFFFF, 0xFFFF, "CX: negative >> 16 = 0xFFFF");
    assert!(emu.flags().f_sf, "SF set");
}

#[test]
fn test_sar_si_1_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xd1, 0xfe, 0xf4]; // SAR SI, 1
    emu.regs_mut().rsi = 0x8003;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi & 0xFFFF, 0xC001, "SI: 0x8003 >> 1 = 0xC001 (sign extended)");
    assert!(emu.flags().f_cf, "CF set (LSB was 1)");
}

// ============================================================================
// 32-bit SAR tests
// ============================================================================

#[test]
fn test_sar_eax_1_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xd1, 0xf8, 0xf4]; // SAR EAX, 1
    emu.regs_mut().rax = 0x2468ACF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x12345678, "EAX: 0x2468ACF0 >> 1 = 0x12345678");
    assert!(!emu.flags().f_cf, "CF clear");
}

#[test]
fn test_sar_eax_1_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xd1, 0xf8, 0xf4]; // SAR EAX, 1
    emu.regs_mut().rax = 0x80000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xC0000000, "EAX: 0x80000000 >> 1 = 0xC0000000 (sign extended)");
    assert!(!emu.flags().f_cf, "CF clear");
    assert!(emu.flags().f_sf, "SF set");
}

#[test]
fn test_sar_ebx_cl_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xd3, 0xfb, 0xf4]; // SAR EBX, CL
    emu.regs_mut().rbx = 0x80000000;
    emu.regs_mut().rcx = 0x1F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0xFFFFFFFF, "EBX: 0x80000000 >> 31 = 0xFFFFFFFF (sign extended)");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
    assert!(emu.flags().f_sf, "SF set");
}

#[test]
fn test_sar_ecx_imm8_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xc1, 0xf9, 0x08, 0xf4]; // SAR ECX, 8
    emu.regs_mut().rcx = 0x12345600;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx, 0x00123456, "ECX: 0x12345600 >> 8 = 0x00123456");
}

#[test]
fn test_sar_ecx_imm8_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xc1, 0xf9, 0x08, 0xf4]; // SAR ECX, 8
    emu.regs_mut().rcx = 0x92345600;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx, 0xFF923456, "ECX: 0x92345600 >> 8 = 0xFF923456 (sign extended)");
    assert!(emu.flags().f_sf, "SF set");
}

#[test]
fn test_sar_esi_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xc1, 0xfe, 0x10, 0xf4]; // SAR ESI, 16
    emu.regs_mut().rsi = 0x56780000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi, 0x00005678, "ESI: 0x56780000 >> 16 = 0x00005678");
}

#[test]
fn test_sar_edi_to_zero_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc1, 0xff, 0x1f, // SAR EDI, 31
        0xd1, 0xff,       // SAR EDI, 1
        0xf4,
    ];
    emu.regs_mut().rdi = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdi, 0x00000000, "EDI: positive >> 32 = 0");
    assert!(emu.flags().f_zf, "ZF set");
}

#[test]
fn test_sar_edi_to_ff_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc1, 0xff, 0x1f, // SAR EDI, 31
        0xd1, 0xff,       // SAR EDI, 1
        0xf4,
    ];
    emu.regs_mut().rdi = 0x92345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdi, 0xFFFFFFFF, "EDI: negative >> 32 = 0xFFFFFFFF");
    assert!(emu.flags().f_sf, "SF set");
}

// ============================================================================
// 64-bit SAR tests
// ============================================================================

#[test]
fn test_sar_rax_1_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xd1, 0xf8, 0xf4]; // SAR RAX, 1
    emu.regs_mut().rax = 0x2468ACF13579BDE0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "RAX: >> 1");
    assert!(!emu.flags().f_cf, "CF clear");
}

#[test]
fn test_sar_rax_1_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xd1, 0xf8, 0xf4]; // SAR RAX, 1
    emu.regs_mut().rax = 0x8000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xC000000000000000, "RAX: >> 1 (sign extended)");
    assert!(!emu.flags().f_cf, "CF clear");
    assert!(emu.flags().f_sf, "SF set");
}

#[test]
fn test_sar_rbx_cl_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xd3, 0xfb, 0xf4]; // SAR RBX, CL
    emu.regs_mut().rbx = 0x8000000000000000;
    emu.regs_mut().rcx = 0x3F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0xFFFFFFFFFFFFFFFF, "RBX: >> 63 = all ones (sign extended)");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
    assert!(emu.flags().f_sf, "SF set");
}

#[test]
fn test_sar_rcx_imm8_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xc1, 0xf9, 0x10, 0xf4]; // SAR RCX, 16
    emu.regs_mut().rcx = 0x123456789ABC0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx, 0x0000123456789ABC, "RCX: >> 16");
}

#[test]
fn test_sar_rcx_imm8_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xc1, 0xf9, 0x10, 0xf4]; // SAR RCX, 16
    emu.regs_mut().rcx = 0x923456789ABC0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx, 0xFFFF923456789ABC, "RCX: >> 16 (sign extended)");
    assert!(emu.flags().f_sf, "SF set");
}

#[test]
fn test_sar_rsi_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xc1, 0xfe, 0x20, 0xf4]; // SAR RSI, 32
    emu.regs_mut().rsi = 0x000000009ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x000000009ABCDEF0 >> 32 = 0x0000000000000000 (positive, high bits are 0)
    assert_eq!(emu.regs().rsi, 0x0000000000000000, "RSI: >> 32");
    assert!(emu.flags().f_zf, "ZF set");
}

#[test]
fn test_sar_rdi_to_zero_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xc1, 0xff, 0x3f, // SAR RDI, 63
        0x48, 0xd1, 0xff,       // SAR RDI, 1
        0xf4,
    ];
    emu.regs_mut().rdi = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdi, 0x0000000000000000, "RDI: positive >> 64 = 0");
    assert!(emu.flags().f_zf, "ZF set");
}

#[test]
fn test_sar_rdi_to_ff_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xc1, 0xff, 0x3f, // SAR RDI, 63
        0x48, 0xd1, 0xff,       // SAR RDI, 1
        0xf4,
    ];
    emu.regs_mut().rdi = 0x923456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdi, 0xFFFFFFFFFFFFFFFF, "RDI: negative >> 64 = all ones");
    assert!(emu.flags().f_sf, "SF set");
}

#[test]
fn test_sar_count_masked_64bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xd3, 0xf8, 0xf4]; // SAR RAX, CL
    emu.regs_mut().rax = 0x0000000000000008;
    emu.regs_mut().rcx = 0x43; // 67 & 0x3F = 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000001, "RAX: 0x08 >> 3 = 0x01 (count masked)");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_sar_r8b_1_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x41, 0xd0, 0xf8, 0xf4]; // SAR R8B, 1
    emu.regs_mut().r8 = 0x82;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0xC1, "R8B: 0x82 >> 1 = 0xC1 (sign extended)");
}

#[test]
fn test_sar_r9w_cl_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x41, 0xd3, 0xf9, 0xf4]; // SAR R9W, CL
    emu.regs_mut().r9 = 0x8000;
    emu.regs_mut().rcx = 0x0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r9 & 0xFFFF, 0xFFFF, "R9W: 0x8000 >> 15 = 0xFFFF (sign extended)");
}

#[test]
fn test_sar_r10d_imm8_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x41, 0xc1, 0xfa, 0x08, 0xf4]; // SAR R10D, 8
    emu.regs_mut().r10 = 0x12345600;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10, 0x00123456, "R10D: >> 8");
}

#[test]
fn test_sar_r10d_imm8_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x41, 0xc1, 0xfa, 0x08, 0xf4]; // SAR R10D, 8
    emu.regs_mut().r10 = 0x92345600;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10, 0xFF923456, "R10D: >> 8 (sign extended)");
}

#[test]
fn test_sar_r11_1_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x49, 0xd1, 0xfb, 0xf4]; // SAR R11, 1
    emu.regs_mut().r11 = 0x8000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r11, 0xC000000000000000, "R11: >> 1 (sign extended)");
}

#[test]
fn test_sar_r12_cl_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x49, 0xd3, 0xfc, 0xf4]; // SAR R12, CL
    emu.regs_mut().r12 = 0x9000000000000000;
    emu.regs_mut().rcx = 0x20;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r12, 0xFFFFFFFF90000000, "R12: >> 32 (sign extended)");
}

#[test]
fn test_sar_r15_imm8_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x49, 0xc1, 0xff, 0x10, 0xf4]; // SAR R15, 16
    emu.regs_mut().r15 = 0x123456789ABC0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 0x0000123456789ABC, "R15: >> 16");
}

#[test]
fn test_sar_r15_imm8_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x49, 0xc1, 0xff, 0x10, 0xf4]; // SAR R15, 16
    emu.regs_mut().r15 = 0x923456789ABC0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 0xFFFF923456789ABC, "R15: >> 16 (sign extended)");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_sar_byte_ptr_1_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd0, 0x3d, 0xfa, 0x0f, 0x00, 0x00, // SAR BYTE PTR [rip+0x0FFA], 1
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0x82);

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0xC1, "Memory: 0x82 >> 1 = 0xC1 (sign extended)");
}

#[test]
fn test_sar_dword_ptr_imm8_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc1, 0x3d, 0xf9, 0x0f, 0x00, 0x00, 0x08, // SAR DWORD PTR [rip+0x0FF9], 8
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x12345600);

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x00123456, "Memory: >> 8");
}

#[test]
fn test_sar_dword_ptr_imm8_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc1, 0x3d, 0xf9, 0x0f, 0x00, 0x00, 0x08, // SAR DWORD PTR [rip+0x0FF9], 8
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x92345600);

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(result, 0xFF923456, "Memory: >> 8 (sign extended)");
}

#[test]
fn test_sar_qword_ptr_imm8_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xc1, 0x3d, 0xf8, 0x0f, 0x00, 0x00, 0x10, // SAR QWORD PTR [rip+0x0FF8], 16
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x123456789ABC0000);

    emu.run(None).unwrap();
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x0000123456789ABC, "Memory: >> 16");
}

#[test]
fn test_sar_qword_ptr_imm8_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xc1, 0x3d, 0xf8, 0x0f, 0x00, 0x00, 0x10, // SAR QWORD PTR [rip+0x0FF8], 16
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x923456789ABC0000);

    emu.run(None).unwrap();
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();

    assert_eq!(result, 0xFFFF923456789ABC, "Memory: >> 16 (sign extended)");
}

// ============================================================================
// Edge cases
// ============================================================================

#[test]
fn test_sar_vs_shr_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xc0, 0xf8, 0x04, 0xf4]; // SAR AL, 4
    emu.regs_mut().rax = 0x7F; // Positive
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x07, "AL: 0x7F >> 4 = 0x07 (same as SHR)");
}

#[test]
fn test_sar_vs_shr_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xc0, 0xf8, 0x04, 0xf4]; // SAR AL, 4
    emu.regs_mut().rax = 0xFF; // Negative (all bits set)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // SAR: sign extension, SHR would give 0x0F
    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "AL: 0xFF >> 4 = 0xFF (sign extended)");
}

#[test]
fn test_sar_multiple_operations() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd0, 0xf8, // SAR AL, 1
        0xd0, 0xf8, // SAR AL, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x84; // 1000_0100 (negative)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xE1, "AL: 0x84 >> 2 = 0xE1 (sign extended)");
}
