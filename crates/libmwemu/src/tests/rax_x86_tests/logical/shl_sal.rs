use crate::*;

// SHL/SAL — Shift Left (Logical/Arithmetic)
// SHL and SAL are the same instruction (same opcodes)
//
// Opcodes:
// - D0 /4       SHL r/m8, 1
// - D2 /4       SHL r/m8, CL
// - C0 /4 ib    SHL r/m8, imm8
// - D1 /4       SHL r/m16/32/64, 1
// - D3 /4       SHL r/m16/32/64, CL
// - C1 /4 ib    SHL r/m16/32/64, imm8
//
// Flags:
// - CF: Last bit shifted out
// - OF: Only for 1-bit shifts (MSB of result XOR CF)
// - SF, ZF, PF: Set according to result
// - Count is 0: No flags affected
// - Count is masked to 5 bits (0x1F) for 8/16/32-bit, 6 bits (0x3F) for 64-bit

// ============================================================================
// 8-bit SHL tests
// ============================================================================

#[test]
fn test_shl_al_1_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xd0, 0xe0, 0xf4]; // SHL AL, 1
    emu.regs_mut().rax = 0x42; // 0100_0010
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x84, "AL: 0x42 << 1 = 0x84");
    assert!(!emu.flags().f_cf, "CF clear (MSB was 0)");
    assert!(emu.flags().f_of, "OF: MSB XOR CF = 1 XOR 0 = 1");
    assert!(emu.flags().f_sf, "SF set (bit 7 = 1)");
}

#[test]
fn test_shl_al_1_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xd0, 0xe0, 0xf4]; // SHL AL, 1
    emu.regs_mut().rax = 0x81; // 1000_0001
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x02, "AL: 0x81 << 1 = 0x02");
    assert!(emu.flags().f_cf, "CF set (MSB was 1)");
    assert!(emu.flags().f_of, "OF: MSB XOR CF = 0 XOR 1 = 1");
}

#[test]
fn test_shl_al_1_no_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xd0, 0xe0, 0xf4]; // SHL AL, 1
    emu.regs_mut().rax = 0xC0; // 1100_0000
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x80, "AL: 0xC0 << 1 = 0x80");
    assert!(emu.flags().f_cf, "CF set");
    assert!(!emu.flags().f_of, "OF clear: MSB XOR CF = 1 XOR 1 = 0");
}

#[test]
fn test_shl_bl_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xd2, 0xe3, 0xf4]; // SHL BL, CL
    emu.regs_mut().rbx = 0x01;
    emu.regs_mut().rcx = 0x07;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFF, 0x80, "BL: 0x01 << 7 = 0x80");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
    assert!(emu.flags().f_sf, "SF set");
}

#[test]
fn test_shl_cl_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xc0, 0xe1, 0x03, 0xf4]; // SHL CL, 3
    emu.regs_mut().rcx = 0x11; // 0001_0001
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFF, 0x88, "CL: 0x11 << 3 = 0x88");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
    assert!(emu.flags().f_sf, "SF set");
}

#[test]
fn test_shl_al_to_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xc0, 0xe0, 0x08, 0xf4]; // SHL AL, 8
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL: all bits shifted out");
    assert!(emu.flags().f_zf, "ZF set (result is zero)");
    assert!(!emu.flags().f_sf, "SF clear");
}

#[test]
fn test_shl_count_masked_8bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xd2, 0xe0, 0xf4]; // SHL AL, CL
    emu.regs_mut().rax = 0x11;
    emu.regs_mut().rcx = 0x23; // 35 & 0x1F = 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x88, "AL: 0x11 << 3 = 0x88 (count masked)");
}

#[test]
fn test_shl_count_zero_preserves_flags() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xc0, 0xe0, 0x00, 0xf4]; // SHL AL, 0
    emu.regs_mut().rax = 0x42;
    emu.flags_mut().load(0x2 | flags::F_CF | flags::F_ZF | flags::F_OF);
    let initial_flags = emu.flags().dump();
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL unchanged");
    assert_eq!(emu.flags().dump(), initial_flags, "Flags unchanged when count is 0");
}

#[test]
fn test_shl_dh_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xd0, 0xe6, 0xf4]; // SHL DH, 1
    emu.regs_mut().rdx = 0x4200; // DH = 0x42
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rdx >> 8) & 0xFF, 0x84, "DH: 0x42 << 1 = 0x84");
}

// ============================================================================
// 16-bit SHL tests
// ============================================================================

