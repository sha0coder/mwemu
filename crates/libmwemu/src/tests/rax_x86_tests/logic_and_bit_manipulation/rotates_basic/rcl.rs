// RCL (Rotate through Carry Left) instruction tests
//
// Opcodes:
// D0 /2       RCL r/m8, 1
// D2 /2       RCL r/m8, CL
// C0 /2 ib    RCL r/m8, imm8
// D1 /2       RCL r/m16, 1
// D3 /2       RCL r/m16, CL
// C1 /2 ib    RCL r/m16, imm8
// D1 /2       RCL r/m32, 1
// D3 /2       RCL r/m32, CL
// C1 /2 ib    RCL r/m32, imm8
// REX.W + D1 /2    RCL r/m64, 1
// REX.W + D3 /2    RCL r/m64, CL
// REX.W + C1 /2 ib RCL r/m64, imm8
//
// RCL rotates bits left through the carry flag.
// Unlike ROL, CF participates in the rotation:
// - 8-bit:  Rotates 9 bits (CF + r/m8)
// - 16-bit: Rotates 17 bits (CF + r/m16)
// - 32-bit: Rotates 33 bits (CF + r/m32)
// - 64-bit: Rotates 65 bits (CF + r/m64)
//
// Flags:
// - CF: Receives MSB shifted out, then participates in next rotation
// - OF: Only for 1-bit rotates (CF XOR new MSB)
// - Other flags: Undefined
// - Count is 0: No flags affected

use crate::*;

// ============================================================================
// 8-bit RCL tests
// ============================================================================

#[test]
fn test_rcl_al_1_cf_clear() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL AL, 1 with CF initially clear (opcode D0 /2)
    let code = [
        0xd0, 0xd0, // RCL AL, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x42; // 0100_0010
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0_0100_0010 becomes 0100_0010_0
    assert_eq!(emu.regs().rax & 0xFF, 0x84, "AL: 0x42 RCL 1 (CF=0) = 0x84");
    assert!(!emu.flags().f_cf, "CF: receives old MSB (was 0)");
}

#[test]
fn test_rcl_al_1_cf_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL AL, 1 with CF initially set
    let code = [
        0xd0, 0xd0, // RCL AL, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x42; // 0100_0010
    emu.flags_mut().load(0x2 | flags::F_CF); // CF = 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 1_0100_0010 becomes 0100_0010_1
    assert_eq!(emu.regs().rax & 0xFF, 0x85, "AL: 0x42 RCL 1 (CF=1) = 0x85");
    assert!(!emu.flags().f_cf, "CF: receives old MSB (was 0)");
}

#[test]
fn test_rcl_al_1_with_msb() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL AL, 1 with MSB set
    let code = [
        0xd0, 0xd0, // RCL AL, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x81; // 1000_0001
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0_1000_0001 becomes 1_0000_0010
    assert_eq!(emu.regs().rax & 0xFF, 0x02, "AL: 0x81 RCL 1 (CF=0) = 0x02");
    assert!(emu.flags().f_cf, "CF: receives old MSB (was 1)");
}

#[test]
fn test_rcl_al_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL AL, CL (opcode D2 /2)
    let code = [
        0xd2, 0xd0, // RCL AL, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x01;
    emu.regs_mut().rcx = 0x08; // Rotate by 8 (full byte + CF position)
    emu.flags_mut().load(0x2 | flags::F_CF); // CF = 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x80, "AL: after full 9-bit rotation");
}

#[test]
fn test_rcl_al_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL AL, imm8 (opcode C0 /2 ib)
    let code = [
        0xc0, 0xd0, 0x03, // RCL AL, 3
        0xf4,
    ];
    emu.regs_mut().rax = 0x11; // 0001_0001
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0_0001_0001 rotated left by 3
    assert_eq!(emu.regs().rax & 0xFF, 0x88, "AL: 0x11 RCL 3 (CF=0) = 0x88");
}

#[test]
fn test_rcl_propagates_cf() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd0, 0xd0, // RCL AL, 1
        0xd0, 0xd0, // RCL AL, 1 again (should use CF from first)
        0xf4,
    ];
    emu.regs_mut().rax = 0x80; // 1000_0000
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x01, "AL: CF propagated through rotations");
    assert!(!emu.flags().f_cf, "CF: cleared after second rotation");
}

#[test]
fn test_rcl_count_zero_preserves_flags() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc0, 0xd0, 0x00, // RCL AL, 0
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
// 16-bit RCL tests
// ============================================================================

#[test]
fn test_rcl_ax_1_cf_clear() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL AX, 1 with CF initially clear (opcode 66 D1 /2)
    let code = [
        0x66, 0xd1, 0xd0, // RCL AX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x4321;
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x8642, "AX: 0x4321 RCL 1 (CF=0) = 0x8642");
    assert!(!emu.flags().f_cf, "CF: MSB was 0");
}

