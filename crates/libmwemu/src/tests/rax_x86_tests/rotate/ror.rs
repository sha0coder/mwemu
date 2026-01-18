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
// - OF: Only for 1-bit rotates (MSB XOR (MSB-1))
// - Other flags: Undefined
// - Count is 0: No flags affected

use crate::*;

// ============================================================================
// 8-bit ROR tests
// ============================================================================

#[test]
fn test_ror_al_1() {
    // ROR AL, 1 (opcode D0 /1)
    let code = [
        0xd0, 0xc8, // ROR AL, 1
        0xf4,       // HLT
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x42; // 0100_0010
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x21, "AL: 0x42 ROR 1 = 0x21");
    assert!(!emu.flags().f_cf, "CF: receives LSB (was 0)");
    assert!(!emu.flags().f_of, "OF: MSB XOR (MSB-1) = 0 XOR 0 = 0");
}

#[test]
fn test_ror_al_1_with_lsb() {
    // ROR AL, 1 with LSB set
    let code = [
        0xd0, 0xc8, // ROR AL, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x81; // 1000_0001
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xC0, "AL: 0x81 ROR 1 = 0xC0 (LSB rotates to MSB)");
    assert!(emu.flags().f_cf, "CF: receives LSB (was 1)");
    // OF = XOR of two most-significant bits of result: 0xC0 = 1100_0000, MSB=1, MSB-1=1, so 1 XOR 1 = 0
    assert!(!emu.flags().f_of, "OF: MSB XOR (MSB-1) = 1 XOR 1 = 0");
}

#[test]
fn test_ror_al_cl() {
    // ROR AL, CL (opcode D2 /1)
    let code = [
        0xd2, 0xc8, // ROR AL, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x10; // 0001_0000
    emu.regs_mut().rcx = 0x04; // Rotate by 4
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x01, "AL: 0x10 ROR 4 = 0x01");
    assert!(!emu.flags().f_cf, "CF: last bit rotated was 0");
}

#[test]
fn test_ror_al_imm8() {
    // ROR AL, imm8 (opcode C0 /1 ib)
    let code = [
        0xc0, 0xc8, 0x03, // ROR AL, 3
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x88; // 1000_1000
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x11, "AL: 0x88 ROR 3 = 0x11");
    assert!(!emu.flags().f_cf, "CF: last bit rotated was 0");
}

