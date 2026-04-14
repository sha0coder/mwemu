// SHRD (Double Precision Shift Right) instruction tests
//
// Opcodes:
// 0F AC /r ib      SHRD r/m16, r16, imm8
// 0F AD /r         SHRD r/m16, r16, CL
// 0F AC /r ib      SHRD r/m32, r32, imm8
// 0F AD /r         SHRD r/m32, r32, CL
// REX.W + 0F AC /r ib  SHRD r/m64, r64, imm8
// REX.W + 0F AD /r     SHRD r/m64, r64, CL
//
// SHRD shifts the destination operand right by count bits.
// Bits shifted in from the left come from the source operand.
// Used for multi-precision shifts of 64 bits or more.
//
// Flags:
// - CF: Last bit shifted out of destination
// - OF: Only for 1-bit shifts (MSB changes)
// - SF, ZF, PF: Set according to result
// - AF: Undefined for non-zero count
// - Count is 0: No flags affected

use crate::*;

// ============================================================================
// 16-bit SHRD tests
// ============================================================================

#[test]
fn test_shrd_ax_bx_imm8() {
    let code = [0x66, 0x0f, 0xac, 0xd8, 0x04, 0xf4]; // SHRD AX, BX, 4; HLT
    let mut emu = emu64();
    emu.regs_mut().rax = 0x1234; // Destination
    emu.regs_mut().rbx = 0xABCD; // Source
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    // AX: 0001_0010_0011_0100 shifted right by 4
    // Bits from BX (1010_1011_1100_1101) fill from left
    // Result: 1101_0001_0010_0011
    assert_eq!(emu.regs().rax & 0xFFFF, 0xD123, "AX: 0x1234 SHRD 4 from 0xABCD = 0xD123");
    assert!(!emu.flags().f_cf, "CF: bit shifted out was 0");
}

#[test]
fn test_shrd_ax_bx_cl() {
    let code = [0x66, 0x0f, 0xad, 0xd8, 0xf4]; // SHRD AX, BX, CL; HLT
    let mut emu = emu64();
    emu.regs_mut().rax = 0x1234;
    emu.regs_mut().rbx = 0xABCD;
    emu.regs_mut().rcx = 0x08; // Shift by 8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0xCD12, "AX: 0x1234 SHRD 8 from 0xABCD = 0xCD12");
}

#[test]
fn test_shrd_ax_bx_1bit() {
    let code = [0x66, 0x0f, 0xac, 0xd8, 0x01, 0xf4]; // SHRD AX, BX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0x0002; // 0000_0000_0000_0010
    emu.regs_mut().rbx = 0x0001; // Will put 1 in MSB position
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0x8001, "AX: 0x0002 SHRD 1 with MSB from source");
    assert!(!emu.flags().f_cf, "CF: bit shifted out was 0");
    assert!(emu.flags().f_of, "OF: sign changed from + to -");
}

#[test]
fn test_shrd_ax_full_replacement() {
    let code = [0x66, 0x0f, 0xac, 0xd8, 0x10, 0xf4]; // SHRD AX, BX, 16
    let mut emu = emu64();
    emu.regs_mut().rax = 0x1234;
    emu.regs_mut().rbx = 0xABCD;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0xABCD, "AX: completely replaced by BX");
}

#[test]
fn test_shrd_cx_dx_imm8() {
    let code = [0x66, 0x0f, 0xac, 0xd1, 0x04, 0xf4]; // SHRD CX, DX, 4
    let mut emu = emu64();
    emu.regs_mut().rcx = 0x5678;
    emu.regs_mut().rdx = 0x9ABC;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFFFF, 0xC567, "CX: 0x5678 SHRD 4 from 0x9ABC");
}

