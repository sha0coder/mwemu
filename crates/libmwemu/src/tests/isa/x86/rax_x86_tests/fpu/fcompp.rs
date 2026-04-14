//! Tests for the FCOMPP instruction.
//!
//! FCOMPP - Compare floating-point and pop twice
//!
//! Reference: /Users/int/dev/rax/docs/fcom:fcomp:fcompp.txt
//!
//! Opcode: DE D9 - FCOMPP ; Compare ST(0) with ST(1) and pop twice

use crate::*;

const DATA_ADDR: u64 = 0x2000;

fn write_f64(mem: u64, addr: u64, value: f64) {
    let mut emu = emu64();
    emu.maps.write_bytes_slice(addr, &value.to_le_bytes());
}

fn read_u16(mem: u64, addr: u64) -> u16 {
    let emu = emu64();
    let mut buf = [0u8; 2];
    emu.maps.read_bytes_buff(&mut buf, addr);
    u16::from_le_bytes(buf)
}

#[test]
fn test_fcompp_equal() {
    let mut emu = emu64();
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDE, 0xD9, // FCOMPP
        0xDF, 0xE0, // FSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 10.0);
    emu.maps.write_f64(DATA_ADDR + 8, 10.0);

    emu.run(None).unwrap();
    let status = emu.maps.read_word(0x3000).unwrap();
    // C3=1, C2=0, C0=0 for equal (bits 14, 10, 8)
    assert_eq!(status & 0x4500, 0x4000);
}

#[test]
fn test_fcompp_greater() {
    let mut emu = emu64();
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xD9, // FCOMPP
        0xDF, 0xE0, // FSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.0);
    emu.maps.write_f64(DATA_ADDR + 8, 15.0);

    emu.run(None).unwrap();
    let status = emu.maps.read_word(0x3000).unwrap();
    // ST(0) > ST(1): 15 > 5, C3=0, C2=0, C0=0
    assert_eq!(status & 0x4500, 0x0000);
}

#[test]
fn test_fcompp_less() {
    let mut emu = emu64();
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xD9, // FCOMPP
        0xDF, 0xE0, // FSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 15.0);
    emu.maps.write_f64(DATA_ADDR + 8, 5.0);

    emu.run(None).unwrap();
    let status = emu.maps.read_word(0x3000).unwrap();
    // ST(0) < ST(1): 5 < 15, C3=0, C2=0, C0=1
    assert_eq!(status & 0x4500, 0x0100);
}

#[test]
fn test_fcompp_negative_numbers() {
    let mut emu = emu64();
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xD9, 0xDF, 0xE0, 0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -10.0);
    emu.maps.write_f64(DATA_ADDR + 8, -5.0);

    emu.run(None).unwrap();
    let status = emu.maps.read_word(0x3000).unwrap();
    // -5 > -10, so ST(0) > ST(1)
    assert_eq!(status & 0x4500, 0x0000);
}

#[test]
fn test_fcompp_zero_comparison() {
    let mut emu = emu64();
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xD9, 0xDF, 0xE0, 0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.0);
    emu.maps.write_f64(DATA_ADDR + 8, 0.0);

    emu.run(None).unwrap();
    let status = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(status & 0x4500, 0x4000);
}

#[test]
fn test_fcompp_infinity() {
    let mut emu = emu64();
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xD9, 0xDF, 0xE0, 0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 100.0);
    emu.maps.write_f64(DATA_ADDR + 8, f64::INFINITY);

    emu.run(None).unwrap();
    let status = emu.maps.read_word(0x3000).unwrap();
    // ST(0) > ST(1): infinity > 100
    assert_eq!(status & 0x4500, 0x0000);
}
