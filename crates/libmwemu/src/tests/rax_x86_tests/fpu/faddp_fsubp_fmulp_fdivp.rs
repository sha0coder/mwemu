//! Tests for FPU arithmetic instructions with pop (FADDP, FSUBP, FSUBRP, FMULP, FDIVP, FDIVRP).
//!
//! Reference: Various docs
//! - FADDP: DE C0+i - Add and pop
//! - FSUBP: DE E8+i - Subtract and pop
//! - FSUBRP: DE E0+i - Reverse subtract and pop
//! - FMULP: DE C8+i - Multiply and pop
//! - FDIVP: DE F8+i - Divide and pop
//! - FDIVRP: DE F0+i - Reverse divide and pop

use crate::*;

const DATA_ADDR: u64 = 0x2000;

fn write_f64(mem: u64, addr: u64, value: f64) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &value.to_le_bytes());
}

fn read_f64(mem: u64, addr: u64) -> f64 {
    let emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    f64::from_le_bytes(buf)
}

// ============================================================================
// FADDP - Add and pop
// ============================================================================

#[test]
fn test_faddp_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]  ; 2.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]  ; 3.0
        0xDE, 0xC1, // FADDP ST(1), ST(0)  ; ST(1) = 5.0, pop
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.0);
    emu.maps.write_f64(DATA_ADDR + 8, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 5.0);
}

#[test]
fn test_faddp_st1_st0() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 4.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 6.0
        0xDE, 0xC1, // FADDP ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 4.0);
    emu.maps.write_f64(DATA_ADDR + 8, 6.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 10.0);
}

#[test]
fn test_faddp_chain() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xDE, 0xC1, // FADDP ; ST(0) = 3.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 4.0
        0xDE, 0xC1, // FADDP ; ST(0) = 7.0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);
    emu.maps.write_f64(DATA_ADDR + 16, 4.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 7.0);
}

// ============================================================================
// FSUBP - Subtract and pop
// ============================================================================

#[test]
fn test_fsubp_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 10.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 3.0
        0xDE, 0xE9, // FSUBP ST(1), ST(0)  ; ST(1) = 10 - 3 = 7
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 10.0);
    emu.maps.write_f64(DATA_ADDR + 8, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 7.0);
}

#[test]
fn test_fsubp_negative_result() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 5.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 10.0
        0xDE, 0xE9, // FSUBP ; 5 - 10 = -5
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.0);
    emu.maps.write_f64(DATA_ADDR + 8, 10.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -5.0);
}

// ============================================================================
// FSUBRP - Reverse subtract and pop
// ============================================================================

#[test]
fn test_fsubrp_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 10.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 3.0
        0xDE, 0xE1, // FSUBRP ST(1), ST(0)  ; ST(1) = 3 - 10 = -7
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 10.0);
    emu.maps.write_f64(DATA_ADDR + 8, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -7.0);
}

#[test]
fn test_fsubrp_positive_result() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 5.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 10.0
        0xDE, 0xE1, // FSUBRP ; 10 - 5 = 5
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.0);
    emu.maps.write_f64(DATA_ADDR + 8, 10.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 5.0);
}

// ============================================================================
// FMULP - Multiply and pop
// ============================================================================

#[test]
fn test_fmulp_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 3.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 4.0
        0xDE, 0xC9, // FMULP ST(1), ST(0)  ; ST(1) = 12.0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 3.0);
    emu.maps.write_f64(DATA_ADDR + 8, 4.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 12.0);
}

#[test]
fn test_fmulp_chain() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 2.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 3.0
        0xDE, 0xC9, // FMULP ; 6.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 5.0
        0xDE, 0xC9, // FMULP ; 30.0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.0);
    emu.maps.write_f64(DATA_ADDR + 8, 3.0);
    emu.maps.write_f64(DATA_ADDR + 16, 5.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 30.0);
}

// ============================================================================
// FDIVP - Divide and pop
// ============================================================================

#[test]
fn test_fdivp_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 20.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 4.0
        0xDE, 0xF9, // FDIVP ST(1), ST(0)  ; ST(1) = 20 / 4 = 5
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 20.0);
    emu.maps.write_f64(DATA_ADDR + 8, 4.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 5.0);
}

#[test]
fn test_fdivp_fractional() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 3.0
        0xDE, 0xF9, // FDIVP ; 1 / 3
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 3.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - (1.0 / 3.0)).abs() < 1e-15);
}

// ============================================================================
// FDIVRP - Reverse divide and pop
// ============================================================================

#[test]
fn test_fdivrp_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 4.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 20.0
        0xDE, 0xF1, // FDIVRP ST(1), ST(0)  ; ST(1) = 20 / 4 = 5
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 4.0);
    emu.maps.write_f64(DATA_ADDR + 8, 20.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 5.0);
}

#[test]
fn test_fdivrp_fractional() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 3.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 1.0
        0xDE, 0xF1, // FDIVRP ; 1 / 3
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 3.0);
    emu.maps.write_f64(DATA_ADDR + 8, 1.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - (1.0 / 3.0)).abs() < 1e-15);
}
