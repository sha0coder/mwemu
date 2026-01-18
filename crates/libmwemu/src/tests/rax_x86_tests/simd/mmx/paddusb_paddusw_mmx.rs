//! Tests for PADDUSB and PADDUSW instructions (MMX).
//!
//! PADDUSB - Add Packed Unsigned Bytes with Saturation (MMX)
//! PADDUSW - Add Packed Unsigned Words with Saturation (MMX)
//!
//! Adds packed unsigned integers with saturation.
//! If overflow occurs, result is clamped to maximum value (0xFF for bytes, 0xFFFF for words).
//!
//! Opcodes:
//! - PADDUSB: 0F DC /r
//! - PADDUSW: 0F DD /r
//!
//! Flags affected: None
//!
//! Reference: /Users/int/dev/rax/docs/paddusb:paddusw.txt

use crate::*;

fn write_mm_via_mem(mem: u64, addr: u64, value: u64) {
    let mut emu = emu64();
    emu.maps.write_qword(addr, value);
}

// ============================================================================
// PADDUSB mm, mm/m64 - Add Packed Unsigned Bytes with Saturation
// ============================================================================

#[test]
fn test_paddusb_basic() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0xdc, 0xc1,                               // PADDUSB MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // 1+1=2, 2+2=4, 3+3=6, etc.
    emu.maps.write_qword(0x2000, 0x0807060504030201);
    emu.maps.write_qword(0x2008, 0x0807060504030201);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x100E0C0A08060402, "PADDUSB: basic addition");
}

#[test]
fn test_paddusb_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdc, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // 255 + 1 = saturate to 255
    emu.maps.write_qword(0x2000, 0xFFFFFFFFFFFFFFFF);
    emu.maps.write_qword(0x2008, 0x0101010101010101);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "PADDUSB: saturation at 255");
}

#[test]
fn test_paddusb_max_values() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdc, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // 255 + 255 = saturate to 255
    emu.maps.write_qword(0x2000, 0xFFFFFFFFFFFFFFFF);
    emu.maps.write_qword(0x2008, 0xFFFFFFFFFFFFFFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "PADDUSB: max + max saturates");
}

#[test]
fn test_paddusb_all_zeros() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdc, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0000000000000000);
    emu.maps.write_qword(0x2008, 0x0000000000000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0000000000000000, "PADDUSB: all zeros");
}

#[test]
fn test_paddusb_no_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdc, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // 100 + 100 = 200 (no saturation)
    emu.maps.write_qword(0x2000, 0x6464646464646464);
    emu.maps.write_qword(0x2008, 0x6464646464646464);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xC8C8C8C8C8C8C8C8, "PADDUSB: no saturation");
}

#[test]
fn test_paddusb_edge_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdc, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // 254 + 1 = 255, 254 + 2 = 256 -> saturate to 255
    emu.maps.write_qword(0x2000, 0xFEFEFEFEFEFEFEFE);
    emu.maps.write_qword(0x2008, 0x0201020102010201);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "PADDUSB: edge saturation");
}

#[test]
fn test_paddusb_mixed() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdc, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFF80FF8001000100);
    emu.maps.write_qword(0x2008, 0x0101010101010101);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    // 1+1=2, 0+1=1, 128+1=129, 255+1=255(sat), 128+1=129, 255+1=255(sat)
    assert_eq!(result, 0xFF81FF8102010201, "PADDUSB: mixed results");
}

// ============================================================================
// PADDUSW mm, mm/m64 - Add Packed Unsigned Words with Saturation
// ============================================================================

#[test]
fn test_paddusw_basic() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0xdd, 0xc1,                               // PADDUSW MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // 100+100=200, 200+200=400, 300+300=600, 400+400=800
    emu.maps.write_qword(0x2000, 0x0190012C00C80064);
    emu.maps.write_qword(0x2008, 0x0190012C00C80064);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x03200258019000C8, "PADDUSW: basic addition");
}

#[test]
fn test_paddusw_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdd, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // 65535 + 1 = saturate to 65535
    emu.maps.write_qword(0x2000, 0xFFFFFFFFFFFFFFFF);
    emu.maps.write_qword(0x2008, 0x0001000100010001);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "PADDUSW: saturation at 65535");
}

#[test]
fn test_paddusw_max_values() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdd, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // 65535 + 65535 = saturate to 65535
    emu.maps.write_qword(0x2000, 0xFFFFFFFFFFFFFFFF);
    emu.maps.write_qword(0x2008, 0xFFFFFFFFFFFFFFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "PADDUSW: max + max saturates");
}

#[test]
fn test_paddusw_all_zeros() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdd, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0000000000000000);
    emu.maps.write_qword(0x2008, 0x0000000000000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0000000000000000, "PADDUSW: all zeros");
}

