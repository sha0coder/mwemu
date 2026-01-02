// RCR (Rotate Through Carry Right) instruction tests
//
// Opcodes:
// D0 /3       RCR r/m8, 1
// D2 /3       RCR r/m8, CL
// C0 /3 ib    RCR r/m8, imm8
// D1 /3       RCR r/m16, 1
// D3 /3       RCR r/m16, CL
// C1 /3 ib    RCR r/m16, imm8
// D1 /3       RCR r/m32, 1
// D3 /3       RCR r/m32, CL
// C1 /3 ib    RCR r/m32, imm8
// REX.W + D1 /3    RCR r/m64, 1
// REX.W + D3 /3    RCR r/m64, CL
// REX.W + C1 /3 ib RCR r/m64, imm8
//
// RCR rotates bits right through the carry flag.
// The rotation includes CF: [CF -> MSB -> ... -> LSB -> CF]
// This creates a 9-bit (8+CF), 17-bit (16+CF), 33-bit (32+CF), or 65-bit (64+CF) rotation.
//
// Flags:
// - CF: Receives LSB shifted out, becomes new MSB on next iteration
// - OF: Only for 1-bit rotates (MSB XOR (MSB-1))
// - Other flags: Undefined
// - Count is 0: No flags affected

use crate::*;

// ============================================================================
// 8-bit RCR tests
// ============================================================================

