//! Tests for EMMS instruction.
//!
//! EMMS - Empty MMX Technology State
//!
//! Sets x87 FPU tag word to empty (all 1s). This marks the MMX/x87 registers
//! as available for x87 floating-point use. Must be called after MMX operations
//! before using x87 FP instructions.
//!
//! Flags affected: None
//!
//! Reference: docs/emms.txt

use crate::*;

fn write_mm_via_mem(mem: u64, addr: u64, value: u64) {
    let mut emu = emu64();
    emu.maps.write_qword(addr, value);
}

// ============================================================================
// EMMS (opcode 0F 77)
// ============================================================================

#[test]
fn test_emms_basic() {
    let mut emu = emu64();
    // EMMS after a simple MMX operation
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1234567890ABCDEF);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_after_paddb() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xfc, 0xc1,                               // PADDB MM0, MM1
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0102030405060708);
    emu.maps.write_qword(0x2008, 0x0101010101010101);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_after_paddw() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xfd, 0xc1,                               // PADDW MM0, MM1
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0001000200030004);
    emu.maps.write_qword(0x2008, 0x0001000100010001);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_after_paddd() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xfe, 0xc1,                               // PADDD MM0, MM1
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0000000100000002);
    emu.maps.write_qword(0x2008, 0x0000000100000001);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_after_psubb() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xf8, 0xc1,                               // PSUBB MM0, MM1
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0A09080706050403);
    emu.maps.write_qword(0x2008, 0x0101010101010101);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_after_psubw() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xf9, 0xc1,                               // PSUBW MM0, MM1
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x000A0009000800007);
    emu.maps.write_qword(0x2008, 0x0001000100010001);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_after_psubd() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xfa, 0xc1,                               // PSUBD MM0, MM1
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0000000A00000009);
    emu.maps.write_qword(0x2008, 0x0000000100000001);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_after_pmullw() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xd5, 0xc1,                               // PMULLW MM0, MM1
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0004000300020001);
    emu.maps.write_qword(0x2008, 0x0005000400030002);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_after_pmulhw() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xe5, 0xc1,                               // PMULHW MM0, MM1
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1000100010001000);
    emu.maps.write_qword(0x2008, 0x1000100010001000);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_after_pand() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdb, 0xc1,                               // PAND MM0, MM1
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xFFFFFFFF00000000);
    emu.maps.write_qword(0x2008, 0xFF00FF00FF00FF00);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_after_por() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xeb, 0xc1,                               // POR MM0, MM1
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xFF000000FF000000);
    emu.maps.write_qword(0x2008, 0x00FF000000FF0000);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_after_pxor() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xef, 0xc1,                               // PXOR MM0, MM1
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xFF00FF00FF00FF00);
    emu.maps.write_qword(0x2008, 0xFFFF0000FFFF0000);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_after_pcmpeqb() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x74, 0xc1,                               // PCMPEQB MM0, MM1
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0102030405060708);
    emu.maps.write_qword(0x2008, 0x0102030405060708);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_after_pcmpeqw() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x75, 0xc1,                               // PCMPEQW MM0, MM1
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1234567890ABCDEF);
    emu.maps.write_qword(0x2008, 0x1234567890ABCDEF);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_after_pcmpeqd() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x76, 0xc1,                               // PCMPEQD MM0, MM1
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1234567890ABCDEF);
    emu.maps.write_qword(0x2008, 0x1234567890ABCDEF);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_multiple_ops() {
    let mut emu = emu64();
    // EMMS after sequence of operations
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xfc, 0xc1,                               // PADDB MM0, MM1
        0x0f, 0xd5, 0xc1,                               // PMULLW MM0, MM1
        0x0f, 0xdb, 0xc1,                               // PAND MM0, MM1
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0102030405060708);
    emu.maps.write_qword(0x2008, 0x0101010101010101);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_all_registers() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0
        0x0f, 0x6f, 0x0c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM1
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM2
        0x0f, 0x6f, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM3
        0x0f, 0x6f, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM4
        0x0f, 0x6f, 0x2c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM5
        0x0f, 0x6f, 0x34, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM6
        0x0f, 0x6f, 0x3c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM7
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1234567890ABCDEF);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_standalone() {
    let mut emu = emu64();
    // EMMS without prior MMX operations (should still work)
    let code = vec![
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_double() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x77,                                      // EMMS
        0x0f, 0x77,                                      // EMMS again
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1234567890ABCDEF);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_with_data_preservation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x77,                                      // EMMS
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1234567890ABCDEF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x1234567890ABCDEF, "EMMS: data preserved");
}

#[test]
fn test_emms_after_complex_sequence() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00,
        0x0f, 0xfc, 0xc1,                               // PADDB
        0x0f, 0xfd, 0xc1,                               // PADDW
        0x0f, 0xfe, 0xc1,                               // PADDD
        0x0f, 0xf8, 0xca,                               // PSUBB
        0x0f, 0xd5, 0xca,                               // PMULLW
        0x0f, 0xdb, 0xca,                               // PAND
        0x0f, 0xeb, 0xca,                               // POR
        0x0f, 0xef, 0xca,                               // PXOR
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0102030405060708);
    emu.maps.write_qword(0x2008, 0x0101010101010101);
    emu.maps.write_qword(0x2010, 0xFFFFFFFFFFFFFFFF);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_after_compares() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x74, 0xc1,                               // PCMPEQB
        0x0f, 0x75, 0xc1,                               // PCMPEQW
        0x0f, 0x76, 0xc1,                               // PCMPEQD
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1234567890ABCDEF);
    emu.maps.write_qword(0x2008, 0x1234567890ABCDEF);

    emu.run(None).unwrap();
}

#[test]
fn test_emms_in_loop_pattern() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xfc, 0xc1,                               // PADDB
        0x0f, 0x77,                                      // EMMS
        0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00,
        0x0f, 0xfd, 0xd1,                               // PADDW
        0x0f, 0x77,                                      // EMMS
        0x0f, 0x6f, 0x1c, 0x25, 0x18, 0x20, 0x00, 0x00,
        0x0f, 0xfe, 0xda,                               // PADDD
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0101010101010101);
    emu.maps.write_qword(0x2008, 0x0202020202020202);
    emu.maps.write_qword(0x2010, 0x0003000300030003);
    emu.maps.write_qword(0x2018, 0x0000000400000004);

    emu.run(None).unwrap();
}
