use crate::*;

// IDIV — Signed Divide
//
// Opcodes:
// - F6 /7       IDIV r/m8      Signed divide AX by r/m8
//                              AL := Quotient, AH := Remainder
// - REX + F6 /7 IDIV r/m8*     (with REX for extended regs)
// - F7 /7       IDIV r/m16     Signed divide DX:AX by r/m16
//                              AX := Quotient, DX := Remainder
// - F7 /7       IDIV r/m32     Signed divide EDX:EAX by r/m32
//                              EAX := Quotient, EDX := Remainder
// - REX.W+F7 /7 IDIV r/m64     Signed divide RDX:RAX by r/m64
//                              RAX := Quotient, RDX := Remainder
//
// Operation: SIGNED division of double-width dividend by divisor
//   8-bit:  AL = (signed)AX / (signed)r/m8, AH = remainder
//   16-bit: AX = (signed)DX:AX / (signed)r/m16, DX = remainder
//   32-bit: EAX = (signed)EDX:EAX / (signed)r/m32, EDX = remainder
//   64-bit: RAX = (signed)RDX:RAX / (signed)r/m64, RDX = remainder
//
// Quotient Ranges (signed):
//   8-bit: -128 to +127
//   16-bit: -32768 to +32767
//   32-bit: -2^31 to 2^31-1
//   64-bit: -2^63 to 2^63-1
//
// Flags: CF, OF, SF, ZF, AF, PF are undefined (not tested).
//
// Exceptions: #DE if divisor is 0 or quotient out of range.
//
// CRITICAL: Truncation towards 0. Remainder has same sign as dividend.

// ============================================================================
// 8-bit IDIV (opcode F6 /7) - Dividend in AX, Result in AL (quotient) and AH (remainder)
// ============================================================================

#[test]
fn test_idiv_al_positive_by_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf6, 0xfb, // IDIV BL (F6 /7, ModRM=11_111_011)
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 100; // AX = 100 (dividend)
    emu.regs_mut().rbx = 10;  // BL = 10 (divisor)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 10, "AL (quotient): 100 / 10 = 10");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0, "AH (remainder): 100 % 10 = 0");
}

#[test]
fn test_idiv_al_positive_by_positive_with_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    emu.regs_mut().rax = 107; // AX = 107
    emu.regs_mut().rbx = 10;  // BL = 10
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFF) as i8, 10, "AL: 107 / 10 = 10");
    assert_eq!(((emu.regs().rax >> 8) & 0xFF) as i8, 7, "AH: 107 % 10 = 7");
}

#[test]
fn test_idiv_al_negative_by_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -100 / 10 = -10 remainder 0
    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    emu.regs_mut().rax = (-100i16) as u16 as u64; // AX = -100
    emu.regs_mut().rbx = 10;                       // BL = 10
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFF) as i8, -10, "AL: -100 / 10 = -10");
    assert_eq!(((emu.regs().rax >> 8) & 0xFF) as i8, 0, "AH: remainder = 0");
}

#[test]
fn test_idiv_al_negative_by_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -100 / -10 = 10 remainder 0
    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    emu.regs_mut().rax = (-100i16) as u16 as u64;
    emu.regs_mut().rbx = (-10i8) as u8 as u64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFF) as i8, 10, "AL: -100 / -10 = 10");
    assert_eq!(((emu.regs().rax >> 8) & 0xFF) as i8, 0, "AH: remainder = 0");
}

#[test]
fn test_idiv_al_positive_by_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 100 / -10 = -10 remainder 0
    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    emu.regs_mut().rax = 100;
    emu.regs_mut().rbx = (-10i8) as u8 as u64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFF) as i8, -10, "AL: 100 / -10 = -10");
    assert_eq!(((emu.regs().rax >> 8) & 0xFF) as i8, 0, "AH: remainder = 0");
}

