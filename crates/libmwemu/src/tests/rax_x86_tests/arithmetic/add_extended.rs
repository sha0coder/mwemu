use crate::*;

// ADD — Integer Addition
//
// Opcodes:
// - 04 ib           ADD AL, imm8
// - 05 iw           ADD AX, imm16
// - 05 id           ADD EAX, imm32
// - REX.W + 05 id   ADD RAX, imm32 (sign-extended)
// - 80 /0 ib        ADD r/m8, imm8
// - 81 /0 iw        ADD r/m16, imm16
// - 81 /0 id        ADD r/m32, imm32
// - REX.W + 81 /0 id ADD r/m64, imm32 (sign-extended)
// - 83 /0 ib        ADD r/m16, imm8 (sign-extended)
// - 83 /0 ib        ADD r/m32, imm8 (sign-extended)
// - REX.W + 83 /0 ib ADD r/m64, imm8 (sign-extended)
// - 00 /r           ADD r/m8, r8
// - 01 /r           ADD r/m16, r16
// - 01 /r           ADD r/m32, r32
// - REX.W + 01 /r   ADD r/m64, r64
// - 02 /r           ADD r8, r/m8
// - 03 /r           ADD r16, r/m16
// - 03 /r           ADD r32, r/m32
// - REX.W + 03 /r   ADD r64, r/m64
//
// Operation: DEST := DEST + SRC
// Flags: CF, OF, SF, ZF, AF, PF are set according to result

// ============================================================================
// 8-bit ADD Tests
// ============================================================================

#[test]
fn test_add_al_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x04, 0x05, 0xf4]; // ADD AL, 5; HLT
    emu.regs_mut().rax = 0x0A;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x0F, "AL should be 15 (10 + 5)");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_of, "OF should be clear");
}

#[test]
fn test_add_al_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x04, 0xFF, 0xf4]; // ADD AL, 0xFF
    emu.regs_mut().rax = 0x02;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x01, "AL should wrap to 1");
    assert!(emu.flags().f_cf, "CF should be set (unsigned overflow)");
}

#[test]
fn test_add_r8_r8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x00, 0xd8, 0xf4]; // ADD AL, BL; HLT
    emu.regs_mut().rax = 0x20;
    emu.regs_mut().rbx = 0x15;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x35, "AL should be 0x35");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_add_r8_r8_signed_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x00, 0xc8, 0xf4]; // ADD AL, CL
    emu.regs_mut().rax = 0x7F; // Max positive i8
    emu.regs_mut().rcx = 0x01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x80, "AL should be 0x80");
    assert!(emu.flags().f_of, "OF should be set (signed overflow)");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_add_all_8bit_gp_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = vec![
        (0xd8, 0x10, 0x05), // ADD AL, BL
        (0xc8, 0x10, 0x06), // ADD AL, CL
        (0xd0, 0x10, 0x07), // ADD AL, DL
    ];

    for (modrm, base, addend) in test_cases {
        let code = [0x00, modrm, 0xf4];
        emu.regs_mut().rax = base;
        emu.regs_mut().rbx = addend;
        emu.regs_mut().rcx = addend + 1;
        emu.regs_mut().rdx = addend + 2;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert!((emu.regs().rax & 0xFF) > base, "ADD should increase AL");
    }
}

#[test]
fn test_add_extended_r8_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADD R8B, R9B
    let code = [0x45, 0x00, 0xc8, 0xf4]; // ADD R8B, R9B
    emu.regs_mut().r8 = 0x40;
    emu.regs_mut().r9 = 0x30;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0x70, "R8B should be 0x70");
}

#[test]
fn test_add_r8_from_r9_to_r15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for reg_num in 8..=15 {
        let modrm = 0xC0 | ((reg_num - 8) << 3) | (reg_num - 8);
        let code = [0x45, 0x00, modrm, 0xf4]; // ADD R*B, R*B (double itself)

        // Set value in the specific register
        match reg_num {
            8 => emu.regs_mut().r8 = 0x10,
            9 => emu.regs_mut().r9 = 0x10,
            10 => emu.regs_mut().r10 = 0x10,
            11 => emu.regs_mut().r11 = 0x10,
            12 => emu.regs_mut().r12 = 0x10,
            13 => emu.regs_mut().r13 = 0x10,
            14 => emu.regs_mut().r14 = 0x10,
            15 => emu.regs_mut().r15 = 0x10,
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

        assert_eq!(result & 0xFF, 0x20, "R{} should be 0x20", reg_num);
    }
}

