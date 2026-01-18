// SHR (Shift Logical Right) instruction tests
//
// Opcodes:
// D0 /5       SHR r/m8, 1
// D2 /5       SHR r/m8, CL
// C0 /5 ib    SHR r/m8, imm8
// D1 /5       SHR r/m16, 1
// D3 /5       SHR r/m16, CL
// C1 /5 ib    SHR r/m16, imm8
// D1 /5       SHR r/m32, 1
// D3 /5       SHR r/m32, CL
// C1 /5 ib    SHR r/m32, imm8
// REX.W + D1 /5    SHR r/m64, 1
// REX.W + D3 /5    SHR r/m64, CL
// REX.W + C1 /5 ib SHR r/m64, imm8
//
// SHR performs unsigned division by powers of 2
// Fills empty bit positions with zeros (unlike SAR which fills with sign bit)
//
// Flags:
// - CF: Last bit shifted out
// - OF: MSB of original operand (only for 1-bit shifts)
// - SF, ZF, PF: Set according to result
// - AF: Undefined for non-zero count
// - Count is 0: No flags affected

use crate::*;

// ============================================================================
// 8-bit SHR tests
// ============================================================================

#[test]
fn test_shr_al_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR AL, 1 (opcode D0 /5)
    let code = [
        0xd0, 0xe8, // SHR AL, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x42; // 0100_0010
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x21, "AL: 0x42 >> 1 = 0x21");
    assert!(!emu.flags().f_cf, "CF should be clear (LSB was 0)");
    assert!(!emu.flags().f_of, "OF should be clear (MSB of original was 0)");
    assert!(!emu.flags().f_sf, "SF should be clear");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_shr_al_1_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR AL, 1 with LSB set
    let code = [
        0xd0, 0xe8, // SHR AL, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x43; // 0100_0011
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x21, "AL: 0x43 >> 1 = 0x21");
    assert!(emu.flags().f_cf, "CF should be set (LSB was 1)");
    assert!(!emu.flags().f_of, "OF: MSB of original was 0");
}

#[test]
fn test_shr_al_1_msb_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR with MSB set (tests OF flag for 1-bit shift)
    let code = [
        0xd0, 0xe8, // SHR AL, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x80; // 1000_0000
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x40, "AL: 0x80 >> 1 = 0x40");
    assert!(!emu.flags().f_cf, "CF should be clear (LSB was 0)");
    assert!(emu.flags().f_of, "OF should be set (MSB of original was 1)");
}

#[test]
fn test_shr_al_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR AL, CL (opcode D2 /5)
    let code = [
        0xd2, 0xe8, // SHR AL, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x80; // 1000_0000
    emu.regs_mut().rcx = 0x07; // Shift by 7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x01, "AL: 0x80 >> 7 = 0x01");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
    assert!(!emu.flags().f_sf, "SF should be clear");
}

#[test]
fn test_shr_al_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR AL, imm8 (opcode C0 /5 ib)
    let code = [
        0xc0, 0xe8, 0x03, // SHR AL, 3
        0xf4,
    ];
    emu.regs_mut().rax = 0x88; // 1000_1000
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x11, "AL: 0x88 >> 3 = 0x11");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
    assert!(!emu.flags().f_sf, "SF should be clear");
}

#[test]
fn test_shr_al_to_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc0, 0xe8, 0x08, // SHR AL, 8
        0xf4,
    ];
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL: all bits shifted out");
    assert!(emu.flags().f_zf, "ZF should be set (result is zero)");
    assert!(!emu.flags().f_sf, "SF should be clear");
}

#[test]
fn test_shr_count_masked_8bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd2, 0xe8, // SHR AL, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x88;
    emu.regs_mut().rcx = 0x23; // 35 & 0x1F = 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x11, "AL: 0x88 >> 3 = 0x11 (count masked)");
}

