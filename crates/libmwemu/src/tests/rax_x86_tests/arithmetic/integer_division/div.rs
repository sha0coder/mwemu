use crate::*;

// DIV — Unsigned Divide
//
// Opcodes:
// - F6 /6       DIV r/m8      Unsigned divide AX by r/m8
//                             AL := Quotient, AH := Remainder
// - REX + F6 /6 DIV r/m8*     (with REX for extended regs)
// - F7 /6       DIV r/m16     Unsigned divide DX:AX by r/m16
//                             AX := Quotient, DX := Remainder
// - F7 /6       DIV r/m32     Unsigned divide EDX:EAX by r/m32
//                             EAX := Quotient, EDX := Remainder
// - REX.W+F7 /6 DIV r/m64     Unsigned divide RDX:RAX by r/m64
//                             RAX := Quotient, RDX := Remainder
//
// Operation:
//   8-bit:  temp = AX / SRC; AL = quotient, AH = remainder
//   16-bit: temp = DX:AX / SRC; AX = quotient, DX = remainder
//   32-bit: temp = EDX:EAX / SRC; EAX = quotient, EDX = remainder
//   64-bit: temp = RDX:RAX / SRC; RAX = quotient, RDX = remainder
//
// Flags: CF, OF, SF, ZF, AF, PF are undefined (not tested).
//
// Exceptions: #DE if divisor is 0 or quotient too large for destination.
//
// CRITICAL: Non-integral results are truncated towards 0. Remainder is
// always less than divisor. DIV uses double-width dividend.

// ============================================================================
// 8-bit DIV (opcode F6 /6) - Dividend in AX, Result in AL (quotient) and AH (remainder)
// ============================================================================

#[test]
fn test_div_al_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf6, 0xf3, // DIV BL (F6 /6, ModRM=11_110_011)
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
fn test_div_al_with_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    emu.regs_mut().rax = 107; // AX = 107
    emu.regs_mut().rbx = 10;  // BL = 10
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 10, "AL: 107 / 10 = 10");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 7, "AH: 107 % 10 = 7");
}

#[test]
fn test_div_al_by_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    emu.regs_mut().rax = 255; // AX = 255
    emu.regs_mut().rbx = 1;   // BL = 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 255, "AL: 255 / 1 = 255");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0, "AH: remainder = 0");
}

#[test]
fn test_div_al_dividend_less_than_divisor() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    emu.regs_mut().rax = 5;  // AX = 5
    emu.regs_mut().rbx = 10; // BL = 10
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0, "AL: 5 / 10 = 0");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 5, "AH: 5 % 10 = 5");
}

#[test]
fn test_div_al_exact_division() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    emu.regs_mut().rax = 200; // AX = 200
    emu.regs_mut().rbx = 20;  // BL = 20
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 10, "AL: 200 / 20 = 10");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0, "AH: no remainder");
}

#[test]
fn test_div_al_max_dividend() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX can be up to 65535 for 8-bit division
    // 65535 / 256 = 255 remainder 255
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    emu.regs_mut().rax = 0xFFFF; // AX = 65535
    emu.regs_mut().rbx = 0xFF;   // BL = 255
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert!(regs.is_err(), "DIV should fault when quotient overflows 8-bit");
}

#[test]
fn test_div_al_small_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 15 / 4 = 3 remainder 3
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    emu.regs_mut().rax = 15; // AX = 15
    emu.regs_mut().rbx = 4;  // BL = 4
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 3, "AL: 15 / 4 = 3");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 3, "AH: 15 % 4 = 3");
}

#[test]
fn test_div_cl_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xf1, 0xf4]; // DIV CL
    emu.regs_mut().rax = 100; // AX = 100
    emu.regs_mut().rcx = 7;   // CL = 7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 14, "AL: 100 / 7 = 14");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 2, "AH: 100 % 7 = 2");
}