#[test]
fn test_idiv_al_negative_with_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -107 / 10 = -10 remainder -7 (truncate towards zero)
    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    emu.regs_mut().rax = (-107i16) as u16 as u64;
    emu.regs_mut().rbx = 10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFF) as i8, -10, "AL: -107 / 10 = -10");
    assert_eq!(((emu.regs().rax >> 8) & 0xFF) as i8, -7, "AH: -107 % 10 = -7 (same sign as dividend)");
}

#[test]
fn test_idiv_al_truncation_towards_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -17 / 5 = -3 remainder -2 (truncate towards zero, not floor)
    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    emu.regs_mut().rax = (-17i16) as u16 as u64;
    emu.regs_mut().rbx = 5;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFF) as i8, -3, "AL: -17 / 5 = -3 (towards zero)");
    assert_eq!(((emu.regs().rax >> 8) & 0xFF) as i8, -2, "AH: -17 % 5 = -2");
}

#[test]
fn test_idiv_al_by_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    emu.regs_mut().rax = (-50i16) as u16 as u64;
    emu.regs_mut().rbx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFF) as i8, -50, "AL: -50 / 1 = -50");
    assert_eq!(((emu.regs().rax >> 8) & 0xFF) as i8, 0, "AH: remainder = 0");
}

#[test]
fn test_idiv_al_max_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 127 / 1 = 127 (max i8)
    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    emu.regs_mut().rax = 127;
    emu.regs_mut().rbx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFF) as i8, 127, "AL: max i8");
}

#[test]
fn test_idiv_al_max_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -128 / 1 = -128 (min i8)
    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    emu.regs_mut().rax = (-128i16) as u16 as u64;
    emu.regs_mut().rbx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFF) as i8, -128, "AL: min i8");
}

#[test]
fn test_idiv_cl_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xf9, 0xf4]; // IDIV CL
    emu.regs_mut().rax = (-100i16) as u16 as u64;
    emu.regs_mut().rcx = 7;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFF) as i8, -14, "AL: -100 / 7 = -14");
    assert_eq!(((emu.regs().rax >> 8) & 0xFF) as i8, -2, "AH: -100 % 7 = -2");
}

// ============================================================================
// 16-bit IDIV (opcode F7 /7 with 0x66 prefix) - Dividend in DX:AX
// ============================================================================

#[test]
fn test_idiv_ax_positive_by_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0xf7, 0xfb, // IDIV BX (66 F7 /7)
        0xf4,
    ];
    emu.regs_mut().rax = 10000; // AX = 10000
    emu.regs_mut().rdx = 0;     // DX = 0
    emu.regs_mut().rbx = 100;   // BX = 100
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFFFF) as i16, 100, "AX: 10000 / 100 = 100");
    assert_eq!((emu.regs().rdx & 0xFFFF) as i16, 0, "DX: remainder = 0");
}

#[test]
fn test_idiv_ax_negative_by_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -10000 / 100 = -100 remainder 0
    let code = [0x66, 0xf7, 0xfb, 0xf4]; // IDIV BX
    emu.regs_mut().rax = (-10000i32) as u32 as u64 & 0xFFFF; // AX (low word)
    emu.regs_mut().rdx = ((-10000i32) as u32 >> 16) as u64;  // DX (high word, sign extension)
    emu.regs_mut().rbx = 100;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFFFF) as i16, -100, "AX: -10000 / 100 = -100");
    assert_eq!((emu.regs().rdx & 0xFFFF) as i16, 0, "DX: remainder = 0");
}

#[test]
fn test_idiv_ax_negative_by_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -10000 / -100 = 100
    let code = [0x66, 0xf7, 0xfb, 0xf4]; // IDIV BX
    emu.regs_mut().rax = (-10000i32) as u32 as u64 & 0xFFFF;
    emu.regs_mut().rdx = ((-10000i32) as u32 >> 16) as u64;
    emu.regs_mut().rbx = (-100i16) as u16 as u64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFFFF) as i16, 100, "AX: -10000 / -100 = 100");
}

