// ROL (Rotate Left) instruction tests
//
// Opcodes:
// D0 /0       ROL r/m8, 1
// D2 /0       ROL r/m8, CL
// C0 /0 ib    ROL r/m8, imm8
// D1 /0       ROL r/m16, 1
// D3 /0       ROL r/m16, CL
// C1 /0 ib    ROL r/m16, imm8
// D1 /0       ROL r/m32, 1
// D3 /0       ROL r/m32, CL
// C1 /0 ib    ROL r/m32, imm8
// REX.W + D1 /0    ROL r/m64, 1
// REX.W + D3 /0    ROL r/m64, CL
// REX.W + C1 /0 ib ROL r/m64, imm8
//
// ROL rotates bits left. MSB is shifted into LSB and CF.
// Unlike RCL, CF does not participate in the rotation (it only receives MSB).
//
// Flags:
// - CF: Receives MSB shifted out
// - OF: Only for 1-bit rotates (CF XOR new MSB)
// - Other flags: Undefined
// - Count is 0: No flags affected

use crate::*;

// ============================================================================
// 8-bit ROL tests
// ============================================================================

#[test]
fn test_rol_al_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL AL, 1 (opcode D0 /0)
    let code = [
        0xd0, 0xc0, // ROL AL, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x42; // 0100_0010
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x84, "AL: 0x42 ROL 1 = 0x84");
    assert!(!emu.flags().f_cf, "CF: receives MSB (was 0)");
    assert!(emu.flags().f_of, "OF: CF XOR new MSB = 0 XOR 1 = 1");
}

#[test]
fn test_rol_al_1_with_msb() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL AL, 1 with MSB set
    let code = [
        0xd0, 0xc0, // ROL AL, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x81; // 1000_0001
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03, "AL: 0x81 ROL 1 = 0x03 (MSB rotates to LSB)");
    assert!(emu.flags().f_cf, "CF: receives MSB (was 1)");
}

#[test]
fn test_rol_al_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL AL, CL (opcode D2 /0)
    let code = [
        0xd2, 0xc0, // ROL AL, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x01; // 0000_0001
    emu.regs_mut().rcx = 0x04; // Rotate by 4
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x10, "AL: 0x01 ROL 4 = 0x10");
    assert!(!emu.flags().f_cf, "CF: last bit rotated was 0");
}

#[test]
fn test_rol_al_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL AL, imm8 (opcode C0 /0 ib)
    let code = [
        0xc0, 0xc0, 0x03, // ROL AL, 3
        0xf4,
    ];
    emu.regs_mut().rax = 0x11; // 0001_0001
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x88, "AL: 0x11 ROL 3 = 0x88");
    assert!(!emu.flags().f_cf, "CF: last bit rotated was 0");
}

#[test]
fn test_rol_full_rotation_8bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL by 8 should return to original value
    let code = [
        0xc0, 0xc0, 0x08, // ROL AL, 8
        0xf4,
    ];
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL: full rotation returns to original");
}

#[test]
fn test_rol_count_masked_8bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd2, 0xc0, // ROL AL, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x11;
    emu.regs_mut().rcx = 0x1B; // 27 & 0x1F = 27, but for 8-bit it's mod 9 in some CPUs
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 27 % 8 = 3 for 8-bit operand
    assert_eq!(emu.regs().rax & 0xFF, 0x88, "AL: rotation count masked");
}

#[test]
fn test_rol_count_zero_preserves_flags() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc0, 0xc0, 0x00, // ROL AL, 0
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
// 16-bit ROL tests
// ============================================================================

#[test]
fn test_rol_ax_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL AX, 1 (opcode 66 D1 /0)
    let code = [
        0x66, 0xd1, 0xc0, // ROL AX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x4321;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x8642, "AX: 0x4321 ROL 1 = 0x8642");
    assert!(!emu.flags().f_cf, "CF: MSB was 0");
}

#[test]
fn test_rol_ax_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL AX, CL (opcode 66 D3 /0)
    let code = [
        0x66, 0xd3, 0xc0, // ROL AX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x0001;
    emu.regs_mut().rcx = 0x0F; // Rotate by 15
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x8000, "AX: 0x0001 ROL 15 = 0x8000");
    assert!(!emu.flags().f_cf, "CF: last bit rotated was 0");
}

