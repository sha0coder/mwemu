//! Tests for the INC instruction.
//!
//! INC - Increment by 1
//!
//! Adds 1 to the destination operand while preserving the CF flag.
//!
//! Flags affected: OF, SF, ZF, AF, PF (CF is NOT affected)
//!
//! Reference: docs/inc.txt

use crate::*;

// ============================================================================
// INC r/m8 (opcode FE /0)
// ============================================================================

#[test]
fn test_inc_rm8_register_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC AL
    // FE C0 = INC AL
    // f4 = HLT
    let code = [0xfe, 0xc0, 0xf4];
    emu.regs_mut().rax = 10;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 11, "INC AL: 10 + 1 = 11");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_inc_rm8_register_zero_result() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC AL when AL = 0xFF -> 0x00
    let code = [0xfe, 0xc0, 0xf4];
    emu.regs_mut().rax = 0xFF;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0, "INC AL: 0xFF + 1 = 0x00 (wraps)");
    assert!(emu.flags().f_zf, "ZF should be set (result = 0)");
}

#[test]
fn test_inc_rm8_register_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC AL when AL = 0x7F -> 0x80 (signed overflow)
    let code = [0xfe, 0xc0, 0xf4];
    emu.regs_mut().rax = 0x7F;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0x80, "INC AL: 0x7F + 1 = 0x80");
    assert!(emu.flags().f_of, "OF should be set (signed overflow)");
    assert!(emu.flags().f_sf, "SF should be set (result negative)");
}

#[test]
fn test_inc_rm8_preserves_cf() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC should NOT affect CF flag
    let code = [0xfe, 0xc0, 0xf4];
    emu.regs_mut().rax = 0xFF;
    emu.flags_mut().load(0x2 | flags::F_CF); // CF=1 initially
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0);
    assert!(emu.flags().f_cf, "CF should be preserved (still set)");
}

#[test]
fn test_inc_rm8_memory() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC BYTE PTR [RBX]
    // FE 03 = INC BYTE PTR [RBX]
    let code = [0xfe, 0x03, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.load_code_bytes(&code);

    emu.maps.write_byte(DATA_ADDR, 42);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_byte(DATA_ADDR).unwrap(), 43, "INC [RBX]: 42 + 1 = 43");
}

// ============================================================================
// INC r/m16 (opcode FF /0 with 66 prefix)
// ============================================================================

#[test]
fn test_inc_rm16_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC AX
    // 66 FF C0 = INC AX
    let code = [0x66, 0xff, 0xc0, 0xf4];
    emu.regs_mut().rax = 0x1234;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0x1235, "INC AX: 0x1234 + 1 = 0x1235");
}

#[test]
fn test_inc_rm16_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC AX when AX = 0xFFFF -> 0x0000
    let code = [0x66, 0xff, 0xc0, 0xf4];
    emu.regs_mut().rax = 0xFFFF;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0, "INC AX: 0xFFFF + 1 = 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_inc_rm16_memory() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC WORD PTR [RBX]
    // 66 FF 03 = INC WORD PTR [RBX]
    let code = [0x66, 0xff, 0x03, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.load_code_bytes(&code);

    emu.maps.write_word(DATA_ADDR, 0x1234);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(DATA_ADDR).unwrap(), 0x1235);
}

// ============================================================================
// INC r/m32 (opcode FF /0)
// ============================================================================

#[test]
fn test_inc_rm32_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC EAX
    // FF C0 = INC EAX
    let code = [0xff, 0xc0, 0xf4];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x12345679, "INC EAX: 0x12345678 + 1");
}

#[test]
fn test_inc_rm32_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC EAX when EAX = 0xFFFFFFFF -> 0x00000000
    let code = [0xff, 0xc0, 0xf4];
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0, "INC EAX: 0xFFFFFFFF + 1 = 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_inc_rm32_signed_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC EAX when EAX = 0x7FFFFFFF -> 0x80000000 (signed overflow)
    let code = [0xff, 0xc0, 0xf4];
    emu.regs_mut().rax = 0x7FFFFFFF;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x80000000);
    assert!(emu.flags().f_of, "OF should be set (signed overflow)");
}

#[test]
fn test_inc_rm32_memory() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC DWORD PTR [RBX]
    // FF 03 = INC DWORD PTR [RBX]
    let code = [0xff, 0x03, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.load_code_bytes(&code);

    emu.maps.write_dword(DATA_ADDR, 0x12345678);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(DATA_ADDR).unwrap(), 0x12345679);
}

// ============================================================================
// INC r/m64 (opcode REX.W FF /0)
// ============================================================================

#[test]
fn test_inc_rm64_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC RAX
    // 48 FF C0 = INC RAX
    let code = [0x48, 0xff, 0xc0, 0xf4];
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x123456789ABCDEF1, "INC RAX");
}

