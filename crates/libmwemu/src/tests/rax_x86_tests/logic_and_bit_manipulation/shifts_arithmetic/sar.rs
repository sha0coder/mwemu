// SAR (Shift Arithmetic Right) instruction tests
//
// Opcodes:
// D0 /7       SAR r/m8, 1
// D2 /7       SAR r/m8, CL
// C0 /7 ib    SAR r/m8, imm8
// D1 /7       SAR r/m16, 1
// D3 /7       SAR r/m16, CL
// C1 /7 ib    SAR r/m16, imm8
// D1 /7       SAR r/m32, 1
// D3 /7       SAR r/m32, CL
// C1 /7 ib    SAR r/m32, imm8
// REX.W + D1 /7    SAR r/m64, 1
// REX.W + D3 /7    SAR r/m64, CL
// REX.W + C1 /7 ib SAR r/m64, imm8
//
// SAR performs signed division by powers of 2
// Fills empty bit positions with the sign bit (MSB)
// Rounding is toward negative infinity (not the same as IDIV)
//
// Flags:
// - CF: Last bit shifted out
// - OF: Cleared for all 1-bit shifts
// - SF, ZF, PF: Set according to result
// - AF: Undefined for non-zero count
// - Count is 0: No flags affected


use crate::*;

// ============================================================================
// 8-bit SAR tests
// ============================================================================

#[test]
fn test_sar_al_1_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR AL, 1 with positive number (opcode D0 /7)
    let code = [
        0xd0, 0xf8, // SAR AL, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x42; // 0100_0010 (positive)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x21, "AL: 0x42 >> 1 = 0x21 (sign extended)");
    assert!(!emu.flags().f_cf, "CF: LSB shifted out was 0");
    assert!(!emu.flags().f_of, "OF: always cleared for 1-bit SAR");
    assert!(!emu.flags().f_sf, "SF: result is positive");
    assert!(!emu.flags().f_zf, "ZF: result is not zero");
}

#[test]
fn test_sar_al_1_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR AL, 1 with negative number (sign bit set)
    let code = [
        0xd0, 0xf8, // SAR AL, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x82; // 1000_0010 (negative in signed interpretation)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xC1, "AL: 0x82 >> 1 = 0xC1 (sign bit extended)");
    assert!(!emu.flags().f_cf, "CF: LSB shifted out was 0");
    assert!(!emu.flags().f_of, "OF: always cleared for 1-bit SAR");
    assert!(emu.flags().f_sf, "SF: result is still negative");
}

#[test]
fn test_sar_al_1_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR with LSB set
    let code = [
        0xd0, 0xf8, // SAR AL, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x43; // 0100_0011
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x21, "AL: 0x43 >> 1 = 0x21");
    assert!(emu.flags().f_cf, "CF: LSB shifted out was 1");
    assert!(!emu.flags().f_of, "OF: cleared for 1-bit shifts");
}

#[test]
fn test_sar_al_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR AL, CL (opcode D2 /7)
    let code = [
        0xd2, 0xf8, // SAR AL, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x80; // 1000_0000 (most negative 8-bit value)
    emu.regs_mut().rcx = 0x07; // Shift by 7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "AL: 0x80 >> 7 = 0xFF (all bits set by sign extension)");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out");
    assert!(emu.flags().f_sf, "SF: result is negative");
}

#[test]
fn test_sar_al_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR AL, imm8 (opcode C0 /7 ib)
    let code = [
        0xc0, 0xf8, 0x03, // SAR AL, 3
        0xf4,
    ];
    emu.regs_mut().rax = 0x88; // 1000_1000
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xF1, "AL: 0x88 >> 3 = 0xF1 (sign extended)");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
    assert!(emu.flags().f_sf, "SF: result is negative");
}

#[test]
fn test_sar_signed_division_by_2() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR divides by 2 (signed)
    // -8 / 2 = -4
    let code = [
        0xd0, 0xf8, // SAR AL, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0xF8; // -8 in 8-bit two's complement
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFC, "AL: -8 / 2 = -4 (0xFC)");
    assert!(!emu.flags().f_cf, "CF: LSB was 0");
}

#[test]
fn test_sar_count_masked_8bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd2, 0xf8, // SAR AL, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x88;
    emu.regs_mut().rcx = 0x23; // 35 & 0x1F = 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xF1, "AL: 0x88 >> 3 = 0xF1 (count masked)");
}