#[test]
fn test_rol_ax_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL AX, imm8 (opcode 66 C1 /0 ib)
    let code = [
        0x66, 0xc1, 0xc0, 0x04, // ROL AX, 4
        0xf4,
    ];
    emu.regs_mut().rax = 0x0123;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x1230, "AX: 0x0123 ROL 4 = 0x1230");
}

#[test]
fn test_rol_full_rotation_16bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL by 16 should return to original value
    let code = [
        0x66, 0xc1, 0xc0, 0x10, // ROL AX, 16
        0xf4,
    ];
    emu.regs_mut().rax = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x1234, "AX: full rotation returns to original");
}

// ============================================================================
// 32-bit ROL tests
// ============================================================================

#[test]
fn test_rol_eax_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL EAX, 1 (opcode D1 /0)
    let code = [
        0xd1, 0xc0, // ROL EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x2468ACF0, "EAX: 0x12345678 ROL 1 = 0x2468ACF0");
    assert!(!emu.flags().f_cf, "CF: MSB was 0");
}

#[test]
fn test_rol_eax_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL EAX, CL (opcode D3 /0)
    let code = [
        0xd3, 0xc0, // ROL EAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000001;
    emu.regs_mut().rcx = 0x1F; // Rotate by 31
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x80000000, "EAX: 0x00000001 ROL 31 = 0x80000000");
}

#[test]
fn test_rol_eax_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL EAX, imm8 (opcode C1 /0 ib)
    let code = [
        0xc1, 0xc0, 0x08, // ROL EAX, 8
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x34567812, "EAX: 0x12345678 ROL 8 = 0x34567812");
}

#[test]
fn test_rol_eax_with_msb() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL with MSB set
    let code = [
        0xd1, 0xc0, // ROL EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x80000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000003, "EAX: 0x80000001 ROL 1 = 0x00000003");
    assert!(emu.flags().f_cf, "CF: MSB was 1");
}

#[test]
fn test_rol_full_rotation_32bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL by 32 should return to original value
    let code = [
        0xd3, 0xc0, // ROL EAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rcx = 0x20; // Rotate by 32
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x12345678, "EAX: full rotation returns to original");
}

// ============================================================================
// 64-bit ROL tests
// ============================================================================

#[test]
fn test_rol_rax_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL RAX, 1 (opcode 48 D1 /0)
    let code = [
        0x48, 0xd1, 0xc0, // ROL RAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x2468ACF13579BDE0, "RAX: 0x123456789ABCDEF0 ROL 1");
    assert!(!emu.flags().f_cf, "CF: MSB was 0");
}

#[test]
fn test_rol_rax_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL RAX, CL (opcode 48 D3 /0)
    let code = [
        0x48, 0xd3, 0xc0, // ROL RAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x0000000000000001;
    emu.regs_mut().rcx = 0x3F; // Rotate by 63
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x8000000000000000, "RAX: 0x0000000000000001 ROL 63");
}

#[test]
fn test_rol_rax_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL RAX, imm8 (opcode 48 C1 /0 ib)
    let code = [
        0x48, 0xc1, 0xc0, 0x10, // ROL RAX, 16
        0xf4,
    ];
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x56789ABCDEF01234, "RAX: 0x123456789ABCDEF0 ROL 16");
}

#[test]
fn test_rol_rax_with_msb() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL with MSB set
    let code = [
        0x48, 0xd1, 0xc0, // ROL RAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x8000000000000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000003, "RAX: 0x8000000000000001 ROL 1");
    assert!(emu.flags().f_cf, "CF: MSB was 1");
}

#[test]
fn test_rol_full_rotation_64bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL by 64 should return to original value
    let code = [
        0x48, 0xd3, 0xc0, // ROL RAX, CL
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
fn test_rol_r8b_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL R8B, 1
    let code = [
        0x41, 0xd0, 0xc0, // ROL R8B, 1
        0xf4,
    ];
    emu.regs_mut().r8 = 0x55; // 0101_0101
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0xAA, "R8B: 0x55 ROL 1 = 0xAA");
}