#[test]
fn test_rcl_ax_1_cf_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL AX, 1 with CF initially set
    let code = [
        0x66, 0xd1, 0xd0, // RCL AX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x4321;
    emu.flags_mut().load(0x2 | flags::F_CF); // CF = 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x8643, "AX: 0x4321 RCL 1 (CF=1) = 0x8643");
    assert!(!emu.flags().f_cf, "CF: MSB was 0");
}

#[test]
fn test_rcl_ax_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL AX, CL (opcode 66 D3 /2)
    let code = [
        0x66, 0xd3, 0xd0, // RCL AX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x0001;
    emu.regs_mut().rcx = 0x10; // Rotate by 16 (full word + CF)
    emu.flags_mut().load(0x2 | flags::F_CF); // CF = 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x8000, "AX: after 16 rotations of 17-bit value");
}

#[test]
fn test_rcl_ax_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL AX, imm8 (opcode 66 C1 /2 ib)
    let code = [
        0x66, 0xc1, 0xd0, 0x04, // RCL AX, 4
        0xf4,
    ];
    emu.regs_mut().rax = 0x0123;
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x1230, "AX: 0x0123 RCL 4 (CF=0) = 0x1230");
}

// ============================================================================
// 32-bit RCL tests
// ============================================================================

#[test]
fn test_rcl_eax_1_cf_clear() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL EAX, 1 with CF initially clear (opcode D1 /2)
    let code = [
        0xd1, 0xd0, // RCL EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x2468ACF0, "EAX: 0x12345678 RCL 1 (CF=0)");
    assert!(!emu.flags().f_cf, "CF: MSB was 0");
}

#[test]
fn test_rcl_eax_1_cf_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL EAX, 1 with CF initially set
    let code = [
        0xd1, 0xd0, // RCL EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.flags_mut().load(0x2 | flags::F_CF); // CF = 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x2468ACF1, "EAX: 0x12345678 RCL 1 (CF=1)");
    assert!(!emu.flags().f_cf, "CF: MSB was 0");
}

#[test]
fn test_rcl_eax_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL EAX, CL (opcode D3 /2)
    let code = [
        0xd3, 0xd0, // RCL EAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000001;
    emu.regs_mut().rcx = 0x20; // Rotate by 32 (full dword + CF)
    emu.flags_mut().load(0x2 | flags::F_CF); // CF = 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000001, "EAX: after full 33-bit rotation");
}

#[test]
fn test_rcl_eax_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL EAX, imm8 (opcode C1 /2 ib)
    let code = [
        0xc1, 0xd0, 0x08, // RCL EAX, 8
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x34567809, "EAX: 0x12345678 RCL 8 (CF=0)");
}

#[test]
fn test_rcl_eax_with_msb() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL with MSB set
    let code = [
        0xd1, 0xd0, // RCL EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x80000001;
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000002, "EAX: 0x80000001 RCL 1 (CF=0)");
    assert!(emu.flags().f_cf, "CF: MSB was 1");
}

// ============================================================================
// 64-bit RCL tests
// ============================================================================

#[test]
fn test_rcl_rax_1_cf_clear() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL RAX, 1 with CF initially clear (opcode 48 D1 /2)
    let code = [
        0x48, 0xd1, 0xd0, // RCL RAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x2468ACF13579BDE0, "RAX: 0x123456789ABCDEF0 RCL 1 (CF=0)");
    assert!(!emu.flags().f_cf, "CF: MSB was 0");
}

#[test]
fn test_rcl_rax_1_cf_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL RAX, 1 with CF initially set
    let code = [
        0x48, 0xd1, 0xd0, // RCL RAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.flags_mut().load(0x2 | flags::F_CF); // CF = 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x2468ACF13579BDE1, "RAX: 0x123456789ABCDEF0 RCL 1 (CF=1)");
    assert!(!emu.flags().f_cf, "CF: MSB was 0");
}

#[test]
fn test_rcl_rax_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL RAX, CL (opcode 48 D3 /2)
    let code = [
        0x48, 0xd3, 0xd0, // RCL RAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x0000000000000001;
    emu.regs_mut().rcx = 0x3F; // Rotate by 63
    emu.flags_mut().load(0x2 | flags::F_CF); // CF = 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xC000000000000000, "RAX: bit rotated to MSB position");
}

#[test]
fn test_rcl_rax_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL RAX, imm8 (opcode 48 C1 /2 ib)
    let code = [
        0x48, 0xc1, 0xd0, 0x10, // RCL RAX, 16
        0xf4,
    ];
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x56789ABCDEF0091A, "RAX: 0x123456789ABCDEF0 RCL 16 (CF=0)");
}

#[test]
fn test_rcl_rax_with_msb() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL with MSB set
    let code = [
        0x48, 0xd1, 0xd0, // RCL RAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x8000000000000001;
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000002, "RAX: 0x8000000000000001 RCL 1 (CF=0)");
    assert!(emu.flags().f_cf, "CF: MSB was 1");
}

// ============================================================================
// Extended register tests (R8-R15)
// ============================================================================

