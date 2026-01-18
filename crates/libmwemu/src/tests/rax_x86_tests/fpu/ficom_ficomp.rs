//! Tests for the FICOM and FICOMP instructions.
//!
//! FICOM - Compare integer with floating-point (m16int and m32int)
//! FICOMP - Compare integer and pop
//!
//! Reference: /Users/int/dev/rax/docs/ficom:ficomp.txt
//!
//! Opcode: DE /2 - FICOM m16int
//! Opcode: DA /2 - FICOM m32int
//! Opcode: DE /3 - FICOMP m16int
//! Opcode: DA /3 - FICOMP m32int

use crate::*;

const DATA_ADDR: u64 = 0x2000;

fn write_f64(mem: u64, addr: u64, value: f64) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &value.to_le_bytes());
}

fn write_i16(mem: u64, addr: u64, value: i16) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &value.to_le_bytes());
}

fn write_i32(mem: u64, addr: u64, value: i32) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &value.to_le_bytes());
}

fn read_u16(mem: u64, addr: u64) -> u16 {
    let mut emu = emu64();    let mut buf = [0u8; 2];
    emu.maps.read_bytes_buff(&mut buf, addr);
    u16::from_le_bytes(buf)
}

#[test]
fn test_ficom_m16int_equal() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDE, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00, // FICOM word [0x2008]
        0xDF, 0xE0, // FSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 10.0);
    emu.maps.write_word(DATA_ADDR + 8, (10) as i16 as u16);

    emu.run(None).unwrap();
    let status = emu.maps.read_word(0x3000).unwrap();
    // C3=1, C2=0, C0=0 for equal (bits 14, 10, 8)
    assert_eq!(status & 0x4500, 0x4000);
}

#[test]
fn test_ficom_m16int_greater() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDF, 0xE0, // FSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 15.0);
    emu.maps.write_word(DATA_ADDR + 8, (10) as i16 as u16);

    emu.run(None).unwrap();
    let status = emu.maps.read_word(0x3000).unwrap();
    // C3=0, C2=0, C0=0 for greater
    assert_eq!(status & 0x4500, 0x0000);
}

#[test]
fn test_ficom_m16int_less() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDF, 0xE0, // FSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.0);
    emu.maps.write_word(DATA_ADDR + 8, (10) as i16 as u16);

    emu.run(None).unwrap();
    let status = emu.maps.read_word(0x3000).unwrap();
    // C3=0, C2=0, C0=1 for less
    assert_eq!(status & 0x4500, 0x0100);
}

#[test]
fn test_ficom_m32int_equal() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00, // FICOM dword [0x2008]
        0xDF, 0xE0,
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1000.0);
    emu.maps.write_dword(DATA_ADDR + 8, (1000) as i32 as u32);

    emu.run(None).unwrap();
    let status = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(status & 0x4500, 0x4000);
}

#[test]
fn test_ficom_m32int_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDF, 0xE0,
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -10.0);
    emu.maps.write_dword(DATA_ADDR + 8, (10) as i32 as u32);

    emu.run(None).unwrap();
    let status = emu.maps.read_word(0x3000).unwrap();
    // -10 < 10, so C0=1
    assert_eq!(status & 0x4500, 0x0100);
}

#[test]
fn test_ficomp_m16int_pops_stack() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] 
        0xDE, 0x1C, 0x25, 0x08, 0x20, 0x00, 0x00, // FICOMP word [0x2008]
        0xDF, 0xE0, // FSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 100.0);
    emu.maps.write_word(DATA_ADDR + 8, (100) as i16 as u16);

    emu.run(None).unwrap();
    let status = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(status & 0x4500, 0x4000);
}

#[test]
fn test_ficomp_m32int_pops_stack() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x1C, 0x25, 0x08, 0x20, 0x00, 0x00, // FICOMP dword [0x2008]
        0xDF, 0xE0,
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 500.0);
    emu.maps.write_dword(DATA_ADDR + 8, (500) as i32 as u32);

    emu.run(None).unwrap();
    let status = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(status & 0x4500, 0x4000);
}
