use crate::*;

// BTS - Bit Test and Set
// Tests a bit in a bit string (first operand) and stores the value in CF flag,
// then sets the bit in the bit string.
// The bit string is a sequence of bits in memory or a register.
// The bit position is specified by the second operand (immediate or register).
// Only CF flag is affected; other flags are undefined.
//
// Opcodes:
// 0F AB /r       BTS r/m16, r16     - Test and set bit in r/m16
// 0F AB /r       BTS r/m32, r32     - Test and set bit in r/m32
// REX.W 0F AB /r BTS r/m64, r64     - Test and set bit in r/m64
// 0F BA /5 ib    BTS r/m16, imm8    - Test and set bit in r/m16
// 0F BA /5 ib    BTS r/m32, imm8    - Test and set bit in r/m32
// REX.W 0F BA /5 ib BTS r/m64, imm8 - Test and set bit in r/m64

#[test]
fn test_bts_ax_bx_bit_0_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS AX, BX - test and set bit 0 (initially set)
    let code = [
        0x66, 0x0f, 0xab, 0xd8, // BTS AX, BX
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0000_0000_0001; // bit 0 set
    emu.regs_mut().rbx = 0; // test bit 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 0 was 1)");
    assert_eq!(emu.regs().rax & 0xFFFF, 0b0000_0000_0000_0001, "AX: bit 0 should remain set");
}

#[test]
fn test_bts_ax_bx_bit_0_clear() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS AX, BX - test and set bit 0 (initially clear)
    let code = [
        0x66, 0x0f, 0xab, 0xd8, // BTS AX, BX
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0000_0000_0000; // bit 0 clear
    emu.regs_mut().rbx = 0; // test bit 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 0 was 0)");
    assert_eq!(emu.regs().rax & 0xFFFF, 0b0000_0000_0000_0001, "AX: bit 0 should be set to 1");
}

#[test]
fn test_bts_ax_bx_bit_15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS AX, BX - test and set MSB (bit 15)
    let code = [
        0x66, 0x0f, 0xab, 0xd8, // BTS AX, BX
        0xf4,
    ];
    emu.regs_mut().rax = 0x0000; // bit 15 clear
    emu.regs_mut().rbx = 15; // test bit 15
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 15 was 0)");
    assert_eq!(emu.regs().rax & 0xFFFF, 0x8000, "AX: bit 15 should be set to 1");
}

#[test]
fn test_bts_eax_ebx_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS EAX, EBX - test and set bit 0 (32-bit)
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0000;
    emu.regs_mut().rbx = 0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 0 was 0)");
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0b0000_0001, "EAX: bit 0 should be set to 1");
}

#[test]
fn test_bts_eax_ebx_bit_31() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS EAX, EBX - test and set MSB (bit 31)
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000000; // bit 31 clear
    emu.regs_mut().rbx = 31; // test bit 31
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 31 was 0)");
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x80000000, "EAX: bit 31 should be set to 1");
}

#[test]
fn test_bts_rax_rbx_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS RAX, RBX - test and set bit 0 (64-bit)
    let code = [
        0x48, 0x0f, 0xab, 0xd8, // BTS RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0000;
    emu.regs_mut().rbx = 0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 0 was 0)");
    assert_eq!(emu.regs().rax, 0b0000_0001, "RAX: bit 0 should be set to 1");
}

#[test]
fn test_bts_rax_rbx_bit_63() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS RAX, RBX - test and set MSB (bit 63)
    let code = [
        0x48, 0x0f, 0xab, 0xd8, // BTS RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rax = 0x0000_0000_0000_0000; // bit 63 clear
    emu.regs_mut().rbx = 63; // test bit 63
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 63 was 0)");
    assert_eq!(emu.regs().rax, 0x8000_0000_0000_0000, "RAX: bit 63 should be set to 1");
}

#[test]
fn test_bts_ax_imm8_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS AX, imm8 - test and set bit 0
    let code = [
        0x66, 0x0f, 0xba, 0xe8, 0x00, // BTS AX, 0
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0000_0000_0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 0 was 0)");
    assert_eq!(emu.regs().rax & 0xFFFF, 0b0000_0000_0000_0001, "AX: bit 0 should be set to 1");
}

