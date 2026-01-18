use crate::*;

// NOT — One's Complement Negation
//
// Opcodes:
// - F6 /2       NOT r/m8      Reverse each bit of r/m8
// - REX+F6 /2   NOT r/m8*     (with REX for extended regs)
// - F7 /2       NOT r/m16     Reverse each bit of r/m16
// - F7 /2       NOT r/m32     Reverse each bit of r/m32
// - REX.W+F7 /2 NOT r/m64     Reverse each bit of r/m64
//
// Operation: DEST := NOT DEST (bitwise inversion)
//
// Flags: NONE - NOT does not affect any flags!
//
// CRITICAL: NOT is one's complement (bitwise inversion). Each 0 becomes 1,
// each 1 becomes 0. Equivalent to XOR with all 1s (-1).

// ============================================================================
// 8-bit NOT
// ============================================================================

#[test]
fn test_not_al_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf6, 0xd0, // NOT AL (F6 /2, ModRM=11_010_000)
        0xf4,
    ];
    emu.regs_mut().rax = 0xAA; // 10101010
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 10101010 inverted = 01010101 = 0x55
    assert_eq!(emu.regs().rax & 0xFF, 0x55, "AL: NOT 0xAA = 0x55");
}

#[test]
fn test_not_al_all_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xd0, 0xf4]; // NOT AL
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "AL: NOT 0x00 = 0xFF");
}

#[test]
fn test_not_al_all_ones() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xd0, 0xf4]; // NOT AL
    emu.regs_mut().rax = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL: NOT 0xFF = 0x00");
}

#[test]
fn test_not_al_partial_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xd0, 0xf4]; // NOT AL
    emu.regs_mut().rax = 0x0F; // 00001111
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xF0, "AL: NOT 0x0F = 0xF0");
}

#[test]
fn test_not_bl_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf6, 0xd3, // NOT BL
        0xf4,
    ];
    emu.regs_mut().rbx = 0x3C; // 00111100
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFF, 0xC3, "BL: NOT 0x3C = 0xC3");
}

#[test]
fn test_not_preserves_high_bytes_8bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xd0, 0xf4]; // NOT AL
    emu.regs_mut().rax = 0xDEADBEEF_12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x87, "AL: NOT 0x78 = 0x87");
    assert_eq!(emu.regs().rax & !0xFF, 0xDEADBEEF_12345600, "Upper bytes preserved");
}

// ============================================================================
// 16-bit NOT
// ============================================================================

#[test]
fn test_not_ax_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0xf7, 0xd0, // NOT AX
        0xf4,
    ];
    emu.regs_mut().rax = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xEDCB, "AX: NOT 0x1234 = 0xEDCB");
}

#[test]
fn test_not_ax_all_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xd0, 0xf4]; // NOT AX
    emu.regs_mut().rax = 0x0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xFFFF, "AX: NOT 0 = 0xFFFF");
}

#[test]
fn test_not_ax_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xd0, 0xf4]; // NOT AX
    emu.regs_mut().rax = 0x00FF; // 0000000011111111
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xFF00, "AX: NOT 0x00FF = 0xFF00");
}

// ============================================================================
// 32-bit NOT
// ============================================================================

#[test]
fn test_not_eax_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf7, 0xd0, // NOT EAX
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xEDCBA987, "EAX: NOT 0x12345678 = 0xEDCBA987");
}

#[test]
fn test_not_eax_all_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xd0, 0xf4]; // NOT EAX
    emu.regs_mut().rax = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFF, "EAX: NOT 0 = 0xFFFFFFFF");
}

#[test]
fn test_not_eax_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xd0, 0xf4]; // NOT EAX
    emu.regs_mut().rax = 0xFF00FF00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x00FF00FF, "EAX: NOT 0xFF00FF00 = 0x00FF00FF");
}

#[test]
fn test_not_ebx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xd3, 0xf4]; // NOT EBX
    emu.regs_mut().rbx = 0xAAAAAAAA;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0x55555555, "EBX: NOT 0xAAAAAAAA = 0x55555555");
}

// ============================================================================
// 64-bit NOT
// ============================================================================

#[test]
fn test_not_rax_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xf7, 0xd0, // NOT RAX
        0xf4,
    ];
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xEDCBA9876543210F, "RAX: NOT 0x123456789ABCDEF0");
}

#[test]
fn test_not_rax_all_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xd0, 0xf4]; // NOT RAX
    emu.regs_mut().rax = 0x0000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "RAX: NOT 0 = 0xFFFF...FFFF");
}

#[test]
fn test_not_rax_all_ones() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xd0, 0xf4]; // NOT RAX
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000000, "RAX: NOT 0xFFFF...FFFF = 0");
}