#[test]
fn test_rcr_al_1_cf_clear() {
    // RCR AL, 1 (opcode D0 /3) with CF clear
    let code = [
        0xd0, 0xd8, // RCR AL, 1
        0xf4,       // HLT
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x42; // 0100_0010
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0100_0010 >> 1, CF(0) -> MSB = 0010_0001
    assert_eq!(emu.regs().rax & 0xFF, 0x21, "AL: 0x42 RCR 1 (CF=0) = 0x21");
    assert!(!emu.flags().f_cf, "CF: receives LSB (was 0)");
}

#[test]
fn test_rcr_al_1_cf_set() {
    // RCR AL, 1 with CF set
    let code = [
        0xd0, 0xd8, // RCR AL, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x42; // 0100_0010
    emu.flags_mut().load(0x2 | flags::F_CF); // CF set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0100_0010 >> 1, CF(1) -> MSB = 1010_0001
    assert_eq!(emu.regs().rax & 0xFF, 0xA1, "AL: 0x42 RCR 1 (CF=1) = 0xA1");
    assert!(!emu.flags().f_cf, "CF: receives LSB (was 0)");
}

#[test]
fn test_rcr_al_1_with_lsb() {
    // RCR AL, 1 with LSB set, CF clear
    let code = [
        0xd0, 0xd8, // RCR AL, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x81; // 1000_0001
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 1000_0001 >> 1, CF(0) -> MSB = 0100_0000
    assert_eq!(emu.regs().rax & 0xFF, 0x40, "AL: 0x81 RCR 1 (CF=0) = 0x40");
    assert!(emu.flags().f_cf, "CF: receives LSB (was 1)");
}

#[test]
fn test_rcr_al_1_lsb_and_cf() {
    // RCR AL, 1 with both LSB and CF set
    let code = [
        0xd0, 0xd8, // RCR AL, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x81; // 1000_0001
    emu.flags_mut().load(0x2 | flags::F_CF); // CF set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 1000_0001 >> 1, CF(1) -> MSB = 1100_0000
    assert_eq!(emu.regs().rax & 0xFF, 0xC0, "AL: 0x81 RCR 1 (CF=1) = 0xC0");
    assert!(emu.flags().f_cf, "CF: receives LSB (was 1)");
}

#[test]
fn test_rcr_al_cl() {
    // RCR AL, CL (opcode D2 /3)
    let code = [
        0xd2, 0xd8, // RCR AL, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x10; // 0001_0000
    emu.regs_mut().rcx = 0x04; // Rotate by 4
    emu.flags_mut().load(0x2 | flags::F_CF); // CF set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // After 4 rotations right through carry (9-bit total)
    // Initial: CF=1, AL=0001_0000
    // Rot 1:   CF=0, AL=1000_1000
    // Rot 2:   CF=0, AL=0100_0100
    // Rot 3:   CF=0, AL=0010_0010
    // Rot 4:   CF=0, AL=0001_0001
    assert_eq!(emu.regs().rax & 0xFF, 0x11, "AL: 0x10 RCR 4 (CF=1) = 0x11");
}

#[test]
fn test_rcr_al_imm8() {
    // RCR AL, imm8 (opcode C0 /3 ib)
    let code = [
        0xc0, 0xd8, 0x03, // RCR AL, 3
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x88; // 1000_1000
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x11, "AL: 0x88 RCR 3 (CF=0) = 0x11");
}

#[test]
fn test_rcr_full_rotation_9bit() {
    // RCR by 9 should return to original value (8 bits + CF)
    let code = [
        0xc0, 0xd8, 0x09, // RCR AL, 9
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x42;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL: full 9-bit rotation returns to original");
    assert!(!emu.flags().f_cf, "CF: also returns to original");
}

#[test]
fn test_rcr_count_zero_preserves_flags() {
    // Count of 0 should not affect flags
    let code = [
        0xc0, 0xd8, 0x00, // RCR AL, 0
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x42;
    emu.flags_mut().load(0x2 | flags::F_CF | flags::F_ZF | flags::F_OF);
    let initial_flags = emu.flags().dump();
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL: unchanged");
    assert_eq!(emu.flags().dump() & (flags::F_CF | flags::F_ZF | flags::F_OF),
               initial_flags & (flags::F_CF | flags::F_ZF | flags::F_OF),
               "Flags preserved");
}

#[test]
fn test_rcr_bl() {
    let code = [0xd0, 0xdb, 0xf4]; // RCR BL, 1
    let mut emu = emu64();
    emu.regs_mut().rbx = 0xC5;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rbx & 0xFF, 0x62, "BL: 0xC5 RCR 1 (CF=0) = 0x62");
    assert!(emu.flags().f_cf, "CF: LSB was 1");
}

#[test]
fn test_rcr_cl_reg() {
    let code = [0xc0, 0xd9, 0x02, 0xf4]; // RCR CL, 2
    let mut emu = emu64();
    emu.regs_mut().rcx = 0xCC;
    emu.flags_mut().load(0x2 | flags::F_CF); // CF set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFF, 0x73, "CL: 0xCC RCR 2 (CF=1)");
}

#[test]
fn test_rcr_dl() {
    let code = [0xd2, 0xda, 0xf4]; // RCR DL, CL
    let mut emu = emu64();
    emu.regs_mut().rdx = 0xF0;
    emu.regs_mut().rcx = 0x04;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rdx & 0xFF, 0x0F, "DL: 0xF0 RCR 4 (CF=0) = 0x0F");
}

// ============================================================================
// 16-bit RCR tests
// ============================================================================

#[test]
fn test_rcr_ax_1() {
    let code = [0x66, 0xd1, 0xd8, 0xf4]; // RCR AX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0x8642;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0x4321, "AX: 0x8642 RCR 1 (CF=0) = 0x4321");
}

#[test]
fn test_rcr_ax_1_cf_set() {
    let code = [0x66, 0xd1, 0xd8, 0xf4]; // RCR AX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0x8642;
    emu.flags_mut().load(0x2 | flags::F_CF); // CF set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0xC321, "AX: 0x8642 RCR 1 (CF=1) = 0xC321");
}

#[test]
fn test_rcr_ax_cl() {
    let code = [0x66, 0xd3, 0xd8, 0xf4]; // RCR AX, CL
    let mut emu = emu64();
    emu.regs_mut().rax = 0x1234;
    emu.regs_mut().rcx = 0x04;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0x8123, "AX: 0x1234 RCR 4 (CF=0)");
}

#[test]
fn test_rcr_ax_imm8() {
    let code = [0x66, 0xc1, 0xd8, 0x08, 0xf4]; // RCR AX, 8
    let mut emu = emu64();
    emu.regs_mut().rax = 0x1234;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0x6812, "AX: 0x1234 RCR 8 (CF=0)");
}

#[test]
fn test_rcr_ax_full_rotation() {
    let code = [0x66, 0xc1, 0xd8, 0x11, 0xf4]; // RCR AX, 17
    let mut emu = emu64();
    emu.regs_mut().rax = 0x1234;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0x1234, "AX: full 17-bit rotation");
}

#[test]
fn test_rcr_bx() {
    let code = [0x66, 0xd1, 0xdb, 0xf4]; // RCR BX, 1
    let mut emu = emu64();
    emu.regs_mut().rbx = 0x0001;
    emu.flags_mut().load(0x2 | flags::F_CF); // CF set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rbx & 0xFFFF, 0x8000, "BX: 0x0001 RCR 1 (CF=1) = 0x8000");
}

#[test]
fn test_rcr_cx() {
    let code = [0x66, 0xc1, 0xd9, 0x04, 0xf4]; // RCR CX, 4
    let mut emu = emu64();
    emu.regs_mut().rcx = 0xABCD;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFFFF, 0xAABC, "CX: 0xABCD RCR 4 (CF=0)");
}

#[test]
fn test_rcr_dx_cl() {
    let code = [0x66, 0xd3, 0xda, 0xf4]; // RCR DX, CL
    let mut emu = emu64();
    emu.regs_mut().rdx = 0xFF00;
    emu.regs_mut().rcx = 0x08;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rdx & 0xFFFF, 0x00FF, "DX: 0xFF00 RCR 8 (CF=0)");
}