#[test]
fn test_inc_rm64_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC RAX when RAX = 0xFFFFFFFFFFFFFFFF -> 0x0000000000000000
    let code = [0x48, 0xff, 0xc0, 0xf4];
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0, "INC RAX: max + 1 = 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_inc_rm64_signed_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC RAX when RAX = 0x7FFFFFFFFFFFFFFF -> 0x8000000000000000
    let code = [0x48, 0xff, 0xc0, 0xf4];
    emu.regs_mut().rax = 0x7FFFFFFFFFFFFFFF;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x8000000000000000);
    assert!(emu.flags().f_of, "OF should be set (signed overflow)");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_inc_rm64_memory() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC QWORD PTR [RBX]
    // 48 FF 03 = INC QWORD PTR [RBX]
    let code = [0x48, 0xff, 0x03, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.load_code_bytes(&code);

    emu.maps.write_qword(DATA_ADDR, 0xFEDCBA9876543210);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_qword(DATA_ADDR).unwrap(), 0xFEDCBA9876543211);
}

// ============================================================================
// Different Registers
// ============================================================================

#[test]
fn test_inc_different_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC ECX
    // FF C1 = INC ECX
    let code = [0xff, 0xc1, 0xf4];
    emu.regs_mut().rcx = 99;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 100, "INC ECX: 99 + 1 = 100");
}

#[test]
fn test_inc_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC CL
    // FE C1 = INC CL
    let code = [0xfe, 0xc1, 0xf4];
    emu.regs_mut().rcx = 255;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFF, 0, "INC CL: 255 + 1 = 0 (wraps)");
}

// ============================================================================
// Flag Tests
// ============================================================================

#[test]
fn test_inc_parity_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC should set parity based on result
    // 0x02 + 1 = 0x03 (binary 00000011, 2 bits = even parity)
    let code = [0xfe, 0xc0, 0xf4];
    emu.regs_mut().rax = 0x02;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0x03);
    assert!(emu.flags().f_pf, "PF should be set (even parity)");
}

#[test]
fn test_inc_auxiliary_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AF should be set when carry from bit 3 to bit 4
    // 0x0F + 1 = 0x10 (carry from bit 3)
    let code = [0xfe, 0xc0, 0xf4];
    emu.regs_mut().rax = 0x0F;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0x10);
    assert!(emu.flags().f_af, "AF should be set (carry from bit 3)");
}

#[test]
fn test_inc_no_auxiliary_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x0E + 1 = 0x0F (no carry from bit 3)
    let code = [0xfe, 0xc0, 0xf4];
    emu.regs_mut().rax = 0x0E;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0x0F);
    assert!(!emu.flags().f_af, "AF should be clear");
}

#[test]
fn test_inc_cf_independence() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xfe, 0xc0, 0xf4];
    emu.regs_mut().rax = 0xFF;
    emu.flags_mut().load(0x2); // CF=0
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(!emu.flags().f_cf, "CF should remain clear");

    let code = [0xfe, 0xc0, 0xf4];
    emu.regs_mut().rax = 0xFF;
    emu.flags_mut().load(0x2 | flags::F_CF); // CF=1
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(emu.flags().f_cf, "CF should remain set");
}

// ============================================================================
// Extended Registers (R8-R15)
// ============================================================================

#[test]
fn test_inc_r8_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC R8D
    // 41 FF C0 = INC R8D
    let code = [0x41, 0xff, 0xc0, 0xf4];
    emu.regs_mut().r8 = 100;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().r8, 101, "INC R8D: 100 + 1 = 101");
}

#[test]
fn test_inc_r15_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC R15
    // 49 FF C7 = INC R15
    let code = [0x49, 0xff, 0xc7, 0xf4];
    emu.regs_mut().r15 = 0xFFFFFFFFFFFFFFFF;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().r15, 0, "INC R15: max + 1 = 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_inc_r8l_byte() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC R8L (low byte of R8)
    // 41 FE C0 = INC R8L
    let code = [0x41, 0xfe, 0xc0, 0xf4];
    emu.regs_mut().r8 = 0xFF;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().r8 & 0xFF, 0, "INC R8L: 0xFF + 1 = 0x00");
}

// ============================================================================
// Loop Counter Use Case
// ============================================================================

#[test]
fn test_inc_as_loop_counter() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC EAX
    // INC EAX
    // INC EAX
    let code = [
        0xff, 0xc0, // INC EAX
        0xff, 0xc0, // INC EAX
        0xff, 0xc0, // INC EAX
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 3, "Three INC operations: 0 + 1 + 1 + 1 = 3");
}

#[test]
fn test_inc_preserves_high_bytes() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xfe, 0xc0, 0xf4];
    emu.regs_mut().rax = 0xDEADBEEF_12345678;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0x79);
    assert_eq!(emu.regs().rax & !0xFF, 0xDEADBEEF_12345600, "High bytes should be preserved");
}
