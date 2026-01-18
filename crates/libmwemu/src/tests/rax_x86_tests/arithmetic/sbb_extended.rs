use crate::*;

// SBB — Subtract with Borrow
//
// Opcodes:
// - 1C ib           SBB AL, imm8      Subtract with borrow imm8 from AL
// - 1D iw           SBB AX, imm16     Subtract with borrow imm16 from AX
// - 1D id           SBB EAX, imm32    Subtract with borrow imm32 from EAX
// - REX.W + 1D id   SBB RAX, imm32    Subtract with borrow imm32 (sign-extended) from RAX
// - 80 /3 ib        SBB r/m8, imm8    Subtract with borrow imm8 from r/m8
// - 81 /3 iw        SBB r/m16, imm16  Subtract with borrow imm16 from r/m16
// - 81 /3 id        SBB r/m32, imm32  Subtract with borrow imm32 from r/m32
// - REX.W + 81 /3 id SBB r/m64, imm32 Subtract with borrow imm32 (sign-extended) from r/m64
// - 83 /3 ib        SBB r/m16, imm8   Subtract with borrow sign-extended imm8 from r/m16
// - 83 /3 ib        SBB r/m32, imm8   Subtract with borrow sign-extended imm8 from r/m32
// - REX.W + 83 /3 ib SBB r/m64, imm8  Subtract with borrow sign-extended imm8 from r/m64
// - 18 /r           SBB r/m8, r8      Subtract with borrow r8 from r/m8
// - 19 /r           SBB r/m16, r16    Subtract with borrow r16 from r/m16
// - 19 /r           SBB r/m32, r32    Subtract with borrow r32 from r/m32
// - REX.W + 19 /r   SBB r/m64, r64    Subtract with borrow r64 from r/m64
// - 1A /r           SBB r8, r/m8      Subtract with borrow r/m8 from r8
// - 1B /r           SBB r16, r/m16    Subtract with borrow r/m16 from r16
// - 1B /r           SBB r32, r/m32    Subtract with borrow r/m32 from r32
// - REX.W + 1B /r   SBB r64, r/m64    Subtract with borrow r/m64 from r64
//
// Operation: DEST := DEST - (SRC + CF)
//
// Flags: CF, OF, SF, ZF, AF, PF are set according to result

// ============================================================================
// 8-bit SBB Tests
// ============================================================================

#[test]
fn test_sbb_al_imm8_no_borrow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB AL, 5 with CF=0
    let code = [
        0x1C, 0x05, // SBB AL, 5
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0x0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x0A, "AL should be 10 (15 - 5)");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_sbb_al_imm8_with_borrow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB AL, 5 with CF=1
    let code = [0x1C, 0x05, 0xf4]; // SBB AL, 5; HLT
    emu.regs_mut().rax = 0x0F;
    emu.flags_mut().load(0x01); // Set CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x09, "AL should be 9 (15 - 5 - 1)");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_sbb_al_underflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB AL, 0x10 with CF=1 (causes underflow)
    let code = [0x1C, 0x10, 0xf4]; // SBB AL, 0x10; HLT
    emu.regs_mut().rax = 0x05;
    emu.flags_mut().load(0x01); // Set CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x05 - 0x10 - 1 = -12 = 0xF4
    assert_eq!(emu.regs().rax & 0xFF, 0xF4, "AL should wrap to 0xF4");
    assert!(emu.flags().f_cf, "CF should be set (borrow)");
    assert!(emu.flags().f_sf, "SF should be set (negative result)");
}

#[test]
fn test_sbb_r8_r8_no_borrow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB AL, BL with CF=0
    let code = [
        0x18, 0xd8, // SBB AL, BL (ModRM: 11_011_000)
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0x30;
    emu.regs_mut().rbx = 0x10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x20, "AL should be 0x20 (48 - 16)");
}

