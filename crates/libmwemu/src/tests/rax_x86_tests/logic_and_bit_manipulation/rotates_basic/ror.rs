// ROR (Rotate Right) instruction tests
//
// Opcodes:
// D0 /1       ROR r/m8, 1
// D2 /1       ROR r/m8, CL
// C0 /1 ib    ROR r/m8, imm8
// D1 /1       ROR r/m16, 1
// D3 /1       ROR r/m16, CL
// C1 /1 ib    ROR r/m16, imm8
// D1 /1       ROR r/m32, 1
// D3 /1       ROR r/m32, CL
// C1 /1 ib    ROR r/m32, imm8
// REX.W + D1 /1    ROR r/m64, 1
// REX.W + D3 /1    ROR r/m64, CL
// REX.W + C1 /1 ib ROR r/m64, imm8
//
// ROR rotates bits right. LSB is shifted into MSB and CF.
// Unlike RCR, CF does not participate in the rotation (it only receives LSB).
//
// Flags:
// - CF: Receives LSB shifted out
// - OF: Only for 1-bit rotates (MSB XOR next-to-MSB of result)
// - Other flags: Undefined
// - Count is 0: No flags affected

use crate::*;

// ============================================================================
// 8-bit ROR tests
// ============================================================================

#[test]
fn test_ror_al_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR AL, 1 (opcode D0 /1)
    let code = [
        0xd0, 0xc8, // ROR AL, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x42; // 0100_0010
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x21, "AL: 0x42 ROR 1 = 0x21");
    assert!(!emu.flags().f_cf, "CF: receives LSB (was 0)");
}

#[test]
fn test_ror_al_1_with_lsb() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR AL, 1 with LSB set
    let code = [
        0xd0, 0xc8, // ROR AL, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x43; // 0100_0011
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xA1, "AL: 0x43 ROR 1 = 0xA1 (LSB rotates to MSB)");
    assert!(emu.flags().f_cf, "CF: receives LSB (was 1)");
}

#[test]
fn test_ror_al_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR AL, CL (opcode D2 /1)
    let code = [
        0xd2, 0xc8, // ROR AL, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x80; // 1000_0000
    emu.regs_mut().rcx = 0x04; // Rotate by 4
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x08, "AL: 0x80 ROR 4 = 0x08");
    assert!(!emu.flags().f_cf, "CF: last bit rotated was 0");
}

#[test]
fn test_ror_al_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR AL, imm8 (opcode C0 /1 ib)
    let code = [
        0xc0, 0xc8, 0x03, // ROR AL, 3
        0xf4,
    ];
    emu.regs_mut().rax = 0x11; // 0001_0001
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x22, "AL: 0x11 ROR 3 = 0x22");
    assert!(!emu.flags().f_cf, "CF: last bit rotated was 0");
}

#[test]
fn test_ror_full_rotation_8bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR by 8 should return to original value
    let code = [
        0xc0, 0xc8, 0x08, // ROR AL, 8
        0xf4,
    ];
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL: full rotation returns to original");
}

#[test]
fn test_ror_count_zero_preserves_flags() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc0, 0xc8, 0x00, // ROR AL, 0
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
// 16-bit ROR tests
// ============================================================================

#[test]
fn test_ror_ax_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR AX, 1 (opcode 66 D1 /1)
    let code = [
        0x66, 0xd1, 0xc8, // ROR AX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x4321;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xA190, "AX: 0x4321 ROR 1 = 0xA190");
    assert!(emu.flags().f_cf, "CF: LSB was 1");
}

#[test]
fn test_ror_ax_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR AX, CL (opcode 66 D3 /1)
    let code = [
        0x66, 0xd3, 0xc8, // ROR AX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x8000;
    emu.regs_mut().rcx = 0x0F; // Rotate by 15
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0001, "AX: 0x8000 ROR 15 = 0x0001");
    assert!(!emu.flags().f_cf, "CF: last bit rotated was 0");
}

#[test]
fn test_ror_ax_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR AX, imm8 (opcode 66 C1 /1 ib)
    let code = [
        0x66, 0xc1, 0xc8, 0x04, // ROR AX, 4
        0xf4,
    ];
    emu.regs_mut().rax = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x4123, "AX: 0x1234 ROR 4 = 0x4123");
}