#[test]
fn test_div_preserves_upper_bytes() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    emu.regs_mut().rax = 0xDEADBEEF_CAFE0064; // AX = 100
    emu.regs_mut().rbx = 10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 10, "AL = quotient");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0, "AH = remainder");
    assert_eq!(emu.regs().rax & !0xFFFF, 0xDEADBEEF_CAFE0000, "Upper bytes preserved");
}

// ============================================================================
// 16-bit DIV (opcode F7 /6 with 0x66 prefix) - Dividend in DX:AX
// ============================================================================

#[test]
fn test_div_ax_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0xf7, 0xf3, // DIV BX (66 F7 /6)
        0xf4,
    ];
    emu.regs_mut().rax = 10000; // AX = 10000 (low word of dividend)
    emu.regs_mut().rdx = 0;     // DX = 0 (high word of dividend)
    emu.regs_mut().rbx = 100;   // BX = 100 (divisor)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 100, "AX (quotient): 10000 / 100 = 100");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0, "DX (remainder): 10000 % 100 = 0");
}

#[test]
fn test_div_ax_with_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 12345 / 1000 = 12 remainder 345
    let code = [0x66, 0xf7, 0xf3, 0xf4]; // DIV BX
    emu.regs_mut().rax = 12345;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rbx = 1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 12, "AX: 12345 / 1000 = 12");
    assert_eq!(emu.regs().rdx & 0xFFFF, 345, "DX: 12345 % 1000 = 345");
}

#[test]
fn test_div_ax_with_dx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // DX:AX = 0x00010000 = 65536, divisor = 256
    // 65536 / 256 = 256 remainder 0
    let code = [0x66, 0xf7, 0xf3, 0xf4]; // DIV BX
    emu.regs_mut().rax = 0x0000; // AX (low word)
    emu.regs_mut().rdx = 0x0001; // DX (high word)
    emu.regs_mut().rbx = 256;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 256, "AX: 65536 / 256 = 256");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0, "DX: remainder = 0");
}

#[test]
fn test_div_ax_by_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xf3, 0xf4]; // DIV BX
    emu.regs_mut().rax = 0x1234;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rbx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x1234, "AX: 0x1234 / 1 = 0x1234");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0, "DX: remainder = 0");
}

#[test]
fn test_div_ax_dividend_less_than_divisor() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xf3, 0xf4]; // DIV BX
    emu.regs_mut().rax = 50;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rbx = 100;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0, "AX: 50 / 100 = 0");
    assert_eq!(emu.regs().rdx & 0xFFFF, 50, "DX: 50 % 100 = 50");
}

#[test]
fn test_div_cx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xf1, 0xf4]; // DIV CX
    emu.regs_mut().rax = 20000;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rcx = 300;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 66, "AX: 20000 / 300 = 66");
    assert_eq!(emu.regs().rdx & 0xFFFF, 200, "DX: 20000 % 300 = 200");
}

// ============================================================================
// 32-bit DIV (opcode F7 /6) - Dividend in EDX:EAX
// ============================================================================

#[test]
fn test_div_eax_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf7, 0xf3, // DIV EBX (F7 /6)
        0xf4,
    ];
    emu.regs_mut().rax = 1000000; // EAX = 1000000 (low dword of dividend)
    emu.regs_mut().rdx = 0;       // EDX = 0 (high dword of dividend)
    emu.regs_mut().rbx = 1000;    // EBX = 1000 (divisor)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 1000, "EAX (quotient): 1000000 / 1000 = 1000");
    assert_eq!(emu.regs().rdx, 0, "EDX (remainder): 0");
}

#[test]
fn test_div_eax_with_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 123456789 / 10000 = 12345 remainder 6789
    let code = [0xf7, 0xf3, 0xf4]; // DIV EBX
    emu.regs_mut().rax = 123456789;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rbx = 10000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 12345, "EAX: 123456789 / 10000 = 12345");
    assert_eq!(emu.regs().rdx, 6789, "EDX: 123456789 % 10000 = 6789");
}

