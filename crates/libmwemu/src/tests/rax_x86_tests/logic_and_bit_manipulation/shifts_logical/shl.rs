// SHL (Shift Logical Left) instruction tests
// SAL and SHL are the same instruction (same opcodes)
//
// Opcodes:
// D0 /4       SHL r/m8, 1
// D2 /4       SHL r/m8, CL
// C0 /4 ib    SHL r/m8, imm8
// D1 /4       SHL r/m16, 1
// D3 /4       SHL r/m16, CL
// C1 /4 ib    SHL r/m16, imm8
// D1 /4       SHL r/m32, 1
// D3 /4       SHL r/m32, CL
// C1 /4 ib    SHL r/m32, imm8
// REX.W + D1 /4    SHL r/m64, 1
// REX.W + D3 /4    SHL r/m64, CL
// REX.W + C1 /4 ib SHL r/m64, imm8
//
// Flags:
// - CF: Last bit shifted out
// - OF: Only for 1-bit shifts (MSB of result XOR CF)
// - SF, ZF, PF: Set according to result
// - AF: Undefined for non-zero count
// - Count is 0: No flags affected


use crate::*;

// ============================================================================
// 8-bit SHL tests
// ============================================================================

#[test]
fn test_shl_al_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL AL, 1 (opcode D0 /4)
    let code = [
        0xd0, 0xe0, // SHL AL, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x42; // 0100_0010
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x84, "AL: 0x42 << 1 = 0x84");
    assert!(!emu.flags().f_cf, "CF should be clear (MSB was 0)");
    assert!(emu.flags().f_of, "OF: MSB XOR CF = 1 XOR 0 = 1");
    assert!(emu.flags().f_sf, "SF should be set (bit 7 = 1)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_shl_al_1_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL AL, 1 with MSB set
    let code = [
        0xd0, 0xe0, // SHL AL, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x81; // 1000_0001
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x02, "AL: 0x81 << 1 = 0x02");
    assert!(emu.flags().f_cf, "CF should be set (MSB was 1)");
    assert!(emu.flags().f_of, "OF: MSB XOR CF = 0 XOR 1 = 1");
    assert!(!emu.flags().f_sf, "SF should be clear");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_shl_al_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL AL, CL (opcode D2 /4)
    let code = [
        0xd2, 0xe0, // SHL AL, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x01;
    emu.regs_mut().rcx = 0x07; // Shift by 7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x80, "AL: 0x01 << 7 = 0x80");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
    assert!(emu.flags().f_sf, "SF should be set (bit 7 = 1)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_shl_al_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL AL, imm8 (opcode C0 /4 ib)
    let code = [
        0xc0, 0xe0, 0x03, // SHL AL, 3
        0xf4,
    ];
    emu.regs_mut().rax = 0x11; // 0001_0001
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x88, "AL: 0x11 << 3 = 0x88");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
    assert!(emu.flags().f_sf, "SF should be set");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_shl_al_to_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc0, 0xe0, 0x08, // SHL AL, 8
        0xf4,
    ];
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL: all bits shifted out");
    assert!(emu.flags().f_zf, "ZF should be set (result is zero)");
    assert!(!emu.flags().f_sf, "SF should be clear");
}

#[test]
fn test_shl_count_masked_8bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd2, 0xe0, // SHL AL, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x11;
    emu.regs_mut().rcx = 0x23; // 35 & 0x1F = 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x88, "AL: 0x11 << 3 = 0x88 (count masked)");
}

#[test]
fn test_shl_count_zero_preserves_flags() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc0, 0xe0, 0x00, // SHL AL, 0
        0xf4,
    ];
    emu.regs_mut().rax = 0x42;
    emu.flags_mut().load(0x2 | flags::F_CF | flags::F_ZF | flags::F_OF);
    let initial_flags = emu.flags().dump();
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL unchanged");
    assert_eq!(emu.flags().dump(), initial_flags, "Flags unchanged when count is 0");
}

// ============================================================================
// 16-bit SHL tests
// ============================================================================

#[test]
fn test_shl_ax_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL AX, 1 (opcode 66 D1 /4)
    let code = [
        0x66, 0xd1, 0xe0, // SHL AX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x4321;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x8642, "AX: 0x4321 << 1 = 0x8642");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(emu.flags().f_of, "OF: MSB XOR CF");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_shl_ax_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL AX, CL (opcode 66 D3 /4)
    let code = [
        0x66, 0xd3, 0xe0, // SHL AX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x0001;
    emu.regs_mut().rcx = 0x0F; // Shift by 15
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x8000, "AX: 0x0001 << 15 = 0x8000");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_shl_ax_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL AX, imm8 (opcode 66 C1 /4 ib)
    let code = [
        0x66, 0xc1, 0xe0, 0x04, // SHL AX, 4
        0xf4,
    ];
    emu.regs_mut().rax = 0x0123;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x1230, "AX: 0x0123 << 4 = 0x1230");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_shl_ax_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0xd1, 0xe0, // SHL AX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x8001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0002, "AX: 0x8001 << 1 = 0x0002");
    assert!(emu.flags().f_cf, "CF should be set (MSB was 1)");
    assert!(emu.flags().f_of, "OF: MSB XOR CF = 0 XOR 1 = 1");
}