#[test]
fn test_ror_full_rotation_16bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR by 16 should return to original value
    let code = [
        0x66, 0xc1, 0xc8, 0x10, // ROR AX, 16
        0xf4,
    ];
    emu.regs_mut().rax = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x1234, "AX: full rotation returns to original");
}

// ============================================================================
// 32-bit ROR tests
// ============================================================================

#[test]
fn test_ror_eax_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR EAX, 1 (opcode D1 /1)
    let code = [
        0xd1, 0xc8, // ROR EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x091A2B3C, "EAX: 0x12345678 ROR 1 = 0x091A2B3C");
    assert!(!emu.flags().f_cf, "CF: LSB was 0");
}

#[test]
fn test_ror_eax_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR EAX, CL (opcode D3 /1)
    let code = [
        0xd3, 0xc8, // ROR EAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x80000000;
    emu.regs_mut().rcx = 0x1F; // Rotate by 31
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000001, "EAX: 0x80000000 ROR 31 = 0x00000001");
}

#[test]
fn test_ror_eax_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR EAX, imm8 (opcode C1 /1 ib)
    let code = [
        0xc1, 0xc8, 0x08, // ROR EAX, 8
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x78123456, "EAX: 0x12345678 ROR 8 = 0x78123456");
}

#[test]
fn test_ror_eax_with_lsb() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR with LSB set
    let code = [
        0xd1, 0xc8, // ROR EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x80000000, "EAX: 0x00000001 ROR 1 = 0x80000000");
    assert!(emu.flags().f_cf, "CF: LSB was 1");
}

#[test]
fn test_ror_full_rotation_32bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR by 32 should return to original value
    let code = [
        0xd3, 0xc8, // ROR EAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rcx = 0x20; // Rotate by 32
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x12345678, "EAX: full rotation returns to original");
}

// ============================================================================
// 64-bit ROR tests
// ============================================================================

#[test]
fn test_ror_rax_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR RAX, 1 (opcode 48 D1 /1)
    let code = [
        0x48, 0xd1, 0xc8, // ROR RAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x091A2B3C4D5E6F78, "RAX: 0x123456789ABCDEF0 ROR 1");
    assert!(!emu.flags().f_cf, "CF: LSB was 0");
}

#[test]
fn test_ror_rax_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR RAX, CL (opcode 48 D3 /1)
    let code = [
        0x48, 0xd3, 0xc8, // ROR RAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x8000000000000000;
    emu.regs_mut().rcx = 0x3F; // Rotate by 63
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000001, "RAX: 0x8000000000000000 ROR 63");
}

#[test]
fn test_ror_rax_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR RAX, imm8 (opcode 48 C1 /1 ib)
    let code = [
        0x48, 0xc1, 0xc8, 0x10, // ROR RAX, 16
        0xf4,
    ];
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xDEF0123456789ABC, "RAX: 0x123456789ABCDEF0 ROR 16");
}

#[test]
fn test_ror_rax_with_lsb() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR with LSB set
    let code = [
        0x48, 0xd1, 0xc8, // ROR RAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x0000000000000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x8000000000000000, "RAX: 0x0000000000000001 ROR 1");
    assert!(emu.flags().f_cf, "CF: LSB was 1");
}

#[test]
fn test_ror_full_rotation_64bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR by 64 should return to original value
    let code = [
        0x48, 0xd3, 0xc8, // ROR RAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rcx = 0x40; // Rotate by 64
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "RAX: full rotation returns to original");
}

// ============================================================================
// Extended register tests (R8-R15)
// ============================================================================

#[test]
fn test_ror_r8b_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR R8B, 1
    let code = [
        0x41, 0xd0, 0xc8, // ROR R8B, 1
        0xf4,
    ];
    emu.regs_mut().r8 = 0xAA; // 1010_1010
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0x55, "R8B: 0xAA ROR 1 = 0x55");
    assert!(!emu.flags().f_cf, "CF: LSB was 0");
}

#[test]
fn test_ror_r10w_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR R10W, CL
    let code = [
        0x66, 0x41, 0xd3, 0xca, // ROR R10W, CL
        0xf4,
    ];
    emu.regs_mut().r10 = 0x1234;
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10 & 0xFFFF, 0x4123, "R10W: 0x1234 ROR 4 = 0x4123");
}

