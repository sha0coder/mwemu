use crate::*;

// PINSRB/PINSRD/PINSRQ - Insert Byte/Dword/Qword
// Opcode: 66 0F 3A 20 /r ib       PINSRB xmm1, r32/m8, imm8
//         66 0F 3A 22 /r ib       PINSRD xmm1, r/m32, imm8
//         66 REX.W 0F 3A 22 /r ib PINSRQ xmm1, r/m64, imm8

const DATA_ADDR: u64 = 0x3000;

// ============================================================================
// PINSRB - Insert Byte from Register (16 positions: 0-15)
// ============================================================================

#[test]
fn test_pinsrb_xmm0_eax_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xc0, 0x00, 0xf4]; // PINSRB XMM0, EAX, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm0_eax_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xc0, 0x01, 0xf4]; // PINSRB XMM0, EAX, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm0_eax_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xc0, 0x02, 0xf4]; // PINSRB XMM0, EAX, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm0_eax_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xc0, 0x03, 0xf4]; // PINSRB XMM0, EAX, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm0_eax_pos4() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xc0, 0x04, 0xf4]; // PINSRB XMM0, EAX, 4
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm0_eax_pos5() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xc0, 0x05, 0xf4]; // PINSRB XMM0, EAX, 5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm0_eax_pos6() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xc0, 0x06, 0xf4]; // PINSRB XMM0, EAX, 6
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm0_eax_pos7() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xc0, 0x07, 0xf4]; // PINSRB XMM0, EAX, 7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm0_eax_pos8() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xc0, 0x08, 0xf4]; // PINSRB XMM0, EAX, 8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm0_eax_pos9() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xc0, 0x09, 0xf4]; // PINSRB XMM0, EAX, 9
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm0_eax_pos10() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xc0, 0x0a, 0xf4]; // PINSRB XMM0, EAX, 10
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm0_eax_pos11() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xc0, 0x0b, 0xf4]; // PINSRB XMM0, EAX, 11
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm0_eax_pos12() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xc0, 0x0c, 0xf4]; // PINSRB XMM0, EAX, 12
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm0_eax_pos13() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xc0, 0x0d, 0xf4]; // PINSRB XMM0, EAX, 13
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm0_eax_pos14() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xc0, 0x0e, 0xf4]; // PINSRB XMM0, EAX, 14
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm0_eax_pos15() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xc0, 0x0f, 0xf4]; // PINSRB XMM0, EAX, 15
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PINSRB with different register combinations
#[test]
fn test_pinsrb_xmm1_ebx_pos5() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xcb, 0x05, 0xf4]; // PINSRB XMM1, EBX, 5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm2_ecx_pos7() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xd1, 0x07, 0xf4]; // PINSRB XMM2, ECX, 7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm3_edx_pos9() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xda, 0x09, 0xf4]; // PINSRB XMM3, EDX, 9
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm7_edi_pos13() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x20, 0xff, 0x0d, 0xf4]; // PINSRB XMM7, EDI, 13
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PINSRB with extended XMM registers
#[test]
fn test_pinsrb_xmm8_eax_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x20, 0xc0, 0x03, 0xf4]; // PINSRB XMM8, EAX, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm15_eax_pos11() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x20, 0xf8, 0x0b, 0xf4]; // PINSRB XMM15, EAX, 11
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PINSRB from memory
#[test]
fn test_pinsrb_xmm0_mem_pos0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x20, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, 0xf4
    ]; // PINSRB XMM0, [0x3000], 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm1_mem_pos8() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x20, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x08, 0xf4
    ]; // PINSRB XMM1, [0x3000], 8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrb_xmm7_mem_pos15() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x20, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x0f, 0xf4
    ]; // PINSRB XMM7, [0x3000], 15
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// PINSRD - Insert Dword from Register (4 positions: 0-3)
// ============================================================================