#[test]
fn test_shr_count_zero_preserves_flags() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc0, 0xe8, 0x00, // SHR AL, 0
        0xf4,
    ];
    emu.regs_mut().rax = 0x42;
    emu.flags_mut().load(0x2 | flags::F_CF | flags::F_ZF | flags::F_OF);
    let initial_flags = emu.flags().dump();
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL unchanged");
    assert_eq!(emu.flags().dump(), initial_flags, "Flags unchanged when count is 0");
}

// ============================================================================
// 16-bit SHR tests
// ============================================================================

#[test]
fn test_shr_ax_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR AX, 1 (opcode 66 D1 /5)
    let code = [
        0x66, 0xd1, 0xe8, // SHR AX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x4321;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x2190, "AX: 0x4321 >> 1 = 0x2190");
    assert!(emu.flags().f_cf, "CF should be set (LSB was 1)");
    assert!(!emu.flags().f_of, "OF: MSB of original was 0");
}

#[test]
fn test_shr_ax_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR AX, CL (opcode 66 D3 /5)
    let code = [
        0x66, 0xd3, 0xe8, // SHR AX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x8000;
    emu.regs_mut().rcx = 0x0F; // Shift by 15
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0001, "AX: 0x8000 >> 15 = 0x0001");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
}

#[test]
fn test_shr_ax_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR AX, imm8 (opcode 66 C1 /5 ib)
    let code = [
        0x66, 0xc1, 0xe8, 0x04, // SHR AX, 4
        0xf4,
    ];
    emu.regs_mut().rax = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0123, "AX: 0x1234 >> 4 = 0x0123");
    // CF = bit 3 of 0x1234 = 0 (last bit shifted out)
    assert!(!emu.flags().f_cf, "CF should be clear (bit 3 of 0x1234 was 0)");
}

#[test]
fn test_shr_ax_with_msb() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0xd1, 0xe8, // SHR AX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x8000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x4000, "AX: 0x8000 >> 1 = 0x4000 (zero fill)");
    assert!(!emu.flags().f_cf, "CF should be clear (LSB was 0)");
    assert!(emu.flags().f_of, "OF should be set (MSB of original was 1)");
}

// ============================================================================
// 32-bit SHR tests
// ============================================================================

#[test]
fn test_shr_eax_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR EAX, 1 (opcode D1 /5)
    let code = [
        0xd1, 0xe8, // SHR EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x091A2B3C, "EAX: 0x12345678 >> 1 = 0x091A2B3C");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_of, "OF should be clear");
}

#[test]
fn test_shr_eax_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR EAX, CL (opcode D3 /5)
    let code = [
        0xd3, 0xe8, // SHR EAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x80000000;
    emu.regs_mut().rcx = 0x1F; // Shift by 31
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000001, "EAX: 0x80000000 >> 31 = 0x00000001");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
}

#[test]
fn test_shr_eax_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR EAX, imm8 (opcode C1 /5 ib)
    let code = [
        0xc1, 0xe8, 0x08, // SHR EAX, 8
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00123456, "EAX: 0x12345678 >> 8 = 0x00123456");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_shr_eax_with_msb() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR fills with zeros (unlike SAR)
    let code = [
        0xd1, 0xe8, // SHR EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x80000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x40000000, "EAX: 0x80000000 >> 1 = 0x40000000 (zero fill)");
    assert!(!emu.flags().f_cf, "CF should be clear (LSB was 0)");
    assert!(emu.flags().f_of, "OF should be set (MSB of original was 1)");
}

#[test]
fn test_shr_count_masked_32bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd3, 0xe8, // SHR EAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x80000000;
    emu.regs_mut().rcx = 0x3F; // 63 & 0x1F = 31
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000001, "EAX: 0x80000000 >> 31 (count masked)");
}

// ============================================================================
// 64-bit SHR tests
// ============================================================================

#[test]
fn test_shr_rax_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR RAX, 1 (opcode 48 D1 /5)
    let code = [
        0x48, 0xd1, 0xe8, // SHR RAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x091A2B3C4D5E6F78, "RAX: 0x123456789ABCDEF0 >> 1");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_of, "OF should be clear");
}

