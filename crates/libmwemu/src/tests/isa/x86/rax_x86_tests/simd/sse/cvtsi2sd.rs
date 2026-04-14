use crate::*;

// CVTSI2SD - Convert Signed Integer to Scalar Double Precision
// Opcode: F2 0F 2A /r    CVTSI2SD xmm1, r/m32
//         F2 REX.W 0F 2A /r    CVTSI2SD xmm1, r/m64

const DATA_ADDR: u64 = 0x3000;

// Register Tests
#[test]
fn test_cvtsi2sd_xmm0_eax() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x2a, 0xc0, 0xf4]; // CVTSI2SD XMM0, EAX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_xmm1_ebx() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x2a, 0xcb, 0xf4]; // CVTSI2SD XMM1, EBX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_xmm2_ecx() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x2a, 0xd1, 0xf4]; // CVTSI2SD XMM2, ECX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_xmm3_edx() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x2a, 0xda, 0xf4]; // CVTSI2SD XMM3, EDX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_xmm7_esi() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x2a, 0xfe, 0xf4]; // CVTSI2SD XMM7, ESI
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// 64-bit Tests
#[test]
fn test_cvtsi2sd_xmm0_rax_64() {
    let mut emu = emu64();
    let code = [0xf2, 0x48, 0x0f, 0x2a, 0xc0, 0xf4]; // CVTSI2SD XMM0, RAX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_xmm1_rbx_64() {
    let mut emu = emu64();
    let code = [0xf2, 0x48, 0x0f, 0x2a, 0xcb, 0xf4]; // CVTSI2SD XMM1, RBX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_xmm8_r9_64() {
    let mut emu = emu64();
    let code = [0xf2, 0x4d, 0x0f, 0x2a, 0xc1, 0xf4]; // CVTSI2SD XMM8, R9
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Memory Tests
#[test]
fn test_cvtsi2sd_mem32() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 42;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_mem64() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x48, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i64 = 1234567890;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

// Special Values
#[test]
fn test_cvtsi2sd_zero() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 0;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_max_i32() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = i32::MAX;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_min_i32() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = i32::MIN;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_max_i64() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x48, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i64 = i64::MAX;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_min_i64() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x48, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i64 = i64::MIN;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_positive_small() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 1;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_negative_small() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = -1;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_positive_medium() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 1000000;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_negative_medium() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = -1000000;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

// Powers of 2
#[test]
fn test_cvtsi2sd_power_of_2() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 1024;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_negative_power_of_2() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = -1024;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

// Precision Tests (f64 has 53-bit mantissa, can hold all 32-bit and most 64-bit ints exactly)
#[test]
fn test_cvtsi2sd_large_exact() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 16777217; // Exactly representable in f64
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_64bit_large() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x48, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i64 = 1000000000000;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_64bit_very_large() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x48, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i64 = 9007199254740993; // 2^53 + 1, may lose precision
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

// Misc edge cases
#[test]
fn test_cvtsi2sd_100() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 100;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_minus_100() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = -100;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_12345() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 12345;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_minus_67890() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = -67890;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

// Addressing modes
#[test]
fn test_cvtsi2sd_displacement() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 16).to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2a, 0x40, 0x10, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 999;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_multiple() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x2a, 0x00, // CVTSI2SD XMM0, [RAX]
        0xf2, 0x0f, 0x2a, 0x08, // CVTSI2SD XMM1, [RAX]
        0xf2, 0x0f, 0x2a, 0x10, // CVTSI2SD XMM2, [RAX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 777;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_xmm15() {
    let mut emu = emu64();
    let code = [0xf2, 0x44, 0x0f, 0x2a, 0xf8, 0xf4]; // CVTSI2SD XMM15, EAX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_r8d() {
    let mut emu = emu64();
    let code = [0xf2, 0x41, 0x0f, 0x2a, 0xc0, 0xf4]; // CVTSI2SD XMM0, R8D
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_boundary_4503599627370496() {
    let mut emu = emu64();
    // 2^52 - exactly representable
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x48, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i64 = 4503599627370496;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2sd_boundary_9007199254740992() {
    let mut emu = emu64();
    // 2^53 - largest exactly representable
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x48, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i64 = 9007199254740992;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}
