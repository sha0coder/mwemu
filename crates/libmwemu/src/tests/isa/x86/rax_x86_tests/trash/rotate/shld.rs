// SHLD (Double Precision Shift Left) instruction tests
//
// Opcodes:
// 0F A4 /r ib      SHLD r/m16, r16, imm8
// 0F A5 /r         SHLD r/m16, r16, CL
// 0F A4 /r ib      SHLD r/m32, r32, imm8
// 0F A5 /r         SHLD r/m32, r32, CL
// REX.W + 0F A4 /r ib  SHLD r/m64, r64, imm8
// REX.W + 0F A5 /r     SHLD r/m64, r64, CL
//
// SHLD shifts the destination operand left by count bits.
// Bits shifted in from the right come from the source operand.
// Used for multi-precision shifts of 64 bits or more.
//
// Flags:
// - CF: Last bit shifted out of destination
// - OF: Only for 1-bit shifts (sign change)
// - SF, ZF, PF: Set according to result
// - AF: Undefined for non-zero count
// - Count is 0: No flags affected

//use crate::engine::logic;
//use crate::tests::helpers;
use crate::*;

// ============================================================================
// 16-bit SHLD tests
// ============================================================================

#[test]
fn test_shld_ax_bx_imm8() {
    let code = [0x66, 0x0f, 0xa4, 0xd8, 0x04, 0xf4]; // SHLD AX, BX, 4; HLT
    let mut emu = emu64();
    emu.regs_mut().rax = 0x1234; // Destination
    emu.regs_mut().rbx = 0xABCD; // Source
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    // AX: 0001_0010_0011_0100 shifted left by 4
    // Bits from BX (1010_1011_1100_1101) fill from right
    // Result: 0010_0011_0100_1010
    assert_eq!(emu.regs().rax & 0xFFFF, 0x234A, "AX: 0x1234 SHLD 4 from 0xABCD = 0x234A");
    assert!(emu.flags().f_cf, "CF: bit shifted out was 1");
}

#[test]
fn test_shld_ax_bx_cl() {
    let code = [0x66, 0x0f, 0xa5, 0xd8, 0xf4]; // SHLD AX, BX, CL; HLT
    let mut emu = emu64();
    emu.regs_mut().rax = 0x1234;
    emu.regs_mut().rbx = 0xABCD;
    emu.regs_mut().rcx = 0x08; // Shift by 8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0x34AB, "AX: 0x1234 SHLD 8 from 0xABCD = 0x34AB");
}

#[test]
fn test_shld_ax_bx_1bit() {
    let code = [0x66, 0x0f, 0xa4, 0xd8, 0x01, 0xf4]; // SHLD AX, BX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0x4000; // 0100_0000_0000_0000
    emu.regs_mut().rbx = 0x0001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0x8000, "AX: 0x4000 SHLD 1 = 0x8000");
    assert!(!emu.flags().f_cf, "CF: bit shifted out was 0");
    assert!(emu.flags().f_of, "OF: sign changed from + to -");
}

#[test]
fn test_shld_ax_full_replacement() {
    let code = [0x66, 0x0f, 0xa4, 0xd8, 0x10, 0xf4]; // SHLD AX, BX, 16
    let mut emu = emu64();
    emu.regs_mut().rax = 0x1234;
    emu.regs_mut().rbx = 0xABCD;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0xABCD, "AX: completely replaced by BX");
}

#[test]
fn test_shld_cx_dx_imm8() {
    let code = [0x66, 0x0f, 0xa4, 0xd1, 0x04, 0xf4]; // SHLD CX, DX, 4
    let mut emu = emu64();
    emu.regs_mut().rcx = 0x5678;
    emu.regs_mut().rdx = 0x9ABC;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFFFF, 0x6789, "CX: 0x5678 SHLD 4 from 0x9ABC");
}

#[test]
fn test_shld_dx_si_cl() {
    let code = [0x66, 0x0f, 0xa5, 0xf2, 0xf4]; // SHLD DX, SI, CL
    let mut emu = emu64();
    emu.regs_mut().rdx = 0x1111;
    emu.regs_mut().rsi = 0x2222;
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rdx & 0xFFFF, 0x1112, "DX: 0x1111 SHLD 4 from 0x2222");
}

#[test]
fn test_shld_ax_zero_count() {
    let code = [0x66, 0x0f, 0xa4, 0xd8, 0x00, 0xf4]; // SHLD AX, BX, 0
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
// 32-bit SHLD tests
// ============================================================================

#[test]
fn test_shld_eax_ebx_imm8() {
    let code = [0x0f, 0xa4, 0xd8, 0x04, 0xf4]; // SHLD EAX, EBX, 4
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0xABCDEF01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x2345678A, "EAX: 0x12345678 SHLD 4 from 0xABCDEF01");
}

