use crate::*;

// BTR - Bit Test and Reset
// Tests a bit in a bit string (first operand) and stores the value in CF flag,
// then resets (clears) the bit in the bit string.
// The bit string is a sequence of bits in memory or a register.
// The bit position is specified by the second operand (immediate or register).
// Only CF flag is affected; other flags are undefined.
//
// Opcodes:
// 0F B3 /r       BTR r/m16, r16     - Test and reset bit in r/m16
// 0F B3 /r       BTR r/m32, r32     - Test and reset bit in r/m32
// REX.W 0F B3 /r BTR r/m64, r64     - Test and reset bit in r/m64
// 0F BA /6 ib    BTR r/m16, imm8    - Test and reset bit in r/m16
// 0F BA /6 ib    BTR r/m32, imm8    - Test and reset bit in r/m32
// REX.W 0F BA /6 ib BTR r/m64, imm8 - Test and reset bit in r/m64

#[test]
fn test_btr_ax_bx_bit_0_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR AX, BX - test and reset bit 0 (initially set)
    let code = [
        0x66, 0x0f, 0xb3, 0xd8, // BTR AX, BX
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0000_0000_0001; // bit 0 set
    emu.regs_mut().rbx = 0; // test bit 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 0 was 1)");
    assert_eq!(emu.regs().rax & 0xFFFF, 0b0000_0000_0000_0000, "AX: bit 0 should be reset to 0");
}

#[test]
fn test_btr_ax_bx_bit_0_clear() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR AX, BX - test and reset bit 0 (initially clear)
    let code = [
        0x66, 0x0f, 0xb3, 0xd8, // BTR AX, BX
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0000_0000_0000; // bit 0 clear
    emu.regs_mut().rbx = 0; // test bit 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 0 was 0)");
    assert_eq!(emu.regs().rax & 0xFFFF, 0b0000_0000_0000_0000, "AX: bit 0 should remain 0");
}

#[test]
fn test_btr_ax_bx_bit_15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR AX, BX - test and reset MSB (bit 15)
    let code = [
        0x66, 0x0f, 0xb3, 0xd8, // BTR AX, BX
        0xf4,
    ];
    emu.regs_mut().rax = 0x8000; // bit 15 set
    emu.regs_mut().rbx = 15; // test bit 15
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 15 was 1)");
    assert_eq!(emu.regs().rax & 0xFFFF, 0x0000, "AX: bit 15 should be reset to 0");
}

#[test]
fn test_btr_eax_ebx_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR EAX, EBX - test and reset bit 0 (32-bit)
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0001;
    emu.regs_mut().rbx = 0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 0 was 1)");
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0b0000_0000, "EAX: bit 0 should be reset to 0");
}

#[test]
fn test_btr_eax_ebx_bit_31() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR EAX, EBX - test and reset MSB (bit 31)
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0x80000000; // bit 31 set
    emu.regs_mut().rbx = 31; // test bit 31
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 31 was 1)");
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000000, "EAX: bit 31 should be reset to 0");
}

#[test]
fn test_btr_rax_rbx_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR RAX, RBX - test and reset bit 0 (64-bit)
    let code = [
        0x48, 0x0f, 0xb3, 0xd8, // BTR RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0001;
    emu.regs_mut().rbx = 0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 0 was 1)");
    assert_eq!(emu.regs().rax, 0b0000_0000, "RAX: bit 0 should be reset to 0");
}

#[test]
fn test_btr_rax_rbx_bit_63() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR RAX, RBX - test and reset MSB (bit 63)
    let code = [
        0x48, 0x0f, 0xb3, 0xd8, // BTR RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rax = 0x8000_0000_0000_0000; // bit 63 set
    emu.regs_mut().rbx = 63; // test bit 63
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 63 was 1)");
    assert_eq!(emu.regs().rax, 0x0000_0000_0000_0000, "RAX: bit 63 should be reset to 0");
}

