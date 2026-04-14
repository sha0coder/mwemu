use crate::*;

// MOVNTI - Store Doubleword/Quadword Using Non-Temporal Hint
//
// MOVNTI moves doubleword (32-bit) or quadword (64-bit) integer from general-purpose register
// to memory using non-temporal hint. The non-temporal hint minimizes cache pollution by using
// write combining (WC) protocol.
//
// Use SFENCE or MFENCE for ordering with weakly-ordered memory types.
//
// Opcodes:
// NP 0F C3 /r             MOVNTI m32, r32          - Move doubleword from r32 to m32 using non-temporal hint
// NP REX.W + 0F C3 /r     MOVNTI m64, r64          - Move quadword from r64 to m64 using non-temporal hint

const DEST_ADDR: u64 = 0x4000; // Destination address for testing

// ============================================================================
// MOVNTI Tests - Non-Temporal Store of Integer Values
// ============================================================================

// 32-bit MOVNTI tests
#[test]
fn test_movnti_m32_eax() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], EAX
    let code = [
        0x0f, 0xc3, 0x04, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], EAX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m32_ebx() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], EBX
    let code = [
        0x0f, 0xc3, 0x1c, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], EBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m32_ecx() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], ECX
    let code = [
        0x0f, 0xc3, 0x0c, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], ECX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m32_edx() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], EDX
    let code = [
        0x0f, 0xc3, 0x14, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], EDX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m32_esi() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], ESI
    let code = [
        0x0f, 0xc3, 0x34, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], ESI
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m32_edi() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], EDI
    let code = [
        0x0f, 0xc3, 0x3c, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], EDI
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m32_esp() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], ESP
    let code = [
        0x0f, 0xc3, 0x24, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], ESP
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m32_ebp() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], EBP
    let code = [
        0x0f, 0xc3, 0x2c, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], EBP
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// 64-bit MOVNTI tests
#[test]
fn test_movnti_m64_rax() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], RAX
    let code = [
        0x48, 0x0f, 0xc3, 0x04, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], RAX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m64_rbx() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], RBX
    let code = [
        0x48, 0x0f, 0xc3, 0x1c, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], RBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m64_rcx() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], RCX
    let code = [
        0x48, 0x0f, 0xc3, 0x0c, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], RCX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m64_rdx() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], RDX
    let code = [
        0x48, 0x0f, 0xc3, 0x14, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], RDX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m64_rsi() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], RSI
    let code = [
        0x48, 0x0f, 0xc3, 0x34, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], RSI
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m64_rdi() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], RDI
    let code = [
        0x48, 0x0f, 0xc3, 0x3c, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], RDI
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m64_r8() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], R8
    let code = [
        0x4c, 0x0f, 0xc3, 0x04, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], R8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m64_r9() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], R9
    let code = [
        0x4c, 0x0f, 0xc3, 0x0c, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], R9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m64_r10() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], R10
    let code = [
        0x4c, 0x0f, 0xc3, 0x14, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], R10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m64_r11() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], R11
    let code = [
        0x4c, 0x0f, 0xc3, 0x1c, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], R11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m64_r12() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], R12
    let code = [
        0x4c, 0x0f, 0xc3, 0x24, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], R12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m64_r13() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], R13
    let code = [
        0x4c, 0x0f, 0xc3, 0x2c, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], R13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m64_r14() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], R14
    let code = [
        0x4c, 0x0f, 0xc3, 0x34, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], R14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_m64_r15() {
    let mut emu = emu64();
    // MOVNTI [DEST_ADDR], R15
    let code = [
        0x4c, 0x0f, 0xc3, 0x3c, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], R15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_32bit_zero() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xc3, 0x04, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], EAX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_32bit_max() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xc3, 0x1c, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], EBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_64bit_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0x0f, 0xc3, 0x04, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], RAX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_64bit_max() {
    let mut emu = emu64();
    let code = [
        0x48, 0x0f, 0xc3, 0x1c, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], RBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_non_temporal_hint() {
    let mut emu = emu64();
    let code = [
        0x48, 0x0f, 0xc3, 0x0c, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], RCX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_write_combining() {
    let mut emu = emu64();
    let code = [
        0x48, 0x0f, 0xc3, 0x14, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], RDX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_sequential_32bit() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xc3, 0x34, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], ESI
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movnti_sequential_64bit() {
    let mut emu = emu64();
    let code = [
        0x48, 0x0f, 0xc3, 0x34, 0x25, 0x00, 0x40, 0x00, 0x00, // MOVNTI [0x4000], RSI
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
