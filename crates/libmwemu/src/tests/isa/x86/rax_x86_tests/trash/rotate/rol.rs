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
    // ROL AL, 1 (opcode D0 /0)
    let code = [
        0xd0, 0xc0, // ROL AL, 1
        0xf4,       // HLT
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x42; // 0100_0010
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x84, "AL: 0x42 ROL 1 = 0x84");
    assert!(!emu.flags().f_cf, "CF: receives MSB (was 0)");
    assert!(emu.flags().f_of, "OF: CF XOR new MSB = 0 XOR 1 = 1");
}

#[test]
fn test_rol_al_1_with_msb() {
    // ROL AL, 1 with MSB set
    let code = [
        0xd0, 0xc0, // ROL AL, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x81; // 1000_0001
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03, "AL: 0x81 ROL 1 = 0x03 (MSB rotates to LSB)");
    assert!(emu.flags().f_cf, "CF: receives MSB (was 1)");
}

#[test]
fn test_rol_al_cl() {
    // ROL AL, CL (opcode D2 /0)
    let code = [
        0xd2, 0xc0, // ROL AL, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x01; // 0000_0001
    emu.regs_mut().rcx = 0x04; // Rotate by 4
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x10, "AL: 0x01 ROL 4 = 0x10");
    assert!(!emu.flags().f_cf, "CF: last bit rotated was 0");
}

#[test]
fn test_rol_al_imm8() {
    // ROL AL, imm8 (opcode C0 /0 ib)
    let code = [
        0xc0, 0xc0, 0x03, // ROL AL, 3
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x11; // 0001_0001
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x88, "AL: 0x11 ROL 3 = 0x88");
    assert!(!emu.flags().f_cf, "CF: last bit rotated was 0");
}

