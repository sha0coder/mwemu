use crate::*;

// NOT — One's Complement Negation
//
// Opcodes:
// - F6 /2        NOT r/m8
// - F7 /2        NOT r/m16/32/64
//
// Operation: DEST := NOT DEST (bitwise inversion)
//
// Flags: No flags are affected.
//
// CRITICAL: NOT does NOT affect any flags (unlike other logical operations).

// ============================================================================
// NOT r/m8
// ============================================================================

#[test]
fn test_not_al_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xd0, 0xf4]; // NOT AL
    emu.regs_mut().rax = 0xAA; // 10101010
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x55, "AL: NOT 0xAA = 0x55");
}

#[test]
fn test_not_bl_all_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xd3, 0xf4]; // NOT BL
    emu.regs_mut().rbx = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFF, 0xFF, "BL: NOT 0x00 = 0xFF");
}

#[test]
fn test_not_cl_all_ones() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xd1, 0xf4]; // NOT CL
    emu.regs_mut().rcx = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFF, 0x00, "CL: NOT 0xFF = 0x00");
}

#[test]
fn test_not_dl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xd2, 0xf4]; // NOT DL
    emu.regs_mut().rdx = 0xF0; // 11110000
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx & 0xFF, 0x0F, "DL: NOT 0xF0 = 0x0F");
}

#[test]
fn test_not_dh() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xd6, 0xf4]; // NOT DH
    emu.regs_mut().rdx = 0x5500; // DH = 0x55
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rdx >> 8) & 0xFF, 0xAA, "DH: NOT 0x55 = 0xAA");
}

// ============================================================================
// NOT r/m16
// ============================================================================

#[test]
fn test_not_ax_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xd0, 0xf4]; // NOT AX
    emu.regs_mut().rax = 0xAAAA;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x5555, "AX: NOT 0xAAAA = 0x5555");
}

#[test]
fn test_not_bx_all_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xd3, 0xf4]; // NOT BX
    emu.regs_mut().rbx = 0x0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFF, 0xFFFF, "BX: NOT 0x0000 = 0xFFFF");
}

#[test]
fn test_not_cx_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xd1, 0xf4]; // NOT CX
    emu.regs_mut().rcx = 0xFF00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFFFF, 0x00FF, "CX: NOT 0xFF00 = 0x00FF");
}

#[test]
fn test_not_si() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xd6, 0xf4]; // NOT SI
    emu.regs_mut().rsi = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi & 0xFFFF, 0xEDCB, "SI: NOT 0x1234 = 0xEDCB");
}

// ============================================================================
// NOT r/m32
// ============================================================================

#[test]
fn test_not_eax_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xd0, 0xf4]; // NOT EAX
    emu.regs_mut().rax = 0xAAAAAAAA;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x55555555, "EAX: NOT 0xAAAAAAAA = 0x55555555");
}

#[test]
fn test_not_ebx_all_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xd3, 0xf4]; // NOT EBX
    emu.regs_mut().rbx = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0xFFFFFFFF, "EBX: NOT 0x00000000 = 0xFFFFFFFF");
}

#[test]
fn test_not_ecx_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xd1, 0xf4]; // NOT ECX
    emu.regs_mut().rcx = 0xFFFF0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx, 0x0000FFFF, "ECX: NOT 0xFFFF0000 = 0x0000FFFF");
}

#[test]
fn test_not_esi() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xd6, 0xf4]; // NOT ESI
    emu.regs_mut().rsi = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi, 0xEDCBA987, "ESI: NOT 0x12345678 = 0xEDCBA987");
}

// ============================================================================
// NOT r/m64
// ============================================================================

#[test]
fn test_not_rax_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xd0, 0xf4]; // NOT RAX
    emu.regs_mut().rax = 0xAAAAAAAAAAAAAAAA;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x5555555555555555, "RAX: invert all bits");
}

#[test]
fn test_not_rbx_all_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xd3, 0xf4]; // NOT RBX
    emu.regs_mut().rbx = 0x0000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0xFFFFFFFFFFFFFFFF, "RBX: NOT 0 = all ones");
}

#[test]
fn test_not_rcx_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xd1, 0xf4]; // NOT RCX
    emu.regs_mut().rcx = 0xFFFFFFFF00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx, 0x00000000FFFFFFFF, "RCX: invert pattern");
}

#[test]
fn test_not_rsi() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xd6, 0xf4]; // NOT RSI
    emu.regs_mut().rsi = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi, 0xEDCBA9876543210F, "RSI: invert all bits");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_not_r8b() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x41, 0xf6, 0xd0, 0xf4]; // NOT R8B
    emu.regs_mut().r8 = 0xAA;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0x55, "R8B: NOT 0xAA = 0x55");
}

#[test]
fn test_not_r9w() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x41, 0xf7, 0xd1, 0xf4]; // NOT R9W
    emu.regs_mut().r9 = 0xAAAA;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r9 & 0xFFFF, 0x5555, "R9W: NOT 0xAAAA = 0x5555");
}

#[test]
fn test_not_r10d() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x41, 0xf7, 0xd2, 0xf4]; // NOT R10D
    emu.regs_mut().r10 = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10, 0xEDCBA987, "R10D: invert");
}

#[test]
fn test_not_r11() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x49, 0xf7, 0xd3, 0xf4]; // NOT R11
    emu.regs_mut().r11 = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r11, 0xEDCBA9876543210F, "R11: invert all bits");
}

#[test]
fn test_not_r15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x49, 0xf7, 0xd7, 0xf4]; // NOT R15
    emu.regs_mut().r15 = 0xFFFFFFFF00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 0x00000000FFFFFFFF, "R15: invert");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_not_byte_ptr() {
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
fn test_not_word_ptr() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0xf7, 0x15, 0xf9, 0x0f, 0x00, 0x00, // NOT WORD PTR [rip+0x0FF9]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0xAAAA);

    emu.run(None).unwrap();
    let result = emu.maps.read_word(DATA_ADDR).unwrap();

    assert_eq!(result, 0x5555, "Memory: NOT word");
}

#[test]
fn test_not_dword_ptr() {
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

    assert_eq!(result, 0xEDCBA987, "Memory: NOT dword");
}

#[test]
fn test_not_qword_ptr() {
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

    assert_eq!(result, 0xEDCBA9876543210F, "Memory: NOT qword");
}

// ============================================================================
// Double NOT (should return original value)
// ============================================================================

#[test]
fn test_not_not_al() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf6, 0xd0, // NOT AL
        0xf6, 0xd0, // NOT AL
        0xf4,
    ];
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "Double NOT returns original");
}

#[test]
fn test_not_not_eax() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf7, 0xd0, // NOT EAX
        0xf7, 0xd0, // NOT EAX
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x12345678, "Double NOT EAX returns original");
}

#[test]
fn test_not_not_rax() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xf7, 0xd0, // NOT RAX
        0x48, 0xf7, 0xd0, // NOT RAX
        0xf4,
    ];
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "Double NOT RAX returns original");
}

// ============================================================================
// Verify flags are NOT affected
// ============================================================================

#[test]
fn test_not_preserves_flags() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xd0, 0xf4]; // NOT AL
    emu.regs_mut().rax = 0x00;
    emu.flags_mut().load(0x2 | 0x1 | 0x40 | 0x80 | 0x800); // Set CF, PF, ZF, SF, OF
    let initial_flags = emu.flags().dump();
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.flags().dump() & 0x8D5, initial_flags & 0x8D5, "NOT preserves all flags");
}