#[test]
fn test_sbb_r8_r8_with_borrow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB AL, CL with CF=1
    let code = [0x18, 0xc8, 0xf4]; // SBB AL, CL; HLT
    emu.regs_mut().rax = 0x80;
    emu.regs_mut().rcx = 0x01;
    emu.flags_mut().load(0x01); // Set CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x7E, "AL should be 0x7E (128 - 1 - 1)");
    assert!(!emu.flags().f_sf, "SF should be clear (positive result)");
    assert!(emu.flags().f_of, "OF should be set (signed overflow)");
}

#[test]
fn test_sbb_all_8bit_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = vec![
        (0xd8, "BL"),  // SBB AL, BL
        (0xc8, "CL"),  // SBB AL, CL
        (0xd0, "DL"),  // SBB AL, DL
    ];

    for (modrm, _name) in test_cases {
        let code = [0x18, modrm, 0xf4]; // SBB
        emu.regs_mut().rax = 0x20;
        emu.regs_mut().rbx = 0x05;
        emu.regs_mut().rcx = 0x06;
        emu.regs_mut().rdx = 0x07;
        emu.flags_mut().load(0x01); // Set CF
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        // Result should include borrow
        assert!((emu.regs().rax & 0xFF) < 0x20, "SBB should subtract borrow");
    }
}

#[test]
fn test_sbb_extended_r8_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB R8B, R9B with CF=1
    let code = [
        0x45, 0x18, 0xc8, // SBB R8B, R9B (REX.RB + 18 /r)
        0xf4,             // HLT
    ];
    emu.regs_mut().r8 = 0x50;
    emu.regs_mut().r9 = 0x20;
    emu.flags_mut().load(0x01); // Set CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0x2F, "R8B should be 0x2F (80 - 32 - 1)");
}

// ============================================================================
// 16-bit SBB Tests
// ============================================================================

#[test]
fn test_sbb_ax_imm16_no_borrow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB AX, 0x1234 with CF=0
    let code = [
        0x66, 0x1D, 0x34, 0x12, // SBB AX, 0x1234
        0xf4,                   // HLT
    ];
    emu.regs_mut().rax = 0x5678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x4444, "AX should be 0x4444");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_sbb_ax_imm16_with_borrow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB AX, 0x1000 with CF=1 (causes underflow)
    let code = [0x66, 0x1D, 0x00, 0x10, 0xf4]; // SBB AX, 0x1000
    emu.regs_mut().rax = 0x0500;
    emu.flags_mut().load(0x01); // Set CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x0500 - 0x1000 - 1 = 0xF4FF (underflow)
    assert_eq!(emu.regs().rax & 0xFFFF, 0xF4FF, "AX should wrap to 0xF4FF");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_sbb_r16_r16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB AX, BX with CF=1
    let code = [
        0x66, 0x19, 0xd8, // SBB AX, BX
        0xf4,             // HLT
    ];
    emu.regs_mut().rax = 0x3000;
    emu.regs_mut().rbx = 0x1000;
    emu.flags_mut().load(0x01); // Set CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x1FFF, "AX should be 0x1FFF");
}

#[test]
fn test_sbb_r16_imm8_sign_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB AX, -1 (sign-extended from imm8)
    let code = [
        0x66, 0x83, 0xd8, 0xFF, // SBB AX, -1 (sign-extended)
        0xf4,                   // HLT
    ];
    emu.regs_mut().rax = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x1000 - 0xFFFF - 0 = 0x1001
    assert_eq!(emu.regs().rax & 0xFFFF, 0x1001, "AX should be 0x1001");
}

#[test]
fn test_sbb_extended_r16_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB R10W, R11W with CF=1
    let code = [
        0x66, 0x45, 0x19, 0xda, // SBB R10W, R11W
        0xf4,                   // HLT
    ];
    emu.regs_mut().r10 = 0x8000;
    emu.regs_mut().r11 = 0x7FFF;
    emu.flags_mut().load(0x01); // Set CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10 & 0xFFFF, 0x0000, "R10W should be 0");
}

