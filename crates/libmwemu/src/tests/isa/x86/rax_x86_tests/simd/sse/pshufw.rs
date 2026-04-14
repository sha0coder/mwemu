use crate::*;

// PSHUFW - Shuffle Packed Words (MMX)
//
// Shuffles the words in the source operand according to the encoding specified
// by the immediate operand. The result is stored in the destination.
//
// Opcode:
// NP 0F 70 /r ib    PSHUFW mm1, mm2/m64, imm8

const DATA_ADDR: u64 = 0x3000;

#[test]
fn test_pshufw_mm0_mm1_0x00() {
    let mut emu = emu64();
    let code = [0x0f, 0x70, 0xc1, 0x00, 0xf4]; // PSHUFW MM0, MM1, 0x00; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufw_mm2_mm3_0xff() {
    let mut emu = emu64();
    let code = [0x0f, 0x70, 0xd3, 0xff, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufw_mm4_mm5_0xe4() {
    let mut emu = emu64();
    let code = [0x0f, 0x70, 0xe5, 0xe4, 0xf4]; // Reverse order
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufw_mm6_mm7_0x1b() {
    let mut emu = emu64();
    let code = [0x0f, 0x70, 0xf7, 0x1b, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufw_mm0_mm0_0xaa() {
    let mut emu = emu64();
    let code = [0x0f, 0x70, 0xc0, 0xaa, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufw_mm3_mm3_0x55() {
    let mut emu = emu64();
    let code = [0x0f, 0x70, 0xdb, 0x55, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufw_mm7_mem_0xb1() {
    let mut emu = emu64();
    let code = [0x0f, 0x70, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xb1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufw_mm0_mem_0x4e() {
    let mut emu = emu64();
    let code = [0x0f, 0x70, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x4e, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufw_multiple() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x70, 0xc1, 0xe4, // PSHUFW MM0, MM1, 0xe4
        0x0f, 0x70, 0xd3, 0x1b, // PSHUFW MM2, MM3, 0x1b
        0x0f, 0x70, 0xe5, 0xb1, // PSHUFW MM4, MM5, 0xb1
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufw_all_registers() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x70, 0xc0, 0x00, // PSHUFW MM0, MM0, 0x00
        0x0f, 0x70, 0xc9, 0x55, // PSHUFW MM1, MM1, 0x55
        0x0f, 0x70, 0xd2, 0xaa, // PSHUFW MM2, MM2, 0xaa
        0x0f, 0x70, 0xdb, 0xff, // PSHUFW MM3, MM3, 0xff
        0x0f, 0x70, 0xe4, 0xe4, // PSHUFW MM4, MM4, 0xe4
        0x0f, 0x70, 0xed, 0x1b, // PSHUFW MM5, MM5, 0x1b
        0x0f, 0x70, 0xf6, 0xb1, // PSHUFW MM6, MM6, 0xb1
        0x0f, 0x70, 0xff, 0x4e, // PSHUFW MM7, MM7, 0x4e
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufw_identity() {
    let mut emu = emu64();
    let code = [0x0f, 0x70, 0xc0, 0xe4, 0xf4]; // PSHUFW MM0, MM0, 0xe4 (identity)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufw_broadcast_word0() {
    let mut emu = emu64();
    let code = [0x0f, 0x70, 0xc1, 0x00, 0xf4]; // Broadcast word 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufw_broadcast_word3() {
    let mut emu = emu64();
    let code = [0x0f, 0x70, 0xc1, 0xff, 0xf4]; // Broadcast word 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufw_swap_pairs() {
    let mut emu = emu64();
    let code = [0x0f, 0x70, 0xc1, 0xb1, 0xf4]; // Swap pairs
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufw_reverse() {
    let mut emu = emu64();
    let code = [0x0f, 0x70, 0xc1, 0x1b, 0xf4]; // Reverse order
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