#[test]
fn test_btr_ax_imm8_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR AX, imm8 - test and reset bit 0
    let code = [
        0x66, 0x0f, 0xba, 0xf0, 0x00, // BTR AX, 0
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0000_0000_0001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 0 was 1)");
    assert_eq!(emu.regs().rax & 0xFFFF, 0b0000_0000_0000_0000, "AX: bit 0 should be reset to 0");
}

#[test]
fn test_btr_ax_imm8_bit_15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR AX, imm8 - test and reset bit 15
    let code = [
        0x66, 0x0f, 0xba, 0xf0, 0x0f, // BTR AX, 15
        0xf4,
    ];
    emu.regs_mut().rax = 0x8000; // bit 15 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 15 was 1)");
    assert_eq!(emu.regs().rax & 0xFFFF, 0x0000, "AX: bit 15 should be reset to 0");
}

#[test]
fn test_btr_eax_imm8_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR EAX, imm8 - test and reset bit 0
    let code = [
        0x0f, 0xba, 0xf0, 0x00, // BTR EAX, 0
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 0 was 1)");
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0b0000_0000, "EAX: bit 0 should be reset to 0");
}

#[test]
fn test_btr_eax_imm8_bit_31() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR EAX, imm8 - test and reset bit 31
    let code = [
        0x0f, 0xba, 0xf0, 0x1f, // BTR EAX, 31
        0xf4,
    ];
    emu.regs_mut().rax = 0x80000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 31 was 1)");
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000000, "EAX: bit 31 should be reset to 0");
}

#[test]
fn test_btr_rax_imm8_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR RAX, imm8 - test and reset bit 0
    let code = [
        0x48, 0x0f, 0xba, 0xf0, 0x00, // BTR RAX, 0
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 0 was 1)");
    assert_eq!(emu.regs().rax, 0b0000_0000, "RAX: bit 0 should be reset to 0");
}

#[test]
fn test_btr_rax_imm8_bit_63() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR RAX, imm8 - test and reset bit 63
    let code = [
        0x48, 0x0f, 0xba, 0xf0, 0x3f, // BTR RAX, 63
        0xf4,
    ];
    emu.regs_mut().rax = 0x8000_0000_0000_0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 63 was 1)");
    assert_eq!(emu.regs().rax, 0x0000_0000_0000_0000, "RAX: bit 63 should be reset to 0");
}

#[test]
fn test_btr_idempotent() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX (again)
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 5;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let expected = 0x12345678 & !(1 << 5);
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "EAX: double reset should have same result");
}

#[test]
fn test_btr_alternating_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0xAAAAAAAA; // 1010...1010
    emu.regs_mut().rbx = 1; // reset bit 1 (currently 1)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 1 was 1)");
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xAAAAAAAA & !0x2, "EAX: bit 1 should be reset");
}

#[test]
fn test_btr_preserves_other_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR should only modify the specified bit
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.regs_mut().rbx = 5; // reset bit 5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF & !(1 << 5), "EAX: only bit 5 should change");
}

#[test]
fn test_btr_with_extended_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR R8D, R9D - test with extended registers
    let code = [
        0x45, 0x0f, 0xb3, 0xc8, // BTR R8D, R9D
        0xf4,
    ];
    emu.regs_mut().r8 = 0b0000_0001;
    emu.regs_mut().r9 = 0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 0 was 1)");
    assert_eq!(emu.regs().r8 & 0xFFFFFFFF, 0b0000_0000, "R8D: bit 0 should be reset to 0");
}

#[test]
fn test_btr_r15_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR R15, imm8
    let code = [
        0x49, 0x0f, 0xba, 0xf7, 0x20, // BTR R15, 32
        0xf4,
    ];
    emu.regs_mut().r15 = 0x1_0000_0000; // bit 32 set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 32 was 1)");
    assert_eq!(emu.regs().r15, 0x0, "R15: bit 32 should be reset to 0");
}

#[test]
fn test_btr_mem16_reg() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR [mem], BX
    let code = [
        0x66, 0x0f, 0xb3, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BTR [DATA_ADDR], BX
        0xf4,
    ];
    emu.regs_mut().rbx = 8; // reset bit 8
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0x0100); // bit 8 set
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 8 was 1)");
    let result = emu.maps.read_word(DATA_ADDR).unwrap();
    assert_eq!(result, 0x0000, "Memory: bit 8 should be reset to 0");
}