// ============================================================================
// 32-bit RCR tests
// ============================================================================

#[test]
fn test_rcr_eax_1() {
    let code = [0xd1, 0xd8, 0xf4]; // RCR EAX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0x86430ECA;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x43218765, "EAX: RCR 1 (CF=0)");
}

#[test]
fn test_rcr_eax_1_cf_set() {
    let code = [0xd1, 0xd8, 0xf4]; // RCR EAX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0x86430ECA;
    emu.flags_mut().load(0x2 | flags::F_CF); // CF set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xC3218765, "EAX: RCR 1 (CF=1)");
}

#[test]
fn test_rcr_eax_cl() {
    let code = [0xd3, 0xd8, 0xf4]; // RCR EAX, CL
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rcx = 0x08;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xF0123456, "EAX: RCR 8 (CF=0)");
}

#[test]
fn test_rcr_eax_imm8() {
    let code = [0xc1, 0xd8, 0x10, 0xf4]; // RCR EAX, 16
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xACF01234, "EAX: RCR 16 (CF=0)");
}

#[test]
fn test_rcr_eax_full_rotation() {
    // RCR by 33: count is masked to 5 bits (33 & 31 = 1), so this is RCR by 1
    let code = [0xc1, 0xd8, 0x21, 0xf4]; // RCR EAX, 33
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    // 33 & 31 = 1, so effective count is 1
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x091A2B3C, "EAX: RCR 33 (masked to 1)");
}

#[test]
fn test_rcr_ebx() {
    let code = [0xd1, 0xdb, 0xf4]; // RCR EBX, 1
    let mut emu = emu64();
    emu.regs_mut().rbx = 0x00000001;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0x00000000, "EBX: RCR 1 (CF=0)");
    assert!(emu.flags().f_cf, "CF: LSB was 1");
}

#[test]
fn test_rcr_ecx() {
    let code = [0xc1, 0xd9, 0x04, 0xf4]; // RCR ECX, 4
    let mut emu = emu64();
    emu.regs_mut().rcx = 0xABCDEF01;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFFFFFFFF, 0x2ABCDEF0, "ECX: RCR 4 (CF=0)");
}