#[test]
fn test_shl_ax_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xd1, 0xe0, 0xf4]; // SHL AX, 1
    emu.regs_mut().rax = 0x4321;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x8642, "AX: 0x4321 << 1 = 0x8642");
    assert!(!emu.flags().f_cf, "CF clear");
    assert!(emu.flags().f_of, "OF: MSB XOR CF");
    assert!(emu.flags().f_sf, "SF set");
}

#[test]
fn test_shl_ax_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xd3, 0xe0, 0xf4]; // SHL AX, CL
    emu.regs_mut().rax = 0x0001;
    emu.regs_mut().rcx = 0x0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x8000, "AX: 0x0001 << 15 = 0x8000");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
    assert!(emu.flags().f_sf, "SF set");
}

#[test]
fn test_shl_bx_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xc1, 0xe3, 0x04, 0xf4]; // SHL BX, 4
    emu.regs_mut().rbx = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFF, 0x2340, "BX: 0x1234 << 4 = 0x2340");
}

#[test]
fn test_shl_cx_to_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xc1, 0xe1, 0x10, 0xf4]; // SHL CX, 16
    emu.regs_mut().rcx = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFFFF, 0x0000, "CX: all bits shifted out");
    assert!(emu.flags().f_zf, "ZF set");
}

#[test]
fn test_shl_si_1_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xd1, 0xe6, 0xf4]; // SHL SI, 1
    emu.regs_mut().rsi = 0x8001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi & 0xFFFF, 0x0002, "SI: 0x8001 << 1 = 0x0002");
    assert!(emu.flags().f_cf, "CF set (MSB was 1)");
}

// ============================================================================
// 32-bit SHL tests
// ============================================================================

#[test]
fn test_shl_eax_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xd1, 0xe0, 0xf4]; // SHL EAX, 1
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x2468ACF0, "EAX: 0x12345678 << 1 = 0x2468ACF0");
    assert!(!emu.flags().f_cf, "CF clear");
    assert!(!emu.flags().f_of, "OF clear");
}

#[test]
fn test_shl_ebx_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xd3, 0xe3, 0xf4]; // SHL EBX, CL
    emu.regs_mut().rbx = 0x00000001;
    emu.regs_mut().rcx = 0x1F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0x80000000, "EBX: 0x00000001 << 31 = 0x80000000");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
    assert!(emu.flags().f_sf, "SF set");
}

#[test]
fn test_shl_ecx_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xc1, 0xe1, 0x08, 0xf4]; // SHL ECX, 8
    emu.regs_mut().rcx = 0x00123456;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx, 0x12345600, "ECX: 0x00123456 << 8 = 0x12345600");
}

#[test]
fn test_shl_esi_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xc1, 0xe6, 0x10, 0xf4]; // SHL ESI, 16
    emu.regs_mut().rsi = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi, 0x56780000, "ESI: 0x12345678 << 16 = 0x56780000");
}

#[test]
fn test_shl_edi_to_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc1, 0xe7, 0x1f, // SHL EDI, 31
        0xd1, 0xe7,       // SHL EDI, 1
        0xf4,
    ];
    emu.regs_mut().rdi = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdi, 0x00000000, "EDI: all bits shifted out");
    assert!(emu.flags().f_zf, "ZF set");
}

// ============================================================================
// 64-bit SHL tests
// ============================================================================

#[test]
fn test_shl_rax_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xd1, 0xe0, 0xf4]; // SHL RAX, 1
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x2468ACF13579BDE0, "RAX: << 1");
    assert!(!emu.flags().f_cf, "CF clear");
    assert!(!emu.flags().f_of, "OF clear");
}

#[test]
fn test_shl_rbx_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xd3, 0xe3, 0xf4]; // SHL RBX, CL
    emu.regs_mut().rbx = 0x0000000000000001;
    emu.regs_mut().rcx = 0x3F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0x8000000000000000, "RBX: 0x01 << 63 = 0x8000...0");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
    assert!(emu.flags().f_sf, "SF set");
}

#[test]
fn test_shl_rcx_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xc1, 0xe1, 0x10, 0xf4]; // SHL RCX, 16
    emu.regs_mut().rcx = 0x0000123456789ABC;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx, 0x123456789ABC0000, "RCX: << 16");
}

#[test]
fn test_shl_rsi_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xc1, 0xe6, 0x20, 0xf4]; // SHL RSI, 32
    emu.regs_mut().rsi = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi, 0x9ABCDEF000000000, "RSI: << 32");
}

