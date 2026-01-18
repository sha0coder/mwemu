use crate::*;

// XOR — Logical Exclusive OR
//
// Opcodes: Similar structure to AND/OR but opcode /6
// - 34 ib           XOR AL, imm8
// - 35 iw/id        XOR AX/EAX/RAX, imm16/32
// - 80 /6 ib        XOR r/m8, imm8
// - 81 /6 iw/id     XOR r/m16/32/64, imm16/32
// - 83 /6 ib        XOR r/m16/32/64, imm8 (sign-extended)
// - 30 /r           XOR r/m8, r8
// - 31 /r           XOR r/m16/32/64, r16/32/64
// - 32 /r           XOR r8, r/m8
// - 33 /r           XOR r16/32/64, r/m16/32/64
//
// Operation: DEST := DEST XOR SRC
//
// Flags: OF and CF are CLEARED.
//        SF, ZF, PF are set according to result.
//        AF is undefined.
//
// CRITICAL: XOR is exclusive OR. Each bit is 1 if bits DIFFER, 0 if same.
// Common idioms: XOR reg, reg (zero register), XOR reg, -1 (invert all bits).

// ============================================================================
// XOR with immediate
// ============================================================================

#[test]
fn test_xor_al_imm8_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x34, 0x0F, // XOR AL, 0x0F
        0xf4,
    ];
    emu.regs_mut().rax = 0xAA; // 10101010
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0xAA ^ 0x0F = 10101010 ^ 00001111 = 10100101 = 0xA5
    assert_eq!(emu.regs().rax & 0xFF, 0xA5, "AL: 0xAA XOR 0x0F = 0xA5");
    assert!(!emu.flags().f_zf, "ZF should be clear");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_of, "OF should be clear");
    assert!(emu.flags().f_sf, "SF should be set (high bit = 1)");
}

#[test]
fn test_xor_al_imm8_identity() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // XOR with 0 is identity operation
    let code = [0x34, 0x00, 0xf4]; // XOR AL, 0
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL: 0x42 XOR 0 = 0x42 (identity)");
}

#[test]
fn test_xor_al_imm8_invert() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // XOR with 0xFF inverts all bits
    let code = [0x34, 0xFF, 0xf4]; // XOR AL, 0xFF
    emu.regs_mut().rax = 0xAA; // 10101010
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x55, "AL: 0xAA XOR 0xFF = 0x55 (inverted)");
}

#[test]
fn test_xor_al_imm8_same_value() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // XOR with same value = 0
    let code = [0x34, 0x42, 0xf4]; // XOR AL, 0x42
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0, "AL: 0x42 XOR 0x42 = 0");
    assert!(emu.flags().f_zf, "ZF should be set (zero result)");
}

#[test]
fn test_xor_eax_imm32_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x35, 0xFF, 0x00, 0xFF, 0x00, // XOR EAX, 0x00FF00FF
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFF00FF00, "EAX: toggle specific bytes");
}

#[test]
fn test_xor_rax_imm32_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x35, 0xFF, 0xFF, 0xFF, 0xFF, // XOR RAX, 0xFFFFFFFF (sign-extended)
        0xf4,
    ];
    emu.regs_mut().rax = 0x0000000012345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0xFFFFFFFF sign-extended to 64-bit is 0xFFFFFFFFFFFFFFFF
    assert_eq!(emu.regs().rax, 0xFFFFFFFFEDCBA987, "RAX: invert all bits");
}

// ============================================================================
// XOR r/m with immediate
// ============================================================================

#[test]
fn test_xor_rm8_imm8_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x80, 0xf3, 0xFF, // XOR BL, 0xFF
        0xf4,
    ];
    emu.regs_mut().rbx = 0xAA;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFF, 0x55, "BL: 0xAA XOR 0xFF = 0x55");
}

#[test]
fn test_xor_rm32_imm32_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x81, 0xf3, 0xFF, 0xFF, 0x00, 0x00, // XOR EBX, 0x0000FFFF
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0x1234A987, "EBX: toggle low 16 bits");
}

#[test]
fn test_xor_rm64_imm8_sign_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x83, 0xf3, 0xFF, // XOR RBX, 0xFF (sign-extended to -1)
        0xf4,
    ];
    emu.regs_mut().rbx = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0xEDCBA9876543210F, "RBX: invert all bits");
}