// ============================================================================
// 16-bit ADD Tests
// ============================================================================

#[test]
fn test_add_ax_imm16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x05, 0x34, 0x12, 0xf4]; // ADD AX, 0x1234
    emu.regs_mut().rax = 0x5678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x68AC, "AX should be 0x68AC");
}

#[test]
fn test_add_ax_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x05, 0xFF, 0xFF, 0xf4]; // ADD AX, 0xFFFF
    emu.regs_mut().rax = 0x0002;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0001, "AX should wrap to 1");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_add_r16_r16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x01, 0xd8, 0xf4]; // ADD AX, BX
    emu.regs_mut().rax = 0x1000;
    emu.regs_mut().rbx = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x3000, "AX should be 0x3000");
}

#[test]
fn test_add_r16_imm8_sign_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x83, 0xc0, 0xFF, 0xf4]; // ADD AX, -1 (sign-extended)
    emu.regs_mut().rax = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0FFF, "AX should be 0x0FFF");
}

#[test]
fn test_add_extended_r16_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x01, 0xda, 0xf4]; // ADD R10W, R11W
    emu.regs_mut().r10 = 0x4000;
    emu.regs_mut().r11 = 0x3000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10 & 0xFFFF, 0x7000, "R10W should be 0x7000");
}

#[test]
fn test_add_all_16bit_gp_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let regs_to_test = vec![
        (0xD8, "BX"), (0xC8, "CX"), (0xD0, "DX"),
        (0xF0, "SI"), (0xF8, "DI"), (0xE0, "SP"), (0xE8, "BP"),
    ];

    for (modrm, _name) in regs_to_test {
        let code = [0x66, 0x01, modrm, 0xf4]; // ADD AX, r16
        emu.regs_mut().rax = 0x1000;
        emu.regs_mut().rbx = 0x0100;
        emu.regs_mut().rcx = 0x0200;
        emu.regs_mut().rdx = 0x0300;
        emu.regs_mut().rsi = 0x0400;
        emu.regs_mut().rdi = 0x0500;
        emu.regs_mut().rsp = emu.regs().rsp + 0x0600;
        emu.regs_mut().rbp = 0x0700;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        assert!((emu.regs().rax & 0xFFFF) > 0x1000, "AX should increase");
    }
}

// ============================================================================
// 32-bit ADD Tests
// ============================================================================

#[test]
fn test_add_eax_imm32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x05, 0x78, 0x56, 0x34, 0x12, 0xf4]; // ADD EAX, 0x12345678
    emu.regs_mut().rax = 0x11111111;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x23456789, "EAX should be 0x23456789");
}

#[test]
fn test_add_eax_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x05, 0xFF, 0xFF, 0xFF, 0xFF, 0xf4]; // ADD EAX, 0xFFFFFFFF
    emu.regs_mut().rax = 0x00000002;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x00000001, "EAX should wrap to 1");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_add_r32_r32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x01, 0xd8, 0xf4]; // ADD EAX, EBX
    emu.regs_mut().rax = 0x10000000;
    emu.regs_mut().rbx = 0x20000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x30000000, "EAX should be 0x30000000");
}

#[test]
fn test_add_r32_imm8_sign_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x83, 0xc0, 0x7F, 0xf4]; // ADD EAX, 127
    emu.regs_mut().rax = 0x10000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x1000007F, "EAX should be 0x1000007F");
}

#[test]
fn test_add_extended_r32_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x45, 0x01, 0xec, 0xf4]; // ADD R12D, R13D
    emu.regs_mut().r12 = 0x40000000;
    emu.regs_mut().r13 = 0x30000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r12, 0x70000000, "R12D should be 0x70000000");
}