#[test]
fn test_shl_rdi_to_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xc1, 0xe7, 0x3f, // SHL RDI, 63
        0x48, 0xd1, 0xe7,       // SHL RDI, 1
        0xf4,
    ];
    emu.regs_mut().rdi = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdi, 0x0000000000000000, "RDI: all bits shifted out");
    assert!(emu.flags().f_zf, "ZF set");
}

#[test]
fn test_shl_count_masked_64bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xd3, 0xe0, 0xf4]; // SHL RAX, CL
    emu.regs_mut().rax = 0x0000000000000001;
    emu.regs_mut().rcx = 0x43; // 67 & 0x3F = 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000008, "RAX: 0x01 << 3 = 0x08 (count masked)");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_shl_r8b_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x41, 0xd0, 0xe0, 0xf4]; // SHL R8B, 1
    emu.regs_mut().r8 = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0x84, "R8B: 0x42 << 1 = 0x84");
}

#[test]
fn test_shl_r9w_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x41, 0xd3, 0xe1, 0xf4]; // SHL R9W, CL
    emu.regs_mut().r9 = 0x0001;
    emu.regs_mut().rcx = 0x0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r9 & 0xFFFF, 0x8000, "R9W: 0x0001 << 15 = 0x8000");
}

#[test]
fn test_shl_r10d_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x41, 0xc1, 0xe2, 0x08, 0xf4]; // SHL R10D, 8
    emu.regs_mut().r10 = 0x00123456;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10, 0x12345600, "R10D: << 8");
}

#[test]
fn test_shl_r11_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x49, 0xd1, 0xe3, 0xf4]; // SHL R11, 1
    emu.regs_mut().r11 = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r11, 0x2468ACF13579BDE0, "R11: << 1");
}

#[test]
fn test_shl_r12_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x49, 0xd3, 0xe4, 0xf4]; // SHL R12, CL
    emu.regs_mut().r12 = 0x0000000000000001;
    emu.regs_mut().rcx = 0x20;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r12, 0x0000000100000000, "R12: << 32");
}

#[test]
fn test_shl_r15_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x49, 0xc1, 0xe7, 0x10, 0xf4]; // SHL R15, 16
    emu.regs_mut().r15 = 0x0000123456789ABC;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 0x123456789ABC0000, "R15: << 16");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_shl_byte_ptr_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd0, 0x25, 0xfa, 0x0f, 0x00, 0x00, // SHL BYTE PTR [rip+0x0FFA], 1
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0x42);

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0x84, "Memory: 0x42 << 1 = 0x84");
}

#[test]
fn test_shl_word_ptr_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0xd3, 0x25, 0xf9, 0x0f, 0x00, 0x00, // SHL WORD PTR [rip+0x0FF9], CL
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0x0001);
    emu.regs_mut().rcx = 0x0F;
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0x0001);

    emu.run(None).unwrap();
    let result = emu.maps.read_word(DATA_ADDR).unwrap();

    assert_eq!(result, 0x8000, "Memory: 0x0001 << 15 = 0x8000");
}

#[test]
fn test_shl_dword_ptr_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc1, 0x25, 0xf9, 0x0f, 0x00, 0x00, 0x08, // SHL DWORD PTR [rip+0x0FF9], 8
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x00123456);

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x12345600, "Memory: << 8");
}

#[test]
fn test_shl_qword_ptr_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xc1, 0x25, 0xf8, 0x0f, 0x00, 0x00, 0x10, // SHL QWORD PTR [rip+0x0FF8], 16
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x0000123456789ABC);

    emu.run(None).unwrap();
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x123456789ABC0000, "Memory: << 16");
}

// ============================================================================
// Parity flag tests
// ============================================================================

#[test]
fn test_shl_parity_even() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xc0, 0xe0, 0x02, 0xf4]; // SHL AL, 2
    emu.regs_mut().rax = 0x01; // Shift to 0x04 (one 1-bit = odd, but PF checks low byte)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x04);
    assert!(!emu.flags().f_pf, "PF clear (odd parity)");
}

#[test]
fn test_shl_parity_odd() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xc0, 0xe0, 0x02, 0xf4]; // SHL AL, 2
    emu.regs_mut().rax = 0x03; // Shift to 0x0C (two 1-bits = even parity)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x0C);
    assert!(emu.flags().f_pf, "PF set (even parity)");
}

// ============================================================================
// Edge cases and special tests
// ============================================================================

#[test]
fn test_shl_multiple_operations() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd0, 0xe0, // SHL AL, 1
        0xd0, 0xe0, // SHL AL, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x21; // 0010_0001
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x84, "AL: 0x21 << 2 = 0x84");
}