// ============================================================================
// XOR r/m, r
// ============================================================================

#[test]
fn test_xor_rm8_r8_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x30, 0xd8, // XOR AL, BL
        0xf4,
    ];
    emu.regs_mut().rax = 0xAA; // 10101010
    emu.regs_mut().rbx = 0x55; // 01010101
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "AL: 0xAA XOR 0x55 = 0xFF (all different)");
}

#[test]
fn test_xor_rm8_r8_same_value() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x30, 0xd8, // XOR AL, BL
        0xf4,
    ];
    emu.regs_mut().rax = 0x42;
    emu.regs_mut().rbx = 0x42; // Same value
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0, "AL: 0x42 XOR 0x42 = 0");
    assert!(emu.flags().f_zf, "ZF set");
}

#[test]
fn test_xor_rm32_r32_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x31, 0xd8, // XOR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0xFF00FF00;
    emu.regs_mut().rbx = 0x00FF00FF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFF, "EAX: toggle pattern");
}

#[test]
fn test_xor_rm64_r64_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x31, 0xd8, // XOR RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFF00000000;
    emu.regs_mut().rbx = 0x00000000FFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "RAX: all bits different");
}

// ============================================================================
// XOR r, r/m
// ============================================================================

#[test]
fn test_xor_r8_rm8_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x32, 0xc3, // XOR AL, BL
        0xf4,
    ];
    emu.regs_mut().rax = 0x0F;
    emu.regs_mut().rbx = 0xF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "AL: 0x0F XOR 0xF0 = 0xFF");
}

// ============================================================================
// XOR reg, reg (common idiom to zero register)
// ============================================================================

#[test]
fn test_xor_eax_eax_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // XOR EAX, EAX is common idiom to zero EAX
    let code = [
        0x31, 0xc0, // XOR EAX, EAX
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "EAX: XOR EAX, EAX = 0 (common zero idiom)");
    assert!(emu.flags().f_zf, "ZF set");
    assert!(!emu.flags().f_sf, "SF clear");
    assert!(!emu.flags().f_cf, "CF clear");
    assert!(!emu.flags().f_of, "OF clear");
}

#[test]
fn test_xor_rax_rax_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "RAX: XOR RAX, RAX = 0");
    assert!(emu.flags().f_zf, "ZF set");
}

#[test]
fn test_xor_r8b_r8b_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x45, 0x30, 0xc0, // XOR R8B, R8B
        0xf4,
    ];
    emu.regs_mut().r8 = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0, "R8B: XOR R8B, R8B = 0");
}

// ============================================================================
// Bit toggling use cases
// ============================================================================

#[test]
fn test_xor_toggle_specific_bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x34, 0x10, 0xf4]; // XOR AL, 0x10
    emu.regs_mut().rax = 0x0F; // bit 4 is clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x1F, "Toggle bit 4 on: 0x0F ^ 0x10 = 0x1F");

    let code = [0x34, 0x10, 0xf4]; // XOR AL, 0x10
    emu.regs_mut().rax = 0x1F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x0F, "Toggle bit 4 off: 0x1F ^ 0x10 = 0x0F");
}

#[test]
fn test_xor_toggle_multiple_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x34, 0x55, 0xf4]; // XOR AL, 0x55 (01010101)
    emu.regs_mut().rax = 0xFF; // 11111111
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 11111111 ^ 01010101 = 10101010 = 0xAA
    assert_eq!(emu.regs().rax & 0xFF, 0xAA, "Toggle alternating bits");
}

#[test]
fn test_xor_swap_nibbles() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x34, 0xFF, // XOR AL, 0xFF (invert all)
        0xf4,
    ];
    emu.regs_mut().rax = 0x3C; // 00111100
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xC3, "Inverted: 11000011");
}

// ============================================================================
// OF and CF always cleared
// ============================================================================

#[test]
fn test_xor_clears_of_cf() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x34, 0xFF, 0xf4]; // XOR AL, 0xFF
    emu.regs_mut().rax = 0x00;
    emu.flags_mut().load(0x2 | flags::F_OF | flags::F_CF);
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_of, "OF cleared by XOR");
    assert!(!emu.flags().f_cf, "CF cleared by XOR");
}