#[test]
fn test_add_all_32bit_gp_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let regs_to_test = vec!["EAX", "EBX", "ECX", "EDX", "ESI", "EDI", "ESP", "EBP"];

    for i in 0..regs_to_test.len() {
        let modrm = 0xC0 | (i as u8);
        let code = [0x01, modrm, 0xf4]; // ADD EAX, reg
        emu.regs_mut().rax = 0x10000000;
        emu.regs_mut().rbx = 0x01000000;
        emu.regs_mut().rcx = 0x02000000;
        emu.regs_mut().rdx = 0x03000000;
        emu.regs_mut().rsi = 0x04000000;
        emu.regs_mut().rdi = 0x05000000;
        //emu.regs_mut().rsp = STACK_ADDR + 0x06000000;
        emu.regs_mut().rbp = 0x07000000;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        assert!(emu.regs().rax >= 0x10000000, "EAX should increase");
    }
}

// ============================================================================
// 64-bit ADD Tests
// ============================================================================

#[test]
fn test_add_rax_imm32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x05, 0x78, 0x56, 0x34, 0x12, 0xf4]; // ADD RAX, 0x12345678
    emu.regs_mut().rax = 0x1111111111111111;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x1111111123456789, "RAX should be correct");
}

#[test]
fn test_add_rax_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x05, 0xFF, 0xFF, 0xFF, 0x7F, 0xf4]; // ADD RAX, 0x7FFFFFFF
    emu.regs_mut().rax = 0xFFFFFFFF80000002;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000001, "RAX should wrap");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_add_r64_r64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x01, 0xd8, 0xf4]; // ADD RAX, RBX
    emu.regs_mut().rax = 0x1000000000000000;
    emu.regs_mut().rbx = 0x2000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x3000000000000000, "RAX should be correct sum");
}

#[test]
fn test_add_r64_imm8_sign_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x83, 0xc0, 0xFF, 0xf4]; // ADD RAX, -1
    emu.regs_mut().rax = 0x1000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0FFFFFFFFFFFFFFF, "RAX should be decremented");
}

#[test]
fn test_add_all_64bit_gp_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let all_regs = [
        "RAX", "RCX", "RDX", "RBX", "RSP", "RBP", "RSI", "RDI",
        "R8", "R9", "R10", "R11", "R12", "R13", "R14", "R15"
    ];

    for i in 0..all_regs.len() {
        let (rex, modrm) = if i < 8 {
            (0x48, 0xC0 | (i as u8))
        } else {
            (0x4C, 0xC0 | ((i - 8) as u8))
        };

        let code = [rex, 0x01, modrm, 0xf4]; // ADD RAX, reg
        emu.regs_mut().rax = 0x1000000000000000;
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

        assert!(emu.regs().rax >= 0x1000000000000000, "RAX should increase for {}", all_regs[i]);
    }
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_add_byte_ptr_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x80, 0x05, 0xF9, 0x0F, 0x00, 0x00, 0x10, // ADD BYTE PTR [rip+0x0FF9], 0x10
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0x20);

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0x30, "Memory should be 0x30");
}

#[test]
fn test_add_word_ptr_r16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0x01, 0x1d, 0xF9, 0x0F, 0x00, 0x00, // ADD WORD PTR [rip+0x0FF6], BX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0x1000);
    emu.regs_mut().rbx = 0x2000;

    emu.run(None).unwrap();
    let result = emu.maps.read_word(DATA_ADDR).unwrap();

    assert_eq!(result, 0x3000, "Memory should be 0x3000");
}

#[test]
fn test_add_dword_ptr_r32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x01, 0x1d, 0xFA, 0x0F, 0x00, 0x00, // ADD DWORD PTR [rip+0x0FF7], EBX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x10000000);
    emu.regs_mut().rbx = 0x20000000;

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x30000000, "Memory should be 0x30000000");
}

#[test]
fn test_add_qword_ptr_r64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x01, 0x1d, 0xF9, 0x0F, 0x00, 0x00, // ADD QWORD PTR [rip+0x0FF6], RBX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x1000000000000000);
    emu.regs_mut().rbx = 0x2000000000000000;

    emu.run(None).unwrap();
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x3000000000000000, "Memory should be correct sum");
}