#[test]
fn test_div_eax_with_edx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // EDX:EAX = 0x0000000100000000 = 4294967296
    // 4294967296 / 65536 = 65536 remainder 0
    let code = [0xf7, 0xf3, 0xf4]; // DIV EBX
    emu.regs_mut().rax = 0x00000000; // EAX (low dword)
    emu.regs_mut().rdx = 0x00000001; // EDX (high dword)
    emu.regs_mut().rbx = 65536;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 65536, "EAX: quotient");
    assert_eq!(emu.regs().rdx, 0, "EDX: remainder = 0");
}

#[test]
fn test_div_eax_by_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xf3, 0xf4]; // DIV EBX
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rbx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x12345678, "EAX unchanged (divided by 1)");
    assert_eq!(emu.regs().rdx, 0, "EDX: remainder = 0");
}

#[test]
fn test_div_eax_large_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0xFFFFFFFF / 0x10000 = 0xFFFF remainder 0xFFFF
    let code = [0xf7, 0xf3, 0xf4]; // DIV EBX
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rbx = 0x10000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFF, "EAX: quotient");
    assert_eq!(emu.regs().rdx, 0xFFFF, "EDX: remainder");
}

#[test]
fn test_div_ecx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xf1, 0xf4]; // DIV ECX
    emu.regs_mut().rax = 100000000;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rcx = 9999;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 10001, "EAX: 100000000 / 9999 = 10001");
    assert_eq!(emu.regs().rdx, 1, "EDX: 100000000 % 9999 = 1");
}

// ============================================================================
// 64-bit DIV (opcode REX.W + F7 /6) - Dividend in RDX:RAX
// ============================================================================

#[test]
fn test_div_rax_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xf7, 0xf3, // DIV RBX (REX.W F7 /6)
        0xf4,
    ];
    emu.regs_mut().rax = 1000000000000; // RAX (low qword)
    emu.regs_mut().rdx = 0;             // RDX (high qword)
    emu.regs_mut().rbx = 1000000;       // RBX (divisor)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 1000000, "RAX (quotient): 1e12 / 1e6 = 1e6");
    assert_eq!(emu.regs().rdx, 0, "RDX (remainder): 0");
}

#[test]
fn test_div_rax_with_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xf3, 0xf4]; // DIV RBX
    emu.regs_mut().rax = 123456789012345;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rbx = 10000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 12345678, "RAX: quotient");
    assert_eq!(emu.regs().rdx, 9012345, "RDX: remainder");
}

#[test]
fn test_div_rax_with_rdx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RDX:RAX with high qword
    let code = [0x48, 0xf7, 0xf3, 0xf4]; // DIV RBX
    emu.regs_mut().rax = 0x0000000000000000;
    emu.regs_mut().rdx = 0x0000000000000001; // RDX:RAX = 2^64
    emu.regs_mut().rbx = 0x0000000100000000; // divisor = 2^32
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000100000000, "RAX: quotient = 2^32");
    assert_eq!(emu.regs().rdx, 0, "RDX: remainder = 0");
}

#[test]
fn test_div_rax_by_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xf3, 0xf4]; // DIV RBX
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rbx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "RAX unchanged");
    assert_eq!(emu.regs().rdx, 0, "RDX: remainder = 0");
}

#[test]
fn test_div_rax_large_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xf3, 0xf4]; // DIV RBX
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rbx = 0x100000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFF, "RAX: quotient");
    assert_eq!(emu.regs().rdx, 0xFFFFFFFF, "RDX: remainder");
}

#[test]
fn test_div_rcx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xf1, 0xf4]; // DIV RCX
    emu.regs_mut().rax = 987654321098765;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rcx = 123456789;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 987654321098765 / 123456789 = 8000000 remainder 9098765
    assert_eq!(emu.regs().rax, 8000000, "RAX: quotient");
    assert_eq!(emu.regs().rdx, 9098765, "RDX: remainder");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_div_r8b_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x41, 0xf6, 0xf0, // DIV R8B (REX.B F6 /6)
        0xf4,
    ];
    emu.regs_mut().rax = 200; // AX = 200
    emu.regs_mut().r8 = 15;   // R8B = 15
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 13, "AL: 200 / 15 = 13");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 5, "AH: 200 % 15 = 5");
}