#[test]
fn test_bts_ax_imm8_bit_15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS AX, imm8 - test and set bit 15
    let code = [
        0x66, 0x0f, 0xba, 0xe8, 0x0f, // BTS AX, 15
        0xf4,
    ];
    emu.regs_mut().rax = 0x0000; // bit 15 clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 15 was 0)");
    assert_eq!(emu.regs().rax & 0xFFFF, 0x8000, "AX: bit 15 should be set to 1");
}

#[test]
fn test_bts_eax_imm8_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS EAX, imm8 - test and set bit 0
    let code = [
        0x0f, 0xba, 0xe8, 0x00, // BTS EAX, 0
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 0 was 0)");
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0b0000_0001, "EAX: bit 0 should be set to 1");
}

#[test]
fn test_bts_eax_imm8_bit_31() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS EAX, imm8 - test and set bit 31
    let code = [
        0x0f, 0xba, 0xe8, 0x1f, // BTS EAX, 31
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 31 was 0)");
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x80000000, "EAX: bit 31 should be set to 1");
}

#[test]
fn test_bts_rax_imm8_bit_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS RAX, imm8 - test and set bit 0
    let code = [
        0x48, 0x0f, 0xba, 0xe8, 0x00, // BTS RAX, 0
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 0 was 0)");
    assert_eq!(emu.regs().rax, 0b0000_0001, "RAX: bit 0 should be set to 1");
}

#[test]
fn test_bts_rax_imm8_bit_63() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS RAX, imm8 - test and set bit 63
    let code = [
        0x48, 0x0f, 0xba, 0xe8, 0x3f, // BTS RAX, 63
        0xf4,
    ];
    emu.regs_mut().rax = 0x0000_0000_0000_0000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 63 was 0)");
    assert_eq!(emu.regs().rax, 0x8000_0000_0000_0000, "RAX: bit 63 should be set to 1");
}

#[test]
fn test_bts_idempotent() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0x0f, 0xab, 0xd8, // BTS EAX, EBX (again)
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 5;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let expected = 0x12345678 | (1 << 5);
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "EAX: double set should have same result");
}

#[test]
fn test_bts_alternating_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0xAAAAAAAA; // 1010...1010
    emu.regs_mut().rbx = 0; // set bit 0 (currently 0)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 0 was 0)");
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xAAAAAAAA | 0x1, "EAX: bit 0 should be set");
}

#[test]
fn test_bts_preserves_other_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS should only modify the specified bit
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000000;
    emu.regs_mut().rbx = 5; // set bit 5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1 << 5, "EAX: only bit 5 should be set");
}

#[test]
fn test_bts_with_extended_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS R8D, R9D - test with extended registers
    let code = [
        0x45, 0x0f, 0xab, 0xc8, // BTS R8D, R9D
        0xf4,
    ];
    emu.regs_mut().r8 = 0b0000_0000;
    emu.regs_mut().r9 = 0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 0 was 0)");
    assert_eq!(emu.regs().r8 & 0xFFFFFFFF, 0b0000_0001, "R8D: bit 0 should be set to 1");
}

#[test]
fn test_bts_r15_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS R15, imm8
    let code = [
        0x49, 0x0f, 0xba, 0xef, 0x20, // BTS R15, 32
        0xf4,
    ];
    emu.regs_mut().r15 = 0x0; // bit 32 clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 32 was 0)");
    assert_eq!(emu.regs().r15, 0x1_0000_0000, "R15: bit 32 should be set to 1");
}

#[test]
fn test_bts_mem16_reg() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS [mem], BX
    let code = [
        0x66, 0x0f, 0xab, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BTS [DATA_ADDR], BX
        0xf4,
    ];
    emu.regs_mut().rbx = 8; // set bit 8
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0x0000); // bit 8 clear
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 8 was 0)");
    let result = emu.maps.read_word(DATA_ADDR).unwrap();
    assert_eq!(result, 0x0100, "Memory: bit 8 should be set to 1");
}