#[test]
fn test_not_rax_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xd0, 0xf4]; // NOT RAX
    emu.regs_mut().rax = 0xFF00FF00FF00FF00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x00FF00FF00FF00FF, "RAX: invert alternating bytes");
}

#[test]
fn test_not_rbx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xd3, 0xf4]; // NOT RBX
    emu.regs_mut().rbx = 0xF0F0F0F0F0F0F0F0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0x0F0F0F0F0F0F0F0F, "RBX: NOT pattern");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_not_r8b_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x41, 0xf6, 0xd0, // NOT R8B
        0xf4,
    ];
    emu.regs_mut().r8 = 0xAA;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0x55, "R8B: NOT 0xAA = 0x55");
}

#[test]
fn test_not_r9w_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0x41, 0xf7, 0xd1, // NOT R9W
        0xf4,
    ];
    emu.regs_mut().r9 = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r9 & 0xFFFF, 0xEDCB, "R9W: NOT 0x1234");
}

#[test]
fn test_not_r10d_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x41, 0xf7, 0xd2, // NOT R10D
        0xf4,
    ];
    emu.regs_mut().r10 = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10, 0xEDCBA987, "R10D: NOT 0x12345678");
}

#[test]
fn test_not_r11_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x49, 0xf7, 0xd3, // NOT R11
        0xf4,
    ];
    emu.regs_mut().r11 = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r11, 0xEDCBA9876543210F, "R11: NOT works");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_not_byte_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf6, 0x15, 0xfa, 0x0f, 0x00, 0x00, // NOT BYTE PTR [rip+0x0FFA]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0xAA);

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0x55, "Memory: NOT 0xAA = 0x55");
}

#[test]
fn test_not_word_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0xf7, 0x15, 0xf9, 0x0f, 0x00, 0x00, // NOT WORD PTR [rip+0x0FF9]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0x1234);

    emu.run(None).unwrap();
    let result = emu.maps.read_word(DATA_ADDR).unwrap();

    assert_eq!(result, 0xEDCB, "Memory: NOT 0x1234 = 0xEDCB");
}

#[test]
fn test_not_dword_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf7, 0x15, 0xfa, 0x0f, 0x00, 0x00, // NOT DWORD PTR [rip+0x0FFA]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x12345678);

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(result, 0xEDCBA987, "Memory: NOT 0x12345678");
}

#[test]
fn test_not_qword_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xf7, 0x15, 0xf9, 0x0f, 0x00, 0x00, // NOT QWORD PTR [rip+0x0FF9]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x123456789ABCDEF0);

    emu.run(None).unwrap();
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();

    assert_eq!(result, 0xEDCBA9876543210F, "Memory: NOT works");
}

// ============================================================================
// Flags not affected
// ============================================================================

#[test]
fn test_not_preserves_flags() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xd0, 0xf4]; // NOT AL
    emu.regs_mut().rax = 0x00;
    emu.flags_mut().load(0x2 | flags::F_CF | flags::F_PF | 
        flags::F_AF | flags::F_ZF | flags::F_SF | flags::F_OF);
    let initial_flags = emu.flags().dump();

    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.flags().dump(), initial_flags, "NOT should not affect any flags");
}

// ============================================================================
// Double NOT is identity
// ============================================================================

#[test]
fn test_not_twice_is_identity() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf6, 0xd0, // NOT AL (first time)
        0xf6, 0xd0, // NOT AL (second time)
        0xf4,
    ];
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "NOT(NOT(x)) = x (identity)");
}

// ============================================================================
// Practical use cases
// ============================================================================

#[test]
fn test_not_create_mask() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xd0, 0xf4]; // NOT AL
    emu.regs_mut().rax = 0x0F; // Mask for low nibble
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xF0, "Inverted mask for high nibble");
}

#[test]
fn test_not_equivalent_to_xor_minus_one() {
    let DATA_ADDR = 0x7000;
    // NOT x is equivalent to x XOR -1
    let value = 0x42;

    // NOT approach
    let code_not = [0xf6, 0xd0, 0xf4];
    let mut emu1 = emu64();
    emu1.regs_mut().rax = value;
    emu1.load_code_bytes(&code_not);
    emu1.run(None).unwrap();

    // XOR approach
    let code_xor = [0x34, 0xFF, 0xf4]; // XOR AL, 0xFF
    let mut emu2 = emu64();
    emu2.regs_mut().rax = value;
    emu2.load_code_bytes(&code_xor);
    emu2.run(None).unwrap();

    assert_eq!(emu1.regs().rax & 0xFF, emu2.regs().rax & 0xFF, "NOT x = x XOR 0xFF");
}
