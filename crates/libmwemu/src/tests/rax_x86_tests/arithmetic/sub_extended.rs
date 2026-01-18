use crate::*;

// SUB — Integer Subtraction
//
// Opcodes:
// - 2C ib           SUB AL, imm8
// - 2D iw           SUB AX, imm16
// - 2D id           SUB EAX, imm32
// - REX.W + 2D id   SUB RAX, imm32 (sign-extended)
// - 80 /5 ib        SUB r/m8, imm8
// - 81 /5 iw        SUB r/m16, imm16
// - 81 /5 id        SUB r/m32, imm32
// - REX.W + 81 /5 id SUB r/m64, imm32 (sign-extended)
// - 83 /5 ib        SUB r/m16, imm8 (sign-extended)
// - 83 /5 ib        SUB r/m32, imm8 (sign-extended)
// - REX.W + 83 /5 ib SUB r/m64, imm8 (sign-extended)
// - 28 /r           SUB r/m8, r8
// - 29 /r           SUB r/m16, r16
// - 29 /r           SUB r/m32, r32
// - REX.W + 29 /r   SUB r/m64, r64
// - 2A /r           SUB r8, r/m8
// - 2B /r           SUB r16, r/m16
// - 2B /r           SUB r32, r/m32
// - REX.W + 2B /r   SUB r64, r/m64
//
// Operation: DEST := DEST - SRC
// Flags: CF, OF, SF, ZF, AF, PF are set according to result

// ============================================================================
// 8-bit SUB Tests
// ============================================================================

#[test]
fn test_sub_al_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2C, 0x05, 0xf4]; // SUB AL, 5; HLT
    emu.regs_mut().rax = 0x0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x0A, "AL should be 10 (15 - 5)");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_sub_al_underflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2C, 0x10, 0xf4]; // SUB AL, 0x10
    emu.regs_mut().rax = 0x05;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xF5, "AL should wrap to 0xF5");
    assert!(emu.flags().f_cf, "CF should be set (borrow)");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_sub_r8_r8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x28, 0xd8, 0xf4]; // SUB AL, BL
    emu.regs_mut().rax = 0x30;
    emu.regs_mut().rbx = 0x10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x20, "AL should be 0x20");
}

#[test]
fn test_sub_r8_r8_signed_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x28, 0xc8, 0xf4]; // SUB AL, CL
    emu.regs_mut().rax = 0x80; // Min negative i8
    emu.regs_mut().rcx = 0x01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x7F, "AL should be 0x7F");
    assert!(emu.flags().f_of, "OF should be set (signed overflow)");
}

#[test]
fn test_sub_extended_r8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x45, 0x28, 0xc8, 0xf4]; // SUB R8B, R9B
    emu.regs_mut().r8 = 0x50;
    emu.regs_mut().r9 = 0x20;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0x30, "R8B should be 0x30");
}

#[test]
fn test_sub_all_8bit_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for reg_num in 8..=15 {
        let modrm = 0xC0 | ((reg_num - 8) << 3) | (reg_num - 8);
        let code = [0x45, 0x28, modrm, 0xf4]; // SUB R*B, R*B (result: 0)

        match reg_num {
            8 => emu.regs_mut().r8 = 0x42,
            9 => emu.regs_mut().r9 = 0x42,
            10 => emu.regs_mut().r10 = 0x42,
            11 => emu.regs_mut().r11 = 0x42,
            12 => emu.regs_mut().r12 = 0x42,
            13 => emu.regs_mut().r13 = 0x42,
            14 => emu.regs_mut().r14 = 0x42,
            15 => emu.regs_mut().r15 = 0x42,
            _ => unreachable!(),
        }

        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        let result = match reg_num {
            8 => emu.regs().r8,
            9 => emu.regs().r9,
            10 => emu.regs().r10,
            11 => emu.regs().r11,
            12 => emu.regs().r12,
            13 => emu.regs().r13,
            14 => emu.regs().r14,
            15 => emu.regs().r15,
            _ => unreachable!(),
        };

        assert_eq!(result & 0xFF, 0x00, "R{} - R{} should be 0", reg_num, reg_num);
    }
}