// ============================================================================
// 32-bit SHL tests
// ============================================================================

#[test]
fn test_shl_eax_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL EAX, 1 (opcode D1 /4)
    let code = [
        0xd1, 0xe0, // SHL EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x2468ACF0, "EAX: 0x12345678 << 1 = 0x2468ACF0");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_of, "OF should be clear");
}

#[test]
fn test_shl_eax_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL EAX, CL (opcode D3 /4)
    let code = [
        0xd3, 0xe0, // SHL EAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000001;
    emu.regs_mut().rcx = 0x1F; // Shift by 31
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x80000000, "EAX: 0x00000001 << 31 = 0x80000000");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_shl_eax_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL EAX, imm8 (opcode C1 /4 ib)
    let code = [
        0xc1, 0xe0, 0x08, // SHL EAX, 8
        0xf4,
    ];
    emu.regs_mut().rax = 0x00123456;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x12345600, "EAX: 0x00123456 << 8 = 0x12345600");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_shl_eax_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd1, 0xe0, // SHL EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x80000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000002, "EAX: 0x80000001 << 1 = 0x00000002");
    assert!(emu.flags().f_cf, "CF should be set (MSB was 1)");
    assert!(emu.flags().f_of, "OF: MSB XOR CF = 0 XOR 1 = 1");
}

#[test]
fn test_shl_count_masked_32bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd3, 0xe0, // SHL EAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000001;
    emu.regs_mut().rcx = 0x3F; // 63 & 0x1F = 31
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x80000000, "EAX: 0x00000001 << 31 = 0x80000000 (count masked)");
}

// ============================================================================
// 64-bit SHL tests
// ============================================================================

#[test]
fn test_shl_rax_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL RAX, 1 (opcode 48 D1 /4)
    let code = [
        0x48, 0xd1, 0xe0, // SHL RAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x2468ACF13579BDE0, "RAX: 0x123456789ABCDEF0 << 1");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_of, "OF should be clear");
}

#[test]
fn test_shl_rax_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL RAX, CL (opcode 48 D3 /4)
    let code = [
        0x48, 0xd3, 0xe0, // SHL RAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x0000000000000001;
    emu.regs_mut().rcx = 0x3F; // Shift by 63
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x8000000000000000, "RAX: 0x0000000000000001 << 63");
    assert!(!emu.flags().f_cf, "CF: last bit shifted out was 0");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_shl_rax_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL RAX, imm8 (opcode 48 C1 /4 ib)
    let code = [
        0x48, 0xc1, 0xe0, 0x10, // SHL RAX, 16
        0xf4,
    ];
    emu.regs_mut().rax = 0x0000123456789ABC;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABC0000, "RAX: 0x0000123456789ABC << 16");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_shl_rax_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xd1, 0xe0, // SHL RAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x8000000000000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000002, "RAX: 0x8000000000000001 << 1");
    assert!(emu.flags().f_cf, "CF should be set (MSB was 1)");
    assert!(emu.flags().f_of, "OF: MSB XOR CF = 0 XOR 1 = 1");
}

#[test]
fn test_shl_count_masked_64bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xd3, 0xe0, // SHL RAX, CL
        0xf4,
    ];
    emu.regs_mut().rax = 0x0000000000000001;
    emu.regs_mut().rcx = 0x7F; // 127 & 0x3F = 63
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x8000000000000000, "RAX: 0x0000000000000001 << 63 (count masked to 6 bits)");
}

// ============================================================================
// Extended register tests (R8-R15)
// ============================================================================

#[test]
fn test_shl_r8b_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL R8B, 1
    let code = [
        0x41, 0xd0, 0xe0, // SHL R8B, 1
        0xf4,
    ];
    emu.regs_mut().r8 = 0x55; // 0101_0101
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0xAA, "R8B: 0x55 << 1 = 0xAA");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(emu.flags().f_of, "OF: MSB XOR CF");
}

#[test]
fn test_shl_r10w_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL R10W, CL
    let code = [
        0x66, 0x41, 0xd3, 0xe2, // SHL R10W, CL
        0xf4,
    ];
    emu.regs_mut().r10 = 0x1234;
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10 & 0xFFFF, 0x2340, "R10W: 0x1234 << 4 = 0x2340");
}

#[test]
fn test_shl_r12d_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL R12D, imm8
    let code = [
        0x41, 0xc1, 0xe4, 0x08, // SHL R12D, 8
        0xf4,
    ];
    emu.regs_mut().r12 = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r12 & 0xFFFFFFFF, 0x34567800, "R12D: 0x12345678 << 8 = 0x34567800");
}