// ============================================================================
// 32-bit SBB Tests
// ============================================================================

#[test]
fn test_sbb_eax_imm32_no_borrow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB EAX, 0x12345678 with CF=0
    let code = [
        0x1D, 0x78, 0x56, 0x34, 0x12, // SBB EAX, 0x12345678
        0xf4,                         // HLT
    ];
    emu.regs_mut().rax = 0x23456789;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x11111111, "EAX should be 0x11111111");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_sbb_eax_imm32_with_borrow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB EAX, 0x00000001 with CF=1
    let code = [0x1D, 0x01, 0x00, 0x00, 0x00, 0xf4]; // SBB EAX, 1
    emu.regs_mut().rax = 0x00000001;
    emu.flags_mut().load(0x01); // Set CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x00000001 - 1 - 1 = 0xFFFFFFFF
    assert_eq!(emu.regs().rax, 0xFFFFFFFF, "EAX should wrap to 0xFFFFFFFF");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_sbb_r32_r32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB EAX, EBX with CF=1
    let code = [
        0x19, 0xd8, // SBB EAX, EBX
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0x80000000;
    emu.regs_mut().rbx = 0x7FFFFFFF;
    emu.flags_mut().load(0x01); // Set CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x00000000, "EAX should be 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_sbb_r32_imm8_sign_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB EAX, -1 (sign-extended from imm8)
    let code = [
        0x83, 0xd8, 0xFF, // SBB EAX, -1
        0xf4,             // HLT
    ];
    emu.regs_mut().rax = 0x10000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x10000001, "EAX should be 0x10000001");
}

#[test]
fn test_sbb_extended_r32_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB R12D, R13D with CF=1
    let code = [
        0x45, 0x19, 0xec, // SBB R12D, R13D
        0xf4,             // HLT
    ];
    emu.regs_mut().r12 = 0x00000001;
    emu.regs_mut().r13 = 0x00000001;
    emu.flags_mut().load(0x01); // Set CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r12, 0xFFFFFFFF, "R12D should wrap to 0xFFFFFFFF");
    assert!(emu.flags().f_cf, "CF should be set");
}

// ============================================================================
// 64-bit SBB Tests
// ============================================================================

#[test]
fn test_sbb_rax_imm32_no_borrow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB RAX, 0x12345678 (sign-extended to 64-bit)
    let code = [
        0x48, 0x1D, 0x78, 0x56, 0x34, 0x12, // SBB RAX, 0x12345678
        0xf4,                               // HLT
    ];
    emu.regs_mut().rax = 0x1111111123456789;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x1111111111111111, "RAX should be correct");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_sbb_rax_imm32_with_borrow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB RAX, 0x00000001 with CF=1
    let code = [0x48, 0x1D, 0x01, 0x00, 0x00, 0x00, 0xf4]; // SBB RAX, 1
    emu.regs_mut().rax = 0x0000000000000001;
    emu.flags_mut().load(0x01); // Set CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "RAX should wrap to max u64");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_sbb_r64_r64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB RAX, RBX with CF=1
    let code = [
        0x48, 0x19, 0xd8, // SBB RAX, RBX
        0xf4,             // HLT
    ];
    emu.regs_mut().rax = 0x8000000000000000;
    emu.regs_mut().rbx = 0x7FFFFFFFFFFFFFFF;
    emu.flags_mut().load(0x01); // Set CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000000, "RAX should be 0");
}

#[test]
fn test_sbb_r64_imm8_sign_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB RAX, -1 (sign-extended from imm8)
    let code = [
        0x48, 0x83, 0xd8, 0xFF, // SBB RAX, -1
        0xf4,                   // HLT
    ];
    emu.regs_mut().rax = 0x1000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x1000000000000001, "RAX should be incremented");
}

