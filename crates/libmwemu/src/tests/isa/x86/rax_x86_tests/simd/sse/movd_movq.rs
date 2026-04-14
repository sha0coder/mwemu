use crate::*;

// MOVD/MOVQ - Move Doubleword/Quadword
// Opcode: 66 0F 6E /r         MOVD xmm, r/m32
//         66 REX.W 0F 6E /r   MOVQ xmm, r/m64
//         66 0F 7E /r         MOVD r/m32, xmm
//         66 REX.W 0F 7E /r   MOVQ r/m64, xmm
//         F3 0F 7E /r         MOVQ xmm, xmm/m64

const DATA_ADDR: u64 = 0x3000;

// MOVD xmm, r32 - Move 32-bit GPR to XMM (zero upper)
#[test]
fn test_movd_xmm0_eax() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x6e, 0xc0, 0xf4]; // MOVD XMM0, EAX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movd_xmm1_ebx() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x6e, 0xcb, 0xf4]; // MOVD XMM1, EBX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movd_xmm2_ecx() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x6e, 0xd1, 0xf4]; // MOVD XMM2, ECX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movd_xmm3_edx() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x6e, 0xda, 0xf4]; // MOVD XMM3, EDX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movd_xmm7_esi() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x6e, 0xfe, 0xf4]; // MOVD XMM7, ESI
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movd_xmm8_r8d() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x6e, 0xc0, 0xf4]; // MOVD XMM8, R8D
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movd_xmm15_r15d() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x6e, 0xff, 0xf4]; // MOVD XMM15, R15D
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// MOVD r32, xmm - Extract low 32 bits to GPR
#[test]
fn test_movd_eax_xmm0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x7e, 0xc0, 0xf4]; // MOVD EAX, XMM0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movd_ebx_xmm1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x7e, 0xcb, 0xf4]; // MOVD EBX, XMM1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movd_ecx_xmm2() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x7e, 0xd1, 0xf4]; // MOVD ECX, XMM2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movd_r8d_xmm8() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x7e, 0xc0, 0xf4]; // MOVD R8D, XMM8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// MOVQ xmm, r64 - Move 64-bit GPR to XMM (zero upper)
#[test]
fn test_movq_xmm0_rax() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x6e, 0xc0, 0xf4]; // MOVQ XMM0, RAX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm1_rbx() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x6e, 0xcb, 0xf4]; // MOVQ XMM1, RBX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm2_rcx() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x6e, 0xd1, 0xf4]; // MOVQ XMM2, RCX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm3_rdx() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x6e, 0xda, 0xf4]; // MOVQ XMM3, RDX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm7_rsi() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x6e, 0xfe, 0xf4]; // MOVQ XMM7, RSI
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm8_r8() {
    let mut emu = emu64();
    let code = [0x66, 0x4d, 0x0f, 0x6e, 0xc0, 0xf4]; // MOVQ XMM8, R8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm15_r15() {
    let mut emu = emu64();
    let code = [0x66, 0x4d, 0x0f, 0x6e, 0xff, 0xf4]; // MOVQ XMM15, R15
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// MOVQ r64, xmm - Extract low 64 bits to GPR
#[test]
fn test_movq_rax_xmm0() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x7e, 0xc0, 0xf4]; // MOVQ RAX, XMM0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_rbx_xmm1() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x7e, 0xcb, 0xf4]; // MOVQ RBX, XMM1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_rcx_xmm2() {
    let mut emu = emu64();
    let code = [0x66, 0x48, 0x0f, 0x7e, 0xd1, 0xf4]; // MOVQ RCX, XMM2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_r8_xmm8() {
    let mut emu = emu64();
    let code = [0x66, 0x4d, 0x0f, 0x7e, 0xc0, 0xf4]; // MOVQ R8, XMM8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_r15_xmm15() {
    let mut emu = emu64();
    let code = [0x66, 0x4d, 0x0f, 0x7e, 0xff, 0xf4]; // MOVQ R15, XMM15
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// MOVD xmm, m32 - Load 32 bits from memory, zero upper
#[test]
fn test_movd_xmm0_mem32() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6e, 0x00, 0xf4]); // MOVD XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let val: u32 = 0x12345678;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_movd_xmm5_mem32() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6e, 0x28, 0xf4]); // MOVD XMM5, [RAX]

    emu.load_code_bytes(&full_code);
    let val: u32 = 0xABCDEF00;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

// MOVD m32, xmm - Store low 32 bits to memory
#[test]
fn test_movd_mem32_xmm0() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x7e, 0x00, 0xf4]); // MOVD [RAX], XMM0

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_movd_mem32_xmm7() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x7e, 0x38, 0xf4]); // MOVD [RAX], XMM7

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

// MOVQ xmm, m64 - Load 64 bits from memory, zero upper
#[test]
fn test_movq_xmm0_mem64() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x48, 0x0f, 0x6e, 0x00, 0xf4]); // MOVQ XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let val: u64 = 0x123456789ABCDEF0;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm3_mem64() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x48, 0x0f, 0x6e, 0x18, 0xf4]); // MOVQ XMM3, [RAX]

    emu.load_code_bytes(&full_code);
    let val: u64 = 0xFEDCBA9876543210;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