#[test]
fn test_shrd_dx_si_cl() {
    let code = [0x66, 0x0f, 0xad, 0xf2, 0xf4]; // SHRD DX, SI, CL
    let mut emu = emu64();
    emu.regs_mut().rdx = 0x1111;
    emu.regs_mut().rsi = 0x2222;
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rdx & 0xFFFF, 0x2111, "DX: 0x1111 SHRD 4 from 0x2222");
}

#[test]
fn test_shrd_ax_zero_count() {
    let code = [0x66, 0x0f, 0xac, 0xd8, 0x00, 0xf4]; // SHRD AX, BX, 0
    let mut emu = emu64();
    emu.regs_mut().rax = 0x1234;
    emu.regs_mut().rbx = 0xABCD;
    emu.flags_mut().load(0x2 | flags::F_CF | flags::F_OF);
    let initial_flags = emu.flags().dump();
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0x1234, "AX: unchanged with count 0");
    assert_eq!(emu.flags().dump() & (flags::F_CF | flags::F_OF),
               initial_flags & (flags::F_CF | flags::F_OF), "Flags preserved");
}

// ============================================================================
// 32-bit SHRD tests
// ============================================================================

#[test]
fn test_shrd_eax_ebx_imm8() {
    // SHRD shifts right, low bits of source fill high bits of dest
    // EAX >> 4 = 0x01234567, EBX low 4 bits (0x1) fill high 4 bits
    let code = [0x0f, 0xac, 0xd8, 0x04, 0xf4]; // SHRD EAX, EBX, 4
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0xABCDEF01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x11234567, "EAX: 0x12345678 SHRD 4 from 0xABCDEF01");
}

#[test]
fn test_shrd_eax_ebx_cl() {
    let code = [0x0f, 0xad, 0xd8, 0xf4]; // SHRD EAX, EBX, CL
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0xABCDEF01;
    emu.regs_mut().rcx = 0x08;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x01123456, "EAX: 0x12345678 SHRD 8 from 0xABCDEF01");
}

#[test]
fn test_shrd_eax_carry_flag() {
    let code = [0x0f, 0xac, 0xd8, 0x04, 0xf4]; // SHRD EAX, EBX, 4
    let mut emu = emu64();
    emu.regs_mut().rax = 0x0000000F; // LSBs set
    emu.regs_mut().rbx = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert!(emu.flags().f_cf, "CF: LSBs were shifted out");
}

#[test]
fn test_shrd_eax_1bit() {
    let code = [0x0f, 0xac, 0xd8, 0x01, 0xf4]; // SHRD EAX, EBX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0x00000002;
    emu.regs_mut().rbx = 0x00000001; // MSB will be set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x80000001, "EAX: SHRD 1 with MSB");
    assert!(emu.flags().f_of, "OF: sign changed");
}

#[test]
fn test_shrd_eax_full_replacement() {
    // SHRD EAX, EBX, 32: count is masked to 32 & 31 = 0, so no shift occurs
    let code = [0x0f, 0xac, 0xd8, 0x20, 0xf4]; // SHRD EAX, EBX, 32
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0xABCDEF01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x12345678, "EAX: count masked to 0, no shift");
}

#[test]
fn test_shrd_ecx_edx_imm8() {
    let code = [0x0f, 0xac, 0xd1, 0x0C, 0xf4]; // SHRD ECX, EDX, 12
    let mut emu = emu64();
    emu.regs_mut().rcx = 0x11111111;
    emu.regs_mut().rdx = 0x22222222;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFFFFFFFF, 0x22211111, "ECX: SHRD 12");
}

#[test]
fn test_shrd_edx_esi_cl() {
    let code = [0x0f, 0xad, 0xf2, 0xf4]; // SHRD EDX, ESI, CL
    let mut emu = emu64();
    emu.regs_mut().rdx = 0xFFFFFFFF;
    emu.regs_mut().rsi = 0x00000000;
    emu.regs_mut().rcx = 0x10; // 16 bits
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rdx & 0xFFFFFFFF, 0x0000FFFF, "EDX: SHRD 16");
}