#[test]
fn test_sbb_extended_r64_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB R14, R15 with CF=1
    let code = [
        0x4d, 0x19, 0xfe, // SBB R14, R15
        0xf4,             // HLT
    ];
    emu.regs_mut().r14 = 0x0000000000000001;
    emu.regs_mut().r15 = 0x0000000000000001;
    emu.flags_mut().load(0x01); // Set CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r14, 0xFFFFFFFFFFFFFFFF, "R14 should wrap");
    assert!(emu.flags().f_cf, "CF should be set");
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_sbb_byte_ptr_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB BYTE PTR [mem], 0x10 with CF=1
    let code = [
        0x80, 0x1D, 0xF9, 0x0F, 0x00, 0x00, 0x10, // SBB BYTE PTR [rip+0x0FF9], 0x10
        0xf4,                                      // HLT
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0x30);
    emu.flags_mut().f_cf = true; // Set CF

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0x1F, "Memory should be 0x1F (48 - 16 - 1)");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_sbb_word_ptr_imm16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB WORD PTR [mem], 0x1000 with CF=1
    let code = [
        0x66, 0x81, 0x1D, 0xF7, 0x0F, 0x00, 0x00, 0x00, 0x10, // SBB WORD PTR [rip+0x0FF7], 0x1000
        0xf4,                                                  // HLT
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0x2000);
    emu.flags_mut().f_cf = true; // Set CF

    emu.run(None).unwrap();
    let result = emu.maps.read_word(DATA_ADDR).unwrap();

    assert_eq!(result, 0x0FFF, "Memory should be 0x0FFF");
}

#[test]
fn test_sbb_dword_ptr_r32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB DWORD PTR [mem], EBX with CF=1
    let code = [
        0x19, 0x1d, 0xFA, 0x0F, 0x00, 0x00, // SBB DWORD PTR [rip+0x0FF7], EBX
        0xf4,                               // HLT
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x80000000);
    emu.regs_mut().rbx = 0x40000000;
    emu.flags_mut().f_cf = true; // Set CF

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x3FFFFFFF, "Memory should be 0x3FFFFFFF");
}

#[test]
fn test_sbb_qword_ptr_r64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB QWORD PTR [mem], RBX with CF=1
    let code = [
        0x48, 0x19, 0x1d, 0xF9, 0x0F, 0x00, 0x00, // SBB QWORD PTR [rip+0x0FF6], RBX
        0xf4,                                      // HLT
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x3000000000000000);
    emu.regs_mut().rbx = 0x1000000000000000;
    emu.flags_mut().f_cf = true; // Set CF

    emu.run(None).unwrap();
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x1FFFFFFFFFFFFFFF, "Memory should include borrow");
}

#[test]
fn test_sbb_r64_from_memory() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB RAX, QWORD PTR [mem] with CF=1
    let code = [
        0x48, 0x1B, 0x05, 0xF9, 0x0F, 0x00, 0x00, // SBB RAX, QWORD PTR [rip+0x0FF6]
        0xf4,                                      // HLT
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x1000000000000000);
    emu.regs_mut().rax = 0x2000000000000000;
    emu.flags_mut().f_cf = true; // Set CF

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0FFFFFFFFFFFFFFF, "RAX should be correct difference");
}

// ============================================================================
// Flag Tests
// ============================================================================

#[test]
fn test_sbb_zero_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB resulting in zero should set ZF
    let code = [0x1C, 0x00, 0xf4]; // SBB AL, 0
    emu.regs_mut().rax = 0x01;
    emu.flags_mut().load(0x01); // Set CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x01 - 0 - 1 = 0
    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should be 0");
    assert!(emu.flags().f_zf, "ZF should be set");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_sbb_sign_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB resulting in negative (bit 7 set for 8-bit)
    let code = [0x1C, 0x01, 0xf4]; // SBB AL, 1
    emu.regs_mut().rax = 0x81;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x80, "AL should be 0x80");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_sbb_overflow_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB causing signed overflow
    let code = [0x1C, 0x80, 0xf4]; // SBB AL, 0x80
    emu.regs_mut().rax = 0x7F; // Max positive i8
    emu.flags_mut().load(0x01); // Set CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x7F - 0x80 - 1 = 0xFE (overflow from positive to negative)
    assert_eq!(emu.regs().rax & 0xFF, 0xFE, "AL should be 0xFE");
    assert!(emu.flags().f_of, "OF should be set");
}