// ============================================================================
// 16-bit SUB Tests
// ============================================================================

#[test]
fn test_sub_ax_imm16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x2D, 0x34, 0x12, 0xf4]; // SUB AX, 0x1234
    emu.regs_mut().rax = 0x5678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x4444, "AX should be 0x4444");
}

#[test]
fn test_sub_ax_underflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x2D, 0x00, 0x10, 0xf4]; // SUB AX, 0x1000
    emu.regs_mut().rax = 0x0500;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xF500, "AX should wrap to 0xF500");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_sub_r16_r16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x29, 0xd8, 0xf4]; // SUB AX, BX
    emu.regs_mut().rax = 0x3000;
    emu.regs_mut().rbx = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x2000, "AX should be 0x2000");
}

#[test]
fn test_sub_r16_imm8_sign_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x83, 0xe8, 0xFF, 0xf4]; // SUB AX, -1
    emu.regs_mut().rax = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x1001, "AX should be 0x1001");
}

#[test]
fn test_sub_extended_r16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x29, 0xda, 0xf4]; // SUB R10W, R11W
    emu.regs_mut().r10 = 0x8000;
    emu.regs_mut().r11 = 0x4000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10 & 0xFFFF, 0x4000, "R10W should be 0x4000");
}

// ============================================================================
// 32-bit SUB Tests
// ============================================================================

#[test]
fn test_sub_eax_imm32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2D, 0x78, 0x56, 0x34, 0x12, 0xf4]; // SUB EAX, 0x12345678
    emu.regs_mut().rax = 0x23456789;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x11111111, "EAX should be 0x11111111");
}

#[test]
fn test_sub_eax_underflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2D, 0x02, 0x00, 0x00, 0x00, 0xf4]; // SUB EAX, 2
    emu.regs_mut().rax = 0x00000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFF, "EAX should wrap to 0xFFFFFFFF");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_sub_r32_r32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x29, 0xd8, 0xf4]; // SUB EAX, EBX
    emu.regs_mut().rax = 0x30000000;
    emu.regs_mut().rbx = 0x10000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x20000000, "EAX should be 0x20000000");
}

#[test]
fn test_sub_r32_imm8_sign_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x83, 0xe8, 0x7F, 0xf4]; // SUB EAX, 127
    emu.regs_mut().rax = 0x10000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0FFFFF81, "EAX should be 0x0FFFFF81");
}

#[test]
fn test_sub_extended_r32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x45, 0x29, 0xec, 0xf4]; // SUB R12D, R13D
    emu.regs_mut().r12 = 0x70000000;
    emu.regs_mut().r13 = 0x30000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r12, 0x40000000, "R12D should be 0x40000000");
}

// ============================================================================
// 64-bit SUB Tests
// ============================================================================

#[test]
fn test_sub_rax_imm32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x2D, 0x78, 0x56, 0x34, 0x12, 0xf4]; // SUB RAX, 0x12345678
    emu.regs_mut().rax = 0x1111111123456789;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x1111111111111111, "RAX should be correct");
}

#[test]
fn test_sub_rax_underflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x2D, 0x02, 0x00, 0x00, 0x00, 0xf4]; // SUB RAX, 2
    emu.regs_mut().rax = 0x0000000000000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "RAX should wrap to max u64");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_sub_r64_r64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x29, 0xd8, 0xf4]; // SUB RAX, RBX
    emu.regs_mut().rax = 0x3000000000000000;
    emu.regs_mut().rbx = 0x1000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x2000000000000000, "RAX should be correct");
}

#[test]
fn test_sub_r64_imm8_sign_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x83, 0xe8, 0xFF, 0xf4]; // SUB RAX, -1
    emu.regs_mut().rax = 0x1000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x1000000000000001, "RAX should be incremented");
}

