//! Tests for the FIDIVR instruction.
//!
//! FIDIVR - Reverse divide integer by floating-point (m16int and m32int)
//!
//! Reference: /Users/int/dev/rax/docs/fdivr:fdivrp:fidivr.txt
//!
//! Opcode: DE /7 - FIDIVR m16int  ; ST(0) = m16int / ST(0)
//! Opcode: DA /7 - FIDIVR m32int  ; ST(0) = m32int / ST(0)

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

fn read_f64(mem: u64, addr: u64) -> f64 {
    let mut emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    f64::from_le_bytes(buf)
}

#[test]
fn test_fidivr_m16int_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDE, 0x3C, 0x25, 0x08, 0x20, 0x00, 0x00, // FIDIVR word [0x2008]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.0);
    emu.maps.write_word(DATA_ADDR + 8, (10) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 5.0); // 10 / 2
}

#[test]
fn test_fidivr_m16int_fractional() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x3C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 3.0);
    emu.maps.write_word(DATA_ADDR + 8, (1) as i16 as u16);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - (1.0 / 3.0)).abs() < 1e-15);
}

#[test]
fn test_fidivr_m16int_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x3C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -5.0);
    emu.maps.write_word(DATA_ADDR + 8, (10) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -2.0); // 10 / -5
}

#[test]
fn test_fidivr_m32int_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x3C, 0x25, 0x08, 0x20, 0x00, 0x00, // FIDIVR dword [0x2008]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 10.0);
    emu.maps.write_dword(DATA_ADDR + 8, (1000) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 100.0); // 1000 / 10
}

#[test]
fn test_fidivr_m32int_zero_dividend() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x3C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.0);
    emu.maps.write_dword(DATA_ADDR + 8, (0) as i32 as u32);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}