#[test]
fn test_shr_rax_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR RAX, CL (opcode 48 D3 /5)
    let code = [
        0x48, 0xd3, 0xe8, // SHR RAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x8000000000000000;
    emu.regs_mut().rcx = 0x3F; // Shift by 63
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000001, "RAX: 0x8000000000000000 >> 63");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
}

#[test]
fn test_shr_rax_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR RAX, imm8 (opcode 48 C1 /5 ib)
    let code = [
        0x48, 0xc1, 0xe8, 0x20, // SHR RAX, 32
        0xf4,
    ];
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000012345678, "RAX: high 32 bits shifted to low 32");
    // CF = bit 31 of original = MSB of 0x9ABCDEF0 = 1
    assert!(emu.flags().f_cf, "CF should be set (bit 31 of 0x9ABCDEF0 is 1)");
}

#[test]
fn test_shr_rax_with_msb() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR fills with zeros
    let code = [
        0x48, 0xd1, 0xe8, // SHR RAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x8000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x4000000000000000, "RAX: 0x8000000000000000 >> 1 (zero fill)");
    assert!(!emu.flags().f_cf, "CF should be clear (LSB was 0)");
    assert!(emu.flags().f_of, "OF should be set (MSB of original was 1)");
}

#[test]
fn test_shr_count_masked_64bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xd3, 0xe8, // SHR RAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x8000000000000000;
    emu.regs_mut().rcx = 0x7F; // 127 & 0x3F = 63
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000001, "RAX: 0x8000000000000000 >> 63 (count masked to 6 bits)");
}

// ============================================================================
// Extended register tests (R8-R15)
// ============================================================================

#[test]
fn test_shr_r8b_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR R8B, 1
    let code = [
        0x41, 0xd0, 0xe8, // SHR R8B, 1
        0xf4,
    ];
    emu.regs_mut().r8 = 0xAA; // 1010_1010
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0x55, "R8B: 0xAA >> 1 = 0x55");
    assert!(!emu.flags().f_cf, "CF should be clear (LSB was 0)");
}

#[test]
fn test_shr_r10w_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR R10W, CL
    let code = [
        0x66, 0x41, 0xd3, 0xea, // SHR R10W, CL
        0xf4,
    ];
    emu.regs_mut().r10 = 0x1234;
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10 & 0xFFFF, 0x0123, "R10W: 0x1234 >> 4 = 0x0123");
}

#[test]
fn test_shr_r12d_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR R12D, imm8
    let code = [
        0x41, 0xc1, 0xec, 0x08, // SHR R12D, 8
        0xf4,
    ];
    emu.regs_mut().r12 = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r12 & 0xFFFFFFFF, 0x00123456, "R12D: 0x12345678 >> 8 = 0x00123456");
}

#[test]
fn test_shr_r15_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR R15, 1
    let code = [
        0x49, 0xd1, 0xef, // SHR R15, 1
        0xf4,
    ];
    emu.regs_mut().r15 = 0xFEDCBA9876543210;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 0x7F6E5D4C3B2A1908, "R15: logical right shift by 1 (zero fill)");
    assert!(!emu.flags().f_sf, "SF should be clear (result < 2^63)");
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_shr_byte_ptr_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR byte ptr [DATA_ADDR], 1
    let code = [
        0xd0, 0x2c, 0x25, // SHR byte ptr [DATA_ADDR], 1
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0x82);

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0x41, "Memory: 0x82 >> 1 = 0x41 (zero fill)");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_shr_word_ptr_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR word ptr [DATA_ADDR], CL
    let code = [
        0x66, 0xd3, 0x2c, 0x25, // SHR word ptr [DATA_ADDR], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    emu.regs_mut().rcx = 0x08;
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0xF000);

    emu.run(None).unwrap();
    let result = emu.maps.read_word(DATA_ADDR).unwrap();

    assert_eq!(result, 0x00F0, "Memory: 0xF000 >> 8 = 0x00F0 (zero fill)");
}