#[test]
fn test_idiv_ax_with_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 12345 / 1000 = 12 remainder 345
    let code = [0x66, 0xf7, 0xfb, 0xf4]; // IDIV BX
    emu.regs_mut().rax = 12345;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rbx = 1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFFFF) as i16, 12, "AX: quotient");
    assert_eq!((emu.regs().rdx & 0xFFFF) as i16, 345, "DX: remainder");
}

#[test]
fn test_idiv_ax_negative_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -12345 / 1000 = -12 remainder -345
    let code = [0x66, 0xf7, 0xfb, 0xf4]; // IDIV BX
    let dividend = -12345i32;
    emu.regs_mut().rax = (dividend as u32 & 0xFFFF) as u64;
    emu.regs_mut().rdx = ((dividend as u32 >> 16) & 0xFFFF) as u64;
    emu.regs_mut().rbx = 1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFFFF) as i16, -12, "AX: -12345 / 1000 = -12");
    assert_eq!((emu.regs().rdx & 0xFFFF) as i16, -345, "DX: remainder same sign as dividend");
}

#[test]
fn test_idiv_ax_by_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xfb, 0xf4]; // IDIV BX
    emu.regs_mut().rax = (-5000i16) as u16 as u64;
    emu.regs_mut().rdx = 0xFFFF; // Sign extension for negative
    emu.regs_mut().rbx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFFFF) as i16, -5000, "AX: divided by 1");
}

#[test]
fn test_idiv_cx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xf9, 0xf4]; // IDIV CX
    emu.regs_mut().rax = 20000;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rcx = (-300i16) as u16 as u64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFFFF) as i16, -66, "AX: 20000 / -300 = -66");
}

// ============================================================================
// 32-bit IDIV (opcode F7 /7) - Dividend in EDX:EAX
// ============================================================================

#[test]
fn test_idiv_eax_positive_by_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf7, 0xfb, // IDIV EBX (F7 /7)
        0xf4,
    ];
    emu.regs_mut().rax = 1000000; // EAX
    emu.regs_mut().rdx = 0;       // EDX
    emu.regs_mut().rbx = 1000;    // EBX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax as i32, 1000, "EAX: 1000000 / 1000 = 1000");
    assert_eq!(emu.regs().rdx as i32, 0, "EDX: remainder = 0");
}

#[test]
fn test_idiv_eax_negative_by_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -1000000 / 1000 = -1000
    let code = [0xf7, 0xfb, 0xf4]; // IDIV EBX
    let dividend = -1000000i64;
    emu.regs_mut().rax = (dividend as u64) & 0xFFFFFFFF;
    emu.regs_mut().rdx = ((dividend as u64) >> 32) & 0xFFFFFFFF;
    emu.regs_mut().rbx = 1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax as i32, -1000, "EAX: -1000000 / 1000 = -1000");
    assert_eq!(emu.regs().rdx as i32, 0, "EDX: remainder = 0");
}

#[test]
fn test_idiv_eax_negative_by_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -1000000 / -1000 = 1000
    let code = [0xf7, 0xfb, 0xf4]; // IDIV EBX
    let dividend = -1000000i64;
    emu.regs_mut().rax = (dividend as u64) & 0xFFFFFFFF;
    emu.regs_mut().rdx = ((dividend as u64) >> 32) & 0xFFFFFFFF;
    emu.regs_mut().rbx = (-1000i32) as u32 as u64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax as i32, 1000, "EAX: -1000000 / -1000 = 1000");
}

#[test]
fn test_idiv_eax_with_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 123456789 / 10000 = 12345 remainder 6789
    let code = [0xf7, 0xfb, 0xf4]; // IDIV EBX
    emu.regs_mut().rax = 123456789;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rbx = 10000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax as i32, 12345, "EAX: quotient");
    assert_eq!(emu.regs().rdx as i32, 6789, "EDX: remainder");
}

#[test]
fn test_idiv_eax_negative_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -123456789 / 10000 = -12345 remainder -6789
    let code = [0xf7, 0xfb, 0xf4]; // IDIV EBX
    let dividend = -123456789i64;
    emu.regs_mut().rax = (dividend as u64) & 0xFFFFFFFF;
    emu.regs_mut().rdx = ((dividend as u64) >> 32) & 0xFFFFFFFF;
    emu.regs_mut().rbx = 10000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax as i32, -12345, "EAX: quotient");
    assert_eq!(emu.regs().rdx as i32, -6789, "EDX: remainder (same sign as dividend)");
}