#[test]
fn test_bts_mem32_reg() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS [mem], EBX
    let code = [
        0x0f, 0xab, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BTS [DATA_ADDR], EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 16; // set bit 16
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x00000000); // bit 16 clear
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 16 was 0)");
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();
    assert_eq!(result, 0x00010000, "Memory: bit 16 should be set to 1");
}

#[test]
fn test_bts_mem64_reg() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS [mem], RBX
    let code = [
        0x48, 0x0f, 0xab, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BTS [DATA_ADDR], RBX
        0xf4,
    ];
    emu.regs_mut().rbx = 40; // set bit 40
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x000_0000_0000); // bit 40 clear
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 40 was 0)");
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();
    assert_eq!(result, 0x100_0000_0000, "Memory: bit 40 should be set to 1");
}

#[test]
fn test_bts_mem32_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS [mem], imm8
    let code = [
        0x0f, 0xba, 0x2c, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0c, // BTS [DATA_ADDR], 12
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x0000); // bit 12 clear
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 12 was 0)");
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();
    assert_eq!(result, 0x1000, "Memory: bit 12 should be set to 1");
}

#[test]
fn test_bts_all_bits_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.regs_mut().rbx = 17; // set bit 17 (already set)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (bit 17 was 1)");
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF, "EAX: should remain all ones");
}

#[test]
fn test_bts_all_bits_clear() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000000;
    emu.regs_mut().rbx = 17; // set bit 17
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 17 was 0)");
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1 << 17, "EAX: bit 17 should be set");
}

#[test]
fn test_bts_bit_position_modulo_16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xab, 0xd8, // BTS AX, BX
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0000_0000_0000;
    emu.regs_mut().rbx = 16; // position 16 % 16 = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 0 was 0)");
    assert_eq!(emu.regs().rax & 0xFFFF, 0x0001, "AX: bit 0 should be set");
}

#[test]
fn test_bts_bit_position_modulo_32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0000;
    emu.regs_mut().rbx = 32; // position 32 % 32 = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 0 was 0)");
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x0001, "EAX: bit 0 should be set");
}

#[test]
fn test_bts_bit_position_modulo_64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x0f, 0xab, 0xd8, // BTS RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rax = 0b0000_0000;
    emu.regs_mut().rbx = 64; // position 64 % 64 = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (bit 0 was 0)");
    assert_eq!(emu.regs().rax, 0x0001, "RAX: bit 0 should be set");
}

#[test]
fn test_bts_creates_single_bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0;
    emu.regs_mut().rbx = 20;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1 << 20, "EAX: only bit 20 should be set");
}

#[test]
fn test_bts_multiple_bits_sequential() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX (bit 5)
        0xf4,
    ];
    emu.regs_mut().rax = 0;
    emu.regs_mut().rbx = 5;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1 << 5, "EAX: bit 5 should be set");

    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rbx = 10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, (1 << 5) | (1 << 10), "EAX: bits 5 and 10 should be set");
}

#[test]
fn test_bts_no_effect_on_set_bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS on already set bit should not change operand
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345679; // bit 0 is set
    emu.regs_mut().rbx = 0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x12345679, "EAX: should be unchanged");
}

#[test]
fn test_bts_creates_mask() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // BTS can be used to create bit masks
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000000;
    emu.regs_mut().rbx = 8;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1 << 8, "EAX: creates mask with bit 8 set");
}

#[test]
fn test_bts_sparse_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0x80000000; // only bit 31 set
    emu.regs_mut().rbx = 0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x80000001, "EAX: bits 0 and 31 should be set");
}

#[test]
fn test_bts_high_bit_64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x0f, 0xab, 0xd8, // BTS RAX, RBX
        0xf4,
    ];
    emu.regs_mut().rax = 0x0000_0000_0000_0000;
    emu.regs_mut().rbx = 59;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 1u64 << 59, "RAX: bit 59 should be set");
}

#[test]
fn test_bts_build_bitmask() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];

    let mut result = 0u32;
    for bit_pos in [0, 4, 8, 12, 16, 20, 24, 28] {
        emu.regs_mut().rax = result as u64;
        emu.regs_mut().rbx = bit_pos;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();
        result = (emu.regs().rax & 0xFFFFFFFF) as u32;
    }

    assert_eq!(result, 0x11111111, "EAX: should have pattern 0x11111111");
}