#[test]
fn test_sub_all_64bit_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for i in 0..16 {
        let (rex, modrm) = if i < 8 {
            (0x48, 0xC0 | (i as u8))
        } else {
            (0x4C, 0xC0 | ((i - 8) as u8))
        };

        let code = [rex, 0x29, modrm, 0xf4]; // SUB RAX, reg
        emu.regs_mut().rax = 0x2000000000000000;
        emu.regs_mut().rcx = 0x0100000000000000;
        emu.regs_mut().rdx = 0x0200000000000000;
        emu.regs_mut().rbx = 0x0300000000000000;
        //emu.regs_mut().rsp = STACK_ADDR;
        emu.regs_mut().rbp = 0x0500000000000000;
        emu.regs_mut().rsi = 0x0600000000000000;
        emu.regs_mut().rdi = 0x0700000000000000;
        emu.regs_mut().r8 = 0x0800000000000000;
        emu.regs_mut().r9 = 0x0900000000000000;
        emu.regs_mut().r10 = 0x0A00000000000000;
        emu.regs_mut().r11 = 0x0B00000000000000;
        emu.regs_mut().r12 = 0x0C00000000000000;
        emu.regs_mut().r13 = 0x0D00000000000000;
        emu.regs_mut().r14 = 0x0E00000000000000;
        emu.regs_mut().r15 = 0x0F00000000000000;

        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        assert!(emu.regs().rax <= 0x2000000000000000, "RAX should decrease");
    }
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_sub_byte_ptr_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x80, 0x2D, 0xF9, 0x0F, 0x00, 0x00, 0x10, // SUB BYTE PTR [rip+0x0FF9], 0x10
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0x30);

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0x20, "Memory should be 0x20");
}

#[test]
fn test_sub_qword_ptr_r64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x29, 0x1d, 0xF9, 0x0F, 0x00, 0x00, // SUB QWORD PTR [rip+0x0FF6], RBX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x3000000000000000);
    emu.regs_mut().rbx = 0x1000000000000000;

    emu.run(None).unwrap();
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x2000000000000000, "Memory should be correct");
}

#[test]
fn test_sub_r64_from_memory() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x2B, 0x05, 0xF9, 0x0F, 0x00, 0x00, // SUB RAX, QWORD PTR [rip+0x0FF6]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x1000000000000000);
    emu.regs_mut().rax = 0x3000000000000000;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x2000000000000000, "RAX should be correct");
}

// ============================================================================
// Flag Tests
// ============================================================================

#[test]
fn test_sub_zero_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2C, 0x05, 0xf4]; // SUB AL, 5
    emu.regs_mut().rax = 0x05;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should be 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_sub_sign_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2C, 0x01, 0xf4]; // SUB AL, 1
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "AL should be 0xFF");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_sub_parity_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2C, 0x02, 0xf4]; // SUB AL, 2
    emu.regs_mut().rax = 0x05;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03, "AL should be 3");
    assert!(emu.flags().f_pf, "PF should be set (even parity)");
}

#[test]
fn test_sub_auxiliary_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2C, 0x05, 0xf4]; // SUB AL, 5
    emu.regs_mut().rax = 0x12;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x0D, "AL should be 0x0D");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_sub_overflow_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2C, 0x01, 0xf4]; // SUB AL, 1
    emu.regs_mut().rax = 0x80; // Min negative i8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x7F, "AL should be 0x7F");
    assert!(emu.flags().f_of, "OF should be set");
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_sub_self() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x28, 0xc0, 0xf4]; // SUB AL, AL
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should be 0");
    assert!(emu.flags().f_zf, "ZF should be set");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_sub_preserves_high_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2C, 0x05, 0xf4]; // SUB AL, 5
    emu.regs_mut().rax = 0xDEADBEEF12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax >> 8, 0xDEADBEEF123456, "High bits should be preserved");
}

#[test]
fn test_sub_zero_from_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2C, 0x00, 0xf4]; // SUB AL, 0
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should be 0");
    assert!(emu.flags().f_zf, "ZF should be set");
    assert!(!emu.flags().f_cf, "CF should be clear");
}