#[test]
fn test_rcl_r8b_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL R8B, 1
    let code = [
        0x41, 0xd0, 0xd0, // RCL R8B, 1
        0xf4,
    ];
    emu.regs_mut().r8 = 0x55; // 0101_0101
    emu.flags_mut().load(0x2 | flags::F_CF); // CF = 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0xAB, "R8B: 0x55 RCL 1 (CF=1) = 0xAB");
}

#[test]
fn test_rcl_r10w_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL R10W, CL
    let code = [
        0x66, 0x41, 0xd3, 0xd2, // RCL R10W, CL
        0xf4,
    ];
    emu.regs_mut().r10 = 0x1234;
    emu.regs_mut().rcx = 0x04;
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10 & 0xFFFF, 0x2340, "R10W: 0x1234 RCL 4 (CF=0) = 0x2340");
}

#[test]
fn test_rcl_r12d_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL R12D, imm8
    let code = [
        0x41, 0xc1, 0xd4, 0x08, // RCL R12D, 8
        0xf4,
    ];
    emu.regs_mut().r12 = 0x12345678;
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r12 & 0xFFFFFFFF, 0x34567809, "R12D: 0x12345678 RCL 8 (CF=0)");
}

#[test]
fn test_rcl_r15_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL R15, 1
    let code = [
        0x49, 0xd1, 0xd7, // RCL R15, 1
        0xf4,
    ];
    emu.regs_mut().r15 = 0x0123456789ABCDEF;
    emu.flags_mut().load(0x2 | flags::F_CF); // CF = 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 0x02468ACF13579BDF, "R15: 0x0123456789ABCDEF RCL 1 (CF=1)");
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_rcl_byte_ptr_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL byte ptr [DATA_ADDR], 1
    let code = [
        0xd0, 0x14, 0x25, // RCL byte ptr [DATA_ADDR], 1
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0x42);

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0x84, "Memory: 0x42 RCL 1 (CF=0) = 0x84");
}

#[test]
fn test_rcl_word_ptr_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL word ptr [DATA_ADDR], CL
    let code = [
        0x66, 0xd3, 0x14, 0x25, // RCL word ptr [DATA_ADDR], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    emu.regs_mut().rcx = 0x04;
    emu.flags_mut().load(0x2 | flags::F_CF); // CF = 1
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0x1234);

    emu.run(None).unwrap();
    let result = emu.maps.read_word(DATA_ADDR).unwrap();

    assert_eq!(result, 0x2348, "Memory: 0x1234 RCL 4 (CF=1) = 0x2348");
}

#[test]
fn test_rcl_dword_ptr_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL dword ptr [DATA_ADDR], imm8
    let code = [
        0xc1, 0x14, 0x25, // RCL dword ptr [DATA_ADDR], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x08, // imm8 = 8
        0xf4,
    ];
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x12345678);

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x34567809, "Memory: 0x12345678 RCL 8 (CF=0)");
}

#[test]
fn test_rcl_qword_ptr_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL qword ptr [DATA_ADDR], CL
    let code = [
        0x48, 0xd3, 0x14, 0x25, // RCL qword ptr [DATA_ADDR], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    emu.regs_mut().rcx = 0x10;
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x123456789ABCDEF0);

    emu.run(None).unwrap();
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x56789ABCDEF0091A, "Memory: 0x123456789ABCDEF0 RCL 16 (CF=0)");
}

// ============================================================================
// Practical use cases and edge cases
// ============================================================================

#[test]
fn test_rcl_multi_precision_shift() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RCL is used for multi-precision shifts
    let code = [
        0xd1, 0xd0, // RCL EAX, 1 (low 32 bits)
        0xd1, 0xd3, // RCL EBX, 1 (high 32 bits, receives CF from EAX)
        0xf4,
    ];
    emu.regs_mut().rax = 0x80000000; // Low 32 bits with MSB set
    emu.regs_mut().rbx = 0x12345678; // High 32 bits
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000000, "EAX: low bits shifted");
    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0x2468ACF1, "EBX: high bits with CF from EAX");
}

#[test]
fn test_rcl_overflow_flag_1bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // OF is set to CF XOR MSB after rotation for 1-bit rotates
    let code = [
        0xd1, 0xd0, // RCL EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x40000000; // 0100...
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_of, "OF: CF XOR new MSB = 1");
}

#[test]
fn test_rcl_chained_with_different_cf() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd0, 0xd0, // RCL AL, 1
        0xd0, 0xd3, // RCL BL, 1
        0xd0, 0xd1, // RCL CL, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x80; // AL with MSB set
    emu.regs_mut().rbx = 0x00; // BL = 0
    emu.regs_mut().rcx = 0x00; // CL = 0
    emu.flags_mut().load(0x2); // CF = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL: 0_10000000 -> CF=1, AL=00000000
    // BL: 1_00000000 -> CF=0, BL=00000001
    // CL: 0_00000000 -> CF=0, CL=00000000
    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL: rotated out");
    assert_eq!(emu.regs().rbx & 0xFF, 0x01, "BL: received CF from AL");
    assert_eq!(emu.regs().rcx & 0xFF, 0x00, "CL: received CF from BL");
}