#[test]
fn test_rcr_edx_cl() {
    let code = [0xd3, 0xda, 0xf4]; // RCR EDX, CL
    let mut emu = emu64();
    emu.regs_mut().rdx = 0xFF000000;
    emu.regs_mut().rcx = 0x18; // 24 bits
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rdx & 0xFFFFFFFF, 0x000000FF, "EDX: RCR 24 (CF=0)");
}

#[test]
fn test_rcr_esi() {
    let code = [0xd1, 0xde, 0xf4]; // RCR ESI, 1
    let mut emu = emu64();
    emu.regs_mut().rsi = 0x80000000;
    emu.flags_mut().load(0x2 | flags::F_CF); // CF set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsi & 0xFFFFFFFF, 0xC0000000, "ESI: RCR 1 (CF=1)");
}

#[test]
fn test_rcr_edi() {
    let code = [0xc1, 0xdf, 0x0C, 0xf4]; // RCR EDI, 12
    let mut emu = emu64();
    emu.regs_mut().rdi = 0x12345678;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rdi & 0xFFFFFFFF, 0xCF012345, "EDI: RCR 12 (CF=0)");
}

// ============================================================================
// 64-bit RCR tests
// ============================================================================

#[test]
fn test_rcr_rax_1() {
    let code = [0x48, 0xd1, 0xd8, 0xf4]; // RCR RAX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0x86430ECA86430ECA;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x4321876543218765, "RAX: RCR 1 (CF=0)");
}

#[test]
fn test_rcr_rax_1_cf_set() {
    let code = [0x48, 0xd1, 0xd8, 0xf4]; // RCR RAX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0x86430ECA86430ECA;
    emu.flags_mut().load(0x2 | flags::F_CF); // CF set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0xC321876543218765, "RAX: RCR 1 (CF=1)");
}

#[test]
fn test_rcr_rax_cl() {
    let code = [0x48, 0xd3, 0xd8, 0xf4]; // RCR RAX, CL
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rcx = 0x08;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0xE0123456789ABCDE, "RAX: RCR 8 (CF=0)");
}

#[test]
fn test_rcr_rax_imm8() {
    let code = [0x48, 0xc1, 0xd8, 0x10, 0xf4]; // RCR RAX, 16
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0xBDE0123456789ABC, "RAX: RCR 16 (CF=0)");
}

#[test]
fn test_rcr_rax_32bits() {
    let code = [0x48, 0xc1, 0xd8, 0x20, 0xf4]; // RCR RAX, 32
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x3579BDE012345678, "RAX: RCR 32 (CF=0)");
}

#[test]
fn test_rcr_rbx() {
    let code = [0x48, 0xd1, 0xdb, 0xf4]; // RCR RBX, 1
    let mut emu = emu64();
    emu.regs_mut().rbx = 0x0000000000000001;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rbx, 0x0000000000000000, "RBX: RCR 1 (CF=0)");
    assert!(emu.flags().f_cf, "CF: LSB was 1");
}

#[test]
fn test_rcr_rcx() {
    let code = [0x48, 0xc1, 0xd9, 0x04, 0xf4]; // RCR RCX, 4
    let mut emu = emu64();
    emu.regs_mut().rcx = 0xABCDEF0123456789;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x2ABCDEF012345678, "RCX: RCR 4 (CF=0)");
}

#[test]
fn test_rcr_rdx_cl() {
    let code = [0x48, 0xd3, 0xda, 0xf4]; // RCR RDX, CL
    let mut emu = emu64();
    emu.regs_mut().rdx = 0xFF00000000000000;
    emu.regs_mut().rcx = 0x38; // 56 bits
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rdx, 0x00000000000000FF, "RDX: RCR 56 (CF=0)");
}