#[test]
fn test_idiv_eax_by_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xfb, 0xf4]; // IDIV EBX
    emu.regs_mut().rax = (-50000i32) as u32 as u64;
    emu.regs_mut().rdx = 0xFFFFFFFF; // Sign extension
    emu.regs_mut().rbx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax as i32, -50000, "EAX: divided by 1");
}

#[test]
fn test_idiv_ecx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xf9, 0xf4]; // IDIV ECX
    emu.regs_mut().rax = 100000000;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rcx = (-9999i32) as u32 as u64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax as i32, -10001, "EAX: 100000000 / -9999");
}

// ============================================================================
// 64-bit IDIV (opcode REX.W + F7 /7) - Dividend in RDX:RAX
// ============================================================================

#[test]
fn test_idiv_rax_positive_by_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xf7, 0xfb, // IDIV RBX (REX.W F7 /7)
        0xf4,
    ];
    emu.regs_mut().rax = 1000000000000; // RAX
    emu.regs_mut().rdx = 0;             // RDX
    emu.regs_mut().rbx = 1000000;       // RBX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax as i64, 1000000, "RAX: 1e12 / 1e6 = 1e6");
    assert_eq!(emu.regs().rdx as i64, 0, "RDX: remainder = 0");
}

#[test]
fn test_idiv_rax_negative_by_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -1000000000000 / 1000000 = -1000000
    let code = [0x48, 0xf7, 0xfb, 0xf4]; // IDIV RBX
    let dividend = -1000000000000i64;
    emu.regs_mut().rax = dividend as u64;
    emu.regs_mut().rdx = if dividend < 0 { 0xFFFFFFFFFFFFFFFF } else { 0 };
    emu.regs_mut().rbx = 1000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax as i64, -1000000, "RAX: quotient");
    assert_eq!(emu.regs().rdx as i64, 0, "RDX: remainder = 0");
}

#[test]
fn test_idiv_rax_negative_by_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -1000000000000 / -1000000 = 1000000
    let code = [0x48, 0xf7, 0xfb, 0xf4]; // IDIV RBX
    let dividend = -1000000000000i64;
    emu.regs_mut().rax = dividend as u64;
    emu.regs_mut().rdx = 0xFFFFFFFFFFFFFFFF; // Sign extension
    emu.regs_mut().rbx = (-1000000i64) as u64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax as i64, 1000000, "RAX: -1e12 / -1e6 = 1e6");
}

#[test]
fn test_idiv_rax_with_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 123456789012345 / 10000000 = 12345678 remainder 9012345
    let code = [0x48, 0xf7, 0xfb, 0xf4]; // IDIV RBX
    emu.regs_mut().rax = 123456789012345;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rbx = 10000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax as i64, 12345678, "RAX: quotient");
    assert_eq!(emu.regs().rdx as i64, 9012345, "RDX: remainder");
}

#[test]
fn test_idiv_rax_negative_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -123456789012345 / 10000000 = -12345678 remainder -9012345
    let code = [0x48, 0xf7, 0xfb, 0xf4]; // IDIV RBX
    let dividend = -123456789012345i64;
    emu.regs_mut().rax = dividend as u64;
    emu.regs_mut().rdx = 0xFFFFFFFFFFFFFFFF;
    emu.regs_mut().rbx = 10000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax as i64, -12345678, "RAX: quotient");
    assert_eq!(emu.regs().rdx as i64, -9012345, "RDX: remainder (same sign as dividend)");
}

#[test]
fn test_idiv_rax_by_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xfb, 0xf4]; // IDIV RBX
    emu.regs_mut().rax = (-999999999i64) as u64;
    emu.regs_mut().rdx = 0xFFFFFFFFFFFFFFFF;
    emu.regs_mut().rbx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax as i64, -999999999, "RAX: divided by 1");
}