#[test]
fn test_sar_count_zero_preserves_flags() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc0, 0xf8, 0x00, // SAR AL, 0
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
// 16-bit SAR tests
// ============================================================================

#[test]
fn test_sar_ax_1_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR AX, 1 with positive number (opcode 66 D1 /7)
    let code = [
        0x66, 0xd1, 0xf8, // SAR AX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x4321;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x2190, "AX: 0x4321 >> 1 = 0x2190");
    assert!(emu.flags().f_cf, "CF: LSB was 1");
    assert!(!emu.flags().f_of, "OF: cleared for 1-bit SAR");
}

#[test]
fn test_sar_ax_1_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR AX, 1 with negative number
    let code = [
        0x66, 0xd1, 0xf8, // SAR AX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x8000; // Most negative 16-bit value
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xC000, "AX: 0x8000 >> 1 = 0xC000 (sign extended)");
    assert!(!emu.flags().f_cf, "CF: LSB was 0");
    assert!(emu.flags().f_sf, "SF: result is negative");
}

#[test]
fn test_sar_ax_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR AX, CL (opcode 66 D3 /7)
    let code = [
        0x66, 0xd3, 0xf8, // SAR AX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0xFF00; // Negative
    emu.regs_mut().rcx = 0x08;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xFFFF, "AX: 0xFF00 >> 8 = 0xFFFF (sign extended)");
    assert!(emu.flags().f_sf, "SF: result is negative");
}

#[test]
fn test_sar_ax_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR AX, imm8 (opcode 66 C1 /7 ib)
    let code = [
        0x66, 0xc1, 0xf8, 0x04, // SAR AX, 4
        0xf4,
    ];
    emu.regs_mut().rax = 0x1230;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0123, "AX: 0x1230 >> 4 = 0x0123");
    assert!(!emu.flags().f_sf, "SF: result is positive");
}

// ============================================================================
// 32-bit SAR tests
// ============================================================================

#[test]
fn test_sar_eax_1_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR EAX, 1 with positive number (opcode D1 /7)
    let code = [
        0xd1, 0xf8, // SAR EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x091A2B3C, "EAX: 0x12345678 >> 1 = 0x091A2B3C");
    assert!(!emu.flags().f_cf, "CF: LSB was 0");
    assert!(!emu.flags().f_of, "OF: cleared for 1-bit SAR");
}

#[test]
fn test_sar_eax_1_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR EAX, 1 with negative number
    let code = [
        0xd1, 0xf8, // SAR EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x80000000; // Most negative 32-bit value
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xC0000000, "EAX: 0x80000000 >> 1 = 0xC0000000");
    assert!(!emu.flags().f_cf, "CF: LSB was 0");
    assert!(emu.flags().f_sf, "SF: result is still negative");
}

#[test]
fn test_sar_eax_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR EAX, CL (opcode D3 /7)
    let code = [
        0xd3, 0xf8, // SAR EAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFF0000; // Negative
    emu.regs_mut().rcx = 0x10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF, "EAX: 0xFFFF0000 >> 16 = 0xFFFFFFFF");
    assert!(emu.flags().f_sf, "SF: result is negative");
}

#[test]
fn test_sar_eax_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR EAX, imm8 (opcode C1 /7 ib)
    let code = [
        0xc1, 0xf8, 0x08, // SAR EAX, 8
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345600;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00123456, "EAX: 0x12345600 >> 8 = 0x00123456");
    assert!(!emu.flags().f_sf, "SF: result is positive");
}

#[test]
fn test_sar_count_masked_32bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd3, 0xf8, // SAR EAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x80000000;
    emu.regs_mut().rcx = 0x3F; // 63 & 0x1F = 31
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF, "EAX: 0x80000000 >> 31 = 0xFFFFFFFF (all ones)");
}

// ============================================================================
// 64-bit SAR tests
// ============================================================================

#[test]
fn test_sar_rax_1_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR RAX, 1 with positive number (opcode 48 D1 /7)
    let code = [
        0x48, 0xd1, 0xf8, // SAR RAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x091A2B3C4D5E6F78, "RAX: 0x123456789ABCDEF0 >> 1");
    assert!(!emu.flags().f_cf, "CF: LSB was 0");
    assert!(!emu.flags().f_of, "OF: cleared for 1-bit SAR");
}

