// RCL (Rotate Through Carry Left) instruction tests
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
// The rotation includes CF: [CF <- MSB <- ... <- LSB <- CF]
// This creates a 9-bit (8+CF), 17-bit (16+CF), 33-bit (32+CF), or 65-bit (64+CF) rotation.
//
// Flags:
// - CF: Receives MSB shifted out, becomes new LSB on next iteration
// - OF: Only for 1-bit rotates (CF XOR new MSB)
// - Other flags: Undefined
// - Count is 0: No flags affected

use crate::*;

// ============================================================================
// 8-bit RCL tests
// ============================================================================

#[test]
fn test_rcl_al_1_cf_clear() {
    // RCL AL, 1 (opcode D0 /2) with CF clear
    let code = [
        0xd0, 0xd0, // RCL AL, 1
        0xf4,       // HLT
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x42; // 0100_0010
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    // 0100_0010 << 1, CF(0) -> LSB = 1000_0100
    assert_eq!(emu.regs().rax & 0xFF, 0x84, "AL: 0x42 RCL 1 (CF=0) = 0x84");
    assert!(!emu.flags().f_cf, "CF: receives MSB (was 0)");
}

#[test]
fn test_rcl_al_1_cf_set() {
    // RCL AL, 1 with CF set
    let code = [
        0xd0, 0xd0, // RCL AL, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x42; // 0100_0010
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2 | (1 << flags::F_CF));
        emu.run(None).unwrap();

    // 0100_0010 << 1, CF(1) -> LSB = 1000_0101
    assert_eq!(emu.regs().rax & 0xFF, 0x85, "AL: 0x42 RCL 1 (CF=1) = 0x85");
    assert!(!emu.flags().f_cf, "CF: receives MSB (was 0)");
}

#[test]
fn test_rcl_al_1_with_msb() {
    // RCL AL, 1 with MSB set, CF clear
    let code = [
        0xd0, 0xd0, // RCL AL, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x81; // 1000_0001
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    // 1000_0001 << 1, CF(0) -> LSB = 0000_0010
    assert_eq!(emu.regs().rax & 0xFF, 0x02, "AL: 0x81 RCL 1 (CF=0) = 0x02");
    assert!(emu.flags().f_cf, "CF: receives MSB (was 1)");
}

#[test]
fn test_rcl_al_1_msb_and_cf() {
    // RCL AL, 1 with both MSB and CF set
    let code = [
        0xd0, 0xd0, // RCL AL, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x81; // 1000_0001
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2 | (1 << flags::F_CF));
        emu.run(None).unwrap();

    // 1000_0001 << 1, CF(1) -> LSB = 0000_0011
    assert_eq!(emu.regs().rax & 0xFF, 0x03, "AL: 0x81 RCL 1 (CF=1) = 0x03");
    assert!(emu.flags().f_cf, "CF: receives MSB (was 1)");
}

#[test]
fn test_rcl_al_cl() {
    // RCL AL, CL (opcode D2 /2)
    let code = [
        0xd2, 0xd0, // RCL AL, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x01; // 0000_0001
    emu.regs_mut().rcx = 0x04; // Rotate by 4
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2 | (1 << flags::F_CF));
        emu.run(None).unwrap();

    // After 4 rotations through carry (9-bit total)
    // Initial: CF=1, AL=0000_0001
    // Rot 1:   CF=0, AL=0000_0011
    // Rot 2:   CF=0, AL=0000_0110
    // Rot 3:   CF=0, AL=0000_1100
    // Rot 4:   CF=0, AL=0001_1000
    assert_eq!(emu.regs().rax & 0xFF, 0x18, "AL: 0x01 RCL 4 (CF=1) = 0x18");
}

#[test]
fn test_rcl_al_imm8() {
    // RCL AL, imm8 (opcode C0 /2 ib)
    let code = [
        0xc0, 0xd0, 0x03, // RCL AL, 3
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x11; // 0001_0001
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x88, "AL: 0x11 RCL 3 (CF=0) = 0x88");
}

#[test]
fn test_rcl_full_rotation_9bit() {
    // RCL by 9 should return to original value (8 bits + CF)
    let code = [
        0xc0, 0xd0, 0x09, // RCL AL, 9
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x42;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL: full 9-bit rotation returns to original");
    assert!(!emu.flags().f_cf, "CF: also returns to original");
}

#[test]
fn test_rcl_count_zero_preserves_flags() {
    // Count of 0 should not affect flags
    let code = [
        0xc0, 0xd0, 0x00, // RCL AL, 0
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x42;
            emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2 | (1 << flags::F_CF) | (1 << flags::F_ZF) | (1 << flags::F_OF));
    let initial_flags = emu.flags().dump();
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL: unchanged");
    assert_eq!(emu.flags().dump() & (flags::F_CF | flags::F_ZF | flags::F_OF),
               initial_flags & (flags::F_CF | flags::F_ZF | flags::F_OF),
               "Flags preserved");
}

#[test]
fn test_rcl_bl() {
    // RCL BL, 1
    let code = [
        0xd0, 0xd3, // RCL BL, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rbx = 0xC5; // 1100_0101
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFF, 0x8A, "BL: 0xC5 RCL 1 (CF=0) = 0x8A");
    assert!(emu.flags().f_cf, "CF: MSB was 1");
}

#[test]
fn test_rcl_cl_reg() {
    // RCL CL, imm8
    let code = [
        0xc0, 0xd1, 0x02, // RCL CL, 2
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rcx = 0x33; // 0011_0011
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2 | (1 << flags::F_CF));
        emu.run(None).unwrap();

    // 0011_0011, CF=1
    // Rot 1: 0110_0111, CF=0
    // Rot 2: 1100_1110, CF=0
    assert_eq!(emu.regs().rcx & 0xFF, 0xCE, "CL: 0x33 RCL 2 (CF=1) = 0xCE");
}

#[test]
fn test_rcl_dl() {
    // RCL DL, CL
    let code = [
        0xd2, 0xd2, // RCL DL, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdx = 0x0F;
    emu.regs_mut().rcx = 0x04;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx & 0xFF, 0xF0, "DL: 0x0F RCL 4 (CF=0) = 0xF0");
}

// ============================================================================
// 16-bit RCL tests
// ============================================================================

#[test]
fn test_rcl_ax_1() {
    // RCL AX, 1 (opcode 66 D1 /2)
    let code = [
        0x66, 0xd1, 0xd0, // RCL AX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x4321;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x8642, "AX: 0x4321 RCL 1 (CF=0) = 0x8642");
    assert!(!emu.flags().f_cf, "CF: MSB was 0");
}

#[test]
fn test_rcl_ax_1_cf_set() {
    // RCL AX, 1 with CF set
    let code = [
        0x66, 0xd1, 0xd0, // RCL AX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x4321;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2 | (1 << flags::F_CF));
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x8643, "AX: 0x4321 RCL 1 (CF=1) = 0x8643");
    assert!(!emu.flags().f_cf, "CF: MSB was 0");
}

#[test]
fn test_rcl_ax_cl() {
    // RCL AX, CL (opcode 66 D3 /2)
    let code = [
        0x66, 0xd3, 0xd0, // RCL AX, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x1234;
    emu.regs_mut().rcx = 0x04;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x2340, "AX: 0x1234 RCL 4 (CF=0) = 0x2340");
}

#[test]
fn test_rcl_ax_imm8() {
    // RCL AX, imm8 (opcode 66 C1 /2 ib)
    let code = [
        0x66, 0xc1, 0xd0, 0x08, // RCL AX, 8
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x1234;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x3409, "AX: 0x1234 RCL 8 (CF=0) = 0x3409");
}

#[test]
fn test_rcl_ax_full_rotation() {
    // RCL by 17 should return to original value (16 bits + CF)
    let code = [
        0x66, 0xc1, 0xd0, 0x11, // RCL AX, 17
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x1234;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x1234, "AX: full 17-bit rotation returns to original");
}

#[test]
fn test_rcl_bx() {
    // RCL BX, 1
    let code = [
        0x66, 0xd1, 0xd3, // RCL BX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rbx = 0x8000;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2 | (1 << flags::F_CF));
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFF, 0x0001, "BX: 0x8000 RCL 1 (CF=1) = 0x0001");
    assert!(emu.flags().f_cf, "CF: MSB was 1");
}

#[test]
fn test_rcl_cx() {
    // RCL CX, imm8
    let code = [
        0x66, 0xc1, 0xd1, 0x04, // RCL CX, 4
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rcx = 0xABCD;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFFFF, 0xBCD5, "CX: 0xABCD RCL 4 (CF=0) = 0xBCD5");
}

#[test]
fn test_rcl_dx_cl() {
    // RCL DX, CL
    let code = [
        0x66, 0xd3, 0xd2, // RCL DX, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdx = 0x00FF;
    emu.regs_mut().rcx = 0x08;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx & 0xFFFF, 0xFF00, "DX: 0x00FF RCL 8 (CF=0) = 0xFF00");
}

// ============================================================================
// 32-bit RCL tests
// ============================================================================

#[test]
fn test_rcl_eax_1() {
    // RCL EAX, 1 (opcode D1 /2)
    let code = [
        0xd1, 0xd0, // RCL EAX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x43218765;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x86430ECA, "EAX: 0x43218765 RCL 1 (CF=0) = 0x86430ECA");
}

#[test]
fn test_rcl_eax_1_cf_set() {
    // RCL EAX, 1 with CF set
    let code = [
        0xd1, 0xd0, // RCL EAX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x43218765;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2 | (1 << flags::F_CF));
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x86430ECB, "EAX: 0x43218765 RCL 1 (CF=1) = 0x86430ECB");
}

#[test]
fn test_rcl_eax_cl() {
    // RCL EAX, CL (opcode D3 /2)
    let code = [
        0xd3, 0xd0, // RCL EAX, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rcx = 0x08;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x34567809, "EAX: 0x12345678 RCL 8 (CF=0) = 0x34567809");
}

#[test]
fn test_rcl_eax_imm8() {
    // RCL EAX, imm8 (opcode C1 /2 ib)
    let code = [
        0xc1, 0xd0, 0x10, // RCL EAX, 16
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x5678091A, "EAX: 0x12345678 RCL 16 (CF=0) = 0x5678091A");
}

#[test]
fn test_rcl_eax_full_rotation() {
    // RCL by 33: count is masked to 5 bits (33 & 31 = 1), so this is RCL by 1
    let code = [
        0xc1, 0xd0, 0x21, // RCL EAX, 33
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    // 33 & 31 = 1, so effective count is 1
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x2468ACF0, "EAX: RCL 33 (masked to 1) = 0x2468ACF0");
}

#[test]
fn test_rcl_ebx() {
    // RCL EBX, 1
    let code = [
        0xd1, 0xd3, // RCL EBX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rbx = 0x80000000;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0x00000000, "EBX: 0x80000000 RCL 1 (CF=0) = 0x00000000");
    assert!(emu.flags().f_cf, "CF: MSB was 1");
}

#[test]
fn test_rcl_ecx() {
    // RCL ECX, imm8
    let code = [
        0xc1, 0xd1, 0x04, // RCL ECX, 4
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rcx = 0xABCDEF01;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFFFFFFFF, 0xBCDEF015, "ECX: 0xABCDEF01 RCL 4 (CF=0) = 0xBCDEF015");
}

#[test]
fn test_rcl_edx_cl() {
    // RCL EDX, CL
    let code = [
        0xd3, 0xd2, // RCL EDX, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdx = 0x000000FF;
    emu.regs_mut().rcx = 0x18; // 24 bits
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx & 0xFFFFFFFF, 0xFF000000, "EDX: 0x000000FF RCL 24 (CF=0) = 0xFF000000");
}

#[test]
fn test_rcl_esi() {
    // RCL ESI, 1
    let code = [
        0xd1, 0xd6, // RCL ESI, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rsi = 0x40000000;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2 | (1 << flags::F_CF));
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi & 0xFFFFFFFF, 0x80000001, "ESI: 0x40000000 RCL 1 (CF=1) = 0x80000001");
}

#[test]
fn test_rcl_edi() {
    // RCL EDI, imm8
    let code = [
        0xc1, 0xd7, 0x0C, // RCL EDI, 12
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdi = 0x12345678;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rdi & 0xFFFFFFFF, 0x45678091, "EDI: 0x12345678 RCL 12 (CF=0) = 0x45678091");
}

// ============================================================================
// 64-bit RCL tests
// ============================================================================

#[test]
fn test_rcl_rax_1() {
    // RCL RAX, 1 (opcode REX.W D1 /2)
    let code = [
        0x48, 0xd1, 0xd0, // RCL RAX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x4321876543218765;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x86430ECA86430ECA, "RAX: RCL 1 (CF=0)");
}

#[test]
fn test_rcl_rax_1_cf_set() {
    // RCL RAX, 1 with CF set
    let code = [
        0x48, 0xd1, 0xd0, // RCL RAX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x4321876543218765;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2 | (1 << flags::F_CF));
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x86430ECA86430ECB, "RAX: RCL 1 (CF=1)");
}

#[test]
fn test_rcl_rax_cl() {
    // RCL RAX, CL (opcode REX.W D3 /2)
    let code = [
        0x48, 0xd3, 0xd0, // RCL RAX, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rcx = 0x08;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x3456789ABCDEF009, "RAX: 0x123456789ABCDEF0 RCL 8 (CF=0)");
}

#[test]
fn test_rcl_rax_imm8() {
    // RCL RAX, imm8 (opcode REX.W C1 /2 ib)
    let code = [
        0x48, 0xc1, 0xd0, 0x10, // RCL RAX, 16
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x56789ABCDEF0091A, "RAX: 0x123456789ABCDEF0 RCL 16 (CF=0)");
}

#[test]
fn test_rcl_rax_32bits() {
    // RCL RAX, 32
    let code = [
        0x48, 0xc1, 0xd0, 0x20, // RCL RAX, 32
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x9ABCDEF0091A2B3C, "RAX: RCL 32 (CF=0)");
}

#[test]
fn test_rcl_rbx() {
    // RCL RBX, 1
    let code = [
        0x48, 0xd1, 0xd3, // RCL RBX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rbx = 0x8000000000000000;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0x0000000000000000, "RBX: 0x8000000000000000 RCL 1 (CF=0) = 0");
    assert!(emu.flags().f_cf, "CF: MSB was 1");
}

#[test]
fn test_rcl_rcx() {
    // RCL RCX, imm8
    let code = [
        0x48, 0xc1, 0xd1, 0x04, // RCL RCX, 4
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rcx = 0xABCDEF0123456789;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx, 0xBCDEF01234567895, "RCX: RCL 4 (CF=0)");
}

#[test]
fn test_rcl_rdx_cl() {
    // RCL RDX, CL
    let code = [
        0x48, 0xd3, 0xd2, // RCL RDX, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdx = 0x00000000000000FF;
    emu.regs_mut().rcx = 0x38; // 56 bits
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx, 0xFF00000000000000, "RDX: RCL 56 (CF=0)");
}

#[test]
fn test_rcl_rsi() {
    // RCL RSI, 1
    let code = [
        0x48, 0xd1, 0xd6, // RCL RSI, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rsi = 0x4000000000000000;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2 | (1 << flags::F_CF));
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi, 0x8000000000000001, "RSI: RCL 1 (CF=1)");
}

#[test]
fn test_rcl_rdi() {
    // RCL RDI, imm8
    let code = [
        0x48, 0xc1, 0xd7, 0x0C, // RCL RDI, 12
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdi = 0x123456789ABCDEF0;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().rdi, 0x456789ABCDEF0091, "RDI: RCL 12 (CF=0)");
}

#[test]
fn test_rcl_r8() {
    // RCL R8, 1 (REX.WB D1 /2)
    let code = [
        0x49, 0xd1, 0xd0, // RCL R8, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().r8 = 0xFEDCBA9876543210;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().r8, 0xFDB97530ECA86420, "R8: RCL 1 (CF=0)");
    assert!(emu.flags().f_cf, "CF: MSB was 1");
}

#[test]
fn test_rcl_r9_cl() {
    // RCL R9, CL
    let code = [
        0x49, 0xd3, 0xd1, // RCL R9, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().r9 = 0x0123456789ABCDEF;
    emu.regs_mut().rcx = 0x10; // 16 bits
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().r9, 0x456789ABCDEF0091, "R9: RCL 16 (CF=0)");
}

#[test]
fn test_rcl_r10_imm8() {
    // RCL R10, imm8
    let code = [
        0x49, 0xc1, 0xd2, 0x08, // RCL R10, 8
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().r10 = 0x123456789ABCDEF0;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.run(None).unwrap();

    assert_eq!(emu.regs().r10, 0x3456789ABCDEF009, "R10: RCL 8 (CF=0)");
}

#[test]
fn test_rcl_r15() {
    // RCL R15, 1
    let code = [
        0x49, 0xd1, 0xd7, // RCL R15, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().r15 = 0x1111111111111111;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2 | (1 << flags::F_CF));
        emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 0x2222222222222223, "R15: RCL 1 (CF=1)");
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_rcl_mem8() {
    let DATA_ADDR = 0x7000;

    // RCL byte [DATA_ADDR], 1
    let code = [
        0xd0, 0x14, 0x25, // RCL byte ptr [disp32], 1
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut emu = emu64();
    emu.maps.create_map("test_data", 0x7000, 0x1000, crate::maps::mem64::Permission::READ_WRITE).expect("failed to map test_data");
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.maps.write_byte(DATA_ADDR, 0x81);
    emu.run(None).unwrap();

    assert_eq!(emu.maps.read_byte(DATA_ADDR).unwrap(), 0x02, "Memory: 0x81 RCL 1 (CF=0) = 0x02");
    assert!(emu.flags().f_cf, "CF: MSB was 1");
}

#[test]
fn test_rcl_mem16() {
    let DATA_ADDR = 0x7000;

    // RCL word [DATA_ADDR], 4
    let code = [
        0x66, 0xc1, 0x14, 0x25, // RCL word ptr [disp32], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x04, // imm8: 4
        0xf4,
    ];
    let mut emu = emu64();
    emu.maps.create_map("test_data", 0x7000, 0x1000, crate::maps::mem64::Permission::READ_WRITE).expect("failed to map test_data");
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.maps.write_word(DATA_ADDR, 0x1234);
    emu.run(None).unwrap();

    assert_eq!(emu.maps.read_word(DATA_ADDR).unwrap(), 0x2340, "Memory: 0x1234 RCL 4 (CF=0) = 0x2340");
}

#[test]
fn test_rcl_mem32() {
    let DATA_ADDR = 0x7000;

    // RCL dword [DATA_ADDR], CL
    let code = [
        0xd3, 0x14, 0x25, // RCL dword ptr [disp32], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut emu = emu64();
    emu.maps.create_map("test_data", 0x7000, 0x1000, crate::maps::mem64::Permission::READ_WRITE).expect("failed to map test_data");
    emu.regs_mut().rcx = 0x08;
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.maps.write_dword(DATA_ADDR, 0x12345678);
    emu.run(None).unwrap();

    assert_eq!(emu.maps.read_dword(DATA_ADDR).unwrap(), 0x34567809, "Memory: 0x12345678 RCL 8 (CF=0) = 0x34567809");
}

#[test]
fn test_rcl_mem64() {
    let DATA_ADDR = 0x7000;

    // RCL qword [DATA_ADDR], 16
    let code = [
        0x48, 0xc1, 0x14, 0x25, // RCL qword ptr [disp32], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x10, // imm8: 16
        0xf4,
    ];
    let mut emu = emu64();
    emu.maps.create_map("test_data", 0x7000, 0x1000, crate::maps::mem64::Permission::READ_WRITE).expect("failed to map test_data");
        emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2);
        emu.maps.write_qword(DATA_ADDR, 0x123456789ABCDEF0);
    emu.run(None).unwrap();

    assert_eq!(emu.maps.read_qword(DATA_ADDR).unwrap(), 0x56789ABCDEF0091A, "Memory: RCL 16 (CF=0)");
}