#[test]
fn test_btr_mem32_reg() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR [mem], EBX
    let code = [
        0x0f, 0xb3, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BTR [DATA_ADDR], EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 16; // reset bit 16
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x00010000); // bit 16 set
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 16 was 1)");
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();
    assert_eq!(result, 0x00000000, "Memory: bit 16 should be reset to 0");
}

#[test]
fn test_btr_mem64_reg() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR [mem], RBX
    let code = [
        0x48, 0x0f, 0xb3, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BTR [DATA_ADDR], RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 40; // reset bit 40
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x100_0000_0000); // bit 40 set
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 40 was 1)");
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();
    assert_eq!(result, 0x000_0000_0000, "Memory: bit 40 should be reset to 0");
}

#[test]
fn test_btr_mem32_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR [mem], imm8
    let code = [
        0x0f, 0xba, 0x34, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0c, // BTR [DATA_ADDR], 12
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x1000); // bit 12 set
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 12 was 1)");
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();
    assert_eq!(result, 0x0000, "Memory: bit 12 should be reset to 0");
}

#[test]
fn test_btr_all_bits_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.regs_mut().rbx = 17; // reset bit 17
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 17 was 1)");
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF & !(1 << 17), "EAX: bit 17 should be clear");
}

#[test]
fn test_btr_all_bits_clear() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000000;
    emu.regs_mut().rbx = 17; // reset bit 17
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 17 was 0)");
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000000, "EAX: should remain zero");
}

#[test]
fn test_btr_bit_position_modulo_16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xb3, 0xd8, // BTR AX, BX
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0000_0000_0001; // bit 0 set
    emu.regs_mut().rbx = 16; // position 16 % 16 = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 0 was 1)");
    assert_eq!(emu.regs().rax & 0xFFFF, 0x0000, "AX: bit 0 should be reset");
}

#[test]
fn test_btr_bit_position_modulo_32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0001; // bit 0 set
    emu.regs_mut().rbx = 32; // position 32 % 32 = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 0 was 1)");
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x0000, "EAX: bit 0 should be reset");
}

#[test]
fn test_btr_bit_position_modulo_64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x0f, 0xb3, 0xd8, // BTR RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0001; // bit 0 set
    emu.regs_mut().rbx = 64; // position 64 % 64 = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 0 was 1)");
    assert_eq!(emu.regs().rax, 0x0000, "RAX: bit 0 should be reset");
}

#[test]
fn test_btr_clears_to_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 1 << 20;
    emu.regs_mut().rbx = 20;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX: should be zero after resetting only bit");
}

#[test]
fn test_btr_multiple_bits_sequential() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX (bit 5)
        0xf4,
    ];
    emu.regs_mut().rax = (1 << 5) | (1 << 10) | (1 << 15);
    emu.regs_mut().rbx = 5;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, (1 << 10) | (1 << 15), "EAX: bit 5 should be clear");

    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1 << 15, "EAX: bits 5 and 10 should be clear");
}

#[test]
fn test_btr_no_effect_on_clear_bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR on already clear bit should not change operand
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0; // bit 0 is clear in 0x12345678
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x12345678, "EAX: should be unchanged");
}

#[test]
fn test_btr_creates_mask() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTR can be used to create bit masks
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.regs_mut().rbx = 8;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF & !(1 << 8), "EAX: creates mask with bit 8 clear");
}

#[test]
fn test_btr_sparse_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0x80000001; // bits 0 and 31 set
    emu.regs_mut().rbx = 0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x80000000, "EAX: bit 0 should be clear, bit 31 remains");
}

#[test]
fn test_btr_high_bit_64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x0f, 0xb3, 0xd8, // BTR RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFF_FFFF_FFFF_FFFF;
    emu.regs_mut().rbx = 59;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFF_FFFF_FFFF_FFFF & !(1u64 << 59), "RAX: bit 59 should be clear");
}