// MOVQ m64, xmm - Store low 64 bits to memory
#[test]
fn test_movq_mem64_xmm0() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x48, 0x0f, 0x7e, 0x00, 0xf4]); // MOVQ [RAX], XMM0

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_mem64_xmm6() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x48, 0x0f, 0x7e, 0x30, 0xf4]); // MOVQ [RAX], XMM6

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

// MOVQ xmm, xmm - Copy low 64 bits, zero upper
#[test]
fn test_movq_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x7e, 0xc1, 0xf4]; // MOVQ XMM0, XMM1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x7e, 0xd3, 0xf4]; // MOVQ XMM2, XMM3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x7e, 0xf8, 0xf4]; // MOVQ XMM7, XMM0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm8_xmm15() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x7e, 0xc7, 0xf4]; // MOVQ XMM8, XMM15
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm15_xmm8() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x7e, 0xf8, 0xf4]; // MOVQ XMM15, XMM8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test with displacement addressing
#[test]
fn test_movd_xmm0_mem32_disp() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 8).to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6e, 0x40, 0x08, 0xf4]); // MOVD XMM0, [RAX+8]

    emu.load_code_bytes(&full_code);
    let val: u32 = 0xDEADBEEF;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm1_mem64_disp() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 16).to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x48, 0x0f, 0x6e, 0x48, 0x10, 0xf4]); // MOVQ XMM1, [RAX+16]

    emu.load_code_bytes(&full_code);
    let val: u64 = 0xCAFEBABEDEADBEEF;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

// Test special values
#[test]
fn test_movd_zero() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6e, 0x00, 0xf4]); // MOVD XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let val: u32 = 0;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_movd_all_ones() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6e, 0x00, 0xf4]); // MOVD XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let val: u32 = 0xFFFFFFFF;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_movq_zero() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x48, 0x0f, 0x6e, 0x00, 0xf4]); // MOVQ XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let val: u64 = 0;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_movq_all_ones() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x48, 0x0f, 0x6e, 0x00, 0xf4]); // MOVQ XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let val: u64 = 0xFFFFFFFFFFFFFFFF;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

// Test chained operations
#[test]
fn test_movd_chain() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6e, 0xc0, // MOVD XMM0, EAX
        0x66, 0x0f, 0x6e, 0xcb, // MOVD XMM1, EBX
        0x66, 0x0f, 0x6e, 0xd1, // MOVD XMM2, ECX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_chain() {
    let mut emu = emu64();
    let code = [
        0x66, 0x48, 0x0f, 0x6e, 0xc0, // MOVQ XMM0, RAX
        0x66, 0x48, 0x0f, 0x6e, 0xcb, // MOVQ XMM1, RBX
        0x66, 0x48, 0x0f, 0x6e, 0xd1, // MOVQ XMM2, RCX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movd_roundtrip() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6e, 0xc0, // MOVD XMM0, EAX
        0x66, 0x0f, 0x7e, 0xc3, // MOVD EBX, XMM0
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_roundtrip() {
    let mut emu = emu64();
    let code = [
        0x66, 0x48, 0x0f, 0x6e, 0xc0, // MOVQ XMM0, RAX
        0x66, 0x48, 0x0f, 0x7e, 0xc3, // MOVQ RBX, XMM0
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test high XMM registers with high GPRs
#[test]
fn test_movd_xmm8_r8d_high() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x6e, 0xc0, 0xf4]; // MOVD XMM8, R8D
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movd_xmm9_r9d() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x6e, 0xc9, 0xf4]; // MOVD XMM9, R9D
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movd_xmm14_r14d() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x6e, 0xf6, 0xf4]; // MOVD XMM14, R14D
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm9_r9() {
    let mut emu = emu64();
    let code = [0x66, 0x4d, 0x0f, 0x6e, 0xc9, 0xf4]; // MOVQ XMM9, R9
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm10_r10() {
    let mut emu = emu64();
    let code = [0x66, 0x4d, 0x0f, 0x6e, 0xd2, 0xf4]; // MOVQ XMM10, R10
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm14_r14() {
    let mut emu = emu64();
    let code = [0x66, 0x4d, 0x0f, 0x6e, 0xf6, 0xf4]; // MOVQ XMM14, R14
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test extract from high XMM to high GPR
#[test]
fn test_movd_r9d_xmm9() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x7e, 0xc9, 0xf4]; // MOVD R9D, XMM9
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movd_r12d_xmm12() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x7e, 0xe4, 0xf4]; // MOVD R12D, XMM12
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_r11_xmm11() {
    let mut emu = emu64();
    let code = [0x66, 0x4d, 0x0f, 0x7e, 0xdb, 0xf4]; // MOVQ R11, XMM11
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_r13_xmm13() {
    let mut emu = emu64();
    let code = [0x66, 0x4d, 0x0f, 0x7e, 0xed, 0xf4]; // MOVQ R13, XMM13
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test MOVQ xmm, xmm with high registers
#[test]
fn test_movq_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x7e, 0xd3, 0xf4]; // MOVQ XMM10, XMM11
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x7e, 0xe5, 0xf4]; // MOVQ XMM12, XMM13
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movq_xmm14_xmm1() {
    let mut emu = emu64();
    let code = [0xf3, 0x44, 0x0f, 0x7e, 0xf1, 0xf4]; // MOVQ XMM14, XMM1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