#[test]
fn test_sar_rax_1_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR RAX, 1 with negative number
    let code = [
        0x48, 0xd1, 0xf8, // SAR RAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x8000000000000000; // Most negative 64-bit value
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xC000000000000000, "RAX: 0x8000000000000000 >> 1 = 0xC000000000000000");
    assert!(!emu.flags().f_cf, "CF: LSB was 0");
    assert!(emu.flags().f_sf, "SF: result is still negative");
}

#[test]
fn test_sar_rax_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR RAX, CL (opcode 48 D3 /7)
    let code = [
        0x48, 0xd3, 0xf8, // SAR RAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFFFFFF0000; // Negative
    emu.regs_mut().rcx = 0x10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "RAX: 0xFFFFFFFFFFFF0000 >> 16 = 0xFFFFFFFFFFFFFFFF");
    assert!(emu.flags().f_sf, "SF: result is negative");
}

#[test]
fn test_sar_rax_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR RAX, imm8 (opcode 48 C1 /7 ib)
    let code = [
        0x48, 0xc1, 0xf8, 0x20, // SAR RAX, 32
        0xf4,
    ];
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000012345678, "RAX: high 32 bits shifted to low 32");
    assert!(!emu.flags().f_sf, "SF: result is positive");
}

#[test]
fn test_sar_count_masked_64bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xd3, 0xf8, // SAR RAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x8000000000000000;
    emu.regs_mut().rcx = 0x7F; // 127 & 0x3F = 63
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "RAX: sign extended to all ones");
}

// ============================================================================
// Extended register tests (R8-R15)
// ============================================================================

#[test]
fn test_sar_r8b_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR R8B, 1
    let code = [
        0x41, 0xd0, 0xf8, // SAR R8B, 1
        0xf4,
    ];
    emu.regs_mut().r8 = 0xAA; // 1010_1010 (negative)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0xD5, "R8B: 0xAA >> 1 = 0xD5 (sign extended)");
    assert!(!emu.flags().f_cf, "CF: LSB was 0");
    assert!(emu.flags().f_sf, "SF: result is negative");
}

#[test]
fn test_sar_r10w_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR R10W, CL
    let code = [
        0x66, 0x41, 0xd3, 0xfa, // SAR R10W, CL
        0xf4,
    ];
    emu.regs_mut().r10 = 0xF000; // Negative
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10 & 0xFFFF, 0xFF00, "R10W: 0xF000 >> 4 = 0xFF00 (sign extended)");
}

#[test]
fn test_sar_r12d_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR R12D, imm8
    let code = [
        0x41, 0xc1, 0xfc, 0x08, // SAR R12D, 8
        0xf4,
    ];
    emu.regs_mut().r12 = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r12 & 0xFFFFFFFF, 0x00123456, "R12D: 0x12345678 >> 8 = 0x00123456");
}

#[test]
fn test_sar_r15_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR R15, 1
    let code = [
        0x49, 0xd1, 0xff, // SAR R15, 1
        0xf4,
    ];
    emu.regs_mut().r15 = 0xFEDCBA9876543210; // Negative
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 0xFF6E5D4C3B2A1908, "R15: signed right shift by 1");
    assert!(emu.flags().f_sf, "SF: result is negative");
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_sar_byte_ptr_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR byte ptr [DATA_ADDR], 1
    let code = [
        0xd0, 0x3c, 0x25, // SAR byte ptr [DATA_ADDR], 1
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0x82); // Negative

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0xC1, "Memory: 0x82 >> 1 = 0xC1 (sign extended)");
    assert!(!emu.flags().f_cf, "CF: LSB was 0");
}

#[test]
fn test_sar_word_ptr_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR word ptr [DATA_ADDR], CL
    let code = [
        0x66, 0xd3, 0x3c, 0x25, // SAR word ptr [DATA_ADDR], CL
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

    assert_eq!(result, 0xFFF0, "Memory: 0xF000 >> 8 = 0xFFF0 (sign extended)");
}

#[test]
fn test_sar_dword_ptr_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR dword ptr [DATA_ADDR], imm8
    let code = [
        0xc1, 0x3c, 0x25, // SAR dword ptr [DATA_ADDR], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x10, // imm8 = 16
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x80000000); // Negative

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(result, 0xFFFF8000, "Memory: 0x80000000 >> 16 = 0xFFFF8000");
}

