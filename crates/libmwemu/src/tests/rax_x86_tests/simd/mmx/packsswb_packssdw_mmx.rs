//! Tests for PACKSSWB and PACKSSDW instructions (MMX).
//!
//! PACKSSWB/PACKSSDW - Pack With Signed Saturation
//!
//! Converts packed signed word/dword integers from the destination and source operands
//! into packed signed byte/word integers using signed saturation to handle overflow.
//!
//! - PACKSSWB: Pack 4 signed words from mm1 and 4 from mm2/m64 into 8 signed bytes in mm1
//! - PACKSSDW: Pack 2 signed dwords from mm1 and 2 from mm2/m64 into 4 signed words in mm1
//!
//! Saturation: Values > 0x7F/0x7FFF become 0x7F/0x7FFF, values < 0x80/0x8000 become 0x80/0x8000
//!
//! Flags affected: None
//!
//! Reference: docs/packsswb:packssdw.txt

use crate::*;

// Helper to write 64-bit value to memory
fn write_mem_at_u64(mem: u64, addr: u64, value: u64) {
    let mut emu = emu64();
    emu.maps.write_bytes_slice(addr, &value.to_le_bytes());
}

// Helper to read 64-bit value from memory
fn read_mem_at_u64(mem: u64, addr: u64) -> u64 {
    let mut emu = emu64();
    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    u64::from_le_bytes(buf)
}

fn packsswb_expected(dst: u64, src: u64) -> u64 {
    let mut emu = emu64();
    let mut result = 0u64;
    for i in 0..4 {
        let w = ((dst >> (i * 16)) & 0xFFFF) as i16;
        let b = w.clamp(-128, 127) as i8 as u8;
        result |= (b as u64) << (i * 8);
    }
    for i in 0..4 {
        let w = ((src >> (i * 16)) & 0xFFFF) as i16;
        let b = w.clamp(-128, 127) as i8 as u8;
        result |= (b as u64) << ((i + 4) * 8);
    }
    result
}

fn packssdw_expected(dst: u64, src: u64) -> u64 {
    let mut emu = emu64();
    let mut result = 0u64;
    for i in 0..2 {
        let d = ((dst >> (i * 32)) & 0xFFFF_FFFF) as i32;
        let w = d.clamp(-32768, 32767) as i16 as u16;
        result |= (w as u64) << (i * 16);
    }
    for i in 0..2 {
        let d = ((src >> (i * 32)) & 0xFFFF_FFFF) as i32;
        let w = d.clamp(-32768, 32767) as i16 as u16;
        result |= (w as u64) << ((i + 2) * 16);
    }
    result
}

// ============================================================================
// PACKSSWB mm, mm/m64 (opcode 0F 63 /r) - Pack signed words to signed bytes
// ============================================================================

#[test]
fn test_packsswb_mm_mm_basic() {
    let mut emu = emu64();
    // PACKSSWB MM0, MM1 - basic packing
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0x63, 0xc1,                               // PACKSSWB MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4,                                            // HLT
    ];

    emu.load_code_bytes(&code);

    // MM0 = 4 words: 0x0001, 0x0002, 0x0003, 0x0004 (all in byte range)
    emu.maps.write_qword(0x2000, 0x0004000300020001);
    // MM1 = 4 words: 0x0005, 0x0006, 0x0007, 0x0008
    emu.maps.write_qword(0x2008, 0x0008000700060005);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(
        result,
        packsswb_expected(0x0004000300020001, 0x0008000700060005),
        "PACKSSWB: basic packing"
    );
}

#[test]
fn test_packsswb_mm_mm_positive_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x63, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0100007F00800200);
    emu.maps.write_qword(0x2008, 0x7FFF00010000FFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    // 0200 -> 7F, 0080 -> 7F, 007F -> 7F, 0100 -> 7F, FFFF -> 80, 0000 -> 00, 0001 -> 01, 7FFF -> 7F
    assert_eq!(
        result,
        packsswb_expected(0x0100007F00800200, 0x7FFF00010000FFFF),
        "PACKSSWB: positive saturation"
    );
}