#[test]
fn test_rol_r10w_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL R10W, CL
    let code = [
        0x66, 0x41, 0xd3, 0xc2, // ROL R10W, CL
        0xf4,
    ];
    emu.regs_mut().r10 = 0x1234;
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10 & 0xFFFF, 0x2341, "R10W: 0x1234 ROL 4 = 0x2341");
}

#[test]
fn test_rol_r12d_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL R12D, imm8
    let code = [
        0x41, 0xc1, 0xc4, 0x08, // ROL R12D, 8
        0xf4,
    ];
    emu.regs_mut().r12 = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r12 & 0xFFFFFFFF, 0x34567812, "R12D: 0x12345678 ROL 8 = 0x34567812");
}

#[test]
fn test_rol_r15_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL R15, 1
    let code = [
        0x49, 0xd1, 0xc7, // ROL R15, 1
        0xf4,
    ];
    emu.regs_mut().r15 = 0x0123456789ABCDEF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 0x02468ACF13579BDE, "R15: 0x0123456789ABCDEF ROL 1");
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_rol_byte_ptr_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL byte ptr [DATA_ADDR], 1
    let code = [
        0xd0, 0x04, 0x25, // ROL byte ptr [DATA_ADDR], 1
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

    assert_eq!(result, 0x84, "Memory: 0x42 ROL 1 = 0x84");
}

#[test]
fn test_rol_word_ptr_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL word ptr [DATA_ADDR], CL
    let code = [
        0x66, 0xd3, 0x04, 0x25, // ROL word ptr [DATA_ADDR], CL
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

    assert_eq!(result, 0x2341, "Memory: 0x1234 ROL 4 = 0x2341");
}

#[test]
fn test_rol_dword_ptr_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL dword ptr [DATA_ADDR], imm8
    let code = [
        0xc1, 0x04, 0x25, // ROL dword ptr [DATA_ADDR], imm8
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

    assert_eq!(result, 0x34567812, "Memory: 0x12345678 ROL 8 = 0x34567812");
}

#[test]
fn test_rol_qword_ptr_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL qword ptr [DATA_ADDR], CL
    let code = [
        0x48, 0xd3, 0x04, 0x25, // ROL qword ptr [DATA_ADDR], CL
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

    assert_eq!(result, 0x56789ABCDEF01234, "Memory: 0x123456789ABCDEF0 ROL 16");
}

// ============================================================================
// Practical use cases and edge cases
// ============================================================================

#[test]
fn test_rol_bit_permutation() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL can permute bits
    let code = [
        0xc0, 0xc0, 0x04, // ROL AL, 4
        0xf4,
    ];
    emu.regs_mut().rax = 0xF0; // 1111_0000
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x0F, "AL: 0xF0 ROL 4 = 0x0F");
}

#[test]
fn test_rol_overflow_flag_1bit_same() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // OF is clear when MSB doesn't change after rotation
    let code = [
        0xd1, 0xc0, // ROL EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x40000000; // 0100...
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x80000000, "EAX: 0x40000000 ROL 1");
    assert!(emu.flags().f_of, "OF: MSB changed from 0 to 1");
}

#[test]
fn test_rol_overflow_flag_1bit_different() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // OF when MSB changes
    let code = [
        0xd1, 0xc0, // ROL EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0xC0000000; // 1100...
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x80000001, "EAX: 0xC0000000 ROL 1");
    assert!(!emu.flags().f_of, "OF: MSB stayed the same");
}

#[test]
fn test_rol_circular_buffer_indexing() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ROL can implement circular buffer indexing
    let code = [
        0xc1, 0xc0, 0x03, // ROL EAX, 3
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x91A2B3C0, "EAX: rotated by 3 bits");
}

#[test]
fn test_rol_chained_rotations() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd1, 0xc0, // ROL EAX, 1
        0xd1, 0xc0, // ROL EAX, 1
        0xd1, 0xc0, // ROL EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x91A2B3C0, "EAX: three 1-bit rotations");
}

#[test]
fn test_rol_all_ones() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd1, 0xc0, // ROL EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF, "EAX: all ones stay all ones");
    assert!(emu.flags().f_cf, "CF: MSB was 1");
}

#[test]
fn test_rol_byte_swap_high_low() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc0, 0xc0, 0x04, // ROL AL, 4
        0xf4,
    ];
    emu.regs_mut().rax = 0x12;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x21, "AL: nibbles swapped");
}