#[test]
fn test_shrd_esi_edi_imm8() {
    let code = [0x0f, 0xac, 0xfe, 0x08, 0xf4]; // SHRD ESI, EDI, 8
    let mut emu = emu64();
    emu.regs_mut().rsi = 0xAA55AA55;
    emu.regs_mut().rdi = 0x11223344;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsi & 0xFFFFFFFF, 0x44AA55AA, "ESI: SHRD 8");
}

#[test]
fn test_shrd_edi_ebx_cl() {
    let code = [0x0f, 0xad, 0xdf, 0xf4]; // SHRD EDI, EBX, CL
    let mut emu = emu64();
    emu.regs_mut().rdi = 0x01234567;
    emu.regs_mut().rbx = 0x89ABCDEF;
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rdi & 0xFFFFFFFF, 0xF0123456, "EDI: SHRD 4");
}

#[test]
fn test_shrd_eax_zero_count() {
    let code = [0x0f, 0xac, 0xd8, 0x00, 0xf4]; // SHRD EAX, EBX, 0
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0xABCDEF01;
    emu.flags_mut().load(0x2 | flags::F_CF);
    let initial = emu.regs().rax & 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, initial, "EAX: unchanged");
}

// ============================================================================
// 64-bit SHRD tests
// ============================================================================

#[test]
fn test_shrd_rax_rbx_imm8() {
    let code = [0x48, 0x0f, 0xac, 0xd8, 0x04, 0xf4]; // SHRD RAX, RBX, 4
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rbx = 0xFEDCBA9876543210;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x0123456789ABCDEF, "RAX: SHRD 4");
}

#[test]
fn test_shrd_rax_rbx_cl() {
    let code = [0x48, 0x0f, 0xad, 0xd8, 0xf4]; // SHRD RAX, RBX, CL
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rbx = 0xFEDCBA9876543210;
    emu.regs_mut().rcx = 0x08;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x10123456789ABCDE, "RAX: SHRD 8");
}

#[test]
fn test_shrd_rax_1bit() {
    let code = [0x48, 0x0f, 0xac, 0xd8, 0x01, 0xf4]; // SHRD RAX, RBX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0x0000000000000002;
    emu.regs_mut().rbx = 0x0000000000000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x8000000000000001, "RAX: SHRD 1");
    assert!(emu.flags().f_of, "OF: sign changed");
}

#[test]
fn test_shrd_rax_carry() {
    let code = [0x48, 0x0f, 0xac, 0xd8, 0x08, 0xf4]; // SHRD RAX, RBX, 8
    let mut emu = emu64();
    emu.regs_mut().rax = 0x00000000000000FF;
    emu.regs_mut().rbx = 0x0000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert!(emu.flags().f_cf, "CF: bits shifted out");
}

#[test]
fn test_shrd_rax_16bits() {
    let code = [0x48, 0x0f, 0xac, 0xd8, 0x10, 0xf4]; // SHRD RAX, RBX, 16
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rbx = 0xAAAABBBBCCCCDDDD;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0xDDDD123456789ABC, "RAX: SHRD 16");
}

#[test]
fn test_shrd_rax_32bits() {
    let code = [0x48, 0x0f, 0xac, 0xd8, 0x20, 0xf4]; // SHRD RAX, RBX, 32
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rbx = 0xFEDCBA9876543210;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x7654321012345678, "RAX: SHRD 32");
}

#[test]
fn test_shrd_rax_full_replacement() {
    // SHRD RAX, RBX, 64: count is masked to 64 & 63 = 0, so no shift occurs
    let code = [0x48, 0x0f, 0xac, 0xd8, 0x40, 0xf4]; // SHRD RAX, RBX, 64
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rbx = 0xFEDCBA9876543210;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "RAX: count masked to 0, no shift");
}