#[test]
fn test_packsswb_mm_mm_negative_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x63, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFF00FF7FFF80FFFF);
    emu.maps.write_qword(0x2008, 0x8000FFFFFFFF81FF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    // FFFF -> FF, FF80 -> 80, FF7F -> 80, FF00 -> 80, FF81 -> 81, FFFF -> FF, FFFF -> FF, 8000 -> 80
    assert_eq!(
        result,
        packsswb_expected(0xFF00FF7FFF80FFFF, 0x8000FFFFFFFF81FF),
        "PACKSSWB: negative saturation"
    );
}

#[test]
fn test_packsswb_mm_m64() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x63, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00, // PACKSSWB MM2, [0x2008]
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0010002000300040);
    emu.maps.write_qword(0x2008, 0x0050006000700080);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    // 40 30 20 10 from MM2, then 80 (sat) 70 60 50 from memory
    assert_eq!(
        result,
        packsswb_expected(0x0010002000300040, 0x0050006000700080),
        "PACKSSWB: memory operand"
    );
}

#[test]
fn test_packsswb_all_zeros() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x63, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0000000000000000);
    emu.maps.write_qword(0x2008, 0x0000000000000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(
        result,
        packsswb_expected(0x0000000000000000, 0x0000000000000000),
        "PACKSSWB: all zeros"
    );
}

#[test]
fn test_packsswb_boundary_values() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x63, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x007F0080FF80FF81);
    emu.maps.write_qword(0x2008, 0x00000001FFFFFFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    // FF81 -> 81, FF80 -> 80, 0080 -> 7F (sat), 007F -> 7F, FFFF -> FF, 0001 -> 01, 0000 -> 00
    assert_eq!(
        result,
        packsswb_expected(0x007F0080FF80FF81, 0x00000001FFFFFFFF),
        "PACKSSWB: boundary values"
    );
}

#[test]
fn test_packsswb_mm3_mm4() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x63, 0xdc,                               // PACKSSWB MM3, MM4
        0x0f, 0x7f, 0x1c, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x000A000B000C000D);
    emu.maps.write_qword(0x2008, 0x000E000F00100011);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(
        result,
        packsswb_expected(0x000A000B000C000D, 0x000E000F00100011),
        "PACKSSWB: MM3 with MM4"
    );
}

#[test]
fn test_packsswb_max_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x63, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x7FFF7FFF7FFF7FFF);
    emu.maps.write_qword(0x2008, 0x7FFF7FFF7FFF7FFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(
        result,
        packsswb_expected(0x7FFF7FFF7FFF7FFF, 0x7FFF7FFF7FFF7FFF),
        "PACKSSWB: all max saturation"
    );
}

#[test]
fn test_packsswb_min_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x63, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x8000800080008000);
    emu.maps.write_qword(0x2008, 0x8000800080008000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(
        result,
        packsswb_expected(0x8000800080008000, 0x8000800080008000),
        "PACKSSWB: all min saturation"
    );
}

#[test]
fn test_packsswb_mixed_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x63, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0100007FFF800001);
    emu.maps.write_qword(0x2008, 0x80007FFF0000FFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    // 0001->01, FF80->80, 007F->7F, 0100->7F, FFFF->FF, 0000->00, 7FFF->7F, 8000->80
    assert_eq!(
        result,
        packsswb_expected(0x0100007FFF800001, 0x80007FFF0000FFFF),
        "PACKSSWB: mixed saturation"
    );
}

// ============================================================================
// PACKSSDW mm, mm/m64 (opcode 0F 6B /r) - Pack signed dwords to signed words
// ============================================================================

#[test]
fn test_packssdw_mm_mm_basic() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6b, 0xc1,                               // PACKSSDW MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // MM0 = 2 dwords: 0x00000001, 0x00000002 (all in word range)
    emu.maps.write_qword(0x2000, 0x0000000200000001);
    // MM1 = 2 dwords: 0x00000003, 0x00000004
    emu.maps.write_qword(0x2008, 0x0000000400000003);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(
        result,
        packssdw_expected(0x0000000200000001, 0x0000000400000003),
        "PACKSSDW: basic packing"
    );
}