#[test]
fn test_shl_r15_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL R15, 1
    let code = [
        0x49, 0xd1, 0xe7, // SHL R15, 1
        0xf4,
    ];
    emu.regs_mut().r15 = 0x0123456789ABCDEF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 0x02468ACF13579BDE, "R15: 0x0123456789ABCDEF << 1");
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_shl_byte_ptr_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL byte ptr [DATA_ADDR], 1
    let code = [
        0xd0, 0x24, 0x25, // SHL byte ptr [DATA_ADDR], 1
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0x42);

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0x84, "Memory: 0x42 << 1 = 0x84");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_shl_word_ptr_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL word ptr [DATA_ADDR], CL
    let code = [
        0x66, 0xd3, 0x24, 0x25, // SHL word ptr [DATA_ADDR], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    emu.regs_mut().rcx = 0x04;
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 0x1234);

    emu.run(None).unwrap();
    let result = emu.maps.read_word(DATA_ADDR).unwrap();

    assert_eq!(result, 0x2340, "Memory: 0x1234 << 4 = 0x2340");
}

#[test]
fn test_shl_dword_ptr_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL dword ptr [DATA_ADDR], imm8
    let code = [
        0xc1, 0x24, 0x25, // SHL dword ptr [DATA_ADDR], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x08, // imm8 = 8
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x12345678);

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x34567800, "Memory: 0x12345678 << 8 = 0x34567800");
}

#[test]
fn test_shl_qword_ptr_cl() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL qword ptr [DATA_ADDR], CL
    let code = [
        0x48, 0xd3, 0x24, 0x25, // SHL qword ptr [DATA_ADDR], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    emu.regs_mut().rcx = 0x10;
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x123456789ABCDEF0);

    emu.run(None).unwrap();
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();

    assert_eq!(result, 0x56789ABCDEF00000, "Memory: 0x123456789ABCDEF0 << 16");
}

// ============================================================================
// Practical use cases and edge cases
// ============================================================================

#[test]
fn test_shl_multiply_by_power_of_2() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHL can multiply by powers of 2
    // 5 * 16 = 5 << 4 = 80
    let code = [
        0xc1, 0xe0, 0x04, // SHL EAX, 4
        0xf4,
    ];
    emu.regs_mut().rax = 5;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 80, "EAX: 5 * 16 = 80");
}

#[test]
fn test_shl_align_to_page_boundary() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xc1, 0xe8, 0x0C, // SHR RAX, 12
        0x48, 0xc1, 0xe0, 0x0C, // SHL RAX, 12
        0xf4,
    ];
    emu.regs_mut().rax = 0x123456789ABCDEF7; // Not aligned
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABCD000, "RAX aligned to 4KB boundary");
}

#[test]
fn test_shl_extract_high_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xc1, 0xe0, 0x20, // SHL RAX, 32 (move low 32 bits to high)
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000000FFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFF00000000, "RAX: low 32 bits moved to high");
}

#[test]
fn test_shl_overflow_flag_1bit_same_sign() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // OF is clear when top two bits are same before shift
    let code = [
        0xd1, 0xe0, // SHL EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x40000000; // 01000000...
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x80000000, "EAX: 0x40000000 << 1 = 0x80000000");
    assert!(!emu.flags().f_cf, "CF: bit shifted out was 0");
    assert!(emu.flags().f_of, "OF: MSB(result) XOR CF = 1 XOR 0 = 1");
}

#[test]
fn test_shl_overflow_flag_1bit_different_sign() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // OF is set when top two bits differ before shift
    let code = [
        0xd1, 0xe0, // SHL EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0xC0000000; // 11000000...
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x80000000, "EAX: 0xC0000000 << 1 = 0x80000000");
    assert!(emu.flags().f_cf, "CF: bit shifted out was 1");
    assert!(!emu.flags().f_of, "OF: MSB(result) XOR CF = 1 XOR 1 = 0");
    // OF = old_MSB XOR new_MSB = 1 XOR 1 = 0? Or is it MSB XOR CF?
}

#[test]
fn test_shl_parity_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // PF is set if low byte has even number of 1 bits
    let code = [
        0xd1, 0xe0, // SHL EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x01; // 0000_0001
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x02, "EAX: 0x01 << 1 = 0x02");
    // 0x02 = 0000_0010, one 1 bit (odd), so PF should be clear
    assert!(!emu.flags().f_pf, "PF should be clear (odd parity)");
}

#[test]
fn test_shl_chained_shifts() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd1, 0xe0, // SHL EAX, 1
        0xd1, 0xe0, // SHL EAX, 1
        0xd1, 0xe0, // SHL EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000005; // 5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000028, "EAX: 5 << 3 = 40");
}

#[test]
fn test_shl_all_ones() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xd1, 0xe0, // SHL EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFE, "EAX: 0xFFFFFFFF << 1 = 0xFFFFFFFE");
    assert!(emu.flags().f_cf, "CF: MSB was 1");
    assert!(emu.flags().f_sf, "SF: result is negative");
    assert!(!emu.flags().f_zf, "ZF: result is not zero");
}
