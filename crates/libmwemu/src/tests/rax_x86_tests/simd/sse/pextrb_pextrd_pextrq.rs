use crate::*;

// PEXTRB/PEXTRD/PEXTRQ - Extract Byte/Dword/Qword
// Opcode: 66 0F 3A 14 /r ib       PEXTRB r32/m8, xmm2, imm8
//         66 0F 3A 16 /r ib       PEXTRD r/m32, xmm2, imm8
//         66 REX.W 0F 3A 16 /r ib PEXTRQ r/m64, xmm2, imm8

const DATA_ADDR: u64 = 0x3000;

// ============================================================================
// PEXTRB - Extract Byte to Register (16 positions: 0-15)
// ============================================================================

#[test]
fn test_pextrb_eax_xmm0_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x00, 0xf4]; // PEXTRB EAX, XMM0, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x01, 0xf4]; // PEXTRB EAX, XMM0, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x02, 0xf4]; // PEXTRB EAX, XMM0, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x03, 0xf4]; // PEXTRB EAX, XMM0, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos4() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x04, 0xf4]; // PEXTRB EAX, XMM0, 4
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos5() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x05, 0xf4]; // PEXTRB EAX, XMM0, 5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos6() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x06, 0xf4]; // PEXTRB EAX, XMM0, 6
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos7() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x07, 0xf4]; // PEXTRB EAX, XMM0, 7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos8() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x08, 0xf4]; // PEXTRB EAX, XMM0, 8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos9() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x09, 0xf4]; // PEXTRB EAX, XMM0, 9
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos10() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x0a, 0xf4]; // PEXTRB EAX, XMM0, 10
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos11() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x0b, 0xf4]; // PEXTRB EAX, XMM0, 11
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos12() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x0c, 0xf4]; // PEXTRB EAX, XMM0, 12
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos13() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x0d, 0xf4]; // PEXTRB EAX, XMM0, 13
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos14() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x0e, 0xf4]; // PEXTRB EAX, XMM0, 14
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos15() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x0f, 0xf4]; // PEXTRB EAX, XMM0, 15
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PEXTRB with different registers
#[test]
fn test_pextrb_ebx_xmm1_pos5() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xcb, 0x05, 0xf4]; // PEXTRB EBX, XMM1, 5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_ecx_xmm2_pos7() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xd1, 0x07, 0xf4]; // PEXTRB ECX, XMM2, 7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_edx_xmm3_pos9() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xda, 0x09, 0xf4]; // PEXTRB EDX, XMM3, 9
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_esi_xmm4_pos11() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xe6, 0x0b, 0xf4]; // PEXTRB ESI, XMM4, 11
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_edi_xmm5_pos13() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xef, 0x0d, 0xf4]; // PEXTRB EDI, XMM5, 13
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PEXTRB with extended XMM registers
#[test]
fn test_pextrb_eax_xmm8_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x14, 0xc0, 0x03, 0xf4]; // PEXTRB EAX, XMM8, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_eax_xmm15_pos7() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x14, 0xf8, 0x07, 0xf4]; // PEXTRB EAX, XMM15, 7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PEXTRB to memory
#[test]
fn test_pextrb_mem_xmm0_pos0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x14, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, 0xf4
    ]; // PEXTRB [0x3000], XMM0, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_mem_xmm1_pos8() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x14, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x08, 0xf4
    ]; // PEXTRB [0x3000], XMM1, 8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrb_mem_xmm7_pos15() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x14, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x0f, 0xf4
    ]; // PEXTRB [0x3000], XMM7, 15
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// PEXTRD - Extract Dword to Register (4 positions: 0-3)
// ============================================================================