#[test]
fn test_shld_eax_ebx_cl() {
    let code = [0x0f, 0xa5, 0xd8, 0xf4]; // SHLD EAX, EBX, CL
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0xABCDEF01;
    emu.regs_mut().rcx = 0x08;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x345678AB, "EAX: 0x12345678 SHLD 8 from 0xABCDEF01");
}

#[test]
fn test_shld_eax_carry_flag() {
    // CF receives the LAST bit shifted out (bit SIZE-COUNT = bit 28)
    // 0x80000000 has only bit 31 set, so bit 28 = 0, therefore CF = 0
    // Use 0xF0000000 to have bit 28 set (which becomes the last bit shifted out)
    let code = [0x0f, 0xa4, 0xd8, 0x04, 0xf4]; // SHLD EAX, EBX, 4
    let mut emu = emu64();
    emu.regs_mut().rax = 0xF0000000; // Upper nibble set, bit 28 = 1
    emu.regs_mut().rbx = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert!(emu.flags().f_cf, "CF: bit 28 (last bit shifted out) was 1");
}

#[test]
fn test_shld_eax_1bit() {
    let code = [0x0f, 0xa4, 0xd8, 0x01, 0xf4]; // SHLD EAX, EBX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0x40000000;
    emu.regs_mut().rbx = 0x00000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x80000000, "EAX: 0x40000000 SHLD 1");
    assert!(emu.flags().f_of, "OF: sign changed");
}

#[test]
fn test_shld_eax_full_replacement() {
    // SHLD EAX, EBX, 32: count is masked to 32 & 31 = 0, so no shift occurs
    let code = [0x0f, 0xa4, 0xd8, 0x20, 0xf4]; // SHLD EAX, EBX, 32
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0xABCDEF01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x12345678, "EAX: count masked to 0, no shift");
}

#[test]
fn test_shld_ecx_edx_imm8() {
    let code = [0x0f, 0xa4, 0xd1, 0x0C, 0xf4]; // SHLD ECX, EDX, 12
    let mut emu = emu64();
    emu.regs_mut().rcx = 0x11111111;
    emu.regs_mut().rdx = 0x22222222;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFFFFFFFF, 0x11111222, "ECX: SHLD 12");
}

#[test]
fn test_shld_edx_esi_cl() {
    let code = [0x0f, 0xa5, 0xf2, 0xf4]; // SHLD EDX, ESI, CL
    let mut emu = emu64();
    emu.regs_mut().rdx = 0xFFFFFFFF;
    emu.regs_mut().rsi = 0x00000000;
    emu.regs_mut().rcx = 0x10; // 16 bits
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rdx & 0xFFFFFFFF, 0xFFFF0000, "EDX: SHLD 16");
}

#[test]
fn test_shld_esi_edi_imm8() {
    let code = [0x0f, 0xa4, 0xfe, 0x08, 0xf4]; // SHLD ESI, EDI, 8
    let mut emu = emu64();
    emu.regs_mut().rsi = 0xAA55AA55;
    emu.regs_mut().rdi = 0x11223344;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsi & 0xFFFFFFFF, 0x55AA5511, "ESI: SHLD 8");
}

#[test]
fn test_shld_edi_ebx_cl() {
    let code = [0x0f, 0xa5, 0xdf, 0xf4]; // SHLD EDI, EBX, CL
    let mut emu = emu64();
    emu.regs_mut().rdi = 0x01234567;
    emu.regs_mut().rbx = 0x89ABCDEF;
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rdi & 0xFFFFFFFF, 0x12345678, "EDI: SHLD 4");
}

#[test]
fn test_shld_eax_zero_count() {
    let code = [0x0f, 0xa4, 0xd8, 0x00, 0xf4]; // SHLD EAX, EBX, 0
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
// 64-bit SHLD tests
// ============================================================================

#[test]
fn test_shld_rax_rbx_imm8() {
    let code = [0x48, 0x0f, 0xa4, 0xd8, 0x04, 0xf4]; // SHLD RAX, RBX, 4
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rbx = 0xFEDCBA9876543210;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x23456789ABCDEF0F, "RAX: SHLD 4");
}

#[test]
fn test_shld_rax_rbx_cl() {
    let code = [0x48, 0x0f, 0xa5, 0xd8, 0xf4]; // SHLD RAX, RBX, CL
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rbx = 0xFEDCBA9876543210;
    emu.regs_mut().rcx = 0x08;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x3456789ABCDEF0FE, "RAX: SHLD 8");
}