#[test]
fn test_shr_dword_ptr_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR dword ptr [DATA_ADDR], imm8
    let code = [
        0xc1, 0x2c, 0x25, // SHR dword ptr [DATA_ADDR], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x10, // imm8 = 16
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x80000000);

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x00008000, "Memory: 0x80000000 >> 16 = 0x00008000");
}

#[test]
fn test_shr_qword_ptr_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR qword ptr [DATA_ADDR], CL
    let code = [
        0x48, 0xd3, 0x2c, 0x25, // SHR qword ptr [DATA_ADDR], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    emu.regs_mut().rcx = 0x20;
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0xFFFFFFFF00000000);

    emu.run(None).unwrap();
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x00000000FFFFFFFF, "Memory: 0xFFFFFFFF00000000 >> 32");
}

// ============================================================================
// Practical use cases and edge cases
// ============================================================================

#[test]
fn test_shr_unsigned_divide_by_power_of_2() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR can divide unsigned numbers by powers of 2
    // 100 / 4 = 25
    let code = [
        0xc1, 0xe8, 0x02, // SHR EAX, 2 (divide by 4)
        0xf4,
    ];
    emu.regs_mut().rax = 100;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 25, "EAX: 100 / 4 = 25");
    assert!(!emu.flags().f_sf, "SF: result is positive");
}

#[test]
fn test_shr_vs_sar_negative_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHR treats operand as unsigned, SAR as signed
    // 0xFFFFFFFF >> 1 = 0x7FFFFFFF (SHR)
    // 0xFFFFFFFF >> 1 = 0xFFFFFFFF (SAR would do this)
    let code = [
        0xd1, 0xe8, // SHR EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x7FFFFFFF, "EAX: SHR fills with zero (not sign)");
    assert!(emu.flags().f_cf, "CF: LSB was 1");
    assert!(!emu.flags().f_sf, "SF: result is positive");
}

#[test]
fn test_shr_extract_high_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xc1, 0xe8, 0x20, // SHR RAX, 32
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFF00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x00000000FFFFFFFF, "RAX: high 32 bits moved to low");
}

#[test]
fn test_shr_overflow_flag_1bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // OF is set to MSB of original operand for 1-bit shifts
    let code = [
        0xd1, 0xe8, // SHR EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x40000000; // MSB = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_of, "OF: MSB of original was 0");

    let code2 = [
        0xd1, 0xe8, // SHR EAX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x80000000; // MSB = 1
    emu.load_code_bytes(&code2);
    emu.run(None).unwrap();

    assert!(emu.flags().f_of, "OF: MSB of original was 1");
}

#[test]
fn test_shr_parity_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PF is set based on low byte parity
    let code = [
        0xd1, 0xe8, // SHR EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x06; // 0000_0110
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x03, "EAX: 0x06 >> 1 = 0x03");
    // 0x03 = 0000_0011, two 1 bits (even), so PF should be set
    assert!(emu.flags().f_pf, "PF should be set (even parity)");
}

#[test]
fn test_shr_chained_shifts() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd1, 0xe8, // SHR EAX, 1
        0xd1, 0xe8, // SHR EAX, 1
        0xd1, 0xe8, // SHR EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0xF8000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x1F000000, "EAX: 0xF8000000 >> 3 = 0x1F000000");
}

#[test]
fn test_shr_all_ones() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd1, 0xe8, // SHR EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x7FFFFFFF, "EAX: 0xFFFFFFFF >> 1 = 0x7FFFFFFF");
    assert!(emu.flags().f_cf, "CF: LSB was 1");
    assert!(!emu.flags().f_sf, "SF: result is positive (MSB = 0)");
    assert!(!emu.flags().f_zf, "ZF: result is not zero");
}

#[test]
fn test_shr_isolate_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc1, 0xe8, 0x08, // SHR EAX, 8
        0x25, 0xFF, 0x00, 0x00, 0x00, // AND EAX, 0xFF
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x56, "EAX: extracted byte at bits 8-15");
}
