use crate::*;

// EXTRACTPS - Extract Packed Single Precision Floating-Point Value
// Opcode: 66 0F 3A 17 /r ib       EXTRACTPS reg/m32, xmm1, imm8

const DATA_ADDR: u64 = 0x3000;

// ============================================================================
// EXTRACTPS - Extract Float32 to Register (4 positions: 0-3)
// ============================================================================

#[test]
fn test_extractps_eax_xmm0_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xc0, 0x00, 0xf4]; // EXTRACTPS EAX, XMM0, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_eax_xmm0_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xc0, 0x01, 0xf4]; // EXTRACTPS EAX, XMM0, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_eax_xmm0_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xc0, 0x02, 0xf4]; // EXTRACTPS EAX, XMM0, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_eax_xmm0_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xc0, 0x03, 0xf4]; // EXTRACTPS EAX, XMM0, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// EXTRACTPS with different XMM registers
#[test]
fn test_extractps_eax_xmm1_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xc8, 0x00, 0xf4]; // EXTRACTPS EAX, XMM1, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_eax_xmm1_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xc8, 0x01, 0xf4]; // EXTRACTPS EAX, XMM1, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_eax_xmm2_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xd0, 0x02, 0xf4]; // EXTRACTPS EAX, XMM2, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_eax_xmm3_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xd8, 0x03, 0xf4]; // EXTRACTPS EAX, XMM3, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_eax_xmm4_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xe0, 0x00, 0xf4]; // EXTRACTPS EAX, XMM4, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_eax_xmm5_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xe8, 0x01, 0xf4]; // EXTRACTPS EAX, XMM5, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_eax_xmm6_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xf0, 0x02, 0xf4]; // EXTRACTPS EAX, XMM6, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_eax_xmm7_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xf8, 0x03, 0xf4]; // EXTRACTPS EAX, XMM7, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// EXTRACTPS with different GPRs
#[test]
fn test_extractps_ebx_xmm0_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xc3, 0x01, 0xf4]; // EXTRACTPS EBX, XMM0, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_ecx_xmm1_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xc9, 0x02, 0xf4]; // EXTRACTPS ECX, XMM1, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_edx_xmm2_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xd2, 0x03, 0xf4]; // EXTRACTPS EDX, XMM2, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_esi_xmm3_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xde, 0x00, 0xf4]; // EXTRACTPS ESI, XMM3, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_edi_xmm4_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xe7, 0x01, 0xf4]; // EXTRACTPS EDI, XMM4, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// EXTRACTPS with extended XMM registers
#[test]
fn test_extractps_eax_xmm8_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x17, 0xc0, 0x00, 0xf4]; // EXTRACTPS EAX, XMM8, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_eax_xmm9_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x17, 0xc8, 0x01, 0xf4]; // EXTRACTPS EAX, XMM9, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_eax_xmm15_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x17, 0xf8, 0x03, 0xf4]; // EXTRACTPS EAX, XMM15, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// EXTRACTPS with extended GPRs
#[test]
fn test_extractps_r8d_xmm0_pos0() {
    let mut emu = emu64();
    let code = [0x66, 0x41, 0x0f, 0x3a, 0x17, 0xc0, 0x00, 0xf4]; // EXTRACTPS R8D, XMM0, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_r15d_xmm7_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x41, 0x0f, 0x3a, 0x17, 0xff, 0x02, 0xf4]; // EXTRACTPS R15D, XMM7, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// EXTRACTPS to memory - all positions
#[test]
fn test_extractps_mem_xmm0_pos0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x17, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, 0xf4
    ]; // EXTRACTPS [0x3000], XMM0, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_mem_xmm0_pos1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x17, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x01, 0xf4
    ]; // EXTRACTPS [0x3000], XMM0, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_mem_xmm0_pos2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x17, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x02, 0xf4
    ]; // EXTRACTPS [0x3000], XMM0, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_mem_xmm0_pos3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x17, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x03, 0xf4
    ]; // EXTRACTPS [0x3000], XMM0, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_mem_xmm1_pos0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x17, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, 0xf4
    ]; // EXTRACTPS [0x3000], XMM1, 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_mem_xmm7_pos1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x17, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x01, 0xf4
    ]; // EXTRACTPS [0x3000], XMM7, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_mem_xmm15_pos3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x17, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x03, 0xf4
    ]; // EXTRACTPS [0x3000], XMM15, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Additional combinations with different registers
#[test]
fn test_extractps_ebx_xmm5_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xeb, 0x02, 0xf4]; // EXTRACTPS EBX, XMM5, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_ecx_xmm6_pos1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xf1, 0x01, 0xf4]; // EXTRACTPS ECX, XMM6, 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_edx_xmm7_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x17, 0xfa, 0x03, 0xf4]; // EXTRACTPS EDX, XMM7, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_r8d_xmm9_pos2() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x3a, 0x17, 0xc8, 0x02, 0xf4]; // EXTRACTPS R8D, XMM9, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_r15d_xmm15_pos3() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x3a, 0x17, 0xff, 0x03, 0xf4]; // EXTRACTPS R15D, XMM15, 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_extractps_mem_xmm2_pos2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x17, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, 0x02, 0xf4
    ]; // EXTRACTPS [0x3000], XMM2, 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