#[test]
fn test_shld_rax_1bit() {
    let code = [0x48, 0x0f, 0xa4, 0xd8, 0x01, 0xf4]; // SHLD RAX, RBX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0x4000000000000000;
    emu.regs_mut().rbx = 0x0000000000000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x8000000000000000, "RAX: SHLD 1");
    assert!(emu.flags().f_of, "OF: sign changed");
}

#[test]
fn test_shld_rax_carry() {
    let code = [0x48, 0x0f, 0xa4, 0xd8, 0x08, 0xf4]; // SHLD RAX, RBX, 8
    let mut emu = emu64();
    emu.regs_mut().rax = 0xFF00000000000000;
    emu.regs_mut().rbx = 0x0000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert!(emu.flags().f_cf, "CF: bits shifted out");
}

#[test]
fn test_shld_rax_16bits() {
    let code = [0x48, 0x0f, 0xa4, 0xd8, 0x10, 0xf4]; // SHLD RAX, RBX, 16
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rbx = 0xAAAABBBBCCCCDDDD;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x56789ABCDEF0AAAA, "RAX: SHLD 16");
}

#[test]
fn test_shld_rax_32bits() {
    let code = [0x48, 0x0f, 0xa4, 0xd8, 0x20, 0xf4]; // SHLD RAX, RBX, 32
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rbx = 0xFEDCBA9876543210;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x9ABCDEF0FEDCBA98, "RAX: SHLD 32");
}

#[test]
fn test_shld_rax_full_replacement() {
    // SHLD RAX, RBX, 64: count is masked to 64 & 63 = 0, so no shift occurs
    let code = [0x48, 0x0f, 0xa4, 0xd8, 0x40, 0xf4]; // SHLD RAX, RBX, 64
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rbx = 0xFEDCBA9876543210;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "RAX: count masked to 0, no shift");
}

#[test]
fn test_shld_rcx_rdx_imm8() {
    let code = [0x48, 0x0f, 0xa4, 0xd1, 0x0C, 0xf4]; // SHLD RCX, RDX, 12
    let mut emu = emu64();
    emu.regs_mut().rcx = 0x1111111111111111;
    emu.regs_mut().rdx = 0x2222222222222222;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x1111111111111222, "RCX: SHLD 12");
}

#[test]
fn test_shld_rdx_rsi_cl() {
    let code = [0x48, 0x0f, 0xa5, 0xf2, 0xf4]; // SHLD RDX, RSI, CL
    let mut emu = emu64();
    emu.regs_mut().rdx = 0xFFFFFFFFFFFFFFFF;
    emu.regs_mut().rsi = 0x0000000000000000;
    emu.regs_mut().rcx = 0x10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rdx, 0xFFFFFFFFFFFF0000, "RDX: SHLD 16");
}

#[test]
fn test_shld_rsi_rdi_imm8() {
    let code = [0x48, 0x0f, 0xa4, 0xfe, 0x08, 0xf4]; // SHLD RSI, RDI, 8
    let mut emu = emu64();
    emu.regs_mut().rsi = 0xAA55AA55AA55AA55;
    emu.regs_mut().rdi = 0x1122334455667788;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsi, 0x55AA55AA55AA5511, "RSI: SHLD 8");
}

#[test]
fn test_shld_rdi_rbx_cl() {
    let code = [0x48, 0x0f, 0xa5, 0xdf, 0xf4]; // SHLD RDI, RBX, CL
    let mut emu = emu64();
    emu.regs_mut().rdi = 0x0123456789ABCDEF;
    emu.regs_mut().rbx = 0xFEDCBA9876543210;
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rdi, 0x123456789ABCDEFF, "RDI: SHLD 4");
}

#[test]
fn test_shld_r8_r9_imm8() {
    let code = [0x4d, 0x0f, 0xa4, 0xc8, 0x08, 0xf4]; // SHLD R8, R9, 8
    let mut emu = emu64();
    emu.regs_mut().r8 = 0x123456789ABCDEF0;
    emu.regs_mut().r9 = 0xFEDCBA9876543210;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().r8, 0x3456789ABCDEF0FE, "R8: SHLD 8");
}

#[test]
fn test_shld_r10_r11_cl() {
    let code = [0x4d, 0x0f, 0xa5, 0xda, 0xf4]; // SHLD R10, R11, CL
    let mut emu = emu64();
    emu.regs_mut().r10 = 0xAAAAAAAAAAAAAAAA;
    emu.regs_mut().r11 = 0x5555555555555555;
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().r10, 0xAAAAAAAAAAAAAAA5, "R10: SHLD 4");
}