#[test]
fn test_div_r9w_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0x41, 0xf7, 0xf1, // DIV R9W
        0xf4,
    ];
    emu.regs_mut().rax = 10000;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().r9 = 123;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 81, "AX: 10000 / 123 = 81");
    assert_eq!(emu.regs().rdx & 0xFFFF, 37, "DX: 10000 % 123 = 37");
}

#[test]
fn test_div_r10d_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x41, 0xf7, 0xf2, // DIV R10D
        0xf4,
    ];
    emu.regs_mut().rax = 1000000;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().r10 = 999;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 1001, "EAX: 1000000 / 999 = 1001");
    assert_eq!(emu.regs().rdx, 1, "EDX: 1000000 % 999 = 1");
}

#[test]
fn test_div_r11_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x49, 0xf7, 0xf3, // DIV R11 (REX.WB F7 /6)
        0xf4,
    ];
    emu.regs_mut().rax = 123456789012;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().r11 = 987654;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 125000, "RAX: quotient");
    assert_eq!(emu.regs().rdx, 39012, "RDX: remainder");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_div_byte_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf6, 0x35, 0xfa, 0x0f, 0x00, 0x00, // DIV BYTE PTR [rip+0x0FFA]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 7);

    emu.regs_mut().rax = 50; // AX = 50

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 7, "AL: 50 / 7 = 7");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 1, "AH: 50 % 7 = 1");
}

#[test]
fn test_div_word_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0xf7, 0x35, 0xf9, 0x0f, 0x00, 0x00, // DIV WORD PTR [rip+0x0FF9]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 256);

    emu.regs_mut().rax = 10000;
    emu.regs_mut().rdx = 0;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 39, "AX: 10000 / 256 = 39");
    assert_eq!(emu.regs().rdx & 0xFFFF, 16, "DX: 10000 % 256 = 16");
}

#[test]
fn test_div_dword_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf7, 0x35, 0xfa, 0x0f, 0x00, 0x00, // DIV DWORD PTR [rip+0x0FFA]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 12345);

    emu.regs_mut().rax = 123456789;
    emu.regs_mut().rdx = 0;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 10000, "EAX: 123456789 / 12345 = 10000");
    assert_eq!(emu.regs().rdx, 6789, "EDX: remainder");
}

#[test]
fn test_div_qword_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xf7, 0x35, 0xf9, 0x0f, 0x00, 0x00, // DIV QWORD PTR [rip+0x0FF9]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 1000000000);

    emu.regs_mut().rax = 1000000000000;
    emu.regs_mut().rdx = 0;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 1000, "RAX: 1e12 / 1e9 = 1000");
    assert_eq!(emu.regs().rdx, 0, "RDX: remainder = 0");
}

// ============================================================================
// Special cases
// ============================================================================

#[test]
fn test_div_truncation_towards_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 17 / 5 = 3 remainder 2 (not 3.4)
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    emu.regs_mut().rax = 17;
    emu.regs_mut().rbx = 5;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 3, "AL: 17 / 5 = 3 (truncated)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 2, "AH: 17 % 5 = 2");
}

#[test]
fn test_div_remainder_always_less_than_divisor() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    emu.regs_mut().rax = 254;
    emu.regs_mut().rbx = 17;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let quotient = emu.regs().rax & 0xFF;
    let remainder = (emu.regs().rax >> 8) & 0xFF;
    let divisor = 17;

    assert_eq!(quotient, 14, "254 / 17 = 14");
    assert_eq!(remainder, 16, "254 % 17 = 16");
    assert!(remainder < divisor, "Remainder must be < divisor");
}

#[test]
fn test_div_equal_dividend_divisor() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    emu.regs_mut().rax = 42;
    emu.regs_mut().rbx = 42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 1, "AL: 42 / 42 = 1");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0, "AH: 42 % 42 = 0");
}

// Note: Division by zero and overflow tests would require exception handling.
// These are omitted as they would cause #DE exceptions in real execution.