// ============================================================================
// Parity flag tests
// ============================================================================

#[test]
fn test_xor_parity_even() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x34, 0x02, 0xf4]; // XOR AL, 0x02
    emu.regs_mut().rax = 0x01; // 00000001 ^ 00000010 = 00000011
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03);
    assert!(emu.flags().f_pf, "PF set (even parity)");
}

#[test]
fn test_xor_parity_odd() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x34, 0x04, 0xf4]; // XOR AL, 0x04
    emu.regs_mut().rax = 0x03; // 00000011 ^ 00000100 = 00000111
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x07);
    assert!(!emu.flags().f_pf, "PF clear (odd parity)");
}

// ============================================================================
// Different registers
// ============================================================================

#[test]
fn test_xor_different_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // XOR CL, DL
    let code = [0x30, 0xd1, 0xf4];
    emu.regs_mut().rcx = 0x0F;
    emu.regs_mut().rdx = 0xF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFF, 0xFF, "CL: 0x0F XOR 0xF0 = 0xFF");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_xor_r8b_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x41, 0x80, 0xf0, 0xFF, // XOR R8B, 0xFF
        0xf4,
    ];
    emu.regs_mut().r8 = 0xAA;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0x55, "R8B: 0xAA XOR 0xFF = 0x55");
}

#[test]
fn test_xor_r10d_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x41, 0x81, 0xf2, 0xFF, 0xFF, 0x00, 0x00, // XOR R10D, 0x0000FFFF
        0xf4,
    ];
    emu.regs_mut().r10 = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10, 0x1234A987, "R10D: toggle low 16 bits");
}

#[test]
fn test_xor_r11_r12_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x4d, 0x31, 0xe3, // XOR R11, R12
        0xf4,
    ];
    emu.regs_mut().r11 = 0x123456789ABCDEF0;
    emu.regs_mut().r12 = 0x123456789ABCDEF0; // Same value
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r11, 0, "R11: XOR with same value = 0");
    assert!(emu.flags().f_zf, "ZF set");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_xor_byte_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x80, 0x35, 0xf9, 0x0f, 0x00, 0x00, 0xFF, // XOR BYTE PTR [rip+0x0FF9], 0xFF
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0xAA);

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0x55, "Memory: 0xAA XOR 0xFF = 0x55");
}

#[test]
fn test_xor_dword_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x81, 0x35, 0xf6, 0x0f, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, // XOR DWORD PTR [rip+0x0FF6], 0x0000FFFF
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x12345678);

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x1234A987, "Memory: toggle low 16 bits");
}

// ============================================================================
// Practical use cases
// ============================================================================

#[test]
fn test_xor_encryption_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let key = 0x5A;

    let code = [0x34, key, 0xf4]; // XOR AL, key
    let plaintext = 0x42;
    emu.regs_mut().rax = plaintext;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let ciphertext = emu.regs().rax & 0xFF;

    let code = [0x34, key, 0xf4]; // XOR AL, key (again)
    emu.regs_mut().rax = ciphertext;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, plaintext, "XOR encryption/decryption");
}

#[test]
fn test_xor_commutative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // XOR is commutative: a XOR b = b XOR a
    let a: u8 = 0x12;
    let b: u8 = 0x34;

    // a XOR b
    let code = [0x34, b, 0xf4];
    emu.regs_mut().rax = u64::from(a);
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // b XOR a
    let code = [0x34, a, 0xf4];
    emu.regs_mut().rax = u64::from(b);
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, emu.regs().rax & 0xFF, "XOR is commutative");
}

#[test]
fn test_xor_associative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // XOR is associative: (a XOR b) XOR c = a XOR (b XOR c)
    let a: u8 = 0x12;
    let b: u8 = 0x34;
    let c: u8 = 0x56;

    // (a XOR b) XOR c
    let code = [0x34, b, 0x34, c, 0xf4];
    emu.regs_mut().rax = u64::from(a);
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // a XOR (b XOR c)
    let code = [0x34, (b ^ c), 0xf4];
    emu.regs_mut().rax = u64::from(a);
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, emu.regs().rax & 0xFF, "XOR is associative");
}