#[test]
fn test_shrd_rcx_rdx_imm8() {
    // RCX >> 12 = 0x0001111111111111, RDX low 12 bits (0x222) fill high 12 bits
    let code = [0x48, 0x0f, 0xac, 0xd1, 0x0C, 0xf4]; // SHRD RCX, RDX, 12
    let mut emu = emu64();
    emu.regs_mut().rcx = 0x1111111111111111;
    emu.regs_mut().rdx = 0x2222222222222222;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x2221111111111111, "RCX: SHRD 12");
}

#[test]
fn test_shrd_rdx_rsi_cl() {
    let code = [0x48, 0x0f, 0xad, 0xf2, 0xf4]; // SHRD RDX, RSI, CL
    let mut emu = emu64();
    emu.regs_mut().rdx = 0xFFFFFFFFFFFFFFFF;
    emu.regs_mut().rsi = 0x0000000000000000;
    emu.regs_mut().rcx = 0x10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rdx, 0x0000FFFFFFFFFFFF, "RDX: SHRD 16");
}

#[test]
fn test_shrd_rsi_rdi_imm8() {
    let code = [0x48, 0x0f, 0xac, 0xfe, 0x08, 0xf4]; // SHRD RSI, RDI, 8
    let mut emu = emu64();
    emu.regs_mut().rsi = 0xAA55AA55AA55AA55;
    emu.regs_mut().rdi = 0x1122334455667788;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsi, 0x88AA55AA55AA55AA, "RSI: SHRD 8");
}

#[test]
fn test_shrd_rdi_rbx_cl() {
    let code = [0x48, 0x0f, 0xad, 0xdf, 0xf4]; // SHRD RDI, RBX, CL
    let mut emu = emu64();
    emu.regs_mut().rdi = 0x0123456789ABCDEF;
    emu.regs_mut().rbx = 0xFEDCBA9876543210;
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rdi, 0x00123456789ABCDE, "RDI: SHRD 4");
}

#[test]
fn test_shrd_r8_r9_imm8() {
    let code = [0x4d, 0x0f, 0xac, 0xc8, 0x08, 0xf4]; // SHRD R8, R9, 8
    let mut emu = emu64();
    emu.regs_mut().r8 = 0x123456789ABCDEF0;
    emu.regs_mut().r9 = 0xFEDCBA9876543210;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().r8, 0x10123456789ABCDE, "R8: SHRD 8");
}

#[test]
fn test_shrd_r10_r11_cl() {
    let code = [0x4d, 0x0f, 0xad, 0xda, 0xf4]; // SHRD R10, R11, CL
    let mut emu = emu64();
    emu.regs_mut().r10 = 0xAAAAAAAAAAAAAAAA;
    emu.regs_mut().r11 = 0x5555555555555555;
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().r10, 0x5AAAAAAAAAAAAAAA, "R10: SHRD 4");
}

#[test]
fn test_shrd_r12_r13_imm8() {
    let code = [0x4d, 0x0f, 0xac, 0xec, 0x10, 0xf4]; // SHRD R12, R13, 16
    let mut emu = emu64();
    emu.regs_mut().r12 = 0x123456789ABCDEF0;
    emu.regs_mut().r13 = 0x1111222233334444;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().r12, 0x4444123456789ABC, "R12: SHRD 16");
}

#[test]
fn test_shrd_r14_r15_cl() {
    let code = [0x4d, 0x0f, 0xad, 0xfe, 0xf4]; // SHRD R14, R15, CL
    let mut emu = emu64();
    emu.regs_mut().r14 = 0xF0F0F0F0F0F0F0F0;
    emu.regs_mut().r15 = 0x0F0F0F0F0F0F0F0F;
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().r14, 0xFF0F0F0F0F0F0F0F, "R14: SHRD 4");
}