#[test]
fn test_rol_full_rotation_8bit() {
    // ROL by 8 should return to original value
    let code = [
        0xc0, 0xc0, 0x08, // ROL AL, 8
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL: full rotation returns to original");
}

#[test]
fn test_rol_count_masked_8bit() {
    // Count is masked for 8-bit operands
    let code = [
        0xd2, 0xc0, // ROL AL, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x11;
    emu.regs_mut().rcx = 0x1B; // 27 masked and modulo 8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 27 % 8 = 3 for 8-bit operand
    assert_eq!(emu.regs().rax & 0xFF, 0x88, "AL: rotation count masked");
}

#[test]
fn test_rol_count_zero_preserves_flags() {
    // Count of 0 should not affect flags
    let code = [
        0xc0, 0xc0, 0x00, // ROL AL, 0
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
fn test_rol_bl() {
    // ROL BL, 1
    let code = [
        0xd0, 0xc3, // ROL BL, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rbx = 0xC5; // 1100_0101
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFF, 0x8B, "BL: 0xC5 ROL 1 = 0x8B");
    assert!(emu.flags().f_cf, "CF: MSB was 1");
}

#[test]
fn test_rol_cl_reg() {
    // ROL CL, imm8
    let code = [
        0xc0, 0xc1, 0x02, // ROL CL, 2
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rcx = 0x33; // 0011_0011
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFF, 0xCC, "CL: 0x33 ROL 2 = 0xCC");
}

#[test]
fn test_rol_dl() {
    // ROL DL, CL
    let code = [
        0xd2, 0xc2, // ROL DL, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdx = 0x0F;
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx & 0xFF, 0xF0, "DL: 0x0F ROL 4 = 0xF0");
}

// ============================================================================
// 16-bit ROL tests
// ============================================================================

#[test]
fn test_rol_ax_1() {
    // ROL AX, 1 (opcode 66 D1 /0)
    let code = [
        0x66, 0xd1, 0xc0, // ROL AX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x4321; // 0100_0011_0010_0001
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x8642, "AX: 0x4321 ROL 1 = 0x8642");
    assert!(!emu.flags().f_cf, "CF: MSB was 0");
}

#[test]
fn test_rol_ax_cl() {
    // ROL AX, CL (opcode 66 D3 /0)
    let code = [
        0x66, 0xd3, 0xc0, // ROL AX, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x1234;
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x2341, "AX: 0x1234 ROL 4 = 0x2341");
}

#[test]
fn test_rol_ax_imm8() {
    // ROL AX, imm8 (opcode 66 C1 /0 ib)
    let code = [
        0x66, 0xc1, 0xc0, 0x08, // ROL AX, 8
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x3412, "AX: 0x1234 ROL 8 = 0x3412 (byte swap)");
}

#[test]
fn test_rol_ax_full_rotation() {
    // ROL by 16 should return to original value
    let code = [
        0x66, 0xc1, 0xc0, 0x10, // ROL AX, 16
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x1234, "AX: full rotation returns to original");
}

#[test]
fn test_rol_bx() {
    // ROL BX, 1
    let code = [
        0x66, 0xd1, 0xc3, // ROL BX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rbx = 0x8000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFF, 0x0001, "BX: 0x8000 ROL 1 = 0x0001");
    assert!(emu.flags().f_cf, "CF: MSB was 1");
}

#[test]
fn test_rol_cx() {
    // ROL CX, imm8
    let code = [
        0x66, 0xc1, 0xc1, 0x04, // ROL CX, 4
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rcx = 0xABCD;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFFFF, 0xBCDA, "CX: 0xABCD ROL 4 = 0xBCDA");
}

#[test]
fn test_rol_dx_cl() {
    // ROL DX, CL
    let code = [
        0x66, 0xd3, 0xc2, // ROL DX, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdx = 0x00FF;
    emu.regs_mut().rcx = 0x08;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx & 0xFFFF, 0xFF00, "DX: 0x00FF ROL 8 = 0xFF00");
}

// ============================================================================
// 32-bit ROL tests
// ============================================================================

#[test]
fn test_rol_eax_1() {
    // ROL EAX, 1 (opcode D1 /0)
    let code = [
        0xd1, 0xc0, // ROL EAX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x43218765;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x86430ECA, "EAX: 0x43218765 ROL 1 = 0x86430ECA");
}

#[test]
fn test_rol_eax_cl() {
    // ROL EAX, CL (opcode D3 /0)
    let code = [
        0xd3, 0xc0, // ROL EAX, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rcx = 0x08;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x34567812, "EAX: 0x12345678 ROL 8 = 0x34567812");
}

#[test]
fn test_rol_eax_imm8() {
    // ROL EAX, imm8 (opcode C1 /0 ib)
    let code = [
        0xc1, 0xc0, 0x10, // ROL EAX, 16
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x56781234, "EAX: 0x12345678 ROL 16 = 0x56781234");
}

#[test]
fn test_rol_eax_full_rotation() {
    // ROL by 32 should return to original value
    let code = [
        0xc1, 0xc0, 0x20, // ROL EAX, 32
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x12345678, "EAX: full rotation returns to original");
}

#[test]
fn test_rol_ebx() {
    // ROL EBX, 1
    let code = [
        0xd1, 0xc3, // ROL EBX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rbx = 0x80000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0x00000001, "EBX: 0x80000000 ROL 1 = 0x00000001");
    assert!(emu.flags().f_cf, "CF: MSB was 1");
}

#[test]
fn test_rol_ecx() {
    // ROL ECX, imm8
    let code = [
        0xc1, 0xc1, 0x04, // ROL ECX, 4
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rcx = 0xABCDEF01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx & 0xFFFFFFFF, 0xBCDEF01A, "ECX: 0xABCDEF01 ROL 4 = 0xBCDEF01A");
}

#[test]
fn test_rol_edx_cl() {
    // ROL EDX, CL
    let code = [
        0xd3, 0xc2, // ROL EDX, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdx = 0x000000FF;
    emu.regs_mut().rcx = 0x18; // 24 bits
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx & 0xFFFFFFFF, 0xFF000000, "EDX: 0x000000FF ROL 24 = 0xFF000000");
}

#[test]
fn test_rol_esi() {
    // ROL ESI, 1
    let code = [
        0xd1, 0xc6, // ROL ESI, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rsi = 0x40000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi & 0xFFFFFFFF, 0x80000000, "ESI: 0x40000000 ROL 1 = 0x80000000");
}

#[test]
fn test_rol_edi() {
    // ROL EDI, imm8
    let code = [
        0xc1, 0xc7, 0x0C, // ROL EDI, 12
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdi = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdi & 0xFFFFFFFF, 0x45678123, "EDI: 0x12345678 ROL 12 = 0x45678123");
}

// ============================================================================
// 64-bit ROL tests
// ============================================================================

#[test]
fn test_rol_rax_1() {
    // ROL RAX, 1 (opcode REX.W D1 /0)
    let code = [
        0x48, 0xd1, 0xc0, // ROL RAX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x4321876543218765;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x86430ECA86430ECA, "RAX: ROL 1");
}

#[test]
fn test_rol_rax_cl() {
    // ROL RAX, CL (opcode REX.W D3 /0)
    let code = [
        0x48, 0xd3, 0xc0, // ROL RAX, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rcx = 0x08;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x3456789ABCDEF012, "RAX: 0x123456789ABCDEF0 ROL 8");
}

#[test]
fn test_rol_rax_imm8() {
    // ROL RAX, imm8 (opcode REX.W C1 /0 ib)
    let code = [
        0x48, 0xc1, 0xc0, 0x10, // ROL RAX, 16
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x56789ABCDEF01234, "RAX: 0x123456789ABCDEF0 ROL 16");
}

#[test]
fn test_rol_rax_32bits() {
    // ROL RAX, 32 (half rotation)
    let code = [
        0x48, 0xc1, 0xc0, 0x20, // ROL RAX, 32
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x9ABCDEF012345678, "RAX: ROL 32 swaps high/low dwords");
}

#[test]
fn test_rol_rax_full_rotation() {
    // ROL by 64 should return to original value
    let code = [
        0x48, 0xc1, 0xc0, 0x40, // ROL RAX, 64
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "RAX: full rotation returns to original");
}

#[test]
fn test_rol_rbx() {
    // ROL RBX, 1
    let code = [
        0x48, 0xd1, 0xc3, // ROL RBX, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rbx = 0x8000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0x0000000000000001, "RBX: 0x8000000000000000 ROL 1 = 0x0000000000000001");
    assert!(emu.flags().f_cf, "CF: MSB was 1");
}

#[test]
fn test_rol_rcx() {
    // ROL RCX, imm8
    let code = [
        0x48, 0xc1, 0xc1, 0x04, // ROL RCX, 4
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rcx = 0xABCDEF0123456789;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rcx, 0xBCDEF0123456789A, "RCX: ROL 4");
}

#[test]
fn test_rol_rdx_cl() {
    // ROL RDX, CL
    let code = [
        0x48, 0xd3, 0xc2, // ROL RDX, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdx = 0x00000000000000FF;
    emu.regs_mut().rcx = 0x38; // 56 bits
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx, 0xFF00000000000000, "RDX: ROL 56");
}

#[test]
fn test_rol_rsi() {
    // ROL RSI, 1
    let code = [
        0x48, 0xd1, 0xc6, // ROL RSI, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rsi = 0x4000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsi, 0x8000000000000000, "RSI: ROL 1");
}

#[test]
fn test_rol_rdi() {
    // ROL RDI, imm8
    let code = [
        0x48, 0xc1, 0xc7, 0x0C, // ROL RDI, 12
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdi = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdi, 0x456789ABCDEF0123, "RDI: ROL 12");
}

#[test]
fn test_rol_r8() {
    // ROL R8, 1 (REX.WB D1 /0)
    let code = [
        0x49, 0xd1, 0xc0, // ROL R8, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().r8 = 0xFEDCBA9876543210;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8, 0xFDB97530ECA86421, "R8: ROL 1");
    assert!(emu.flags().f_cf, "CF: MSB was 1");
}

#[test]
fn test_rol_r9_cl() {
    // ROL R9, CL
    let code = [
        0x49, 0xd3, 0xc1, // ROL R9, CL
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().r9 = 0x0123456789ABCDEF;
    emu.regs_mut().rcx = 0x10; // 16 bits
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r9, 0x456789ABCDEF0123, "R9: ROL 16");
}

#[test]
fn test_rol_r10_imm8() {
    // ROL R10, imm8
    let code = [
        0x49, 0xc1, 0xc2, 0x08, // ROL R10, 8
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().r10 = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10, 0x3456789ABCDEF012, "R10: ROL 8");
}

#[test]
fn test_rol_r15() {
    // ROL R15, 1
    let code = [
        0x49, 0xd1, 0xc7, // ROL R15, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().r15 = 0x1111111111111111;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 0x2222222222222222, "R15: ROL 1");
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_rol_mem8() {
    let DATA_ADDR = 0x7000;

    // ROL byte [DATA_ADDR], 1
    let code = [
        0xd0, 0x04, 0x25, // ROL byte ptr [disp32], 1
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0x81);
    emu.run(None).unwrap();

    assert_eq!(emu.maps.read_byte(DATA_ADDR).unwrap(), 0x03, "Memory: 0x81 ROL 1 = 0x03");
    assert!(emu.flags().f_cf, "CF: MSB was 1");
}

#[test]
fn test_rol_mem16() {
    let DATA_ADDR = 0x7000;

    // ROL word [DATA_ADDR], 4
    let code = [
        0x66, 0xc1, 0x04, 0x25, // ROL word ptr [disp32], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x04, // imm8: 4
        0xf4,
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0x1234);
    emu.run(None).unwrap();

    assert_eq!(emu.maps.read_word(DATA_ADDR).unwrap(), 0x2341, "Memory: 0x1234 ROL 4 = 0x2341");
}

#[test]
fn test_rol_mem32() {
    let DATA_ADDR = 0x7000;

    // ROL dword [DATA_ADDR], CL
    let code = [
        0xd3, 0x04, 0x25, // ROL dword ptr [disp32], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rcx = 0x08;
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x12345678);
    emu.run(None).unwrap();

    assert_eq!(emu.maps.read_dword(DATA_ADDR).unwrap(), 0x34567812, "Memory: 0x12345678 ROL 8 = 0x34567812");
}

#[test]
fn test_rol_mem64() {
    let DATA_ADDR = 0x7000;

    // ROL qword [DATA_ADDR], 16
    let code = [
        0x48, 0xc1, 0x04, 0x25, // ROL qword ptr [disp32], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x10, // imm8: 16
        0xf4,
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x123456789ABCDEF0);
    emu.run(None).unwrap();

    assert_eq!(emu.maps.read_qword(DATA_ADDR).unwrap(), 0x56789ABCDEF01234, "Memory: ROL 16");
}