#[test]
fn test_sbb_parity_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x1C, 0x02, 0xf4]; // SBB AL, 2
    emu.regs_mut().rax = 0x05;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03, "AL should be 3");
    assert!(emu.flags().f_pf, "PF should be set (even parity)");
}

#[test]
fn test_sbb_auxiliary_carry_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB with auxiliary borrow (borrow from bit 3 to bit 4)
    let code = [0x1C, 0x05, 0xf4]; // SBB AL, 5
    emu.regs_mut().rax = 0x12;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x0D, "AL should be 0x0D");
    assert!(emu.flags().f_af, "AF should be set");
}

// ============================================================================
// Multi-precision Arithmetic Tests
// ============================================================================

#[test]
fn test_sbb_chain_128bit_subtraction() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x29, 0xd8, // SUB RAX, RBX (low 64 bits)
        0x49, 0x19, 0xc8, // SBB R8, RCX (high 64 bits, with borrow)
        0xf4,             // HLT
    ];
    emu.regs_mut().rax = 0x0000000000000000; // Low 64 bits of first number
    emu.regs_mut().r8 = 0x0000000000000002;  // High 64 bits of first number
    emu.regs_mut().rbx = 0x0000000000000001; // Low 64 bits of second number
    emu.regs_mut().rcx = 0x0000000000000001; // High 64 bits of second number
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "Low 64 bits should be max u64");
    assert_eq!(emu.regs().r8, 0x0000000000000000, "High 64 bits should be 0 (with borrow)");
}

#[test]
fn test_sbb_preserves_high_bits_8bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x1C, 0x05, 0xf4]; // SBB AL, 5
    emu.regs_mut().rax = 0xDEADBEEF12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax >> 8, 0xDEADBEEF123456, "High bits should be preserved");
}

#[test]
fn test_sbb_preserves_high_bits_16bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x1D, 0x00, 0x10, 0xf4]; // SBB AX, 0x1000
    emu.regs_mut().rax = 0xDEADBEEF12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax >> 16, 0xDEADBEEF1234, "High bits should be preserved");
}

#[test]
fn test_sbb_preserves_high_bits_32bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x1D, 0x00, 0x00, 0x00, 0x10, 0xf4]; // SBB EAX, 0x10000000
    emu.regs_mut().rax = 0xDEADBEEF12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // EAX operation zeros high 32 bits
    assert_eq!(emu.regs().rax >> 32, 0, "High 32 bits should be zeroed for 32-bit op");
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_sbb_zero_minus_zero_minus_borrow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0 - 0 - 1 = -1
    let code = [0x1C, 0x00, 0xf4]; // SBB AL, 0
    emu.regs_mut().rax = 0x00;
    emu.flags_mut().load(0x01); // Set CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "AL should be 0xFF");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_sbb_self_with_borrow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB AL, AL with CF=1 should give -1
    let code = [0x18, 0xc0, 0xf4]; // SBB AL, AL
    emu.regs_mut().rax = 0x42;
    emu.flags_mut().load(0x01); // Set CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL - AL - 1 = -1 = 0xFF
    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "AL should be 0xFF");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_sbb_self_without_borrow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB AL, AL with CF=0 should give 0
    let code = [0x18, 0xc0, 0xf4]; // SBB AL, AL
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL - AL - 0 = 0
    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should be 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(emu.flags().f_zf, "ZF should be set");
}