#[test]
fn test_add_r64_from_memory() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x03, 0x05, 0xF9, 0x0F, 0x00, 0x00, // ADD RAX, QWORD PTR [rip+0x0FF6]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x2000000000000000);
    emu.regs_mut().rax = 0x1000000000000000;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x3000000000000000, "RAX should be correct sum");
}

// ============================================================================
// Flag Tests
// ============================================================================

#[test]
fn test_add_zero_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x04, 0x00, 0xf4]; // ADD AL, 0
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should be 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_add_sign_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x04, 0x7F, 0xf4]; // ADD AL, 0x7F
    emu.regs_mut().rax = 0x01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x80, "AL should be 0x80");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_add_parity_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x04, 0x02, 0xf4]; // ADD AL, 2
    emu.regs_mut().rax = 0x01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03, "AL should be 3");
    assert!(emu.flags().f_pf, "PF should be set (even parity)");
}

#[test]
fn test_add_auxiliary_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x04, 0x0A, 0xf4]; // ADD AL, 0x0A
    emu.regs_mut().rax = 0x08;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x12, "AL should be 0x12");
    assert!(emu.flags().f_af, "AF should be set");
}

#[test]
fn test_add_overflow_positive_to_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x04, 0x01, 0xf4]; // ADD AL, 1
    emu.regs_mut().rax = 0x7F; // Max positive i8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x80, "AL should be 0x80");
    assert!(emu.flags().f_of, "OF should be set");
}

#[test]
fn test_add_overflow_negative_to_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x04, 0xFF, 0xf4]; // ADD AL, -1
    emu.regs_mut().rax = 0x80; // Min negative i8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x7F, "AL should be 0x7F");
    assert!(emu.flags().f_of, "OF should be set");
}

// ============================================================================
// Edge Cases and Special Tests
// ============================================================================

#[test]
fn test_add_zero_to_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x04, 0x00, 0xf4]; // ADD AL, 0
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should be 0");
    assert!(emu.flags().f_zf, "ZF should be set");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_of, "OF should be clear");
}

#[test]
fn test_add_self() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x00, 0xc0, 0xf4]; // ADD AL, AL
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x84, "AL should be doubled");
}

#[test]
fn test_add_preserves_high_bits_8bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x04, 0x05, 0xf4]; // ADD AL, 5
    emu.regs_mut().rax = 0xDEADBEEF12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax >> 8, 0xDEADBEEF123456, "High bits should be preserved");
}

#[test]
fn test_add_preserves_high_bits_16bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x05, 0x00, 0x10, 0xf4]; // ADD AX, 0x1000
    emu.regs_mut().rax = 0xDEADBEEF12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax >> 16, 0xDEADBEEF1234, "High bits should be preserved");
}

#[test]
fn test_add_zeros_high_bits_32bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x05, 0x00, 0x00, 0x00, 0x10, 0xf4]; // ADD EAX, 0x10000000
    emu.regs_mut().rax = 0xDEADBEEF12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax >> 32, 0, "High 32 bits should be zeroed for 32-bit op");
}

#[test]
fn test_add_max_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x01, 0xd8, 0xf4]; // ADD RAX, RBX
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    emu.regs_mut().rbx = 0x0000000000000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000000, "RAX should wrap to 0");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_add_commutative() {
    let code1 = [0x00, 0xd8, 0xf4]; // ADD AL, BL
    let code2 = [0x00, 0xc3, 0xf4]; // ADD BL, AL

    let mut emu1 = emu64();
    emu1.regs_mut().rax = 0x42;
    emu1.regs_mut().rbx = 0x17;
    emu1.load_code_bytes(&code1);
    emu1.run(None).unwrap();

    let mut emu2 = emu64();
    emu2.regs_mut().rax = 0x42;
    emu2.regs_mut().rbx = 0x17;
    emu2.load_code_bytes(&code2);
    emu2.run(None).unwrap();

    assert_eq!(emu1.regs().rax & 0xFF, emu2.regs().rbx & 0xFF, "Results should match");
}
