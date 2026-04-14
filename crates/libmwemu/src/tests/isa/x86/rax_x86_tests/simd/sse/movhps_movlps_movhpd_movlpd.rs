use crate::*;

// MOVHPS - Move High Packed Single Precision
// MOVLPS - Move Low Packed Single Precision
// MOVHPD - Move High Packed Double Precision
// MOVLPD - Move Low Packed Double Precision
//
// Opcodes:
// NP 0F 16 /r    MOVHPS xmm, m64     - Move 64 bits from m64 to high quadword of xmm
// NP 0F 17 /r    MOVHPS m64, xmm     - Move high quadword from xmm to m64
// NP 0F 12 /r    MOVLPS xmm, m64     - Move 64 bits from m64 to low quadword of xmm
// NP 0F 13 /r    MOVLPS m64, xmm     - Move low quadword from xmm to m64
// 66 0F 16 /r    MOVHPD xmm, m64     - Move 64 bits from m64 to high quadword of xmm
// 66 0F 17 /r    MOVHPD m64, xmm     - Move high quadword from xmm to m64
// 66 0F 12 /r    MOVLPD xmm, m64     - Move 64 bits from m64 to low quadword of xmm
// 66 0F 13 /r    MOVLPD m64, xmm     - Move low quadword from xmm to m64

const DATA_ADDR: u64 = 0x3000;

// MOVHPS Tests
#[test]
fn test_movhps_mem_to_xmm0() {
    let mut emu = emu64();
    let code = [0x0f, 0x16, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhps_mem_to_xmm7() {
    let mut emu = emu64();
    let code = [0x0f, 0x16, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhps_xmm0_to_mem() {
    let mut emu = emu64();
    let code = [0x0f, 0x17, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhps_xmm7_to_mem() {
    let mut emu = emu64();
    let code = [0x0f, 0x17, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhps_mem_to_xmm15() {
    let mut emu = emu64();
    let code = [0x44, 0x0f, 0x16, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhps_xmm15_to_mem() {
    let mut emu = emu64();
    let code = [0x44, 0x0f, 0x17, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// MOVLPS Tests
#[test]
fn test_movlps_mem_to_xmm0() {
    let mut emu = emu64();
    let code = [0x0f, 0x12, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlps_mem_to_xmm7() {
    let mut emu = emu64();
    let code = [0x0f, 0x12, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlps_xmm0_to_mem() {
    let mut emu = emu64();
    let code = [0x0f, 0x13, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlps_xmm7_to_mem() {
    let mut emu = emu64();
    let code = [0x0f, 0x13, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlps_mem_to_xmm15() {
    let mut emu = emu64();
    let code = [0x44, 0x0f, 0x12, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlps_xmm15_to_mem() {
    let mut emu = emu64();
    let code = [0x44, 0x0f, 0x13, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// MOVHPD Tests
#[test]
fn test_movhpd_mem_to_xmm0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x16, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhpd_mem_to_xmm7() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x16, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhpd_xmm0_to_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x17, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhpd_xmm7_to_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x17, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhpd_mem_to_xmm15() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x16, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhpd_xmm15_to_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x17, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// MOVLPD Tests
#[test]
fn test_movlpd_mem_to_xmm0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x12, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlpd_mem_to_xmm7() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x12, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlpd_xmm0_to_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x13, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlpd_xmm7_to_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x13, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlpd_mem_to_xmm15() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x12, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlpd_xmm15_to_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x13, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Mixed Tests
#[test]
fn test_movhps_movlps_combined() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x12, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVLPS XMM0, [0x3000]
        0x0f, 0x16, 0x04, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVHPS XMM0, [0x3008]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhpd_movlpd_combined() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x12, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVLPD XMM0, [0x3000]
        0x66, 0x0f, 0x16, 0x04, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVHPD XMM0, [0x3008]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhps_round_trip() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x17, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVHPS [0x3000], XMM0
        0x0f, 0x16, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVHPS XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlps_round_trip() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x13, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVLPS [0x3000], XMM0
        0x0f, 0x12, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVLPS XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_multiple_movhps() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x16, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVHPS XMM0, [0x3000]
        0x0f, 0x16, 0x0c, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVHPS XMM1, [0x3008]
        0x0f, 0x16, 0x14, 0x25, 0x10, 0x30, 0x00, 0x00, // MOVHPS XMM2, [0x3010]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_all_movlpd_registers() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x12, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVLPD XMM0, [0x3000]
        0x66, 0x0f, 0x12, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVLPD XMM1, [0x3000]
        0x66, 0x0f, 0x12, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVLPD XMM2, [0x3000]
        0x66, 0x0f, 0x12, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVLPD XMM3, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