#[test]
fn test_rcr_rsi() {
    let code = [0x48, 0xd1, 0xde, 0xf4]; // RCR RSI, 1
    let mut emu = emu64();
    emu.regs_mut().rsi = 0x8000000000000000;
    emu.flags_mut().load(0x2 | flags::F_CF); // CF set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsi, 0xC000000000000000, "RSI: RCR 1 (CF=1)");
}

#[test]
fn test_rcr_rdi() {
    let code = [0x48, 0xc1, 0xdf, 0x0C, 0xf4]; // RCR RDI, 12
    let mut emu = emu64();
    emu.regs_mut().rdi = 0x123456789ABCDEF0;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rdi, 0xDE0123456789ABCD, "RDI: RCR 12 (CF=0)");
}

#[test]
fn test_rcr_r8() {
    let code = [0x49, 0xd1, 0xd8, 0xf4]; // RCR R8, 1
    let mut emu = emu64();
    emu.regs_mut().r8 = 0xFEDCBA9876543210;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().r8, 0x7F6E5D4C3B2A1908, "R8: RCR 1 (CF=0)");
}

#[test]
fn test_rcr_r9_cl() {
    let code = [0x49, 0xd3, 0xd9, 0xf4]; // RCR R9, CL
    let mut emu = emu64();
    emu.regs_mut().r9 = 0x0123456789ABCDEF;
    emu.regs_mut().rcx = 0x10; // 16 bits
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().r9, 0x9BDE0123456789AB, "R9: RCR 16 (CF=0)");
}

#[test]
fn test_rcr_r10_imm8() {
    let code = [0x49, 0xc1, 0xda, 0x08, 0xf4]; // RCR R10, 8
    let mut emu = emu64();
    emu.regs_mut().r10 = 0x123456789ABCDEF0;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().r10, 0xE0123456789ABCDE, "R10: RCR 8 (CF=0)");
}

#[test]
fn test_rcr_r15() {
    let code = [0x49, 0xd1, 0xdf, 0xf4]; // RCR R15, 1
    let mut emu = emu64();
    emu.regs_mut().r15 = 0x2222222222222222;
    emu.flags_mut().load(0x2 | flags::F_CF); // CF set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().r15, 0x9111111111111111, "R15: RCR 1 (CF=1)");
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_rcr_mem8() {
    let DATA_ADDR = 0x7000;
    let code = [
        0xd0, 0x1c, 0x25, // RCR byte ptr [disp32], 1
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut emu = emu64();
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0x81);
    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_byte(DATA_ADDR).unwrap(), 0x40, "Memory: 0x81 RCR 1 (CF=0) = 0x40");
    assert!(emu.flags().f_cf, "CF: LSB was 1");
}

#[test]
fn test_rcr_mem16() {
    let DATA_ADDR = 0x7000;
    let code = [
        0x66, 0xc1, 0x1c, 0x25, // RCR word ptr [disp32], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x04, // imm8: 4
        0xf4,
    ];
    let mut emu = emu64();
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0x1234);
    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(DATA_ADDR).unwrap(), 0x8123, "Memory: 0x1234 RCR 4 (CF=0)");
}

#[test]
fn test_rcr_mem32() {
    let DATA_ADDR = 0x7000;
    let code = [
        0xd3, 0x1c, 0x25, // RCR dword ptr [disp32], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rcx = 0x08;
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x12345678);
    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(DATA_ADDR).unwrap(), 0xF0123456, "Memory: 0x12345678 RCR 8 (CF=0)");
}

#[test]
fn test_rcr_mem64() {
    let DATA_ADDR = 0x7000;
    let code = [
        0x48, 0xc1, 0x1c, 0x25, // RCR qword ptr [disp32], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x10, // imm8: 16
        0xf4,
    ];
    let mut emu = emu64();
    emu.flags_mut().load(0x2); // CF clear
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x123456789ABCDEF0);
    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_qword(DATA_ADDR).unwrap(), 0xBDE0123456789ABC, "Memory: RCR 16 (CF=0)");
}