#[test]
fn test_ror_full_rotation_8bit() {
    // ROR by 8 should return to original value
    let code = [
        0xc0, 0xc8, 0x08, // ROR AL, 8
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL: full rotation returns to original");
}

#[test]
fn test_ror_count_masked_8bit() {
    // Count is masked for 8-bit operands
    let code = [
        0xd2, 0xc8, // ROR AL, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x88;
    emu.regs_mut().rcx = 0x1B; // 27 masked and modulo 8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 27 % 8 = 3 for 8-bit operand
    assert_eq!(emu.regs().rax & 0xFF, 0x11, "AL: rotation count masked");
}

#[test]
fn test_ror_count_zero_preserves_flags() {
    // Count of 0 should not affect flags
    let code = [
        0xc0, 0xc8, 0x00, // ROR AL, 0
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
fn test_ror_bl() {
    // ROR BL, 1
    let code = [
        0xd0, 0xcb, // ROR BL, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rbx = 0xC5; // 1100_0101
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFF, 0xE2, "BL: 0xC5 ROR 1 = 0xE2");
    assert!(emu.flags().f_cf, "CF: LSB was 1");
}

#[test]
fn test_ror_cl_reg() {
    // ROR CL, imm8
    let code = [
        0xc0, 0xc9, 0x02, // ROR CL, 2
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rcx = 0xCC; // 1100_1100
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFF, 0x33, "CL: 0xCC ROR 2 = 0x33");
}

#[test]
fn test_ror_dl() {
    // ROR DL, CL
    let code = [
        0xd2, 0xca, // ROR DL, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdx = 0xF0;
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx & 0xFF, 0x0F, "DL: 0xF0 ROR 4 = 0x0F");
}

// ============================================================================
// 16-bit ROR tests
// ============================================================================

#[test]
fn test_ror_ax_1() {
    // ROR AX, 1 (opcode 66 D1 /1)
    let code = [
        0x66, 0xd1, 0xc8, // ROR AX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x8642; // 1000_0110_0100_0010
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x4321, "AX: 0x8642 ROR 1 = 0x4321");
    assert!(!emu.flags().f_cf, "CF: LSB was 0");
}

#[test]
fn test_ror_ax_cl() {
    // ROR AX, CL (opcode 66 D3 /1)
    let code = [
        0x66, 0xd3, 0xc8, // ROR AX, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x1234;
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x4123, "AX: 0x1234 ROR 4 = 0x4123");
}

#[test]
fn test_ror_ax_imm8() {
    // ROR AX, imm8 (opcode 66 C1 /1 ib)
    let code = [
        0x66, 0xc1, 0xc8, 0x08, // ROR AX, 8
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x3412, "AX: 0x1234 ROR 8 = 0x3412 (byte swap)");
}

#[test]
fn test_ror_ax_full_rotation() {
    // ROR by 16 should return to original value
    let code = [
        0x66, 0xc1, 0xc8, 0x10, // ROR AX, 16
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x1234, "AX: full rotation returns to original");
}

#[test]
fn test_ror_bx() {
    // ROR BX, 1
    let code = [
        0x66, 0xd1, 0xcb, // ROR BX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rbx = 0x0001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFF, 0x8000, "BX: 0x0001 ROR 1 = 0x8000");
    assert!(emu.flags().f_cf, "CF: LSB was 1");
}

#[test]
fn test_ror_cx() {
    // ROR CX, imm8
    let code = [
        0x66, 0xc1, 0xc9, 0x04, // ROR CX, 4
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rcx = 0xABCD;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFFFF, 0xDABC, "CX: 0xABCD ROR 4 = 0xDABC");
}

#[test]
fn test_ror_dx_cl() {
    // ROR DX, CL
    let code = [
        0x66, 0xd3, 0xca, // ROR DX, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdx = 0xFF00;
    emu.regs_mut().rcx = 0x08;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx & 0xFFFF, 0x00FF, "DX: 0xFF00 ROR 8 = 0x00FF");
}

// ============================================================================
// 32-bit ROR tests
// ============================================================================

#[test]
fn test_ror_eax_1() {
    // ROR EAX, 1 (opcode D1 /1)
    let code = [
        0xd1, 0xc8, // ROR EAX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x86430ECA;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x43218765, "EAX: 0x86430ECA ROR 1 = 0x43218765");
}

#[test]
fn test_ror_eax_cl() {
    // ROR EAX, CL (opcode D3 /1)
    let code = [
        0xd3, 0xc8, // ROR EAX, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rcx = 0x08;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x78123456, "EAX: 0x12345678 ROR 8 = 0x78123456");
}

#[test]
fn test_ror_eax_imm8() {
    // ROR EAX, imm8 (opcode C1 /1 ib)
    let code = [
        0xc1, 0xc8, 0x10, // ROR EAX, 16
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x56781234, "EAX: 0x12345678 ROR 16 = 0x56781234");
}

#[test]
fn test_ror_eax_full_rotation() {
    // ROR by 32 should return to original value
    let code = [
        0xc1, 0xc8, 0x20, // ROR EAX, 32
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x12345678, "EAX: full rotation returns to original");
}

#[test]
fn test_ror_ebx() {
    // ROR EBX, 1
    let code = [
        0xd1, 0xcb, // ROR EBX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rbx = 0x00000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0x80000000, "EBX: 0x00000001 ROR 1 = 0x80000000");
    assert!(emu.flags().f_cf, "CF: LSB was 1");
}

#[test]
fn test_ror_ecx() {
    // ROR ECX, imm8
    let code = [
        0xc1, 0xc9, 0x04, // ROR ECX, 4
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rcx = 0xABCDEF01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFFFFFFFF, 0x1ABCDEF0, "ECX: 0xABCDEF01 ROR 4 = 0x1ABCDEF0");
}

#[test]
fn test_ror_edx_cl() {
    // ROR EDX, CL
    let code = [
        0xd3, 0xca, // ROR EDX, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdx = 0xFF000000;
    emu.regs_mut().rcx = 0x18; // 24 bits
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx & 0xFFFFFFFF, 0x000000FF, "EDX: 0xFF000000 ROR 24 = 0x000000FF");
}

#[test]
fn test_ror_esi() {
    // ROR ESI, 1
    let code = [
        0xd1, 0xce, // ROR ESI, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rsi = 0x80000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi & 0xFFFFFFFF, 0x40000000, "ESI: 0x80000000 ROR 1 = 0x40000000");
}

#[test]
fn test_ror_edi() {
    // ROR EDI, imm8
    let code = [
        0xc1, 0xcf, 0x0C, // ROR EDI, 12
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdi = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdi & 0xFFFFFFFF, 0x67812345, "EDI: 0x12345678 ROR 12 = 0x67812345");
}

// ============================================================================
// 64-bit ROR tests
// ============================================================================

#[test]
fn test_ror_rax_1() {
    // ROR RAX, 1 (opcode REX.W D1 /1)
    let code = [
        0x48, 0xd1, 0xc8, // ROR RAX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x86430ECA86430ECA;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x4321876543218765, "RAX: ROR 1");
}

#[test]
fn test_ror_rax_cl() {
    // ROR RAX, CL (opcode REX.W D3 /1)
    let code = [
        0x48, 0xd3, 0xc8, // ROR RAX, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rcx = 0x08;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xF0123456789ABCDE, "RAX: 0x123456789ABCDEF0 ROR 8");
}

#[test]
fn test_ror_rax_imm8() {
    // ROR RAX, imm8 (opcode REX.W C1 /1 ib)
    let code = [
        0x48, 0xc1, 0xc8, 0x10, // ROR RAX, 16
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xDEF0123456789ABC, "RAX: 0x123456789ABCDEF0 ROR 16");
}

#[test]
fn test_ror_rax_32bits() {
    // ROR RAX, 32 (half rotation)
    let code = [
        0x48, 0xc1, 0xc8, 0x20, // ROR RAX, 32
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x9ABCDEF012345678, "RAX: ROR 32 swaps high/low dwords");
}

#[test]
fn test_ror_rax_full_rotation() {
    // ROR by 64 should return to original value
    let code = [
        0x48, 0xc1, 0xc8, 0x40, // ROR RAX, 64
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "RAX: full rotation returns to original");
}

#[test]
fn test_ror_rbx() {
    // ROR RBX, 1
    let code = [
        0x48, 0xd1, 0xcb, // ROR RBX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rbx = 0x0000000000000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0x8000000000000000, "RBX: 0x0000000000000001 ROR 1 = 0x8000000000000000");
    assert!(emu.flags().f_cf, "CF: LSB was 1");
}

#[test]
fn test_ror_rcx() {
    // ROR RCX, imm8
    let code = [
        0x48, 0xc1, 0xc9, 0x04, // ROR RCX, 4
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rcx = 0xABCDEF0123456789;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx, 0x9ABCDEF012345678, "RCX: ROR 4");
}

#[test]
fn test_ror_rdx_cl() {
    // ROR RDX, CL
    let code = [
        0x48, 0xd3, 0xca, // ROR RDX, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdx = 0xFF00000000000000;
    emu.regs_mut().rcx = 0x38; // 56 bits
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx, 0x00000000000000FF, "RDX: ROR 56");
}

#[test]
fn test_ror_rsi() {
    // ROR RSI, 1
    let code = [
        0x48, 0xd1, 0xce, // ROR RSI, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rsi = 0x8000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi, 0x4000000000000000, "RSI: ROR 1");
}

#[test]
fn test_ror_rdi() {
    // ROR RDI, imm8
    let code = [
        0x48, 0xc1, 0xcf, 0x0C, // ROR RDI, 12
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdi = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdi, 0xEF0123456789ABCD, "RDI: ROR 12");
}

#[test]
fn test_ror_r8() {
    // ROR R8, 1 (REX.WB D1 /1)
    let code = [
        0x49, 0xd1, 0xc8, // ROR R8, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().r8 = 0xFEDCBA9876543210;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8, 0x7F6E5D4C3B2A1908, "R8: ROR 1");
    assert!(!emu.flags().f_cf, "CF: LSB was 0");
}

#[test]
fn test_ror_r9_cl() {
    // ROR R9, CL
    let code = [
        0x49, 0xd3, 0xc9, // ROR R9, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().r9 = 0x0123456789ABCDEF;
    emu.regs_mut().rcx = 0x10; // 16 bits
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r9, 0xCDEF0123456789AB, "R9: ROR 16");
}

#[test]
fn test_ror_r10_imm8() {
    // ROR R10, imm8
    let code = [
        0x49, 0xc1, 0xca, 0x08, // ROR R10, 8
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().r10 = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10, 0xF0123456789ABCDE, "R10: ROR 8");
}

#[test]
fn test_ror_r15() {
    // ROR R15, 1
    let code = [
        0x49, 0xd1, 0xcf, // ROR R15, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().r15 = 0x2222222222222222;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 0x1111111111111111, "R15: ROR 1");
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_ror_mem8() {
    let DATA_ADDR = 0x7000;

    // ROR byte [DATA_ADDR], 1
    let code = [
        0xd0, 0x0c, 0x25, // ROR byte ptr [disp32], 1
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut emu = emu64();
    emu.maps.create_map("test_data", 0x7000, 0x1000, crate::maps::mem64::Permission::READ_WRITE).expect("failed to map test_data");
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0x81);
    emu.run(None).unwrap();

    assert_eq!(emu.maps.read_byte(DATA_ADDR).unwrap(), 0xC0, "Memory: 0x81 ROR 1 = 0xC0");
    assert!(emu.flags().f_cf, "CF: LSB was 1");
}

#[test]
fn test_ror_mem16() {
    let DATA_ADDR = 0x7000;

    // ROR word [DATA_ADDR], 4
    let code = [
        0x66, 0xc1, 0x0c, 0x25, // ROR word ptr [disp32], imm8
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
    emu.maps.write_word(DATA_ADDR, 0x1234);
    emu.run(None).unwrap();

    assert_eq!(emu.maps.read_word(DATA_ADDR).unwrap(), 0x4123, "Memory: 0x1234 ROR 4 = 0x4123");
}

#[test]
fn test_ror_mem32() {
    let DATA_ADDR = 0x7000;

    // ROR dword [DATA_ADDR], CL
    let code = [
        0xd3, 0x0c, 0x25, // ROR dword ptr [disp32], CL
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
    emu.maps.write_dword(DATA_ADDR, 0x12345678);
    emu.run(None).unwrap();

    assert_eq!(emu.maps.read_dword(DATA_ADDR).unwrap(), 0x78123456, "Memory: 0x12345678 ROR 8 = 0x78123456");
}

#[test]
fn test_ror_mem64() {
    let DATA_ADDR = 0x7000;

    // ROR qword [DATA_ADDR], 16
    let code = [
        0x48, 0xc1, 0x0c, 0x25, // ROR qword ptr [disp32], imm8
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
    emu.maps.write_qword(DATA_ADDR, 0x123456789ABCDEF0);
    emu.run(None).unwrap();

    assert_eq!(emu.maps.read_qword(DATA_ADDR).unwrap(), 0xDEF0123456789ABC, "Memory: ROR 16");
}