#[test]
fn test_shld_r12_r13_imm8() {
    let code = [0x4d, 0x0f, 0xa4, 0xec, 0x10, 0xf4]; // SHLD R12, R13, 16
    let mut emu = emu64();
    emu.regs_mut().r12 = 0x123456789ABCDEF0;
    emu.regs_mut().r13 = 0x1111222233334444;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().r12, 0x56789ABCDEF01111, "R12: SHLD 16");
}

#[test]
fn test_shld_r14_r15_cl() {
    let code = [0x4d, 0x0f, 0xa5, 0xfe, 0xf4]; // SHLD R14, R15, CL
    let mut emu = emu64();
    emu.regs_mut().r14 = 0xF0F0F0F0F0F0F0F0;
    emu.regs_mut().r15 = 0x0F0F0F0F0F0F0F0F;
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().r14, 0x0F0F0F0F0F0F0F00, "R14: SHLD 4");
}

#[test]
fn test_shld_rax_zero_count() {
    let code = [0x48, 0x0f, 0xa4, 0xd8, 0x00, 0xf4]; // SHLD RAX, RBX, 0
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
fn test_shld_mem16_imm8() {
    let DATA_ADDR = 0x7000;
    let code = [
        0x66, 0x0f, 0xa4, 0x14, 0x25, // SHLD word ptr [disp32], DX, imm8
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
    assert_eq!(emu.maps.read_word(DATA_ADDR).unwrap(), 0x234A, "Memory: 0x1234 SHLD 4 from 0xABCD");
}

#[test]
fn test_shld_mem32_cl() {
    let DATA_ADDR = 0x7000;
    let code = [
        0x0f, 0xa5, 0x14, 0x25, // SHLD dword ptr [disp32], EDX, CL
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
    assert_eq!(emu.maps.read_dword(DATA_ADDR).unwrap(), 0x345678AB, "Memory: SHLD 8");
}

#[test]
fn test_shld_mem64_imm8() {
    let DATA_ADDR = 0x7000;
    let code = [
        0x48, 0x0f, 0xa4, 0x14, 0x25, // SHLD qword ptr [disp32], RDX, imm8
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
    assert_eq!(emu.maps.read_qword(DATA_ADDR).unwrap(), 0x56789ABCDEF0AAAA, "Memory: SHLD 16");
}

// ============================================================================
// Flag tests
// ============================================================================

#[test]
fn test_shld_sf_flag() {
    let code = [0x0f, 0xa4, 0xd8, 0x01, 0xf4]; // SHLD EAX, EBX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0x40000000; // Will become negative
    emu.regs_mut().rbx = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert!(emu.flags().f_sf, "SF: result is negative");
}

#[test]
fn test_shld_zf_flag() {
    // SHLD by 16: upper 16 bits of EAX shift out, lower 16 bits shift up
    // With EAX = 0x00000000 and EBX = 0x00000000, result is 0
    let code = [0x0f, 0xa4, 0xd8, 0x10, 0xf4]; // SHLD EAX, EBX, 16
    let mut emu = emu64();
    emu.regs_mut().rax = 0x00000000; // All zeros
    emu.regs_mut().rbx = 0x00000000; // All zeros
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert!(emu.flags().f_zf, "ZF: result is zero");
}

#[test]
fn test_shld_pf_flag() {
    let code = [0x0f, 0xa4, 0xd8, 0x18, 0xf4]; // SHLD EAX, EBX, 24
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0x000000FF; // Low byte will be 0xFF (even parity)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert!(emu.flags().f_pf, "PF: even parity in low byte");
}

// ============================================================================
// Edge cases
// ============================================================================

#[test]
fn test_shld_masked_count_32bit() {
    // Count should be masked to 5 bits for 32-bit operands
    let code = [0x0f, 0xa4, 0xd8, 0x24, 0xf4]; // SHLD EAX, EBX, 36
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0xABCDEF01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    // 36 & 0x1F = 4
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x2345678A, "Count masked to 4");
}

#[test]
fn test_shld_all_ones() {
    let code = [0x0f, 0xa4, 0xd8, 0x10, 0xf4]; // SHLD EAX, EBX, 16
    let mut emu = emu64();
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF, "All ones stays all ones");
}

#[test]
fn test_shld_alternating_bits() {
    // SHLD EAX, EBX, 1: shifts EAX left by 1, brings in bit 31 of EBX (0) at LSB
    // 0xAAAAAAAA << 1 = 0x55555554 (MSB shifted out), LSB = 0 from EBX[31]
    let code = [0x0f, 0xa4, 0xd8, 0x01, 0xf4]; // SHLD EAX, EBX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0xAAAAAAAA;
    emu.regs_mut().rbx = 0x55555555;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x55555554, "Alternating bits shift");
}
