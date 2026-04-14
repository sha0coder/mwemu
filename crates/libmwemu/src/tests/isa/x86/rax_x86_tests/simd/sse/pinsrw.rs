use crate::*;

// PINSRW - Insert Word
// Opcode: 66 0F C4 /r ib          PINSRW xmm, r32/m16, imm8

const DATA_ADDR: u64 = 0x3000;

// ============================================================================
// PINSRW - Insert Word from Register (8 positions: 0-7)
// ============================================================================

#[test]
fn test_pinsrw_xmm0_eax_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xc0, 0x00, 0xf4]; // PINSRW XMM0, EAX, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm0_eax_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xc0, 0x01, 0xf4]; // PINSRW XMM0, EAX, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm0_eax_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xc0, 0x02, 0xf4]; // PINSRW XMM0, EAX, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm0_eax_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xc0, 0x03, 0xf4]; // PINSRW XMM0, EAX, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm0_eax_pos4() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xc0, 0x04, 0xf4]; // PINSRW XMM0, EAX, 4
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm0_eax_pos5() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xc0, 0x05, 0xf4]; // PINSRW XMM0, EAX, 5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm0_eax_pos6() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xc0, 0x06, 0xf4]; // PINSRW XMM0, EAX, 6
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm0_eax_pos7() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xc0, 0x07, 0xf4]; // PINSRW XMM0, EAX, 7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PINSRW with different XMM registers
#[test]
fn test_pinsrw_xmm1_eax_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xc8, 0x00, 0xf4]; // PINSRW XMM1, EAX, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm2_eax_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xd0, 0x03, 0xf4]; // PINSRW XMM2, EAX, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm3_eax_pos5() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xd8, 0x05, 0xf4]; // PINSRW XMM3, EAX, 5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm4_eax_pos7() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xe0, 0x07, 0xf4]; // PINSRW XMM4, EAX, 7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm5_eax_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xe8, 0x02, 0xf4]; // PINSRW XMM5, EAX, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm6_eax_pos4() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xf0, 0x04, 0xf4]; // PINSRW XMM6, EAX, 4
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm7_eax_pos6() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xf8, 0x06, 0xf4]; // PINSRW XMM7, EAX, 6
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PINSRW with different GPRs
#[test]
fn test_pinsrw_xmm0_ebx_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xc3, 0x01, 0xf4]; // PINSRW XMM0, EBX, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm1_ecx_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xc9, 0x03, 0xf4]; // PINSRW XMM1, ECX, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm2_edx_pos5() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xd2, 0x05, 0xf4]; // PINSRW XMM2, EDX, 5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm3_esi_pos7() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xde, 0x07, 0xf4]; // PINSRW XMM3, ESI, 7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm4_edi_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xe7, 0x02, 0xf4]; // PINSRW XMM4, EDI, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PINSRW with extended XMM registers
#[test]
fn test_pinsrw_xmm8_eax_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0xc4, 0xc0, 0x00, 0xf4]; // PINSRW XMM8, EAX, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm9_eax_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0xc4, 0xc8, 0x03, 0xf4]; // PINSRW XMM9, EAX, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm10_eax_pos5() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0xc4, 0xd0, 0x05, 0xf4]; // PINSRW XMM10, EAX, 5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm15_eax_pos7() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0xc4, 0xf8, 0x07, 0xf4]; // PINSRW XMM15, EAX, 7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PINSRW with extended GPRs
#[test]
fn test_pinsrw_xmm0_r8d_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x41, 0x0f, 0xc4, 0xc0, 0x01, 0xf4]; // PINSRW XMM0, R8D, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm7_r15d_pos6() {
    let mut emu = emu64();
    let code = [0x66, 0x41, 0x0f, 0xc4, 0xff, 0x06, 0xf4]; // PINSRW XMM7, R15D, 6
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PINSRW from memory - all positions
#[test]
fn test_pinsrw_xmm0_mem_pos0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xc4, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, 0xf4
    ]; // PINSRW XMM0, [0x3000], 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm0_mem_pos1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xc4, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x01, 0xf4
    ]; // PINSRW XMM0, [0x3000], 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm0_mem_pos2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xc4, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x02, 0xf4
    ]; // PINSRW XMM0, [0x3000], 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm0_mem_pos3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xc4, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x03, 0xf4
    ]; // PINSRW XMM0, [0x3000], 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm0_mem_pos4() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xc4, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x04, 0xf4
    ]; // PINSRW XMM0, [0x3000], 4
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm0_mem_pos5() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xc4, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x05, 0xf4
    ]; // PINSRW XMM0, [0x3000], 5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm0_mem_pos6() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xc4, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x06, 0xf4
    ]; // PINSRW XMM0, [0x3000], 6
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm0_mem_pos7() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xc4, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x07, 0xf4
    ]; // PINSRW XMM0, [0x3000], 7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PINSRW from memory with different XMM registers
#[test]
fn test_pinsrw_xmm1_mem_pos2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xc4, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x02, 0xf4
    ]; // PINSRW XMM1, [0x3000], 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm7_mem_pos5() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xc4, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x05, 0xf4
    ]; // PINSRW XMM7, [0x3000], 5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm15_mem_pos7() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0xc4, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x07, 0xf4
    ]; // PINSRW XMM15, [0x3000], 7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Additional combinations
#[test]
fn test_pinsrw_xmm5_ebx_pos4() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xeb, 0x04, 0xf4]; // PINSRW XMM5, EBX, 4
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm6_ecx_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xf1, 0x01, 0xf4]; // PINSRW XMM6, ECX, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pinsrw_xmm7_edx_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xc4, 0xfa, 0x03, 0xf4]; // PINSRW XMM7, EDX, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