#[test]
fn test_packssdw_mm_mm_positive_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6b, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x00010000000080000);
    emu.maps.write_qword(0x2008, 0x7FFFFFFF00000001);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    // 0x8000->7FFF, 0x10000->7FFF, 0x1->0001, 0x7FFFFFFF->7FFF
    assert_eq!(
        result,
        packssdw_expected(0x00010000000080000, 0x7FFFFFFF00000001),
        "PACKSSDW: positive saturation"
    );
}

#[test]
fn test_packssdw_mm_mm_negative_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6b, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFFFF0000FFFF8000);
    emu.maps.write_qword(0x2008, 0x80000000FFFFFFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    // FFFF8000->8000, FFFF0000->8000, FFFFFFFF->FFFF, 80000000->8000
    assert_eq!(
        result,
        packssdw_expected(0xFFFF0000FFFF8000, 0x80000000FFFFFFFF),
        "PACKSSDW: negative saturation"
    );
}

#[test]
fn test_packssdw_mm_m64() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6b, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00, // PACKSSDW MM2, [0x2008]
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0000100000002000);
    emu.maps.write_qword(0x2008, 0x0000300000004000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(
        result,
        packssdw_expected(0x0000100000002000, 0x0000300000004000),
        "PACKSSDW: memory operand"
    );
}

#[test]
fn test_packssdw_all_zeros() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6b, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0000000000000000);
    emu.maps.write_qword(0x2008, 0x0000000000000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(
        result,
        packssdw_expected(0x0000000000000000, 0x0000000000000000),
        "PACKSSDW: all zeros"
    );
}

#[test]
fn test_packssdw_boundary_values() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6b, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x00007FFFFFF8000);
    emu.maps.write_qword(0x2008, 0x00000001FFFFFFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    // FFFF8000->8000, 0x7FFF->7FFF, FFFFFFFF->FFFF, 0x1->0001
    assert_eq!(
        result,
        packssdw_expected(0x00007FFFFFF8000, 0x00000001FFFFFFFF),
        "PACKSSDW: boundary values"
    );
}

#[test]
fn test_packssdw_mm5_mm6() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x2c, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6b, 0xee,                               // PACKSSDW MM5, MM6
        0x0f, 0x7f, 0x2c, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x00000ABC00000DEF);
    emu.maps.write_qword(0x2008, 0x0000123400005678);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(
        result,
        packssdw_expected(0x00000ABC00000DEF, 0x0000123400005678),
        "PACKSSDW: MM5 with MM6"
    );
}

#[test]
fn test_packssdw_max_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6b, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x7FFFFFFF7FFFFFFF);
    emu.maps.write_qword(0x2008, 0x7FFFFFFF7FFFFFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(
        result,
        packssdw_expected(0x7FFFFFFF7FFFFFFF, 0x7FFFFFFF7FFFFFFF),
        "PACKSSDW: all max saturation"
    );
}

#[test]
fn test_packssdw_min_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6b, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x8000000080000000);
    emu.maps.write_qword(0x2008, 0x8000000080000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(
        result,
        packssdw_expected(0x8000000080000000, 0x8000000080000000),
        "PACKSSDW: all min saturation"
    );
}

#[test]
fn test_packssdw_mixed_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6b, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0001000000007FFF);
    emu.maps.write_qword(0x2008, 0x80000000FFFFFFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    // 0x7FFF->7FFF, 0->0000, FFFFFFFF->FFFF, 80000000->8000
    assert_eq!(
        result,
        packssdw_expected(0x0001000000007FFF, 0x80000000FFFFFFFF),
        "PACKSSDW: mixed saturation"
    );
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_packsswb_sequential_values() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x63, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0001000200030004);
    emu.maps.write_qword(0x2008, 0x0005000600070008);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(
        result,
        packsswb_expected(0x0001000200030004, 0x0005000600070008),
        "PACKSSWB: sequential values"
    );
}