#[test]
fn test_pextrd_eax_xmm0_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x16, 0xc0, 0x00, 0xf4]; // PEXTRD EAX, XMM0, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrd_eax_xmm0_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x16, 0xc0, 0x01, 0xf4]; // PEXTRD EAX, XMM0, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrd_eax_xmm0_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x16, 0xc0, 0x02, 0xf4]; // PEXTRD EAX, XMM0, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrd_eax_xmm0_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x16, 0xc0, 0x03, 0xf4]; // PEXTRD EAX, XMM0, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PEXTRD with different registers
#[test]
fn test_pextrd_ebx_xmm1_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x16, 0xcb, 0x00, 0xf4]; // PEXTRD EBX, XMM1, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrd_ecx_xmm2_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x16, 0xd1, 0x01, 0xf4]; // PEXTRD ECX, XMM2, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrd_edx_xmm3_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x16, 0xda, 0x02, 0xf4]; // PEXTRD EDX, XMM3, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrd_esi_xmm4_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x16, 0xe6, 0x03, 0xf4]; // PEXTRD ESI, XMM4, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrd_edi_xmm5_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x16, 0xef, 0x01, 0xf4]; // PEXTRD EDI, XMM5, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrd_r8d_xmm6_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x41, 0x0f, 0x3a, 0x16, 0xf0, 0x02, 0xf4]; // PEXTRD R8D, XMM6, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrd_r15d_xmm7_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x41, 0x0f, 0x3a, 0x16, 0xff, 0x03, 0xf4]; // PEXTRD R15D, XMM7, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PEXTRD with extended XMM registers
#[test]
fn test_pextrd_eax_xmm8_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x16, 0xc0, 0x00, 0xf4]; // PEXTRD EAX, XMM8, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrd_eax_xmm15_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x16, 0xf8, 0x02, 0xf4]; // PEXTRD EAX, XMM15, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PEXTRD to memory
#[test]
fn test_pextrd_mem_xmm0_pos0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x16, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, 0xf4
    ]; // PEXTRD [0x3000], XMM0, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrd_mem_xmm1_pos1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x16, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x01, 0xf4
    ]; // PEXTRD [0x3000], XMM1, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrd_mem_xmm7_pos3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x16, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x03, 0xf4
    ]; // PEXTRD [0x3000], XMM7, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// PEXTRQ - Extract Qword to Register (2 positions: 0-1)
// ============================================================================

#[test]
fn test_pextrq_rax_xmm0_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x16, 0xc0, 0x00, 0xf4]; // PEXTRQ RAX, XMM0, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrq_rax_xmm0_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x16, 0xc0, 0x01, 0xf4]; // PEXTRQ RAX, XMM0, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PEXTRQ with different registers
#[test]
fn test_pextrq_rbx_xmm1_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x16, 0xcb, 0x00, 0xf4]; // PEXTRQ RBX, XMM1, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrq_rcx_xmm2_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x16, 0xd1, 0x01, 0xf4]; // PEXTRQ RCX, XMM2, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrq_rdx_xmm3_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x16, 0xda, 0x00, 0xf4]; // PEXTRQ RDX, XMM3, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrq_rsi_xmm4_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x16, 0xe6, 0x01, 0xf4]; // PEXTRQ RSI, XMM4, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrq_rdi_xmm5_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x16, 0xef, 0x00, 0xf4]; // PEXTRQ RDI, XMM5, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrq_r8_xmm6_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x49, 0x0f, 0x3a, 0x16, 0xf0, 0x01, 0xf4]; // PEXTRQ R8, XMM6, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrq_r15_xmm7_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x49, 0x0f, 0x3a, 0x16, 0xff, 0x00, 0xf4]; // PEXTRQ R15, XMM7, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PEXTRQ with extended XMM registers
#[test]
fn test_pextrq_rax_xmm8_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x4c, 0x0f, 0x3a, 0x16, 0xc0, 0x00, 0xf4]; // PEXTRQ RAX, XMM8, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrq_rax_xmm15_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x4c, 0x0f, 0x3a, 0x16, 0xf8, 0x01, 0xf4]; // PEXTRQ RAX, XMM15, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrq_r8_xmm9_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x4d, 0x0f, 0x3a, 0x16, 0xc8, 0x00, 0xf4]; // PEXTRQ R8, XMM9, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrq_r15_xmm15_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x4d, 0x0f, 0x3a, 0x16, 0xff, 0x01, 0xf4]; // PEXTRQ R15, XMM15, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PEXTRQ to memory
#[test]
fn test_pextrq_mem_xmm0_pos0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x48, 0x0f, 0x3a, 0x16, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, 0xf4
    ]; // PEXTRQ [0x3000], XMM0, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrq_mem_xmm1_pos1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x48, 0x0f, 0x3a, 0x16, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x01, 0xf4
    ]; // PEXTRQ [0x3000], XMM1, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pextrq_mem_xmm7_pos0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x48, 0x0f, 0x3a, 0x16, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, 0xf4
    ]; // PEXTRQ [0x3000], XMM7, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