#[test]
fn test_pinsrd_xmm0_eax_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x22, 0xc0, 0x00, 0xf4]; // PINSRD XMM0, EAX, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrd_xmm0_eax_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x22, 0xc0, 0x01, 0xf4]; // PINSRD XMM0, EAX, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrd_xmm0_eax_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x22, 0xc0, 0x02, 0xf4]; // PINSRD XMM0, EAX, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrd_xmm0_eax_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x22, 0xc0, 0x03, 0xf4]; // PINSRD XMM0, EAX, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PINSRD with different register combinations
#[test]
fn test_pinsrd_xmm1_ebx_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x22, 0xcb, 0x00, 0xf4]; // PINSRD XMM1, EBX, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrd_xmm2_ecx_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x22, 0xd1, 0x01, 0xf4]; // PINSRD XMM2, ECX, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrd_xmm3_edx_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x22, 0xda, 0x02, 0xf4]; // PINSRD XMM3, EDX, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrd_xmm4_esi_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x22, 0xe6, 0x03, 0xf4]; // PINSRD XMM4, ESI, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrd_xmm5_edi_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x22, 0xef, 0x01, 0xf4]; // PINSRD XMM5, EDI, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrd_xmm6_r8d_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x41, 0x0f, 0x3a, 0x22, 0xf0, 0x02, 0xf4]; // PINSRD XMM6, R8D, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrd_xmm7_r15d_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x41, 0x0f, 0x3a, 0x22, 0xff, 0x03, 0xf4]; // PINSRD XMM7, R15D, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PINSRD with extended XMM registers
#[test]
fn test_pinsrd_xmm8_eax_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x22, 0xc0, 0x00, 0xf4]; // PINSRD XMM8, EAX, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrd_xmm15_eax_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x22, 0xf8, 0x02, 0xf4]; // PINSRD XMM15, EAX, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PINSRD from memory
#[test]
fn test_pinsrd_xmm0_mem_pos0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x22, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, 0xf4
    ]; // PINSRD XMM0, [0x3000], 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrd_xmm1_mem_pos1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x22, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x01, 0xf4
    ]; // PINSRD XMM1, [0x3000], 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrd_xmm7_mem_pos3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x22, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x03, 0xf4
    ]; // PINSRD XMM7, [0x3000], 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// PINSRQ - Insert Qword from Register (2 positions: 0-1)
// ============================================================================

#[test]
fn test_pinsrq_xmm0_rax_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x22, 0xc0, 0x00, 0xf4]; // PINSRQ XMM0, RAX, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrq_xmm0_rax_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x22, 0xc0, 0x01, 0xf4]; // PINSRQ XMM0, RAX, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PINSRQ with different register combinations
#[test]
fn test_pinsrq_xmm1_rbx_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x22, 0xcb, 0x00, 0xf4]; // PINSRQ XMM1, RBX, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrq_xmm2_rcx_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x22, 0xd1, 0x01, 0xf4]; // PINSRQ XMM2, RCX, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrq_xmm3_rdx_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x22, 0xda, 0x00, 0xf4]; // PINSRQ XMM3, RDX, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrq_xmm4_rsi_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x22, 0xe6, 0x01, 0xf4]; // PINSRQ XMM4, RSI, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrq_xmm5_rdi_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x22, 0xef, 0x00, 0xf4]; // PINSRQ XMM5, RDI, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrq_xmm6_r8_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x49, 0x0f, 0x3a, 0x22, 0xf0, 0x01, 0xf4]; // PINSRQ XMM6, R8, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrq_xmm7_r15_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x49, 0x0f, 0x3a, 0x22, 0xff, 0x00, 0xf4]; // PINSRQ XMM7, R15, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PINSRQ with extended XMM registers
#[test]
fn test_pinsrq_xmm8_rax_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x4c, 0x0f, 0x3a, 0x22, 0xc0, 0x00, 0xf4]; // PINSRQ XMM8, RAX, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrq_xmm15_rax_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x4c, 0x0f, 0x3a, 0x22, 0xf8, 0x01, 0xf4]; // PINSRQ XMM15, RAX, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrq_xmm9_r8_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x4d, 0x0f, 0x3a, 0x22, 0xc8, 0x00, 0xf4]; // PINSRQ XMM9, R8, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrq_xmm15_r15_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x4d, 0x0f, 0x3a, 0x22, 0xff, 0x01, 0xf4]; // PINSRQ XMM15, R15, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PINSRQ from memory
#[test]
fn test_pinsrq_xmm0_mem_pos0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x48, 0x0f, 0x3a, 0x22, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, 0xf4
    ]; // PINSRQ XMM0, [0x3000], 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrq_xmm1_mem_pos1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x48, 0x0f, 0x3a, 0x22, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x01, 0xf4
    ]; // PINSRQ XMM1, [0x3000], 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrq_xmm7_mem_pos0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x48, 0x0f, 0x3a, 0x22, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, 0xf4
    ]; // PINSRQ XMM7, [0x3000], 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