#[test]
fn test_packssdw_sequential_values() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6b, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0000000100000002);
    emu.maps.write_qword(0x2008, 0x0000000300000004);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(
        result,
        packssdw_expected(0x0000000100000002, 0x0000000300000004),
        "PACKSSDW: sequential values"
    );
}

#[test]
fn test_packsswb_alternating_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x63, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x7FFF0001800000FF);
    emu.maps.write_qword(0x2008, 0x00017FFFFFFF8000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    // 00FF->FF, 8000->80, 0001->01, 7FFF->7F, 8000->80, FFFF->FF, 7FFF->7F, 0001->01
    assert_eq!(
        result,
        packsswb_expected(0x7FFF0001800000FF, 0x00017FFFFFFF8000),
        "PACKSSWB: alternating saturation"
    );
}

#[test]
fn test_packssdw_alternating_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6b, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x7FFFFFFF00000001);
    emu.maps.write_qword(0x2008, 0x0000000180000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    // 0x1->0001, 0x7FFFFFFF->7FFF, 0x80000000->8000, 0x1->0001
    assert_eq!(
        result,
        packssdw_expected(0x7FFFFFFF00000001, 0x0000000180000000),
        "PACKSSDW: alternating saturation"
    );
}

#[test]
fn test_packsswb_register_independence() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00,
        0x0f, 0x63, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,
        0x0f, 0x7f, 0x14, 0x25, 0x20, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0001000200030004);
    emu.maps.write_qword(0x2008, 0x0005000600070008);
    emu.maps.write_qword(0x2010, 0x1111111111111111);

    emu.run(None).unwrap();

    let mm0_result = emu.maps.read_qword(0x2018).unwrap();
    assert_eq!(
        mm0_result,
        packsswb_expected(0x0001000200030004, 0x0005000600070008),
        "PACKSSWB: MM0 result"
    );

    let mm2_result = emu.maps.read_qword(0x2020).unwrap();
    assert_eq!(mm2_result, 0x1111111111111111, "PACKSSWB: MM2 unchanged");
}

#[test]
fn test_packssdw_register_independence() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00,
        0x0f, 0x6b, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,
        0x0f, 0x7f, 0x14, 0x25, 0x20, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0000000100000002);
    emu.maps.write_qword(0x2008, 0x0000000300000004);
    emu.maps.write_qword(0x2010, 0x2222222222222222);

    emu.run(None).unwrap();

    let mm0_result = emu.maps.read_qword(0x2018).unwrap();
    assert_eq!(
        mm0_result,
        packssdw_expected(0x0000000100000002, 0x0000000300000004),
        "PACKSSDW: MM0 result"
    );

    let mm2_result = emu.maps.read_qword(0x2020).unwrap();
    assert_eq!(mm2_result, 0x2222222222222222, "PACKSSDW: MM2 unchanged");
}

#[test]
fn test_packsswb_mm7_mm0() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x3c, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x63, 0xf8,
        0x0f, 0x7f, 0x3c, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x00200030007F0080);
    emu.maps.write_qword(0x2008, 0xFF80FF00010020FF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    // 0080->7F, 007F->7F, 0030->30, 0020->20, 20FF->7F (sat), 0100->7F (sat), FF00->80 (sat), FF80->80
    assert_eq!(
        result,
        packsswb_expected(0x00200030007F0080, 0xFF80FF00010020FF),
        "PACKSSWB: MM7 with MM0"
    );
}

#[test]
fn test_packssdw_mm7_mm0() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x3c, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6b, 0xf8,
        0x0f, 0x7f, 0x3c, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0001000000007FFF);
    emu.maps.write_qword(0x2008, 0xFFFF8000FFFFFFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    // 0x7FFF->7FFF, 0->0000, FFFFFFFF->FFFF, FFFF8000->8000
    assert_eq!(
        result,
        packssdw_expected(0x0001000000007FFF, 0xFFFF8000FFFFFFFF),
        "PACKSSDW: MM7 with MM0"
    );
}