#[test]
fn test_shrd_rax_zero_count() {
    let code = [0x48, 0x0f, 0xac, 0xd8, 0x00, 0xf4]; // SHRD RAX, RBX, 0
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rbx = 0xFEDCBA9876543210;
    emu.flags_mut().load(0x2 | flags::F_CF);
    let initial = emu.regs().rax;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, initial, "RAX: unchanged");
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_shrd_mem16_imm8() {
    let DATA_ADDR = 0x7000;
    let code = [
        0x66, 0x0f, 0xac, 0x14, 0x25, // SHRD word ptr [disp32], DX, imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x04, // imm8: 4
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdx = 0xABCD;
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0x1234);
    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(DATA_ADDR).unwrap(), 0xD123, "Memory: 0x1234 SHRD 4 from 0xABCD");
}

#[test]
fn test_shrd_mem32_cl() {
    let DATA_ADDR = 0x7000;
    let code = [
        0x0f, 0xad, 0x14, 0x25, // SHRD dword ptr [disp32], EDX, CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdx = 0xABCDEF01;
    emu.regs_mut().rcx = 0x08;
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x12345678);
    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(DATA_ADDR).unwrap(), 0x01123456, "Memory: SHRD 8");
}

#[test]
fn test_shrd_mem64_imm8() {
    let DATA_ADDR = 0x7000;
    let code = [
        0x48, 0x0f, 0xac, 0x14, 0x25, // SHRD qword ptr [disp32], RDX, imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x10, // imm8: 16
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rdx = 0xAAAABBBBCCCCDDDD;
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x123456789ABCDEF0);
    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_qword(DATA_ADDR).unwrap(), 0xDDDD123456789ABC, "Memory: SHRD 16");
}

// ============================================================================
// Flag tests
// ============================================================================

#[test]
fn test_shrd_sf_flag() {
    let code = [0x0f, 0xac, 0xd8, 0x01, 0xf4]; // SHRD EAX, EBX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0x00000002;
    emu.regs_mut().rbx = 0x00000001; // Will make result negative
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert!(emu.flags().f_sf, "SF: result is negative");
}

#[test]
fn test_shrd_zf_flag() {
    // Both operands are zero, so result is always zero regardless of shift count
    let code = [0x0f, 0xac, 0xd8, 0x10, 0xf4]; // SHRD EAX, EBX, 16
    let mut emu = emu64();
    emu.regs_mut().rax = 0x00000000;
    emu.regs_mut().rbx = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert!(emu.flags().f_zf, "ZF: result is zero");
}

#[test]
fn test_shrd_pf_flag() {
    let code = [0x0f, 0xac, 0xd8, 0x18, 0xf4]; // SHRD EAX, EBX, 24
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0xFF000000; // Low byte will be 0xFF (even parity)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert!(emu.flags().f_pf, "PF: even parity in low byte");
}

// ============================================================================
// Edge cases
// ============================================================================

#[test]
fn test_shrd_masked_count_32bit() {
    // Count should be masked to 5 bits for 32-bit operands
    // 36 & 0x1F = 4, so same as SHRD by 4
    // EAX >> 4 = 0x01234567, EBX low 4 bits (0x1) fill high 4 bits
    let code = [0x0f, 0xac, 0xd8, 0x24, 0xf4]; // SHRD EAX, EBX, 36
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0xABCDEF01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    // 36 & 0x1F = 4
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x11234567, "Count masked to 4");
}

#[test]
fn test_shrd_all_ones() {
    let code = [0x0f, 0xac, 0xd8, 0x10, 0xf4]; // SHRD EAX, EBX, 16
    let mut emu = emu64();
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF, "All ones stays all ones");
}

#[test]
fn test_shrd_alternating_bits() {
    // SHRD by 1: EAX >> 1 = 0x55555555, EBX LSB (1) fills MSB
    // Result = 0x80000000 | 0x55555555 = 0xD5555555
    let code = [0x0f, 0xac, 0xd8, 0x01, 0xf4]; // SHRD EAX, EBX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0xAAAAAAAA;
    emu.regs_mut().rbx = 0x55555555;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xD5555555, "Alternating bits shift");
}