#[test]
fn test_ror_r12d_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR R12D, imm8
    let code = [
        0x41, 0xc1, 0xcc, 0x08, // ROR R12D, 8
        0xf4,
    ];
    emu.regs_mut().r12 = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r12 & 0xFFFFFFFF, 0x78123456, "R12D: 0x12345678 ROR 8 = 0x78123456");
}

#[test]
fn test_ror_r15_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR R15, 1
    let code = [
        0x49, 0xd1, 0xcf, // ROR R15, 1
        0xf4,
    ];
    emu.regs_mut().r15 = 0xFEDCBA9876543210;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 0x7F6E5D4C3B2A1908, "R15: 0xFEDCBA9876543210 ROR 1");
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_ror_byte_ptr_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR byte ptr [DATA_ADDR], 1
    let code = [
        0xd0, 0x0c, 0x25, // ROR byte ptr [DATA_ADDR], 1
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0x42);

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0x21, "Memory: 0x42 ROR 1 = 0x21");
}

#[test]
fn test_ror_word_ptr_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR word ptr [DATA_ADDR], CL
    let code = [
        0x66, 0xd3, 0x0c, 0x25, // ROR word ptr [DATA_ADDR], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0x1234);

    emu.run(None).unwrap();
    let result = emu.maps.read_word(DATA_ADDR).unwrap();

    assert_eq!(result, 0x4123, "Memory: 0x1234 ROR 4 = 0x4123");
}

#[test]
fn test_ror_dword_ptr_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR dword ptr [DATA_ADDR], imm8
    let code = [
        0xc1, 0x0c, 0x25, // ROR dword ptr [DATA_ADDR], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x08, // imm8 = 8
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x12345678);

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x78123456, "Memory: 0x12345678 ROR 8 = 0x78123456");
}

#[test]
fn test_ror_qword_ptr_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR qword ptr [DATA_ADDR], CL
    let code = [
        0x48, 0xd3, 0x0c, 0x25, // ROR qword ptr [DATA_ADDR], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    emu.regs_mut().rcx = 0x10;
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x123456789ABCDEF0);

    emu.run(None).unwrap();
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();

    assert_eq!(result, 0xDEF0123456789ABC, "Memory: 0x123456789ABCDEF0 ROR 16");
}

// ============================================================================
// Practical use cases and edge cases
// ============================================================================

#[test]
fn test_ror_byte_swap_endianness() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc1, 0xc8, 0x08, // ROR EAX, 8
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x78123456, "EAX: bytes rotated");
}

#[test]
fn test_ror_overflow_flag_1bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // OF is set to MSB XOR next-to-MSB for 1-bit rotates
    let code = [
        0xd1, 0xc8, // ROR EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000001; // ...0001
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x80000000, "EAX: 0x00000001 ROR 1");
    assert!(emu.flags().f_of, "OF: MSB XOR next-to-MSB = 1");
}

#[test]
fn test_ror_bit_permutation() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR can permute bits
    let code = [
        0xc0, 0xc8, 0x04, // ROR AL, 4
        0xf4,
    ];
    emu.regs_mut().rax = 0xF0; // 1111_0000
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x0F, "AL: 0xF0 ROR 4 = 0x0F");
}

#[test]
fn test_ror_chained_rotations() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd1, 0xc8, // ROR EAX, 1
        0xd1, 0xc8, // ROR EAX, 1
        0xd1, 0xc8, // ROR EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x02468ACF, "EAX: three 1-bit rotations");
}

#[test]
fn test_ror_all_ones() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd1, 0xc8, // ROR EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF, "EAX: all ones stay all ones");
    assert!(emu.flags().f_cf, "CF: LSB was 1");
}

#[test]
fn test_ror_nibble_swap() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc0, 0xc8, 0x04, // ROR AL, 4
        0xf4,
    ];
    emu.regs_mut().rax = 0x12;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x21, "AL: nibbles swapped");
}

#[test]
fn test_ror_extract_low_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROR can help extract specific bit fields
    let code = [
        0xc1, 0xc8, 0x10, // ROR EAX, 16
        0xf4,
    ];
    emu.regs_mut().rax = 0x12340000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00001234, "EAX: high word moved to low");
}

#[test]
fn test_ror_alternating_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd1, 0xc8, // ROR EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0xAAAAAAAA; // 1010_1010...
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x55555555, "EAX: alternating bits rotated");
    assert!(!emu.flags().f_cf, "CF: LSB was 0");
}