#[test]
fn test_sar_qword_ptr_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR qword ptr [DATA_ADDR], CL
    let code = [
        0x48, 0xd3, 0x3c, 0x25, // SAR qword ptr [DATA_ADDR], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    emu.regs_mut().rcx = 0x20;
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0xFFFFFFFF00000000); // Negative

    emu.run(None).unwrap();
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();

    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "Memory: 0xFFFFFFFF00000000 >> 32 = all ones");
}

// ============================================================================
// Practical use cases and edge cases
// ============================================================================

#[test]
fn test_sar_signed_divide_by_power_of_2() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR can divide signed numbers by powers of 2
    // -16 / 4 = -4
    let code = [
        0xc1, 0xf8, 0x02, // SAR EAX, 2 (divide by 4)
        0xf4,
    ];
    emu.regs_mut().rax = (-16i32) as u32 as u64; // -16
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFFFFFFFF) as i32, -4, "EAX: -16 / 4 = -4");
    assert!(emu.flags().f_sf, "SF: result is negative");
}

#[test]
fn test_sar_rounding_toward_negative_infinity() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR rounds toward negative infinity (unlike IDIV which rounds toward zero)
    // -9 >> 2 = -3 (not -2 like IDIV would give)
    let code = [
        0xc1, 0xf8, 0x02, // SAR EAX, 2
        0xf4,
    ];
    emu.regs_mut().rax = (-9i32) as u32 as u64; // -9
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFFFFFFFF) as i32, -3, "EAX: -9 >> 2 = -3 (rounds toward -∞)");
}

#[test]
fn test_sar_positive_divide() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR divides positive numbers correctly
    // 100 / 4 = 25
    let code = [
        0xc1, 0xf8, 0x02, // SAR EAX, 2
        0xf4,
    ];
    emu.regs_mut().rax = 100;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 25, "EAX: 100 / 4 = 25");
    assert!(!emu.flags().f_sf, "SF: result is positive");
}

#[test]
fn test_sar_all_ones_stays_negative_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd1, 0xf8, // SAR EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFF; // -1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF, "EAX: -1 >> 1 = -1 (stays all ones)");
    assert!(emu.flags().f_cf, "CF: LSB was 1");
    assert!(emu.flags().f_sf, "SF: result is negative");
}

#[test]
fn test_sar_to_zero_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc1, 0xf8, 0x1F, // SAR EAX, 31
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678; // Positive
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX: positive number >> 31 = 0");
    assert!(emu.flags().f_zf, "ZF: result is zero");
    assert!(!emu.flags().f_sf, "SF: result is not negative");
}

#[test]
fn test_sar_to_negative_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc1, 0xf8, 0x1F, // SAR EAX, 31
        0xf4,
    ];
    emu.regs_mut().rax = 0x80000001; // Negative
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF, "EAX: negative number >> 31 = -1 (all ones)");
    assert!(emu.flags().f_sf, "SF: result is negative");
}

#[test]
fn test_sar_extract_sign_bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SAR by width-1 extracts sign bit (0 or -1)
    let code = [
        0xc1, 0xf8, 0x1F, // SAR EAX, 31
        0xf4,
    ];
    emu.regs_mut().rax = 0x7FFFFFFF; // Most positive 32-bit value
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX: positive >> 31 = 0");

    let code2 = [
        0xc1, 0xf8, 0x1F, // SAR EAX, 31
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x80000000; // Most negative
    emu.load_code_bytes(&code2);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF, "EAX: negative >> 31 = -1");
}

#[test]
fn test_sar_parity_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PF is set based on low byte parity
    let code = [
        0xd1, 0xf8, // SAR EAX, 1
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
fn test_sar_chained_shifts() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd1, 0xf8, // SAR EAX, 1
        0xd1, 0xf8, // SAR EAX, 1
        0xd1, 0xf8, // SAR EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0xF8000000; // Negative
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFF000000, "EAX: 0xF8000000 >> 3 = 0xFF000000");
    assert!(emu.flags().f_sf, "SF: result is still negative");
}

#[test]
fn test_sar_sign_extension_propagation() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xc1, 0xf8, 0x01, // SAR RAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0xFEDCBA9876543210; // Negative (MSB = 1)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0x8000000000000000, 0x8000000000000000, "MSB should remain 1");
    assert!(emu.flags().f_sf, "SF: result is negative");
}