#[test]
fn test_idiv_rcx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xf9, 0xf4]; // IDIV RCX
    emu.regs_mut().rax = 987654321098765;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rcx = (-123456789i64) as u64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax as i64, -8000000, "RAX: quotient");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_idiv_r8b_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x41, 0xf6, 0xf8, // IDIV R8B (REX.B F6 /7)
        0xf4,
    ];
    emu.regs_mut().rax = (-200i16) as u16 as u64;
    emu.regs_mut().r8 = 15;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFF) as i8, -13, "AL: -200 / 15 = -13");
    assert_eq!(((emu.regs().rax >> 8) & 0xFF) as i8, -5, "AH: -200 % 15 = -5");
}

#[test]
fn test_idiv_r9w_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0x41, 0xf7, 0xf9, // IDIV R9W
        0xf4,
    ];
    emu.regs_mut().rax = 10000;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().r9 = (-123i16) as u16 as u64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFFFF) as i16, -81, "AX: 10000 / -123 = -81");
}

#[test]
fn test_idiv_r10d_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x41, 0xf7, 0xfa, // IDIV R10D
        0xf4,
    ];
    emu.regs_mut().rax = 1000000;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().r10 = (-999i32) as u32 as u64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax as i32, -1001, "EAX: 1000000 / -999 = -1001");
}

#[test]
fn test_idiv_r11_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x49, 0xf7, 0xfb, // IDIV R11 (REX.WB F7 /7)
        0xf4,
    ];
    emu.regs_mut().rax = 123456789012;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().r11 = (-987654i64) as u64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax as i64, -125000, "RAX: quotient");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_idiv_byte_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf6, 0x3d, 0xfa, 0x0f, 0x00, 0x00, // IDIV BYTE PTR [rip+0x0FFA]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, (-7i8) as u8);

    emu.regs_mut().rax = 50;

    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFF) as i8, -7, "AL: 50 / -7 = -7");
    assert_eq!(((emu.regs().rax >> 8) & 0xFF) as i8, 1, "AH: 50 % -7 = 1");
}

#[test]
fn test_idiv_dword_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf7, 0x3d, 0xfa, 0x0f, 0x00, 0x00, // IDIV DWORD PTR [rip+0x0FFA]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, (-12345i32) as u32);

    emu.regs_mut().rax = 123456789;
    emu.regs_mut().rdx = 0;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax as i32, -10000, "EAX: quotient");
}

// ============================================================================
// Special cases
// ============================================================================

#[test]
fn test_idiv_equal_dividend_divisor_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 42 / 42 = 1 remainder 0
    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    emu.regs_mut().rax = 42;
    emu.regs_mut().rbx = 42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFF) as i8, 1, "AL: 42 / 42 = 1");
    assert_eq!(((emu.regs().rax >> 8) & 0xFF) as i8, 0, "AH: remainder = 0");
}

#[test]
fn test_idiv_equal_dividend_divisor_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -42 / -42 = 1 remainder 0
    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    emu.regs_mut().rax = (-42i16) as u16 as u64;
    emu.regs_mut().rbx = (-42i8) as u8 as u64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!((emu.regs().rax & 0xFF) as i8, 1, "AL: -42 / -42 = 1");
    assert_eq!(((emu.regs().rax >> 8) & 0xFF) as i8, 0, "AH: remainder = 0");
}

#[test]
fn test_idiv_remainder_sign_same_as_dividend() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();

    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    emu.regs_mut().rax = 17;
    emu.regs_mut().rbx = 5;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(((emu.regs().rax >> 8) & 0xFF) as i8, 2, "Positive remainder");

    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    emu.regs_mut().rax = (-17i16) as u16 as u64;
    emu.regs_mut().rbx = 5;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(((emu.regs().rax >> 8) & 0xFF) as i8, -2, "Negative remainder (same sign as dividend)");
}

// Note: Division by zero and overflow tests would require exception handling.