#[test]
fn test_paddusw_no_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdd, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // 30000 + 30000 = 60000 (no saturation)
    emu.maps.write_qword(0x2000, 0x7530753075307530);
    emu.maps.write_qword(0x2008, 0x7530753075307530);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xEA60EA60EA60EA60, "PADDUSW: no saturation");
}

#[test]
fn test_paddusw_edge_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdd, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // 65534 + 1 = 65535, 65534 + 2 = 65536 -> saturate to 65535
    emu.maps.write_qword(0x2000, 0xFFFEFFFEFFFEFFFE);
    emu.maps.write_qword(0x2008, 0x0002000100020001);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "PADDUSW: edge saturation");
}

#[test]
fn test_paddusw_mixed() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdd, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFFFF800000010001);
    emu.maps.write_qword(0x2008, 0x0001000100010001);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    // 1+1=2, 1+1=2, 32768+1=32769, 65535+1=65535(sat)
    assert_eq!(result, 0xFFFF800100020002, "PADDUSW: mixed results");
}

// Additional tests

#[test]
fn test_paddusb_mm_m64() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM2, [0x2000]
        0x0f, 0xdc, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00, // PADDUSB MM2, [0x2008]
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM2
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0102030405060708);
    emu.maps.write_qword(0x2008, 0x0102030405060708);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x020406080A0C0E10, "PADDUSB: memory operand");
}

#[test]
fn test_paddusw_mm_m64() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM2, [0x2000]
        0x0f, 0xdd, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00, // PADDUSW MM2, [0x2008]
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM2
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0001000200030004);
    emu.maps.write_qword(0x2008, 0x0001000200030004);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0002000400060008, "PADDUSW: memory operand");
}

#[test]
fn test_paddusb_sequential() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdc, 0xc1,                               // PADDUSB MM0, MM1
        0x0f, 0xdc, 0xc1,                               // PADDUSB MM0, MM1 (again)
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0A0A0A0A0A0A0A0A);
    emu.maps.write_qword(0x2008, 0x0505050505050505);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x1414141414141414, "PADDUSB: sequential operations");
}

#[test]
fn test_paddusw_sequential() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdd, 0xc1,                               // PADDUSW MM0, MM1
        0x0f, 0xdd, 0xc1,                               // PADDUSW MM0, MM1 (again)
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x03E803E803E803E8); // 1000
    emu.maps.write_qword(0x2008, 0x01F401F401F401F4); // 500

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x07D007D007D007D0, "PADDUSW: sequential operations");
}

#[test]
fn test_paddusb_saturation_progressive() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdc, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // 250+10=255(saturates), 240+20=255(saturates), 200+60=255(saturates), 150+110=255(saturates)
    emu.maps.write_qword(0x2000, 0x9664C8F096FA96FA);
    emu.maps.write_qword(0x2008, 0x6E3C1C0A6E050600);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    let _result = emu.maps.read_qword(0x2010).unwrap();
}

#[test]
fn test_paddusw_saturation_progressive() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdd, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFFF0FFF0FFF0FFF0);
    emu.maps.write_qword(0x2008, 0x0020002000200020);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "PADDUSW: progressive saturation");
}

#[test]
fn test_paddusb_all_mm_registers() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x2c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM5, [0x2000]
        0x0f, 0x6f, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM6, [0x2008]
        0x0f, 0xdc, 0xee,                               // PADDUSB MM5, MM6
        0x0f, 0x7f, 0x2c, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM5
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0102030405060708);
    emu.maps.write_qword(0x2008, 0x0102030405060708);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x020406080A0C0E10, "PADDUSB: MM5 and MM6");
}

#[test]
fn test_paddusw_all_mm_registers() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x3c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM7, [0x2000]
        0x0f, 0x6f, 0x1c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM3, [0x2008]
        0x0f, 0xdd, 0xfb,                               // PADDUSW MM7, MM3
        0x0f, 0x7f, 0x3c, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM7
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0001000200030004);
    emu.maps.write_qword(0x2008, 0x0001000200030004);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0002000400060008, "PADDUSW: MM7 and MM3");
}

#[test]
fn test_paddusb_byte_patterns() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdc, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x8040201008040201);
    emu.maps.write_qword(0x2008, 0x8040201008040201);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    // 1+1=2, 2+2=4, 4+4=8, 8+8=16, 16+16=32, 32+32=64, 64+64=128, 128+128=256->255(sat)
    assert_eq!(result, 0xFF80402010080402, "PADDUSB: byte patterns");
}

#[test]
fn test_paddusw_word_patterns() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdd, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x8000400020001000);
    emu.maps.write_qword(0x2008, 0x8000400020001000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    // 4096+4096=8192, 8192+8192=16384, 16384+16384=32768, 32768+32768=65536->65535(sat)
    assert_eq!(result, 0xFFFF800040002000, "PADDUSW: word patterns");
}
